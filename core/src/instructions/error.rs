use thiserror::{self, Error};

#[derive(Debug, Error)]
pub enum InstructionError {
    #[error("Unrecognized instruction {0}")]
    UnknownInstruction(String),
    #[error("Failed to execute instruction {0}")]
    InstructionExecutionFailure(String),
}
