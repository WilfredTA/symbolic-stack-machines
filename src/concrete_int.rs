use std::{ops::{Add, Sub}, num::TryFromIntError};

use crate::{instructions::bitwise::Binary, memory::symbolic_concrete_index::MemVal};

pub type Wraps = i128;

// TODO(will) -- is deriving both Clone and Copy a code smell?

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct ConcreteInt(Wraps);

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
