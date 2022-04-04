use crate::{
    memory::{Mem, MemOpRecord, MemRecord, ReadOnlyMem, WriteableMem},
    stack::{Stack, StackOpRecord, StackRecord},
};

use super::{ExecRecord, VMInstruction};

pub struct PUSH<T>(pub T);

impl<T: Copy, ValStack: Stack<StackVal = T>, M: Mem, PathConstraint>
    VMInstruction<ValStack, M, PathConstraint> for PUSH<T>
{
    fn exec(
        &self,
        _stack: &ValStack,
        _memory: &M,
    ) -> super::InstructionResult<super::ExecRecord<ValStack, M, PathConstraint>> {
        let mut change_log = ExecRecord::default();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Push(self.0)],
        });

        Ok(change_log)
    }
}

pub struct STOP;

impl<T: Copy, ValStack: Stack<StackVal = T>, M: Mem, PathConstraint>
    VMInstruction<ValStack, M, PathConstraint> for STOP
{
    fn exec(
        &self,
        _stack: &ValStack,
        _memory: &M,
    ) -> super::InstructionResult<super::ExecRecord<ValStack, M, PathConstraint>> {
        let mut change_log = ExecRecord::default();

        change_log.halt = true;

        Ok(change_log)
    }
}

pub struct JUMPI;

impl<T, ValStack: Stack<StackVal = T>, M, PathConstraint> VMInstruction<ValStack, M, PathConstraint>
    for JUMPI
where
    usize: From<T>,
    M: Mem,
{
    fn exec(
        &self,
        stack: &ValStack,
        _memory: &M,
    ) -> super::InstructionResult<super::ExecRecord<ValStack, M, PathConstraint>> {
        let mut change_log = ExecRecord::default();

        let dest = usize::from(stack.peek(0).unwrap());
        let cond = usize::from(stack.peek(1).unwrap());

        if cond != 0 {
            change_log.pc_change = Some(dest);
        }

        Ok(change_log)
    }
}

pub struct MLOAD;

impl<T, S, M, PC> VMInstruction<S, M, PC> for MLOAD
where
    T: Copy,
    M: ReadOnlyMem<Index = T, MemVal = T>,
    S: Stack<StackVal = T>,
{
    fn exec(&self, stack: &S, memory: &M) -> super::InstructionResult<super::ExecRecord<S, M, PC>> {
        let mut change_log = ExecRecord::default();

        let mem_idx = stack.peek(0).unwrap();
        let mem_val = memory.read(mem_idx).unwrap().unwrap();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Pop(mem_idx), StackOpRecord::Push((mem_val))],
        });

        Ok(change_log)
    }
}

pub struct MSTORE;

impl<T, S, M, PC> VMInstruction<S, M, PC> for MSTORE
where
    T: Copy,
    M: WriteableMem<Index = T, MemVal = T>,
    S: Stack<StackVal = T>,
{
    fn exec(&self, stack: &S, _memory: &M) -> super::InstructionResult<ExecRecord<S, M, PC>> {
        let mut change_log = ExecRecord::default();

        let mem_idx = stack.peek(0).unwrap();
        let mem_val = stack.peek(1).unwrap();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Pop(mem_idx), StackOpRecord::Pop(mem_val)],
        });

        change_log.mem_diff = Some(MemRecord { diff: vec![
            MemOpRecord::Write((mem_idx, mem_val))
        ] });

        Ok(change_log)
    }
}
