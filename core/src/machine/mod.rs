pub mod r#abstract;
pub mod error;
pub mod inner_interpreter;
pub mod outer_interpreter;
mod types;

use error::MachineError;

pub type MachineResult<T> = Result<T, MachineError>;
