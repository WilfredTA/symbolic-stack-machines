use crate::instructions::{sym, SymbolicVMInstruction};
use crate::memory::symbolic_concrete_index::MemConcreteIntToSymbolicInt;
use crate::memory::WriteableMem;
use crate::stack::SymbolicIntStack;
use crate::{memory::Mem, stack::Stack};

use super::concrete::BaseConcreteMachine;
use super::{BaseMachine, ConcreteMachine, Program, SymbolicMachine};

pub struct BaseSymbolicMachine<'a, S, M, SI>
where
    S: Stack,
    M: Mem,
{
    // TODO should just take an abstract Machine
    concrete_machine: BaseConcreteMachine<'a, S, M, SymbolicVMInstruction<S, M, SI>>,
}

impl<'a, S, M, SI> BaseSymbolicMachine<'a, S, M, SI>
where
    S: Stack,
    M: Mem,
{
    pub fn new(stack: S, mem: M, pgm: &'a Program<SymbolicVMInstruction<S, M, SI>>) -> Self {
        Self {
            concrete_machine: BaseConcreteMachine::new(stack, mem, pgm),
        }
    }
}

impl<'a, S, M, SI> BaseMachine<S, M, Option<S::StackVal>, SymbolicVMInstruction<S, M, SI>>
    for BaseSymbolicMachine<'a, S, M, SI>
where
    S: Stack + Clone,
    M: WriteableMem + Clone,
{
    fn peek_instruction(&self) -> Option<&SymbolicVMInstruction<S, M, SI>> {
        self.concrete_machine.peek_instruction()
    }

    fn can_exec(&self) -> bool {
        self.concrete_machine.can_exec()
    }

    fn return_value(&self) -> Option<S::StackVal> {
        self.concrete_machine.return_value()
    }
}

impl<'a, S, M, SI> SymbolicMachine<S, M, Option<S::StackVal>, SymbolicVMInstruction<S, M, SI>>
    for BaseSymbolicMachine<'a, S, M, SI>
where
    S: Stack + Clone,
    M: WriteableMem + Clone,
{
    fn sym_exec(&self) -> Vec<Box<Self>> {
        match self.concrete_machine.peek_instruction().unwrap() {
            SymbolicVMInstruction::C(_) => {
                let concrete_machine = self.concrete_machine.exec();
                vec![Box::new(Self { concrete_machine })]
            }

            // TODO(HERE) - now we need some way to dispatch on the particular symbolic instructions
            SymbolicVMInstruction::S(_) => todo!(),
        }
    }
}

pub type SymbolicIntMachine<'a> =
    BaseSymbolicMachine<'a, SymbolicIntStack, MemConcreteIntToSymbolicInt, sym::JUMPI>;
