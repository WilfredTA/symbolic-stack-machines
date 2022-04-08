use crate::{memory::Mem, stack::Stack};

pub trait SymbolicVMInstruction: std::fmt::Debug {
    type S: Stack;
    type M: Mem;
    type C;

    fn sym_exec(
        &self,
        s: &Self::S,
        m: &Self::M,
        pc: usize,
    ) -> Vec<(Self::S, Self::M, usize, Vec<Self::C>)>;
}

pub type DynSymbolicVMInstruction<S, M, C> = Box<dyn SymbolicVMInstruction<S = S, M = M, C = C>>;
