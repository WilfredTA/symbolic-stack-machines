pub mod concrete;
pub mod error;
pub mod symbolic;

use error::MachineError;

use crate::{instructions::{helpers::UNREACHABLE, VMInstruction}, stack::Stack, memory::Mem};

pub type MachineResult<T> = Result<T, MachineError>;

pub type ConcreteVMInstruction<S, M> = Box<dyn VMInstruction<S, M>>;
pub enum SymbolicVMInstruction<S, M, SI> {
    C(ConcreteVMInstruction<S, M>),
    S(SI),
}

impl <S, M, SI> From<ConcreteVMInstruction<S, M>> for SymbolicVMInstruction<S, M, SI> {
    fn from(c: ConcreteVMInstruction<S, M>) -> Self {
        SymbolicVMInstruction::C(c)
    }
}

type Program<T> = Vec<T>;
pub type ConcreteProgram<S, M> = Program<ConcreteVMInstruction<S, M>>;
pub type SymbolicProgram<S, M, SI> = Program<SymbolicVMInstruction<S, M, SI>>;

pub fn convert_symbolic_program_to_concrete<S, M, SI>(
    sp: SymbolicProgram<S, M, SI>,
) -> ConcreteProgram<S, M>
where
    S: Stack,
    M: Mem,
{
    sp
        .into_iter()
        .map(|x| {
            let f = match x {
                SymbolicVMInstruction::C(c) => {
                    c
                },
                SymbolicVMInstruction::S(_) => UNREACHABLE(),
            };
            f
        })
        .collect()
}

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
