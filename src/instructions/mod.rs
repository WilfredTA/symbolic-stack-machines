pub mod error;

mod concrete;
mod symbolic;
mod hybrid;

use error::InstructionError;

pub use concrete::{ConcreteVMInstruction, ExecRecord};

pub type InstructionResult<T> = Result<T, InstructionError>;

