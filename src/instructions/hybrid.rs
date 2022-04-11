use crate::{memory::Mem, stack::Stack};

use super::{concrete::DynConcreteVMInstruction, symbolic::DynSymbolicVMInstruction, InstructionResult, ExecRecord, ConcreteVMInstruction};

#[derive(Debug)]
pub enum HybridVMInstruction<S, M, C> {
    #[allow(dead_code)]
    C(DynConcreteVMInstruction<S, M>),
    #[allow(dead_code)]
    S(DynSymbolicVMInstruction<S, M, C>),
}

impl<S, M, C> ConcreteVMInstruction<S, M> for HybridVMInstruction<S, M, C>
where
    S: Stack,
    M: Mem,
    C: std::fmt::Debug,
{
    fn exec(&self, stack: &S, memory: &M) -> InstructionResult<ExecRecord<S, M>> {
        match self {
            HybridVMInstruction::C(c) => c.exec(stack, memory),
            HybridVMInstruction::S(_) => panic!("Executed symbolic instruction as concrete instruction. This is a bug in the machine implementation."),
        }
    }
}

impl<S, M, C> From<DynConcreteVMInstruction<S, M>> for HybridVMInstruction<S, M, C> {
    fn from(c: DynConcreteVMInstruction<S, M>) -> Self {
        HybridVMInstruction::C(c)
    }
}
