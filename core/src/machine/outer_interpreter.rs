use crate::{
    constraint::Constraint,
    instructions::{AbstractInstruction, EnvExtension},
    memory::{Mem, WriteableMem},
    stack::Stack,
};

use super::{
    inner_interpreter::{AbstractExecBranch, InnerInterpreter},
    r#abstract::AbstractMachine,
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

pub struct SymbolicOuterInterpreter<'a, S, M, E, I, InstructionStepResult, InterpreterStepResult>
where
    S: Stack,
    M: Mem,
    E: EnvExtension,
    I: AbstractInstruction<S, M, E, InstructionStepResult>,
{
    inner_interpreter:
        dyn InnerInterpreter<'a, S, M, E, I, InstructionStepResult, InterpreterStepResult>,
}

pub type SingleBranch<'a, S, M, E, I, C> = (AbstractMachine<'a, S, M, E, I>, Vec<Constraint<C>>);

impl<'a, S, M, E, I, InstructionStepResult, C>
    OuterInterpreter<Vec<SingleBranch<'a, S, M, E, I, C>>, AbstractMachine<'a, S, M, E, I>>
    for SymbolicOuterInterpreter<
        'a,
        S,
        M,
        E,
        I,
        InstructionStepResult,
        AbstractExecBranch<'a, S, M, E, I, C>,
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
    ) -> MachineResult<Vec<SingleBranch<'a, S, M, E, I, C>>> {
        let mut trace_tree: Vec<SingleBranch<'a, S, M, E, I, C>> = vec![(m, vec![])];

        let mut leaves: Vec<SingleBranch<'a, S, M, E, I, C>> = vec![];

        loop {
            let start_branch = trace_tree.pop();
            if let Some((mach, constraints)) = start_branch {
                if mach.can_continue() {
                    let new_machines = self.inner_interpreter.step(mach)?;

                    new_machines
                        .into_iter()
                        .for_each(|(new_mach, constraints_to_add)| {
                            let mut new_constraints: Vec<Constraint<C>> = constraints.clone();
                            new_constraints.extend(constraints_to_add);
                            trace_tree.push((new_mach, new_constraints));
                        });
                } else {
                    leaves.push((mach, constraints));
                }
            } else {
                break;
            }
        }

        Ok(leaves)

        // TODO(will) - This doesn't include the `PathSummary` portion yet where we use the solver
        // to find the reachable paths and return models. The type signature of `ConcreteOuterInterpreter` is already incredibly
        // complex and I'd prefer to not add the additional generics that the solver is going to entail
        // in a separate PR.
        //
        // NOTE(will) - This type signature will have to change
        //          #[derive(Default)]
        //          pub struct PathSummary<IType, T, M>
        //          where
        //              IType: AbstractInstruction,
        //          {
        //              pub reachable: Vec<(SingleBranch<IType, T>, SatResult<M>)>,
        //              pub unreachable: Vec<(SingleBranch<IType, T>, SatResult<M>)>,
        //          }
        //
        //         let mut summary = PathSummary {
        //             reachable: vec![],
        //             unreachable: vec![],
        //         };

        //         if let Some(mut solver) = solver {
        //             for leaf in leaves {
        //                 let constraints = &leaf.1;
        //                 for constraint in constraints {
        //                     solver.generic_assert(constraint);
        //                 }
        //                 let sat = solver.solve();
        //                 if let SatResult::Sat(m) = sat {
        //                     summary.reachable.push((leaf, SatResult::Sat(m)));
        //                 } else {
        //                     summary.unreachable.push((leaf, SatResult::Unsat));
        //                 }
        //             }
        //         }

        //         Ok(summary)
    }
}
