use crate::instructions::{sym, HybridVMInstruction, SymbolicVMInstruction};
use crate::memory::symbolic_concrete_index::MemConcreteIntToSymbolicInt;
use crate::memory::WriteableMem;
use crate::stack::SymbolicIntStack;
use crate::symbolic_int::SymbolicIntConstraint;
use crate::{memory::Mem, stack::Stack};

use super::concrete::BaseConcreteMachine;
use super::{BaseMachine, ConcreteMachine, Program, SymbolicMachine};

#[derive(Debug)]
pub struct BaseSymbolicMachine<'a, S, M, C>
where
    S: Stack,
    M: Mem,
    C: std::fmt::Debug
{
    constraints: Vec<C>,

    // TODO should just take an abstract Machine
    concrete_machine: BaseConcreteMachine<'a, S, M, HybridVMInstruction<S, M, C>>,
}

impl<'a, S, M, C> BaseSymbolicMachine<'a, S, M, C>
where
    S: Stack,
    M: Mem,
    C: std::fmt::Debug
{
    pub fn new(
        stack: S,
        mem: M,
        pgm: &'a Program<HybridVMInstruction<S, M, C>>,
        pc: Option<usize>,
        constraints: Option<Vec<C>>,
    ) -> Self {
        Self {
            concrete_machine: BaseConcreteMachine::new(stack, mem, pgm, pc),
            constraints: constraints.unwrap_or(vec![]),
        }
    }
}

impl<'a, S, M, C> BaseMachine<S, M, Option<S::StackVal>, HybridVMInstruction<S, M, C>>
    for BaseSymbolicMachine<'a, S, M, C>
where
    S: Stack + Clone,
    M: WriteableMem + Clone,
    C: std::fmt::Debug
{
    fn peek_instruction(&self) -> Option<&HybridVMInstruction<S, M, C>> {
        self.concrete_machine.peek_instruction()
    }

    fn can_exec(&self) -> bool {
        self.concrete_machine.can_exec()
    }

    fn return_value(&self) -> Option<S::StackVal> {
        self.concrete_machine.return_value()
    }
}

impl<'a, S, M, C> SymbolicMachine<S, M, Option<S::StackVal>, HybridVMInstruction<S, M, C>>
    for BaseSymbolicMachine<'a, S, M, C>
where
    S: Stack + Clone,
    M: WriteableMem + Clone,
    C: Clone + std::fmt::Debug,
{
    fn sym_exec(&self) -> Vec<Box<Self>> {
        match self.concrete_machine.peek_instruction().unwrap() {
            HybridVMInstruction::C(_) => {
                let concrete_machine = self.concrete_machine.exec();
                vec![Box::new(Self {
                    concrete_machine,
                    constraints: self.constraints.clone(),
                })]
            }

            // TODO(HERE) - now we need some way to dispatch on the particular symbolic instructions
            HybridVMInstruction::S(s) => s
                .sym_exec(
                    &self.concrete_machine.stack,
                    &self.concrete_machine.mem,
                    self.concrete_machine.pc,
                )
                .into_iter()
                // TODO add constraint to machine
                .map(|(stack, mem, pc, mut constraints)| {
                    self.constraints
                        .iter()
                        .for_each(|c| constraints.push((*c).clone()));

                    Box::new(Self::new(
                        stack,
                        mem,
                        self.concrete_machine.pgm,
                        Some(pc),
                        Some(constraints),
                    ))
                })
                .collect(),
        }
    }
}

pub type SymbolicIntMachine<'a> =
    BaseSymbolicMachine<'a, SymbolicIntStack, MemConcreteIntToSymbolicInt, SymbolicIntConstraint>;
