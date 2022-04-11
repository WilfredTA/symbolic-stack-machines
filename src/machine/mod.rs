mod concrete;
mod symbolic;

pub mod error;
pub use concrete::ConcreteIntMachine;
pub use symbolic::{SymbolicIntMachine, SymbolicIntMachineInnerConcrete};

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

pub trait SymbolicMachine<S, M, RV, I, C>: BaseMachine<S, M, RV, I> + Sized
where
{
    fn sym_exec(&self) -> Vec<Self>;
}

pub fn run_machine<S, M, RV, I, Ma: ConcreteMachine<S, M, RV, I>>(m: Ma) -> RV {
    let mut mm = m;

    while mm.can_exec() {
        mm = mm.exec();
    }

    mm.return_value()
}

pub fn run_sym_machine<S, M, RV, I, C, Ma>(m: Ma) -> Vec<RV>
where
    Ma: SymbolicMachine<S, M, RV, I, C>,
{
    let mut rv = vec![];

    let mut queue = vec![m];

    while !queue.is_empty() {
        let cur = queue.pop().unwrap();

        let mut new_ms = cur.sym_exec();

        while !new_ms.is_empty() {
            let new_m = new_ms.pop().unwrap();

            if new_m.can_exec() {
                queue.push(new_m);
            } else {
                rv.push(new_m);
            }
        }
    }

    rv.into_iter()
        .map(|x| {
            x.return_value()
        })
        .collect()
}