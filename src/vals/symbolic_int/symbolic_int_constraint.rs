use std::ops::Not;

use crate::instructions::Constrain;

use super::SymbolicInt;

#[derive(Clone, Debug)]
pub enum SymbolicIntConstraint {
    Eq(SymbolicInt, SymbolicInt),
    Not(Box<SymbolicIntConstraint>),
}

impl Not for SymbolicIntConstraint {
    type Output = Self;

    fn not(self) -> Self::Output {
        SymbolicIntConstraint::Not(Box::new(self))
    }
}

impl Constrain for SymbolicInt {
    type Constraint = SymbolicIntConstraint;

    fn assert_eq(self, other: Self) -> Self::Constraint {
        SymbolicIntConstraint::Eq(self, other)
    }
}
