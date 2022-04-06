use std::{
    num::TryFromIntError,
    ops::{Add, Sub},
};

use crate::{
    instructions::bitwise::Binary, machine_eq::MachineEq, memory::symbolic_concrete_index::MemVal,
};

pub type Wraps = i128;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolicInt {
    C(Wraps),
    S(Inner),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Inner {
    Sym,
    Add(Box<SymbolicInt>, Box<SymbolicInt>),
    Sub(Box<SymbolicInt>, Box<SymbolicInt>),
    Eq(Box<SymbolicInt>, Box<SymbolicInt>),
    Ite(Box<SymbolicInt>, Box<SymbolicInt>, Box<SymbolicInt>),
}

impl MachineEq for SymbolicInt {
    fn machine_eq(&self, other: &Self) -> Self {
        match (self, other) {
            (SymbolicInt::C(l), SymbolicInt::C(r)) => SymbolicInt::C((l == r) as Wraps),
            (SymbolicInt::C(l), SymbolicInt::S(r)) => {
                Inner::Eq(C(l.clone()), r.clone().into()).into()
            }
            (SymbolicInt::S(l), SymbolicInt::C(r)) => {
                Inner::Eq(l.clone().into(), C(r.clone())).into()
            }
            (SymbolicInt::S(l), SymbolicInt::S(r)) => {
                Inner::Eq(l.clone().into(), r.clone().into()).into()
            }
        }
    }

    fn machine_ite(self, then: Self, xelse: Self) -> Self {
        match self {
            SymbolicInt::C(x) => {
                if x != 0 {
                    then
                } else {
                    xelse
                }
            }
            SymbolicInt::S(x) => Inner::Ite(x.into(), then.into(), xelse.into()).into(),
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

pub fn SYM() -> SymbolicInt {
    SymbolicInt::S(Inner::Sym)
}
