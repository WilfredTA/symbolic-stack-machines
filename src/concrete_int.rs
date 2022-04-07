use std::{
    num::TryFromIntError,
    ops::{Add, Sub},
};

use crate::{
    instructions::bitwise::Binary, machine_eq::MachineEq, memory::symbolic_concrete_index::MemVal,
};

pub type Wraps = i128;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConcreteInt(Wraps);

impl MachineEq for ConcreteInt {
    type Pred = bool;

    fn machine_eq(&self, other: &Self) -> Self::Pred {
        self.0 == other.0
    }

    fn machine_ite(pred: Self::Pred, then: Self, xelse: Self) -> Self {
        if pred {
            then
        } else {
            xelse
        }
    }
}

impl MemVal for ConcreteInt {}

impl From<Wraps> for ConcreteInt {
    fn from(x: Wraps) -> Self {
        ConcreteInt(x)
    }
}

impl Default for ConcreteInt {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Add for ConcreteInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self.0 + rhs.0).into()
    }
}

impl Sub for ConcreteInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.0 - rhs.0).into()
    }
}

impl Binary for ConcreteInt {
    fn one() -> Self {
        Self::from(1)
    }
}

impl TryInto<usize> for ConcreteInt {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<usize, Self::Error> {
        self.0.try_into()
    }
}
