use std::{
    num::TryFromIntError,
    ops::{Add, Not, Sub},
};

use z3::ast::Ast;

use crate::{
    instructions::{bitwise::Binary, sym::Constrain},
    machine_eq::MachineEq,
    memory::symbolic_concrete_index::MemVal,
    solvers::z3::Z3Constraint,
};

pub type Wraps = i64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolicInt {
    C(Wraps),
    S(Inner),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Inner {
    Sym(String),
    Add(Box<SymbolicInt>, Box<SymbolicInt>),
    Sub(Box<SymbolicInt>, Box<SymbolicInt>),
    Ite(EqCheck, Box<SymbolicInt>, Box<SymbolicInt>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EqCheck {
    l: Box<SymbolicInt>,
    r: Box<SymbolicInt>,
}

#[derive(Clone, Debug)]
pub enum SymbolicIntConstraint {
    Eq(SymbolicInt, SymbolicInt),
    NotEq(SymbolicInt, SymbolicInt),
}

impl Z3Constraint for SymbolicIntConstraint {
    fn z3_constraint<'ctx>(
        &self,
        ctx: &'ctx z3::Context,
        solve: &mut Vec<(String, z3::ast::Int<'ctx>)>,
    ) -> z3::ast::Bool<'ctx> {
        match self {
            SymbolicIntConstraint::Eq(l, r) => l.z3_int(ctx, solve)._eq(&r.z3_int(ctx, solve)),
            SymbolicIntConstraint::NotEq(l, r) => {
                l.z3_int(ctx, solve)._eq(&r.z3_int(ctx, solve)).not()
            }
        }
    }
}

impl SymbolicInt {
    pub fn z3_int<'ctx>(
        &self,
        ctx: &'ctx z3::Context,
        solve: &mut Vec<(String, z3::ast::Int<'ctx>)>,
    ) -> z3::ast::Int<'ctx> {
        match self {
            SymbolicInt::C(i) => z3::ast::Int::from_i64(ctx, *i as i64),
            SymbolicInt::S(i) => i.z3_int(ctx, solve),
        }
    }
}

impl Inner {
    pub fn z3_int<'ctx>(
        &self,
        ctx: &'ctx z3::Context,
        solve: &mut Vec<(String, z3::ast::Int<'ctx>)>,
    ) -> z3::ast::Int<'ctx> {
        match self {
            Inner::Sym(name) => {
                let rv = z3::ast::Int::new_const(ctx, z3::Symbol::from(name.as_str()));
                // TODO(will) - ideally, we would be able to store a direct reference 
                // for both and wouldn't have to clone
                solve.push((name.clone(), rv.clone()));
                rv
            }
            Inner::Add(l, r) => l.z3_int(ctx, solve).add(r.z3_int(ctx, solve)),
            Inner::Sub(l, r) => l.z3_int(ctx, solve).sub(r.z3_int(ctx, solve)),
            Inner::Ite(pred, then, xelse) => pred
                .z3_bool(ctx, solve)
                .ite(&then.z3_int(ctx, solve), &xelse.z3_int(ctx, solve)),
        }
    }
}

impl EqCheck {
    pub fn z3_bool<'ctx>(
        &self,
        ctx: &'ctx z3::Context,
        solve: &mut Vec<(String, z3::ast::Int<'ctx>)>,
    ) -> z3::ast::Bool<'ctx> {
        self.l.z3_int(ctx, solve)._eq(&self.r.z3_int(ctx, solve))
    }
}

impl Constrain for SymbolicInt {
    type Constraint = SymbolicIntConstraint;

    fn assert_eq(self, other: SymbolicInt) -> SymbolicIntConstraint {
        SymbolicIntConstraint::Eq(self, other)
    }

    fn assert_not_eq(self, other: SymbolicInt) -> SymbolicIntConstraint {
        SymbolicIntConstraint::NotEq(self, other)
    }
}

pub enum MachineEqPred {
    C(bool),
    S(EqCheck),
}

impl MachineEq for SymbolicInt {
    type Pred = MachineEqPred;

    fn machine_eq(&self, other: &Self) -> Self::Pred {
        match (self, other) {
            (SymbolicInt::C(l), SymbolicInt::C(r)) => MachineEqPred::C(l == r),
            (SymbolicInt::C(l), SymbolicInt::S(r)) => MachineEqPred::S(EqCheck {
                l: C(l.clone()),
                r: r.clone().into(),
            }),
            (SymbolicInt::S(l), SymbolicInt::C(r)) => MachineEqPred::S(EqCheck {
                l: l.clone().into(),
                r: C(r.clone()),
            }),
            (SymbolicInt::S(l), SymbolicInt::S(r)) => MachineEqPred::S(EqCheck {
                l: l.clone().into(),
                r: r.clone().into(),
            }),
        }
    }

    fn machine_ite(p: Self::Pred, then: Self, xelse: Self) -> Self {
        match p {
            MachineEqPred::C(p) => {
                if p {
                    then
                } else {
                    xelse
                }
            }
            MachineEqPred::S(p) => Inner::Ite(p, then.into(), xelse.into()).into(),
        }
    }
}

impl Into<SymbolicInt> for Inner {
    fn into(self) -> SymbolicInt {
        SymbolicInt::S(self)
    }
}

impl Into<Box<SymbolicInt>> for Inner {
    fn into(self) -> Box<SymbolicInt> {
        Box::new(self.into())
    }
}

impl MemVal for SymbolicInt {}

impl From<Wraps> for SymbolicInt {
    fn from(x: Wraps) -> Self {
        SymbolicInt::C(x)
    }
}

impl Default for SymbolicInt {
    fn default() -> Self {
        SymbolicInt::C(0)
    }
}

pub fn C(x: Wraps) -> Box<SymbolicInt> {
    SymbolicInt::from(x).into()
}

impl Add for SymbolicInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SymbolicInt::C(l), SymbolicInt::C(r)) => (l + r).into(),
            (SymbolicInt::C(l), SymbolicInt::S(r)) => Inner::Add(C(l), r.into()).into(),
            (SymbolicInt::S(l), SymbolicInt::C(r)) => Inner::Add(l.into(), C(r)).into(),
            (SymbolicInt::S(l), SymbolicInt::S(r)) => Inner::Add(l.into(), r.into()).into(),
        }
    }
}

impl Sub for SymbolicInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SymbolicInt::C(l), SymbolicInt::C(r)) => (l - r).into(),
            (SymbolicInt::C(l), SymbolicInt::S(r)) => Inner::Sub(C(l), r.into()).into(),
            (SymbolicInt::S(l), SymbolicInt::C(r)) => Inner::Sub(l.into(), C(r)).into(),
            (SymbolicInt::S(l), SymbolicInt::S(r)) => Inner::Sub(l.into(), r.into()).into(),
        }
    }
}

impl Binary for SymbolicInt {
    fn one() -> Self {
        Self::from(1)
    }
}

impl TryInto<usize> for SymbolicInt {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<usize, Self::Error> {
        match self {
            SymbolicInt::C(x) => x.try_into(),
            _ => panic!("cannot convert symbolic int into usize"),
        }
    }
}

pub fn SYM(s: String) -> SymbolicInt {
    SymbolicInt::S(Inner::Sym(s))
}
