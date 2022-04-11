use std::marker::PhantomData;

use crate::{
    instructions::HybridVMInstruction,
    memory::{Mem, MemConcreteIntToSymbolicInt},
    stack::{Stack, SymbolicIntStack},
    vals::{SymbolicInt, SymbolicIntConstraint},
};

use super::{concrete::BaseConcreteMachine, BaseMachine, ConcreteMachine, SymbolicMachine};

#[derive(Debug)]
pub struct BaseSymbolicMachine<S, M, RV, C, Ma>
where
    S: Stack,
    M: Mem,
    C: std::fmt::Debug,
    Ma: ConcreteMachine<S, M, RV, HybridVMInstruction<S, M, C>>,
{
    constraints: Vec<C>,
    concrete_machine: Ma,

    stack_pd: PhantomData<S>,
    mem_pd: PhantomData<M>,
    rv_pd: PhantomData<RV>,
}

impl<S, M, RV, C, Ma> BaseSymbolicMachine<S, M, RV, C, Ma>
where
    S: Stack,
    M: Mem,
    C: std::fmt::Debug,
    Ma: ConcreteMachine<S, M, RV, HybridVMInstruction<S, M, C>>,
{
    pub fn new(ma: Ma, constraints: Option<Vec<C>>) -> Self {
        Self {
            concrete_machine: ma,
            constraints: constraints.unwrap_or(vec![]),

            stack_pd: PhantomData,
            mem_pd: PhantomData,
            rv_pd: PhantomData,
        }
    }
}

impl<S, M, RV, C, Ma> BaseMachine<S, M, RV, HybridVMInstruction<S, M, C>>
    for BaseSymbolicMachine<S, M, RV, C, Ma>
where
    S: Stack,
    M: Mem,
    C: std::fmt::Debug,
    Ma: ConcreteMachine<S, M, RV, HybridVMInstruction<S, M, C>>,
{
    fn peek_instruction(&self) -> Option<&HybridVMInstruction<S, M, C>> {
        self.concrete_machine.peek_instruction()
    }

    fn can_exec(&self) -> bool {
        self.concrete_machine.can_exec()
    }

    fn return_value(&self) -> RV {
        self.concrete_machine.return_value()
    }

    fn stack(&self) -> &S {
        self.concrete_machine.stack()
    }

    fn mem(&self) -> &M {
        self.concrete_machine.mem()
    }

    fn pc(&self) -> usize {
        self.concrete_machine.pc()
    }
}

impl<S, M, RV, C, Ma> SymbolicMachine<S, M, RV, HybridVMInstruction<S, M, C>, C>
    for BaseSymbolicMachine<S, M, RV, C, Ma>
where
    S: Stack,
    M: Mem,
    C: std::fmt::Debug + Clone,
    Ma: ConcreteMachine<S, M, RV, HybridVMInstruction<S, M, C>>,
{
    fn sym_exec(&self) -> Vec<Self> {
        match self.concrete_machine.peek_instruction().unwrap() {
            HybridVMInstruction::C(_) => {
                let concrete_machine = self.concrete_machine.exec();

                vec![Self {
                    concrete_machine,
                    constraints: self.constraints.clone(),

                    stack_pd: PhantomData,
                    mem_pd: PhantomData,
                    rv_pd: PhantomData,
                }]
            }

            HybridVMInstruction::S(s) => s
                .sym_exec(
                    &self.concrete_machine.stack(),
                    &self.concrete_machine.mem(),
                    self.concrete_machine.pc(),
                )
                .into_iter()
                .map(|(stack, mem, pc, mut constraints)| {
                    self.constraints
                        .iter()
                        .for_each(|c| constraints.push((*c).clone()));

                    let concrete_machine = self.concrete_machine.clone_machine(stack, mem, pc);

                    Self {
                        concrete_machine,
                        constraints: self.constraints.clone(),

                        stack_pd: PhantomData,
                        mem_pd: PhantomData,
                        rv_pd: PhantomData,
                    }
                })
                .collect(),
        }
    }
}

pub type SymbolicIntMachine<'a> = BaseSymbolicMachine<
    SymbolicIntStack,
    MemConcreteIntToSymbolicInt,
    Option<SymbolicInt>,
    SymbolicIntConstraint,
    SymbolicIntMachineInnerConcrete<'a>,
>;

pub type SymbolicIntMachineInnerConcrete<'a> = BaseConcreteMachine<
    'a,
    SymbolicIntStack,
    MemConcreteIntToSymbolicInt,
    HybridVMInstruction<SymbolicIntStack, MemConcreteIntToSymbolicInt, SymbolicIntConstraint>,
>;
