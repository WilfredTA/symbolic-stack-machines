use crate::{
    instructions::{ConcreteVMInstruction, VMInstruction},
    memory::{symbolic_concrete_index::MemConcreteIntToConcreteInt, Mem, WriteableMem},
    stack::{ConcreteIntStack, Stack},
};

use super::{Machine, Program};

pub struct BaseConcreteMachine<'a, S, M, I>
where
    S: Stack,
    M: Mem,
    I: VMInstruction<S, M>,
{
    stack: S,
    mem: M,
    pgm: &'a Program<I>,
    pc: usize,
}

impl<'a, S, M, I> BaseConcreteMachine<'a, S, M, I>
where
    S: Stack,
    M: Mem,
    I: VMInstruction<S, M>,
{
    pub fn new(stack: S, mem: M, pgm: &'a Program<I>) -> Self {
        Self {
            mem,
            stack,
            pgm,
            pc: 0,
        }
    }
}

impl<'a, S, M, I> Machine<S, M, Option<S::StackVal>> for BaseConcreteMachine<'a, S, M, I>
where
    S: Stack + Clone,
    M: WriteableMem + Clone,
    I: VMInstruction<S, M>,
{
    fn can_exec(&self) -> bool {
        self.pc < self.pgm.len()
    }

    fn return_value(&self) -> Option<S::StackVal> {
        self.stack.peek(0)
    }

    fn exec(&self) -> Self {
        let inst = self.pgm.get(self.pc).unwrap();
        let rec = inst.exec(&self.stack, &self.mem).unwrap();

        let stack = {
            if let Some(stack_diff) = rec.stack_diff {
                stack_diff.apply(self.stack.clone()).unwrap()
            } else {
                self.stack.clone()
            }
        };

        let mem = {
            if let Some(mem_diff) = rec.mem_diff {
                mem_diff.apply(self.mem.clone()).unwrap()
            } else {
                self.mem.clone()
            }
        };

        let pc = {
            if let Some(pc_change) = rec.pc_change {
                pc_change
            } else {
                self.pc + 1
            }
        };

        let pc = if rec.halt { self.pgm.len() } else { pc };

        Self {
            mem,
            stack,
            pgm: self.pgm,
            pc,
        }
    }
}

pub type ConcreteIntMachine<'a> = BaseConcreteMachine<
    'a,
    ConcreteIntStack,
    MemConcreteIntToConcreteInt,
    ConcreteVMInstruction<ConcreteIntStack, MemConcreteIntToConcreteInt>,
>;
