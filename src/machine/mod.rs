pub mod concrete;
pub mod error;
pub mod symbolic;

use error::MachineError;

use crate::{
    solvers::{self, z3::Z3Constraint},
    symbolic_int::{self, SymbolicInt},
};

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

pub trait SymbolicMachine<S, M, RV, I, C>: BaseMachine<S, M, RV, I>
where
    // TODO should be abstract on more than just Z3
    C: Z3Constraint,
{
    // TODO(will) - can we not box the return value?
    fn sym_exec(&self) -> Vec<Box<Self>>;
    fn constraints(&self) -> Vec<C>;
}

// TODO should be abstract over more than i64

#[derive(Debug, PartialEq, Eq)]
pub struct SymbolicMachineOutput {
    pub symbolic: Option<SymbolicInt>,
    pub concrete: Option<i64>,
    pub model: Vec<(String, i64)>,
}

pub fn run_sym_machine<S, M, I, Ma, C>(m: Ma) -> Vec<SymbolicMachineOutput>
where
    Ma: SymbolicMachine<S, M, Option<SymbolicInt>, I, C>,
    C: Z3Constraint,
{
    let mut rv = vec![];

    let mut queue = vec![m];

    while !queue.is_empty() {
        let cur = queue.pop().unwrap();

        let mut new_ms = cur.sym_exec();

        while !new_ms.is_empty() {
            let new_m = *new_ms.pop().unwrap();

            // TODO -- should only check solver when the model changes
            if solvers::z3::solve(new_m.constraints(), None).is_some() {
                if new_m.can_exec() {
                    queue.push(new_m);
                } else {
                    rv.push(new_m);
                }
            }
        }
    }

    rv.into_iter()
        .map(|x| {
            let rv = x.return_value();

            let (model, xrv) = solvers::z3::solve(x.constraints(), rv.clone()).unwrap();

            SymbolicMachineOutput {
                symbolic: rv,
                concrete: xrv,
                model,
            }
        })
        .collect()
}
