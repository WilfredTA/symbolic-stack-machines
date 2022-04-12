use crate::{memory::{SymbolicIndexIndexVal, SymbolicIndexMemVal, ConcreteIndexMemVal}, instructions::Binary, vals::MachineEq};

use std::{
    num::TryFromIntError,
    ops::{Add, Sub},
};

pub type Wraps = i64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolicInt {
    // Concrete
    C(Wraps),
    // Symbolic
    S(Inner),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Inner {
    Sym(String),
    Add(Box<SymbolicInt>, Box<SymbolicInt>),
    Sub(Box<SymbolicInt>, Box<SymbolicInt>),
    Ite(EqCheck, Box<SymbolicInt>, Box<SymbolicInt>),
    // The result of a read from a series of writes
    RW(Box<(Vec<(SymbolicInt, SymbolicInt)>, SymbolicInt)>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EqCheck {
    pub l: Box<SymbolicInt>,
    pub r: Box<SymbolicInt>,
}

#[allow(non_snake_case)]
pub fn SYM(s: &str) -> SymbolicInt {
    SymbolicInt::S(Inner::Sym(s.to_string()))
}

#[allow(non_snake_case)]
pub fn C(x: Wraps) -> Box<SymbolicInt> {
    SymbolicInt::from(x).into()
}

impl SymbolicIndexIndexVal for SymbolicInt {}
impl SymbolicIndexMemVal<SymbolicInt> for SymbolicInt {
    fn read(writes: Vec<(SymbolicInt, Self)>, read: SymbolicInt) -> Self {
        SymbolicInt::S(Inner::RW(Box::new((writes, read))))
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

impl TryInto<usize> for SymbolicInt {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<usize, Self::Error> {
        match self {
            SymbolicInt::C(x) => x.try_into(),
            _ => panic!("cannot convert symbolic int into usize"),
        }
    }
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

impl ConcreteIndexMemVal for SymbolicInt { }

impl Binary for SymbolicInt {
    fn one() -> Self {
        1.into()
    }
}
