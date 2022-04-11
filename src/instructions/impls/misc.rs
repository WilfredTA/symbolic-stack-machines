use crate::{
    instructions::{ConcreteVMInstruction, ExecRecord, InstructionResult, Binary},
    memory::{ReadOnlyMem, WriteableMem, MemRecord, MemOpRecord, Mem},
    stack::{Stack, StackOpRecord, StackRecord},
};

#[derive(Debug)]
pub struct PUSH<T>(pub T);

impl<T, S, M> ConcreteVMInstruction<S, M> for PUSH<T>
where
    T: Clone + std::fmt::Debug,
    S: Stack<StackVal = T>,
    M: Mem,
{
    fn exec(&self, _stack: &S, _memory: &M) -> InstructionResult<ExecRecord<S, M>> {
        let mut change_log = ExecRecord::default();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Push(self.0.clone())],
        });

        Ok(change_log)
    }
}

#[derive(Debug)]
pub struct STOP;

impl<S, M> ConcreteVMInstruction<S, M> for STOP
where
    S: Stack,
    M: Mem,
{
    fn exec(&self, _stack: &S, _memory: &M) -> InstructionResult<ExecRecord<S, M>> {
        let mut change_log = ExecRecord::default();

        change_log.halt = true;

        Ok(change_log)
    }
}

#[derive(Debug)]
pub struct JUMPI;

impl<T, S, M> ConcreteVMInstruction<S, M> for JUMPI
where
    T: Binary + Eq + TryInto<usize>,
    S: Stack<StackVal = T>,
    M: Mem,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
{
    fn exec(&self, stack: &S, _memory: &M) -> InstructionResult<ExecRecord<S, M>> {
        let mut change_log = ExecRecord::default();

        let dest: T = stack.peek(0).unwrap();
        let cond: T = stack.peek(1).unwrap();

        if cond != T::zero() {
            let x = dest.try_into().unwrap();
            change_log.pc_change = Some(x);
        }

        Ok(change_log)
    }
}

#[derive(Debug)]
pub struct MLOAD;

impl<T, S, M> ConcreteVMInstruction<S, M> for MLOAD
where
    T: Default + Clone,
    S: Stack<StackVal = T>,
    M: ReadOnlyMem<Index = T, MemVal = T>,
{
    fn exec(&self, stack: &S, memory: &M) -> InstructionResult<ExecRecord<S, M>> {
        let mut change_log = ExecRecord::default();

        let mem_idx: T = stack.peek(0).unwrap();
        let mem_val = memory
            .read(mem_idx.clone())
            .unwrap()
            .unwrap();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![
                StackOpRecord::Pop(mem_idx),
                StackOpRecord::Push(mem_val),
            ],
        });

        Ok(change_log)
    }
}

#[derive(Debug)]
pub struct MSTORE;

impl<T, S, M> ConcreteVMInstruction<S, M> for MSTORE
where 
    T: Clone,
    S: Stack<StackVal = T>,
    M: WriteableMem<Index = T, MemVal = T>,
{
    fn exec(&self, stack: &S, _memory: &M) -> InstructionResult<ExecRecord<S, M>> {
        let mut change_log = ExecRecord::default();

        let mem_idx: T = stack.peek(0).unwrap();
        let mem_val: T = stack.peek(1).unwrap();

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
