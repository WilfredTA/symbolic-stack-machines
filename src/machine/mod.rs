pub mod concrete;
pub mod error;
pub mod symbolic;

use error::MachineError;

use crate::instructions::VMInstruction;

pub type MachineResult<T> = Result<T, MachineError>;

pub type Program<S, M> = Vec<Box<dyn VMInstruction<S, M>>>;

pub trait Machine<S, M, RV> {
    fn can_exec(&self) -> bool;
    fn exec(&self) -> Self;
    fn return_value(&self) -> RV;
}

pub fn run_machine<S, M, RV, Ma: Machine<S, M, RV>>(m: Ma) -> RV {
    let mut mm = m;

    while mm.can_exec() {
        mm = mm.exec();
    }

    mm.return_value()
}
