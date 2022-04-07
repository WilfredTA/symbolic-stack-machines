pub mod concrete;
pub mod error;
pub mod symbolic;

use error::MachineError;

pub type MachineResult<T> = Result<T, MachineError>;

pub type Program<T> = Vec<T>;

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

pub trait SymbolicMachine<S, M, RV, I>: BaseMachine<S, M, RV, I> {
    // TODO(will) - can we not box the return value?
    fn sym_exec(&self) -> Vec<Box<Self>>;
}

pub fn run_sym_machine<S, M, RV, I, Ma: SymbolicMachine<S, M, RV, I>>(m: Ma) -> Vec<Ma> {
    let mut rv = vec![];

    let mut queue = vec![m];

    while !queue.is_empty() {
        let cur = queue.pop().unwrap();

        let mut new_ms = cur.sym_exec();

        while !new_ms.is_empty() {
            let new_m = *new_ms.pop().unwrap();

            if new_m.can_exec() {
                queue.push(new_m);
            } else {
                rv.push(new_m);
            }
        }
    }

    rv
}
