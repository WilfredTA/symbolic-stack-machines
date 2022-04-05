pub mod error;
use crate::instructions::*;
use crate::memory::symbolic_concrete_index::MemIntToInt;
use crate::memory::{Mem, ReadOnlyMem, WriteableMem};
use crate::{memory::RWMem, stack::*};
use error::MachineError;
use z3::ast::Bool;
use z3::{Context, Model, SatResult, Solver};

pub type MachineResult<T> = Result<T, MachineError>;

pub type Program<I> = Vec<I>;

#[derive(Debug)]
pub struct SymbolicContext<PathConstraint> {
    pub constraints: Vec<PathConstraint>,
}

#[derive(Debug)]
pub struct BaseMachine<S, M, PathConstraint>
where
    S: Stack,
    M: Mem,
{
    stack: S,
    mem: M,
    pgm: Program<Box<dyn VMInstruction<S, M, ()>>>,
    pc: usize,
    context: Option<SymbolicContext<PathConstraint>>,
}

impl<S, M, PathConstraint> BaseMachine<S, M, PathConstraint>
where
    S: Stack + std::fmt::Debug + Clone,
    M: Mem + WriteableMem + std::fmt::Debug + Clone,
{
    pub fn new(stack: S, mem: M) -> Self {
        Self {
            mem,
            stack,
            pgm: vec![],
            pc: 0,
            context: None,
        }
    }

    pub fn new_with_ctx(stack: S, mem: M) -> Self {
        let context = SymbolicContext {
            constraints: vec![],
        }
        .into();

        Self {
            mem,
            stack,
            pgm: vec![],
            pc: 0,
            context,
        }
    }

    pub fn run_sym<'a>(
        self,
        pgm: &Program<Box<dyn VMInstruction<S, M, ()>>>,
        ctx: &'a Context,
    ) -> (
        Vec<((usize, S, M, Vec<z3::ast::Bool<'a>>), Option<Model<'a>>)>,
        Vec<((usize, S, M, Vec<z3::ast::Bool<'a>>), Option<Model<'a>>)>,
    ) {
        type Branch<'a, S, M> = (usize, S, M, Vec<Bool<'a>>);
        let stack = self.stack.clone();
        let mem = self.mem.clone();
        let execute = |pc: &mut usize,
                       pgm: &Program<Box<dyn VMInstruction<S, M, ()>>>,
                       mut stack: S,
                       mut mem: M|
         -> (Option<Branch<S, M>>, Option<Branch<S, M>>) {
            // for inst in &pgm[pc.clone()..] {
            //     let rec = inst.exec(&stack, &mem).unwrap();
            //     println!("EXEC RECORD CONSTRAINTS: {:?}", rec.path_constraints);
            //     if rec.halt || pc.clone() == pgm.len() {
            //         return (None, None);
            //     } else {
            //         println!("STACK BEFORE APPLY: {:?}", stack);
            //         stack = {
            //             if let Some(stack_diff) = rec.stack_diff {
            //                 stack_diff.apply(stack).unwrap()
            //             } else {
            //                 stack
            //             }
            //         };
            //         println!("STACK AFTER APPLY: {:?}", stack);

            //         mem = {
            //             if let Some(mem_diff) = rec.mem_diff {
            //                 mem_diff.apply(mem).unwrap()
            //             } else {
            //                 mem
            //             }
            //         };

            //         if rec.path_constraints.len() == 1 {
            //             let curr_path_constraints = rec.path_constraints.first().cloned().unwrap();
            //             return (
            //                 Some((
            //                     pc.clone() + 1,
            //                     stack.clone(),
            //                     mem.clone(),
            //                     curr_path_constraints,
            //                 )),
            //                 None,
            //             );
            //         }

            //         if rec.path_constraints.len() == 2 {
            //             let branch_one_rules = rec.path_constraints.first().cloned().unwrap();
            //             let branch_two_rules = rec.path_constraints.get(1).cloned().unwrap();

            //             return (
            //                 Some((pc.clone() + 1, stack.clone(), mem.clone(), branch_one_rules)),
            //                 Some((
            //                     rec.pc_change.unwrap(),
            //                     stack.clone(),
            //                     mem.clone(),
            //                     branch_two_rules,
            //                 )),
            //             );
            //         }
            //         return (
            //             Some((pc.clone() + 1, stack.clone(), mem.clone(), vec![])),
            //             None,
            //         );
            //     }
            // }
            return (None, None);
        };

        let mut trace_tree: Vec<Branch<S, M>> = vec![];
        trace_tree.push((0, stack.clone(), mem.clone(), vec![]));
        let mut leaves: Vec<Branch<S, M>> = vec![];
        loop {
            let start_branch = trace_tree.pop();
            if let Some(start_branch) = start_branch {
                let (mut pc, stack, mem, mut constraints) = start_branch;
                let branches = execute(&mut pc, pgm, stack.clone(), mem.clone());
                println!("BRANCHES AFTER ONE EXEC: {:?}", branches);
                match branches {
                    (None, None) => {
                        // A branch reached the end of program or halted; move to next traversal and
                        // store this possible end state
                        leaves.push((pc, stack, mem, constraints));
                    }
                    (None, Some(_)) => {
                        panic!("This should never happen");
                    }
                    (Some(branch), None) => {
                        // Only one possible path but constraints were added
                        constraints.extend(branch.3);
                        trace_tree.push((branch.0, branch.1, branch.2, constraints));
                    }
                    (Some(b1), Some(b2)) => {
                        // Branch condition has been introduced; traverse down b1 then b2
                        let mut b2_constraints = constraints.clone();
                        b2_constraints.extend(b2.3);
                        constraints.extend(b1.3);

                        trace_tree.push((b2.0, b2.1, b2.2, b2_constraints));
                        trace_tree.push((b1.0, b1.1, b1.2, constraints));
                    }
                }
            } else {
                break;
            }
        }

        // println!("Final LEAVES: {:?}", leaves);

        let mut reachable = vec![];
        let mut unreachable = vec![];

        for leaf in leaves {
            let solver = Solver::new(ctx);
            let constraints = &leaf.3;
            for constraint in constraints {
                solver.assert(&constraint);
            }
            let sat = solver.check();
            if let SatResult::Sat = sat {
                reachable.push((leaf, solver.get_model()));
            } else {
                unreachable.push((leaf, None));
            }
        }
        println!("Unreachable leaves: {:?}", unreachable);
        println!("Reachable leaves: {:?}", reachable);
        return (reachable, unreachable);
    }
    pub fn run(self, pgm: &Program<Box<dyn VMInstruction<S, M, ()>>>) -> Option<S::StackVal>
    where
        M: Clone,
        S: Clone,
    {
        let mut stack = self.stack.clone();
        let mut mem = self.mem.clone();

        for inst in pgm {
            let rec = inst.exec(&stack, &mem).unwrap();
            stack = {
                if let Some(stack_diff) = rec.stack_diff {
                    stack_diff.apply(stack).unwrap()
                } else {
                    stack
                }
            };

            mem = {
                if let Some(mem_diff) = rec.mem_diff {
                    mem_diff.apply(mem).unwrap()
                } else {
                    mem
                }
            };
        }

        stack.peek(0)
    }
}

pub type ConcreteIntMachine =
    BaseMachine<IntStack, MemIntToInt, ()>;
