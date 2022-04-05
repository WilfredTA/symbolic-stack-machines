use crate::{
    memory::{Mem, MemOpRecord, MemRecord, ReadOnlyMem, WriteableMem},
    stack::{Stack, StackOpRecord, StackRecord},
};

use super::{ExecRecord, VMInstruction};

use std::fmt::Debug;

#[derive(Debug)]
pub struct PUSH<T>(pub T);

impl<T, S, M, PC> VMInstruction<S, M, PC> for PUSH<T>
where
    T: Clone,
    S: Stack<StackVal = T>,
    M: Mem,
{
    fn exec(
        &self,
        _stack: &S,
        _memory: &M,
    ) -> super::InstructionResult<super::ExecRecord<S, M, PC>> {
        let mut change_log = ExecRecord::default();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Push(self.0.clone())],
        });

        Ok(change_log)
    }
}

#[derive(Debug)]
pub struct STOP;

impl<S, M, PC> VMInstruction<S, M, PC> for STOP
where
    S: Stack,
    M: Mem,
{
    fn exec(
        &self,
        _stack: &S,
        _memory: &M,
    ) -> super::InstructionResult<super::ExecRecord<S, M, PC>> {
        let mut change_log = ExecRecord::default();

        change_log.halt = true;

        Ok(change_log)
    }
}

#[derive(Debug)]
pub struct JUMPI;

impl<T, S, M, PC> VMInstruction<S, M, PC> for JUMPI
where
    T: Default + Eq + TryInto<usize>,
    S: Stack<StackVal = T>,
    M: Mem,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
{
    fn exec(
        &self,
        stack: &S,
        _memory: &M,
    ) -> super::InstructionResult<super::ExecRecord<S, M, PC>> {
        let mut change_log = ExecRecord::default();

        let dest = stack.peek(0).unwrap();
        let cond = stack.peek(1).unwrap();

        if cond != T::default() {
            let x = dest.try_into().unwrap();
            change_log.pc_change = Some(x);
        }

        Ok(change_log)
    }
}

pub struct MLOAD;

impl<T, S, M, PC> VMInstruction<S, M, PC> for MLOAD
where
    T: TryInto<M::Index> + Clone,
    M: ReadOnlyMem<MemVal = T>,
    S: Stack<StackVal = T>,
    <T as TryInto<<M as Mem>::Index>>::Error: std::fmt::Debug,
{
    fn exec(&self, stack: &S, memory: &M) -> super::InstructionResult<super::ExecRecord<S, M, PC>> {
        let mut change_log = ExecRecord::default();

        let mem_idx = stack.peek(0).unwrap();
        let mem_val = memory
            .read(mem_idx.clone().try_into().unwrap())
            .unwrap()
            .unwrap();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Pop(mem_idx), StackOpRecord::Push(mem_val)],
        });

        Ok(change_log)
    }
}

pub struct MSTORE;

impl<T, S, M, PC> VMInstruction<S, M, PC> for MSTORE
where
    T: TryInto<M::Index> + Clone,
    M: WriteableMem<MemVal = T>,
    S: Stack<StackVal = T>,
    <T as TryInto<<M as Mem>::Index>>::Error: std::fmt::Debug,
{
    fn exec(&self, stack: &S, _memory: &M) -> super::InstructionResult<ExecRecord<S, M, PC>> {
        let mut change_log = ExecRecord::default();

        let mem_idx = stack.peek(0).unwrap();
        let mem_val = stack.peek(1).unwrap();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![
                StackOpRecord::Pop(mem_idx.clone()),
                StackOpRecord::Pop(mem_val.clone()),
            ],
        });

        change_log.mem_diff = Some(MemRecord {
            diff: vec![MemOpRecord::Write((mem_idx.try_into().unwrap(), mem_val))],
        });

        Ok(change_log)
    }
}
