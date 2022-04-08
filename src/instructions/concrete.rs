use std::ops::Deref;

use crate::{stack::{Stack, StackRecord}, memory::{MemRecord, Mem}};

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

pub trait ConcreteVMInstruction: std::fmt::Debug {
    type S: Stack;
    type M: Mem;

    fn exec(
        &self,
        stack: &Self::S,
        memory: &Self::M,
    ) -> InstructionResult<ExecRecord<Self::S, Self::M>>;
}

pub type DynConcreteVMInstruction<S, M> = Box<dyn ConcreteVMInstruction<S = S, M = M>>;

impl<S, M> ConcreteVMInstruction for DynConcreteVMInstruction<S, M>
where
    S: Stack,
    M: Mem,
{
    type S = S;
    type M = M;

    fn exec(&self, stack: &S, memory: &M) -> InstructionResult<ExecRecord<S, M>> {
        self.deref().exec(stack, memory)
    }
}
