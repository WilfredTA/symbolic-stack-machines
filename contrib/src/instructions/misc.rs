use symbolic_stack_machines_core::{
    constraint::Constraint,
    instructions::{AbstractExecRecord, AbstractInstruction, EnvExtension, InstructionResult},
    memory::{Mem, MemOpRecord, MemRecord, ReadOnlyMem, WriteableMem},
    stack::{Stack, StackOpRecord, StackRecord},
};

pub struct PUSH<T>(pub T);

impl<T, S, M, Extension, ReturnRecord, C> AbstractInstruction<S, M, Extension, ReturnRecord, C>
    for PUSH<T>
where
    T: Clone + std::fmt::Debug,
    S: Stack<StackVal = T>,
    M: Mem,
    Extension: EnvExtension,
    C: Into<Constraint<C>>,
{
    fn exec(
        &self,
        _stack: &S,
        _memory: &M,
        _ext: &Extension,
    ) -> InstructionResult<AbstractExecRecord<S, M, Extension::DiffRecordType, C>> {
        let mut change_log = AbstractExecRecord::default();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Push(self.0.clone())],
        });

        Ok(change_log)
    }
}

pub struct STOP;

impl<S, M, Extension, ReturnRecord, C> AbstractInstruction<S, M, Extension, ReturnRecord, C>
    for STOP
where
    S: Stack,
    M: Mem,
    Extension: EnvExtension,
    C: Into<Constraint<C>>,
{
    fn exec(
        &self,
        _stack: &S,
        _memory: &M,
        _ext: &Extension,
    ) -> InstructionResult<AbstractExecRecord<S, M, Extension::DiffRecordType, C>> {
        let mut change_log = AbstractExecRecord::default();

        change_log.halt = true;

        Ok(change_log)
    }
}

pub struct JUMPI;

impl<T, S, M, Extension, ReturnRecord, C> AbstractInstruction<S, M, Extension, ReturnRecord, C>
    for JUMPI
where
    T: From<u8> + Eq + TryInto<usize>,
    S: Stack<StackVal = T>,
    M: Mem,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
    Extension: EnvExtension,
    C: Into<Constraint<C>>,
{
    fn exec(
        &self,
        stack: &S,
        _memory: &M,
        _ext: &Extension,
    ) -> InstructionResult<AbstractExecRecord<S, M, Extension::DiffRecordType, C>> {
        let mut change_log = AbstractExecRecord::default();

        let dest: T = stack.peek(0).unwrap();
        let cond: T = stack.peek(1).unwrap();

        if cond != T::from(0) {
            let x = dest.try_into().unwrap();
            change_log.pc_change = Some(x);
        }

        Ok(change_log)
    }
}

pub struct MLOAD;

impl<T, S, M, Extension, ReturnRecord, C> AbstractInstruction<S, M, Extension, ReturnRecord, C>
    for MLOAD
where
    T: Default + Clone,
    S: Stack<StackVal = T>,
    M: ReadOnlyMem<Index = T, MemVal = T>,
    Extension: EnvExtension,
    C: Into<Constraint<C>>,
{
    fn exec(
        &self,
        stack: &S,
        memory: &M,
        _ext: &Extension,
    ) -> InstructionResult<AbstractExecRecord<S, M, Extension::DiffRecordType, C>> {
        let mut change_log = AbstractExecRecord::default();

        let mem_idx: T = stack.peek(0).unwrap();
        let mem_val = memory.read(mem_idx.clone()).unwrap().unwrap();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Pop(mem_idx), StackOpRecord::Push(mem_val)],
        });

        Ok(change_log)
    }
}

pub struct MSTORE;

impl<T, S, M, Extension, ReturnRecord, C> AbstractInstruction<S, M, Extension, ReturnRecord, C>
    for MSTORE
where
    T: Clone,
    S: Stack<StackVal = T>,
    M: WriteableMem<Index = T, MemVal = T>,
    Extension: EnvExtension,
    C: Into<Constraint<C>>,
{
    fn exec(
        &self,
        stack: &S,
        _memory: &M,
        _ext: &Extension,
    ) -> InstructionResult<AbstractExecRecord<S, M, Extension::DiffRecordType, C>> {
        let mut change_log = AbstractExecRecord::default();

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
