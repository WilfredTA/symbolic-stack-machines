use std::{
    num::TryFromIntError,
    ops::{Add, Sub},
};

use crate::{instructions::bitwise::Binary, memory::symbolic_concrete_index::MemVal};

type Wraps = i128;

// TODO COPY

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