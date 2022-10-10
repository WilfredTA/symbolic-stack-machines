use crate::{constraint::Constraint, instructions::AbstractInstruction, environment::Env};

use super::{
    inner_interpreter::{AbstractExecBranch, InnerInterpreter},
    r#abstract::AbstractMachine,
    MachineResult,
};

pub trait OuterInterpreter<Output, M> {
    fn run(&self, m: M) -> MachineResult<Output>;
}

pub struct ConcreteOuterInterpreter<'a, I, InstructionStepResult, InterpreterStepResult, E, V>
where
    I: AbstractInstruction<InstructionStepResult, E, V>,
    V: Default + Clone,
    E: Env,
{
    pub inner_interpreter:
        Box<dyn InnerInterpreter<'a, I, InstructionStepResult, InterpreterStepResult, E, V>>,
}

impl<'a, I, InstructionStepResult, E, V> OuterInterpreter<AbstractMachine<'a, I, E, V>, AbstractMachine<'a, I, E, V>>
    for ConcreteOuterInterpreter<'a, I, InstructionStepResult, AbstractMachine<'a, I, E, V>, E, V>
where
    I: AbstractInstruction<InstructionStepResult, E, V>,
    V: Default + Clone,
    E: Env
{
    fn run(&self, m: AbstractMachine<'a, I, E, V>) -> MachineResult<AbstractMachine<'a, I, E, V>> {
        let mut x = m;

        while x.can_continue() {
            x = self.inner_interpreter.step(x)?;
        }

        Ok(x)
    }
}

pub struct SymbolicOuterInterpreter<'a, I, InstructionStepResult, InterpreterStepResult, E, V>
where
    I: AbstractInstruction<InstructionStepResult, E, V>,
    V: Default + Clone,
    E: Env,
{
    inner_interpreter: dyn InnerInterpreter<'a, I, InstructionStepResult, InterpreterStepResult, E, V>,
}

pub type SingleBranch<'a, I, E, V> = (AbstractMachine<'a, I, E, V>, Vec<Constraint>);

impl<'a, I, InstructionStepResult, E, V>
    OuterInterpreter<Vec<SingleBranch<'a, I, E, V>>, AbstractMachine<'a, I, E, V>>
    for SymbolicOuterInterpreter<'a, I, InstructionStepResult, AbstractExecBranch<'a, I, E, V>, E, V>
where
    I: AbstractInstruction<InstructionStepResult, E, V>,
    V: Default + Clone,
    E: Env
{
    fn run(&self, m: AbstractMachine<'a, I, E, V>) -> MachineResult<Vec<SingleBranch<'a, I, E, V>>> {
        let mut trace_tree: Vec<SingleBranch<'a, I, E, V>> = vec![(m, vec![])];

        let mut leaves: Vec<SingleBranch<'a, I, E, V>> = vec![];

        loop {
            let start_branch = trace_tree.pop();
            if let Some((mach, constraints)) = start_branch {
                if mach.can_continue() {
                    let new_machines = self.inner_interpreter.step(mach)?;

                    new_machines
                        .into_iter()
                        .for_each(|(new_mach, constraints_to_add)| {
                            let mut new_constraints: Vec<Constraint> = constraints.clone();
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
