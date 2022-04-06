use crate::memory::symbolic_concrete_index::MemConcreteIntToSymbolicInt;
use crate::memory::WriteableMem;
use crate::stack::SymbolicIntStack;
use crate::{memory::Mem, stack::Stack};

use super::concrete::BaseConcreteMachine;
use super::{Machine, Program};

pub struct BaseSymbolicMachine<'a, S, M>
where
    S: Stack,
    M: Mem,
{
    concrete_machine: BaseConcreteMachine<'a, S, M>,
}

impl<'a, S, M> BaseSymbolicMachine<'a, S, M>
where
    S: Stack,
    M: Mem,
{
    pub fn new(stack: S, mem: M, pgm: &'a Program<S, M>) -> Self {
        Self {
            concrete_machine: BaseConcreteMachine::new(stack, mem, pgm),
        }
    }
}

impl<'a, S, M> Machine<S, M, Option<S::StackVal>> for BaseSymbolicMachine<'a, S, M>
where
    S: Stack + Clone,
    M: WriteableMem + Clone,
{
    fn can_exec(&self) -> bool {
        self.concrete_machine.can_exec()
    }

    fn exec(&self) -> Self {
        let concrete_machine = self.concrete_machine.exec();

        Self { concrete_machine }
    }

    fn return_value(&self) -> Option<S::StackVal> {
        self.concrete_machine.return_value()
    }
}

pub type SymbolicIntMachine<'a> =
    BaseConcreteMachine<'a, SymbolicIntStack, MemConcreteIntToSymbolicInt>;
