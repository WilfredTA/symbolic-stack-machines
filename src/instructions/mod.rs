pub mod error;
pub mod val;
use crate::memory::*;
use crate::stack::*;
use error::InstructionError;

pub type InstructionResult<T> = Result<T, InstructionError>;
pub struct ExecRecord<S, M>
where
    M: WriteableMem,
    S: Stack,
{
    pub stack_diff: Option<StackRecord<S>>,
    pub mem_diff: Option<MemRecord<M>>,
    pub pc_change: Option<usize>,
    pub halt: bool,
}

pub trait VMInstruction {
    type ValStack: Stack;
    type Mem: RWMem;
    fn exec(
        &self,
        stack: &Self::ValStack,
        memory: &Self::Mem,
    ) -> InstructionResult<ExecRecord<Self::ValStack, Self::Mem>>;
}
