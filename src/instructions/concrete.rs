use std::ops::Deref;

use crate::{
    memory::{Mem, MemRecord},
    stack::{Stack, StackRecord},
};

use super::InstructionResult;

pub struct ExecRecord<S, M>
where
    M: Mem,
    S: Stack,
{
    pub stack_diff: Option<StackRecord<S>>,
    pub mem_diff: Option<MemRecord<M>>,
    pub pc_change: Option<usize>,
    pub halt: bool,
}

impl<S: Stack, M: Mem> Default for ExecRecord<S, M> {
    fn default() -> Self {
        ExecRecord {
            stack_diff: None,
            mem_diff: None,
            pc_change: None,
            halt: false,
        }
    }
}

pub trait ConcreteVMInstruction<S, M>: std::fmt::Debug
where
    S: Stack,
    M: Mem,
{
    fn exec(
        &self,
        stack: &S,
        memory: &M,
    ) -> InstructionResult<ExecRecord<S, M>>;
}

pub type DynConcreteVMInstruction<S, M> = Box<dyn ConcreteVMInstruction<S, M>>;

impl<S, M> ConcreteVMInstruction<S,M> for DynConcreteVMInstruction<S, M>
where
    S: Stack,
    M: Mem,
{
    fn exec(&self, stack: &S, memory: &M) -> InstructionResult<ExecRecord<S, M>> {
        self.deref().exec(stack, memory)
    }
}
