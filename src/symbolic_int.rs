use std::{ops::{Add, Sub}, num::TryFromIntError};

use crate::{memory::symbolic_concrete_index::MemVal, instructions::bitwise::Binary};

type Wraps = i128;

// TODO COPY

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolicInt {
    C(Wraps),
    S(Box<SymbolicInt>)
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

impl Add for SymbolicInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SymbolicInt::C(x), SymbolicInt::C(y)) => SymbolicInt::C(x + y),
            (SymbolicInt::C(_), SymbolicInt::S(_)) => todo!(),
            (SymbolicInt::S(_), SymbolicInt::C(_)) => todo!(),
            (SymbolicInt::S(_), SymbolicInt::S(_)) => todo!(),
        }
    }
}

impl Sub for SymbolicInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SymbolicInt::C(x), SymbolicInt::C(y)) => SymbolicInt::C(x - y),
            (SymbolicInt::C(_), SymbolicInt::S(_)) => todo!(),
            (SymbolicInt::S(_), SymbolicInt::C(_)) => todo!(),
            (SymbolicInt::S(_), SymbolicInt::S(_)) => todo!(),
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
            _ => panic!("cannot convert symbolic int into usize")
        }
    }
}
