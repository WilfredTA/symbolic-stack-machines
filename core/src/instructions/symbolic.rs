use crate::{memory::Mem, stack::Stack};

pub trait SymbolicVMInstruction<S: Stack, M: Mem, C>: std::fmt::Debug {
    fn sym_exec(
        &self,
        s: &S,
        m: &M,
        pc: usize,
    ) -> Vec<(S, M, usize, Vec<C>)>;
}

pub type DynSymbolicVMInstruction<S, M, C> = Box<dyn SymbolicVMInstruction<S, M, C>>;
