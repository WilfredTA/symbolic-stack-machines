use crate::{
    instructions::{DynConcreteVMInstruction, ConcreteVMInstruction},
    memory::{concrete_index::MemConcreteIntToConcreteInt, Mem, WriteableMem},
    stack::{ConcreteIntStack, Stack},
};

use super::{ConcreteMachine, Program, BaseMachine};

#[derive(Debug)]
pub struct BaseConcreteMachine<'a, S, M, I>
where
    S: Stack,
    M: Mem,
    I: ConcreteVMInstruction<S, M>,
{
    pub stack: S,
    pub mem: M,
    pub pgm: &'a Program<I>,
    pub pc: usize,
}

impl<'a, S, M, I> BaseConcreteMachine<'a, S, M, I>
where
    S: Stack,
    M: Mem,
    I: ConcreteVMInstruction<S, M>,
{
    pub fn new(stack: S, mem: M, pgm: &'a Program<I>, pc: Option<usize>) -> Self {
        Self {
            mem,
            stack,
            pgm,
            pc: pc.unwrap_or(0),
        }
    }
}

impl<'a, S, M, I> BaseMachine<S, M, Option<S::StackVal>, I> for BaseConcreteMachine<'a, S, M, I>
where
    S: Stack + Clone,
    M: WriteableMem + Clone,
    I: ConcreteVMInstruction<S, M>,
{
    fn can_exec(&self) -> bool {
        self.pc < self.pgm.len()
    }

    fn peek_instruction(&self) -> Option<&I> {
        self.pgm.get(self.pc)
    }

    fn return_value(&self) -> Option<S::StackVal> {
        self.stack.peek(0)
    }
}

impl<'a, S, M, I> ConcreteMachine<S, M, Option<S::StackVal>, I> for BaseConcreteMachine<'a, S, M, I>
where
    S: Stack + Clone,
    M: WriteableMem + Clone,
    I: ConcreteVMInstruction<S, M>,
{
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
    DynConcreteVMInstruction<ConcreteIntStack, MemConcreteIntToConcreteInt>,
>;
