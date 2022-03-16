pub mod error;
pub mod val;
use z3::ast::Ast;
use z3::ast::Bool;
use z3::ast::Int;

use crate::memory::symbolic::BaseSymbolicMem;
use crate::memory::*;
use crate::stack::*;
use error::InstructionError;

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
