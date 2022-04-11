mod concrete;

pub mod error;
pub use concrete::ConcreteIntMachine;

use error::MachineError;

use crate::instructions::DynConcreteVMInstruction;

pub type MachineResult<T> = Result<T, MachineError>;

pub type Program<I> = Vec<I>;
pub type ConcreteProgram<S, M> = Program<DynConcreteVMInstruction<S, M>>;

pub trait BaseMachine<S, M, RV, I> {
    fn peek_instruction(&self) -> Option<&I>;
    fn can_exec(&self) -> bool;
    fn return_value(&self) -> RV;
}

pub trait ConcreteMachine<S, M, RV, I>: BaseMachine<S, M, RV, I> {
    fn exec(&self) -> Self;
}

pub fn run_machine<S, M, RV, I, Ma: ConcreteMachine<S, M, RV, I>>(m: Ma) -> RV {
    let mut mm = m;

    while mm.can_exec() {
        mm = mm.exec();
    }

    mm.return_value()
}
