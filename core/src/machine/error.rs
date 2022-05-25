use thiserror::{self, Error};

use crate::instructions::error::InstructionError;

#[derive(Debug, Error)]
pub enum MachineError {
    #[error(transparent)]
    InstructionError(#[from] InstructionError),
}
