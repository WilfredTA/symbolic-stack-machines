use crate::instructions::{sym, HybridVMInstruction, SymbolicVMInstruction};
use crate::memory::symbolic_concrete_index::MemConcreteIntToSymbolicInt;
use crate::memory::WriteableMem;
use crate::stack::SymbolicIntStack;
use crate::{memory::Mem, stack::Stack};

use super::concrete::BaseConcreteMachine;
use super::{BaseMachine, ConcreteMachine, Program, SymbolicMachine};

pub struct BaseSymbolicMachine<'a, S, M>
where
    S: Stack,
    M: Mem,
{
    // TODO should just take an abstract Machine
    concrete_machine: BaseConcreteMachine<'a, S, M, HybridVMInstruction<S, M>>,
}

impl<'a, S, M> BaseSymbolicMachine<'a, S, M>
where
    S: Stack,
    M: Mem,
{
    pub fn new(
        stack: S,
        mem: M,
        pgm: &'a Program<HybridVMInstruction<S, M>>,
        pc: Option<usize>,
    ) -> Self {
        Self {
            concrete_machine: BaseConcreteMachine::new(stack, mem, pgm, pc),
        }
    }
}

impl<'a, S, M> BaseMachine<S, M, Option<S::StackVal>, HybridVMInstruction<S, M>>
    for BaseSymbolicMachine<'a, S, M>
where
    S: Stack + Clone,
    M: WriteableMem + Clone,
{
    fn peek_instruction(&self) -> Option<&HybridVMInstruction<S, M>> {
        self.concrete_machine.peek_instruction()
    }

    fn can_exec(&self) -> bool {
        self.concrete_machine.can_exec()
    }

    fn return_value(&self) -> Option<S::StackVal> {
        self.concrete_machine.return_value()
    }
}

impl<'a, S, M> SymbolicMachine<S, M, Option<S::StackVal>, HybridVMInstruction<S, M>>
    for BaseSymbolicMachine<'a, S, M>
where
    S: Stack + Clone,
    M: WriteableMem + Clone,
{
    fn sym_exec(&self) -> Vec<Box<Self>> {
        match self.concrete_machine.peek_instruction().unwrap() {
            HybridVMInstruction::C(_) => {
                let concrete_machine = self.concrete_machine.exec();
                vec![Box::new(Self { concrete_machine })]
            }

            // TODO(HERE) - now we need some way to dispatch on the particular symbolic instructions
            HybridVMInstruction::S(s) => s
                .sym_exec(
                    &self.concrete_machine.stack,
                    &self.concrete_machine.mem,
                    self.concrete_machine.pc,
                )
                .into_iter()
                .map(|(stack, mem, pc)| {
                    Box::new(Self::new(
                        stack,
                        mem,
                        self.concrete_machine.pgm,
                        Some(pc),
                    ))
                })
                .collect(),
        }
    }
}

pub type SymbolicIntMachine<'a> =
    BaseSymbolicMachine<'a, SymbolicIntStack, MemConcreteIntToSymbolicInt>;
