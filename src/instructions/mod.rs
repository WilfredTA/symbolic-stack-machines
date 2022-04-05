pub mod error;
pub mod val;
pub mod arith;
pub mod misc;
pub mod bitwise;
pub mod sym;

use crate::memory::*;
use crate::stack::*;
use error::InstructionError;
use std::fmt::Debug;

pub type InstructionResult<T> = Result<T, InstructionError>;
pub struct ExecRecord<S, M, PathConstraint>
where
    S: Stack,
    M: Mem
{
    pub stack_diff: Option<StackRecord<S>>,
    pub mem_diff: Option<MemRecord<M>>,
    // Each inner vec represents a new path in the program
    pub path_constraints: Vec<Vec<PathConstraint>>,
    pub pc_change: Option<usize>,
    pub halt: bool,
}

impl <S: Stack, M: Mem, PathConstraint> Default for ExecRecord<S, M, PathConstraint> {
    fn default() -> Self {
        ExecRecord {
            stack_diff: None,
            mem_diff: None,
            path_constraints: vec![],
            pc_change: None,
            halt: false,
        }
    }
}

pub trait VMInstruction<S: Stack, M: Mem, PathConstraint>: Debug {
    fn exec(
        &self,
        stack: &S,
        memory: &M,
    ) -> InstructionResult<ExecRecord<S, M, PathConstraint>>;
}
