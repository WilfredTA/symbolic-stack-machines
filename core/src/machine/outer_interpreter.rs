use crate::{
    environment::EnvExtension,
    instructions::AbstractInstruction,
    memory::{Mem, WriteableMem},
    solver::Solver,
    stack::Stack,
};

use super::{
    inner_interpreter::InnerInterpreter,
    r#abstract::AbstractMachine,
    types::{
        AbstractExecBranch, ReachableSymbolicMachineResult, SingleBranch, SymbolicMachineResults,
        UnreachableSymbolicMachineResult,
    },
    MachineResult,
};

pub trait OuterInterpreter<Output, M> {
    fn run(&self, m: M) -> MachineResult<Output>;
}

pub struct ConcreteOuterInterpreter<'a, S, M, E, I, InstructionStepResult, InterpreterStepResult>
where
    S: Stack,
    M: Mem,
    E: EnvExtension,
    I: AbstractInstruction<S, M, E, InstructionStepResult>,
{
    pub inner_interpreter:
        Box<dyn InnerInterpreter<'a, S, M, E, I, InstructionStepResult, InterpreterStepResult>>,
}

impl<'a, S, M, E, I, InstructionStepResult>
    OuterInterpreter<AbstractMachine<'a, S, M, E, I>, AbstractMachine<'a, S, M, E, I>>
    for ConcreteOuterInterpreter<
        'a,
        S,
        M,
        E,
        I,
        InstructionStepResult,
        AbstractMachine<'a, S, M, E, I>,
    >
where
    S: Stack,
    M: WriteableMem,
    E: EnvExtension,
    I: AbstractInstruction<S, M, E, InstructionStepResult>,
{
    fn run(
        &self,
        m: AbstractMachine<'a, S, M, E, I>,
    ) -> MachineResult<AbstractMachine<'a, S, M, E, I>> {
        let mut x = m;

        while x.can_continue() {
            x = self.inner_interpreter.step(x)?;
        }

        Ok(x)
    }
}

pub struct SymbolicOuterInterpreter<
    'a,
    S,
    M,
    E,
    I,
    InstructionStepResult,
    InterpreterStepResult,
    Assertion,
    SolverResult,
> where
    S: Stack,
    M: Mem,
    E: EnvExtension,
    I: AbstractInstruction<S, M, E, InstructionStepResult>,
{
    inner_interpreter:
        Box<dyn InnerInterpreter<'a, S, M, E, I, InstructionStepResult, InterpreterStepResult>>,
    solver: Box<dyn Solver<Assertion, SolverResult>>,
}

impl<'a, S, M, E, I, InstructionStepResult, C, SolverResult>
    OuterInterpreter<
        SymbolicMachineResults<'a, S, M, E, I, C, SolverResult>,
        AbstractMachine<'a, S, M, E, I>,
    >
    for SymbolicOuterInterpreter<
        'a,
        S,
        M,
        E,
        I,
        InstructionStepResult,
        AbstractExecBranch<'a, S, M, E, I, C>,
        C,
        SolverResult,
    >
where
    S: Stack,
    M: WriteableMem,
    E: EnvExtension,
    I: AbstractInstruction<S, M, E, InstructionStepResult>,
    C: Clone,
{
    fn run(
        &self,
        m: AbstractMachine<'a, S, M, E, I>,
    ) -> MachineResult<SymbolicMachineResults<'a, S, M, E, I, C, SolverResult>> {
        let mut queue: Vec<SingleBranch<'a, S, M, E, I, C>> = vec![(m, vec![])];

        let mut reachable: Vec<ReachableSymbolicMachineResult<'a, S, M, E, I, C, SolverResult>> =
            vec![];
        let mut unreachable: Vec<UnreachableSymbolicMachineResult<'a, S, M, E, I, C>> = vec![];

        loop {
            let start_branch = queue.pop();
            if let Some((machine, constraints)) = start_branch {
                if machine.can_continue() {
                    let new_machines = self.inner_interpreter.step(machine)?;

                    new_machines
                        .into_iter()
                        .for_each(|(new_mach, constraints_to_add)| {
                            let mut new_constraints: Vec<C> = constraints.clone();

                            if constraints_to_add.is_empty() {
                                queue.push((new_mach, new_constraints));
                            } else {
                                new_constraints.extend(constraints_to_add);
                                if self.solver.solve(&new_constraints).is_some() {
                                    queue.push((new_mach, new_constraints));
                                } else {
                                    unreachable.push(UnreachableSymbolicMachineResult {
                                        machine: new_mach,
                                        constraints: new_constraints,
                                    })
                                }
                            }
                        });
                } else {
                    match self.solver.solve(&constraints) {
                        Some(model) => reachable.push(ReachableSymbolicMachineResult {
                            machine,
                            constraints,
                            model,
                        }),
                        None => unreachable.push(UnreachableSymbolicMachineResult {
                            machine,
                            constraints,
                        }),
                    }
                }
            } else {
                break;
            }
        }

        Ok(SymbolicMachineResults {
            reachable,
            unreachable,
        })
    }
}
