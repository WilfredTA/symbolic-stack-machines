mod concrete;
mod symbolic;

pub mod error;
pub use concrete::{run_machine, ConcreteIntMachine};
pub use symbolic::{
    run_sym_machine, SymbolicIntMachine, SymbolicIntMachineInnerConcrete, SymbolicMachineOutput,
};

use error::MachineError;

use crate::instructions::{DynConcreteVMInstruction, HybridVMInstruction};

pub type MachineResult<T> = Result<T, MachineError>;

pub type Program<I> = Vec<I>;
pub type ConcreteProgram<S, M> = Program<DynConcreteVMInstruction<S, M>>;
pub type SymbolicProgram<S, M, C> = Program<HybridVMInstruction<S, M, C>>;

pub trait BaseMachine<S, M, RV, I> {
    fn peek_instruction(&self) -> Option<&I>;
    fn can_exec(&self) -> bool;
    fn return_value(&self) -> RV;

    fn stack(&self) -> &S;
    fn mem(&self) -> &M;
    fn pc(&self) -> usize;
}

pub trait ConcreteMachine<S, M, RV, I>: BaseMachine<S, M, RV, I> {
    fn exec(&self) -> Self;
    fn clone_machine(&self, s: S, m: M, pc: usize) -> Self;
}

pub trait SymbolicMachine<S, M, RV, I, C>: BaseMachine<S, M, RV, I> + Sized {
    fn sym_exec(&self) -> Vec<Self>;
    fn constraints(&self) -> &Vec<C>;
}
