pub mod arith;
pub mod bitwise;
pub mod error;
pub mod helpers;
pub mod misc;
pub mod sym;
pub mod sym_helpers;
pub mod val;

use std::ops::Deref;

use crate::memory::*;
use crate::stack::*;
use error::InstructionError;

use self::misc::UNREACHABLE;

pub type InstructionResult<T> = Result<T, InstructionError>;
pub struct ExecRecord<S, M>
where
    S: Stack,
    M: Mem,
{
    pub stack_diff: Option<StackRecord<S>>,
    pub mem_diff: Option<MemRecord<M>>,
    // Each inner vec represents a new path in the program
    // pub path_constraints: Vec<Vec<PathConstraint>>,
    pub pc_change: Option<usize>,
    pub halt: bool,
}

impl<S: Stack, M: Mem> Default for ExecRecord<S, M> {
    fn default() -> Self {
        ExecRecord {
            stack_diff: None,
            mem_diff: None,
            // path_constraints: vec![],
            pc_change: None,
            halt: false,
        }
    }
}

pub trait VMInstruction<S: Stack, M: Mem> {
    fn exec(&self, stack: &S, memory: &M) -> InstructionResult<ExecRecord<S, M>>;
}

pub type ConcreteVMInstruction<S, M> = Box<dyn VMInstruction<S, M>>;

pub enum SymbolicVMInstruction<S, M, SI> {
    C(ConcreteVMInstruction<S, M>),
    S(SI),
}

impl<S, M, SI> VMInstruction<S, M> for SymbolicVMInstruction<S, M, SI>
where
    S: Stack,
    M: Mem,
{
    fn exec(&self, stack: &S, memory: &M) -> InstructionResult<ExecRecord<S, M>> {
        match self {
            SymbolicVMInstruction::C(c) => c.exec(stack, memory),
            SymbolicVMInstruction::S(s) => UNREACHABLE.exec(stack, memory),
        }
    }
}

impl<S, M, SI> From<ConcreteVMInstruction<S, M>> for SymbolicVMInstruction<S, M, SI> {
    fn from(c: ConcreteVMInstruction<S, M>) -> Self {
        SymbolicVMInstruction::C(c)
    }
}

impl<S, M> VMInstruction<S, M> for ConcreteVMInstruction<S, M>
where
    S: Stack,
    M: Mem,
{
    fn exec(&self, stack: &S, memory: &M) -> InstructionResult<ExecRecord<S, M>> {
        self.deref().exec(stack, memory)
    }
}
