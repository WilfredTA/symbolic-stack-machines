pub mod error;
pub mod val;
use crate::memory::*;
use crate::stack::*;
use error::InstructionError;
use z3::ast::Bool;

pub type InstructionResult<T> = Result<T, InstructionError>;
pub struct ExecRecord<'a, S, M>
where
    M: WriteableMem,
    S: Stack,
{
    pub stack_diff: Option<StackRecord<S>>,
    pub mem_diff: Option<MemRecord<M>>,
    // Each inner vec represents a new path in the program
    pub path_constraints: Vec<Vec<Bool<'a>>>,
    pub pc_change: Option<usize>,
    pub halt: bool,
}

pub trait VMInstruction<'a> {
    type ValStack: Stack;
    type Mem: RWMem;
    fn exec(
        &self,
        stack: &Self::ValStack,
        memory: &Self::Mem,
    ) -> InstructionResult<ExecRecord<'a, Self::ValStack, Self::Mem>>;
}
