use std::marker::PhantomData;

use crate::{
    instructions::HybridVMInstruction,
    memory::{Mem, MemConcreteIntToSymbolicInt},
    solvers::Solver,
    stack::{Stack, SymbolicIntStack},
    vals::{SymbolicInt, SymbolicIntConstraint, SymbolicIntRV},
};

use super::{concrete::BaseConcreteMachine, BaseMachine, ConcreteMachine, SymbolicMachine};

#[derive(Debug)]
pub struct BaseSymbolicMachine<S, M, ConcreteRV, C, Ma, SymbolicRV>
where
    S: Stack,
    M: Mem,
    C: std::fmt::Debug,
    Ma: ConcreteMachine<S, M, ConcreteRV, HybridVMInstruction<S, M, C>>,
{
    constraints: Vec<C>,
    concrete_machine: Ma,

    stack_pd: PhantomData<S>,
    mem_pd: PhantomData<M>,
    rv_pd: PhantomData<ConcreteRV>,
    s_rv_pd: PhantomData<SymbolicRV>,
}

impl<S, M, ConcreteRV, C, Ma, SymbolicRV> BaseSymbolicMachine<S, M, ConcreteRV, C, Ma, SymbolicRV>
where
    S: Stack,
    M: Mem,
    C: std::fmt::Debug,
    Ma: ConcreteMachine<S, M, ConcreteRV, HybridVMInstruction<S, M, C>>,
{
    pub fn new(ma: Ma, constraints: Option<Vec<C>>) -> Self {
        Self {
            concrete_machine: ma,
            constraints: constraints.unwrap_or(vec![]),

            stack_pd: PhantomData,
            mem_pd: PhantomData,
            rv_pd: PhantomData,
            s_rv_pd: PhantomData,
        }
    }
}

impl<S, M, ConcreteRV, C, Ma, SymbolicRV>
    BaseMachine<S, M, SymbolicRV, HybridVMInstruction<S, M, C>>
    for BaseSymbolicMachine<S, M, ConcreteRV, C, Ma, SymbolicRV>
where
    S: Stack,
    M: Mem,
    C: std::fmt::Debug,
    Ma: ConcreteMachine<S, M, ConcreteRV, HybridVMInstruction<S, M, C>>,
    SymbolicRV: From<ConcreteRV>,
{
    fn peek_instruction(&self) -> Option<&HybridVMInstruction<S, M, C>> {
        self.concrete_machine.peek_instruction()
    }

    fn can_exec(&self) -> bool {
        self.concrete_machine.can_exec()
    }

    fn return_value(&self) -> SymbolicRV {
        SymbolicRV::from(self.concrete_machine.return_value())
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

impl<S, M, ConcreteRV, C, Ma, SymbolicRV>
    SymbolicMachine<S, M, SymbolicRV, HybridVMInstruction<S, M, C>, C>
    for BaseSymbolicMachine<S, M, ConcreteRV, C, Ma, SymbolicRV>
where
    S: Stack,
    M: Mem,
    C: std::fmt::Debug + Clone,
    Ma: ConcreteMachine<S, M, ConcreteRV, HybridVMInstruction<S, M, C>>,
    SymbolicRV: From<ConcreteRV>,
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
                    s_rv_pd: PhantomData,
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
                        constraints: constraints,

                        stack_pd: PhantomData,
                        mem_pd: PhantomData,
                        rv_pd: PhantomData,
                        s_rv_pd: PhantomData,
                    }
                })
                .collect(),
        }
    }

    fn constraints(&self) -> &Vec<C> {
        &self.constraints
    }
}

pub type SymbolicIntMachine<'a> = BaseSymbolicMachine<
    SymbolicIntStack,
    MemConcreteIntToSymbolicInt,
    Option<SymbolicInt>,
    SymbolicIntConstraint,
    SymbolicIntMachineInnerConcrete<'a>,
    SymbolicIntRV,
>;

pub type SymbolicIntMachineInnerConcrete<'a> = BaseConcreteMachine<
    'a,
    SymbolicIntStack,
    MemConcreteIntToSymbolicInt,
    HybridVMInstruction<SymbolicIntStack, MemConcreteIntToSymbolicInt, SymbolicIntConstraint>,
>;

#[derive(Debug, PartialEq, Eq)]
pub struct SymbolicMachineOutput<RV, ModelOutput, RVOutput> {
    pub symbolic: RV,
    pub concrete: RVOutput,
    pub model: Vec<(String, ModelOutput)>,
    pub additional_model: Vec<(String, RVOutput)>,
}

pub fn run_sym_machine<S, M, RV, I, C, Ma, So, ModelOutput, RVOutput>(
    m: Ma,
    solver: So,
) -> Vec<SymbolicMachineOutput<RV, ModelOutput, RVOutput>>
where
    Ma: SymbolicMachine<S, M, RV, I, C>,
    So: Solver<C, RV, ModelOutput, RVOutput>,
    RV: Clone,
{
    let mut rv = vec![];

    let mut queue = vec![m];

    while !queue.is_empty() {
        let cur = queue.pop().unwrap();

        let mut new_ms = cur.sym_exec();

        while !new_ms.is_empty() {
            let new_m = new_ms.pop().unwrap();

            // TODO(will) -- should not have to call solver on every step
            if solver.solve(new_m.constraints(), vec![]).is_some() {
                if new_m.can_exec() {
                    queue.push(new_m);
                } else {
                    rv.push(new_m);
                }
            }
        }
    }

    rv.into_iter()
        .map(|m| {
            let symbolic = m.return_value();

            let mut model = match solver.solve(m.constraints(), vec![symbolic.clone()]) {
                Some(model) => model,
                None => panic!("final machine state not solvable"),
            };

            let concrete = model.additional.pop().unwrap().unwrap();

            SymbolicMachineOutput {
                symbolic,
                concrete,
                model: model.model,
                additional_model: model.additional_model,
            }
        })
        .collect()
}
