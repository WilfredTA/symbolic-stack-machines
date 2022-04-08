pub mod error;

use crate::instructions::*;
use crate::memory::ReadOnlyMem;
use crate::{memory::RWMem, stack::*};
use error::MachineError;

pub type MachineResult<T> = Result<T, MachineError>;

pub type Program<I> = Vec<I>;

pub struct BaseMachine<Mem, MachineStack, I, MemIdx, MemVal, StackVal>
where
    Mem: RWMem + ReadOnlyMem<Index = MemIdx, MemVal = MemVal>,
    MachineStack: Stack<StackVal = StackVal>,
    I: VMInstruction<Mem = Mem, ValStack = MachineStack>,
    StackVal: Into<MemIdx> + Into<MemVal>,
{
    mem: Mem,
    stack: MachineStack,
    #[allow(dead_code)]
    pgm: Program<I>,
    #[allow(dead_code)]
    pc: usize,
}

impl<Mem, MachineStack, I, MemIdx, MemVal, StackVal>
    BaseMachine<Mem, MachineStack, I, MemIdx, MemVal, StackVal>
where
    Mem: RWMem + ReadOnlyMem<Index = MemIdx, MemVal = MemVal> + std::fmt::Debug + Clone,
    MachineStack: Stack<StackVal = StackVal> + std::fmt::Debug + Clone,
    I: VMInstruction<Mem = Mem, ValStack = MachineStack>,
    StackVal: Into<MemIdx> + Into<MemVal>,
{
    pub fn new(stack: MachineStack, mem: Mem) -> Self {
        Self {
            mem,
            stack,
            pgm: vec![],
            pc: 0,
        }
    }

    pub fn run(self, pgm: &Program<I>) -> Option<MachineStack::StackVal>
    where
        Mem: Clone,
        MachineStack: Clone,
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
