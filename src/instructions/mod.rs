pub mod error;

mod concrete;
mod symbolic;
mod hybrid;
mod impls;

use std::ops::Not;

use error::InstructionError;

pub use concrete::{ConcreteVMInstruction, ExecRecord};

pub type InstructionResult<T> = Result<T, InstructionError>;

pub trait Binary: Default {
    fn one() -> Self;
    fn zero() -> Self {
        Self::default()
    }
}

pub trait Constrain {
    type Constraint: Not + std::ops::Not<Output = Self::Constraint>;

    fn assert_eq(self, other: Self) -> Self::Constraint;
}
