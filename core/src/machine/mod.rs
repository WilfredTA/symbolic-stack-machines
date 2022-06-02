pub mod r#abstract;
pub mod error;
pub mod inner_interpreter;
pub mod outer_interpreter;

use std::rc::Rc;

use crate::instructions::*;
use crate::memory::ReadOnlyMem;
use crate::{
    memory::{memory_models::MemIntToInt, RWMem},
    stack::*,
};
use error::MachineError;
use z3::ast::{Bool, Int};
use z3::{Context, Model, SatResult, Solver};

pub type MachineResult<T> = Result<T, MachineError>;

pub type Program<'a, I> = Vec<I>;

pub struct SymbolicContext<'a> {
    pub constraints: Vec<z3::ast::Bool<'a>>,
    pub ctx: Rc<&'a Context>,
}

pub struct BaseMachine<'a, Mem, MachineStack, I, MemIdx, MemVal, StackVal>
where
    Mem: RWMem + ReadOnlyMem<Index = MemIdx, MemVal = MemVal>,
    MachineStack: Stack<StackVal = StackVal>,
    I: VMInstruction<'a, Mem = Mem, ValStack = MachineStack>,
    StackVal: Into<MemIdx> + Into<MemVal>,
{
    mem: Mem,
    stack: MachineStack,
    #[allow(dead_code)]
    pgm: Program<'a, I>,
    #[allow(dead_code)]
    pc: usize,
    context: Option<SymbolicContext<'a>>,
}

impl<'a, Mem, MachineStack, I, MemIdx, MemVal, StackVal>
    BaseMachine<'a, Mem, MachineStack, I, MemIdx, MemVal, StackVal>
where
    Mem: RWMem + ReadOnlyMem<Index = MemIdx, MemVal = MemVal> + std::fmt::Debug + Clone,
    MachineStack: Stack<StackVal = StackVal> + std::fmt::Debug + Clone,
    I: VMInstruction<'a, Mem = Mem, ValStack = MachineStack>,
    StackVal: Into<MemIdx> + Into<MemVal>,
{
    pub fn new(stack: MachineStack, mem_init: Mem::InitArgs) -> Self {
        let mem = Mem::init(mem_init);
        Self {
            mem,
            stack,
            pgm: vec![],
            pc: 0,
            context: None,
        }
    }

    pub fn run_sym(
        self,
        pgm: &Program<'a, I>,
    ) -> (
        Vec<(
            (usize, MachineStack, Mem, Vec<z3::ast::Bool<'a>>),
            Option<Model<'a>>,
        )>,
        Vec<(
            (usize, MachineStack, Mem, Vec<z3::ast::Bool<'a>>),
            Option<Model<'a>>,
        )>,
    ) {
        type Branch<'a, S, M> = (usize, S, M, Vec<Bool<'a>>);
        let stack = self.stack.clone();
        let mem = self.mem.clone();
        let context = self.context.unwrap();
        let execute = |pc: &mut usize,
                       pgm: &Program<'a, I>,
                       mut stack: MachineStack,
                       mut mem: Mem|
         -> (
            Option<Branch<MachineStack, Mem>>,
            Option<Branch<MachineStack, Mem>>,
        ) {
            for inst in &pgm[pc.clone()..] {
                let rec = inst.exec(&stack, &mem).unwrap();
                println!("EXEC RECORD CONSTRAINTS: {:?}", rec.path_constraints);
                if rec.halt || pc.clone() == pgm.len() {
                    return (None, None);
                } else {
                    println!("STACK BEFORE APPLY: {:?}", stack);
                    stack = {
                        if let Some(stack_diff) = rec.stack_diff {
                            stack_diff.apply(stack).unwrap()
                        } else {
                            stack
                        }
                    };
                    println!("STACK AFTER APPLY: {:?}", stack);

                    mem = {
                        if let Some(mem_diff) = rec.mem_diff {
                            mem_diff.apply(mem).unwrap()
                        } else {
                            mem
                        }
                    };

                    if rec.path_constraints.len() == 1 {
                        let curr_path_constraints = rec.path_constraints.first().cloned().unwrap();
                        return (
                            Some((
                                pc.clone() + 1,
                                stack.clone(),
                                mem.clone(),
                                curr_path_constraints,
                            )),
                            None,
                        );
                    }

                    if rec.path_constraints.len() == 2 {
                        let branch_one_rules = rec.path_constraints.first().cloned().unwrap();
                        let branch_two_rules = rec.path_constraints.get(1).cloned().unwrap();

                        return (
                            Some((pc.clone() + 1, stack.clone(), mem.clone(), branch_one_rules)),
                            Some((
                                rec.pc_change.unwrap(),
                                stack.clone(),
                                mem.clone(),
                                branch_two_rules,
                            )),
                        );
                    }
                    return (
                        Some((pc.clone() + 1, stack.clone(), mem.clone(), vec![])),
                        None,
                    );
                }
            }
            return (None, None);
        };

        let mut trace_tree: Vec<Branch<MachineStack, Mem>> = vec![];
        trace_tree.push((0, stack.clone(), mem.clone(), vec![]));
        let mut leaves: Vec<Branch<MachineStack, Mem>> = vec![];
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
            let solver = Solver::new(&context.ctx);
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
    pub fn run(self, pgm: &Program<'a, I>) -> Option<MachineStack::StackVal>
    where
        Mem: Clone,
        MachineStack: Clone,
    {
        let mut stack = self.stack.clone();
        let mut mem = self.mem.clone();

        for inst in pgm {
            let rec = inst.exec(&stack, &self.mem).unwrap();
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

// Implement machine initialization for a specific memory model
impl<'a, MachineStack, I>
    BaseMachine<'a, MemIntToInt<'a>, MachineStack, I, Int<'a>, Int<'a>, Int<'a>>
where
    MachineStack: Stack<StackVal = Int<'a>>,
    I: VMInstruction<'a, Mem = MemIntToInt<'a>, ValStack = MachineStack>,
{
    // For symbolic memory
    pub fn new_with_ctx(stack: MachineStack, mem_init_args: Rc<&'a Context>) -> Self {
        let mem = MemIntToInt::init(mem_init_args.clone());
        let ctx = SymbolicContext {
            constraints: vec![],
            ctx: mem_init_args.clone(),
        };

        Self {
            mem,
            stack,
            pgm: vec![],
            pc: 0,
            context: Some(ctx),
        }
    }
}
