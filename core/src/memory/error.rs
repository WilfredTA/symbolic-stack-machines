use thiserror::{self, Error};

#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("Invalid Index {0}")]
    InvalidIndex(usize),
    #[error("Value not supported {0}")]
    ValueNotSupported(String),
}
