pub mod error;
use std::borrow::Borrow;
use std::rc::Rc;

use crate::instructions::*;
use crate::{
    memory::{symbolic::BaseSymbolicMem, RWMem},
    stack::*,
};
use error::MachineError;
use z3::ast::Int;
use z3::{Config, Context, Model, SatResult, Solver};

pub type MachineResult<T> = Result<T, MachineError>;

pub type Program<'a, I> = Vec<I>;

pub struct SymbolicContext<'a> {
    pub constraints: Vec<z3::ast::Bool<'a>>,
    pub ctx: Rc<&'a Context>,
}

pub struct BaseMachine<'a, Mem, MachineStack, I>
where
    Mem: RWMem,
    MachineStack: Stack,
    I: VMInstruction<'a, Mem = Mem, ValStack = MachineStack>,
{
    mem: Mem,
    stack: MachineStack,
    pgm: Program<'a, I>,
    pc: usize,
    context: Option<SymbolicContext<'a>>,
}

impl<'a, Mem, MachineStack, I> BaseMachine<'a, Mem, MachineStack, I>
where
    Mem: RWMem,
    MachineStack: Stack,
    I: VMInstruction<'a, Mem = Mem, ValStack = MachineStack>,
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

    pub fn run_sym(self, pgm: &Program<'a, I>) -> (SatResult, Option<Model<'a>>)
    where
        Mem: Clone,
        MachineStack: Clone,
    {
        let mut stack = self.stack.clone();
        let mut mem = self.mem.clone();
        let mut context = self.context.unwrap();
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

            if rec.path_constraints.len() > 0 {
                let curr_path_constraints = rec.path_constraints.first().cloned().unwrap();
                context.constraints.extend(curr_path_constraints);
            }
        }

        let solver = Solver::new(&context.ctx);
        for constraint in &context.constraints {
            solver.assert(constraint);
        }

        let is_sat = solver.check();
        let model = solver.get_model();
        println!("SAT: {:?}\nModel: {:?}", is_sat, model);

        (is_sat, model)
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

impl<'a, MachineStack, I> BaseMachine<'a, BaseSymbolicMem<'a>, MachineStack, I>
where
    MachineStack: Stack,
    I: VMInstruction<'a, Mem = BaseSymbolicMem<'a>, ValStack = MachineStack>,
{
    // For symbolic memory
    pub fn new_with_ctx(stack: MachineStack, mem_init_args: Rc<&'a Context>) -> Self {
        let mem = BaseSymbolicMem::init(mem_init_args.clone());
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
