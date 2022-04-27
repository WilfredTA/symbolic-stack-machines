use thiserror::{self, Error};

#[derive(Debug, Error)]
pub enum StackError {
    #[error("Invalid stack value {0}")]
    IncorrectVal(String),
    #[error("Stack is empty")]
    EmptyStack,
    #[error("Stack overflow")]
    StackOverflow,
}
