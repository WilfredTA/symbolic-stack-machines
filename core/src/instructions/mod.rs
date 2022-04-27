pub mod error;

mod concrete;
mod hybrid;
mod impls;
mod symbolic;

use std::ops::Not;

use error::InstructionError;

pub use concrete::{ConcreteVMInstruction, DynConcreteVMInstruction, ExecRecord};
pub use impls::{helpers, sym_helpers};
pub use hybrid::HybridVMInstruction;

pub type InstructionResult<T> = Result<T, InstructionError>;

// Tannr: Example kind of
// pub trait VMInstruct {
//     type StackVal;
//     type MemVal;
    
//     pub fn exec(&self) -> ExecRecord<Self::StackVal, Self::MemVal>;
// }
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
