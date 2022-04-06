use crate::{
    memory::{symbolic_concrete_index::MemConcreteIntToConcreteInt, Mem, WriteableMem},
    stack::{ConcreteIntStack, Stack},
};

use super::{Machine, ConcreteProgram};

pub struct BaseConcreteMachine<'a, S, M>
where
    S: Stack,
    M: Mem,
{
    stack: S,
    mem: M,
    pgm: &'a ConcreteProgram<S, M>,
    pc: usize,
}

impl<'a, S, M> BaseConcreteMachine<'a, S, M>
where
    S: Stack,
    M: Mem,
{
    pub fn new(stack: S, mem: M, pgm: &'a ConcreteProgram<S, M>) -> Self {
        Self {
            mem,
            stack,
            pgm,
            pc: 0,
        }
    }
}

impl<'a, S, M> Machine<S, M, Option<S::StackVal>> for BaseConcreteMachine<'a, S, M>
where
    S: Stack + Clone,
    M: WriteableMem + Clone,
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

pub type ConcreteIntMachine<'a> =
    BaseConcreteMachine<'a, ConcreteIntStack, MemConcreteIntToConcreteInt>;
