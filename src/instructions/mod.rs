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

pub trait ConcreteVMInstruction<S: Stack, M: Mem>: std::fmt::Debug {
    fn exec(&self, stack: &S, memory: &M) -> InstructionResult<ExecRecord<S, M>>;
}

pub trait SymbolicVMInstruction<S: Stack, M: Mem, C>: std::fmt::Debug {
    fn sym_exec(&self, s: &S, m: &M, pc: usize) -> Vec<(S, M, usize, Vec<C>)>;
}

pub type DynConcreteVMInstruction<S, M> = Box<dyn ConcreteVMInstruction<S, M>>;

pub type DynSymbolicVMInstruction<S, M, C> = Box<dyn SymbolicVMInstruction<S, M, C>>;

#[derive(Debug)]
pub enum HybridVMInstruction<S, M, C> {
    C(DynConcreteVMInstruction<S, M>),
    S(DynSymbolicVMInstruction<S, M, C>),
}

impl<S, M, C> ConcreteVMInstruction<S, M> for HybridVMInstruction<S, M, C>
where
    S: Stack,
    M: Mem,
    C: std::fmt::Debug
{
    fn exec(&self, stack: &S, memory: &M) -> InstructionResult<ExecRecord<S, M>> {
        match self {
            HybridVMInstruction::C(c) => c.exec(stack, memory),
            HybridVMInstruction::S(s) => UNREACHABLE.exec(stack, memory),
        }
    }
}

impl<S, M, C> From<DynConcreteVMInstruction<S, M>> for HybridVMInstruction<S, M, C> {
    fn from(c: DynConcreteVMInstruction<S, M>) -> Self {
        HybridVMInstruction::C(c)
    }
}

impl<S, M> ConcreteVMInstruction<S, M> for DynConcreteVMInstruction<S, M>
where
    S: Stack,
    M: Mem,
{
    fn exec(&self, stack: &S, memory: &M) -> InstructionResult<ExecRecord<S, M>> {
        self.deref().exec(stack, memory)
    }
}
