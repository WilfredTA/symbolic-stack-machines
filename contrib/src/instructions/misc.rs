use symbolic_stack_machines_core::{
    environment::EnvExtension,
    instructions::{
        AbstractExecRecord, AbstractInstruction, ConcreteAbstractExecRecord, InstructionResult,
    },
    memory::{Mem, MemOpRecord, MemRecord, ReadOnlyMem, WriteableMem},
    stack::{Stack, StackOpRecord, StackRecord, StackVal, ZERO},
};

pub struct PUSH(pub StackVal);

impl<M, Extension>
    AbstractInstruction<
        M,
        Extension,
        ConcreteAbstractExecRecord<M, Extension::DiffRecordType>,
    > for PUSH
where
    M: Mem,
    Extension: EnvExtension,
{
    fn exec(
        &self,
        _stack: &Stack,
        _memory: &M,
        _ext: &Extension,
    ) -> InstructionResult<ConcreteAbstractExecRecord<M, Extension::DiffRecordType>> {
        let mut change_log = AbstractExecRecord::default();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Push(self.0.clone())],
        });

        Ok(change_log)
    }
}

pub struct STOP;

impl<M, Extension>
    AbstractInstruction<
        M,
        Extension,
        ConcreteAbstractExecRecord<M, Extension::DiffRecordType>,
    > for STOP
where
    M: Mem,
    Extension: EnvExtension,
{
    fn exec(
        &self,
        _stack: &Stack,
        _memory: &M,
        _ext: &Extension,
    ) -> InstructionResult<ConcreteAbstractExecRecord<M, Extension::DiffRecordType>> {
        let mut change_log = AbstractExecRecord::default();

        change_log.halt = true;

        Ok(change_log)
    }
}

pub struct JUMPI;

impl<M, Extension>
    AbstractInstruction<
        M,
        Extension,
        ConcreteAbstractExecRecord<M, Extension::DiffRecordType>,
    > for JUMPI
where
    M: Mem,
    Extension: EnvExtension,
{
    fn exec(
        &self,
        stack: &Stack,
        _memory: &M,
        _ext: &Extension,
    ) -> InstructionResult<ConcreteAbstractExecRecord<M, Extension::DiffRecordType>> {
        let mut change_log = AbstractExecRecord::default();

        let dest = stack.peek(0).unwrap();
        let cond = stack.peek(1).unwrap();

        if *cond != ZERO {
            let x = Into::<usize>::into(*dest);
            change_log.pc_change = Some(x);
        }

        Ok(change_log)
    }
}

pub struct MLOAD;

impl<M, Extension>
    AbstractInstruction<
        M,
        Extension,
        ConcreteAbstractExecRecord<M, Extension::DiffRecordType>,
    > for MLOAD
where
    M: ReadOnlyMem<Index = StackVal, MemVal = StackVal>,
    Extension: EnvExtension,
{
    fn exec(
        &self,
        stack: &Stack,
        memory: &M,
        _ext: &Extension,
    ) -> InstructionResult<ConcreteAbstractExecRecord<M, Extension::DiffRecordType>> {
        let mut change_log = AbstractExecRecord::default();

        let mem_idx = stack.peek(0).unwrap();
        let mem_val = memory.read(mem_idx.clone()).unwrap().unwrap();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Pop, StackOpRecord::Push(mem_val)],
        });

        Ok(change_log)
    }
}

pub struct MSTORE;

impl<M, Extension>
    AbstractInstruction<
        M,
        Extension,
        ConcreteAbstractExecRecord<M, Extension::DiffRecordType>,
    > for MSTORE
where
    M: WriteableMem<Index = StackVal, MemVal = StackVal>,
    Extension: EnvExtension,
{
    fn exec(
        &self,
        stack: &Stack,
        _memory: &M,
        _ext: &Extension,
    ) -> InstructionResult<ConcreteAbstractExecRecord<M, Extension::DiffRecordType>> {
        let mut change_log = AbstractExecRecord::default();

        let mem_idx = stack.peek(0).unwrap();
        let mem_val = stack.peek(1).unwrap();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![
                StackOpRecord::Pop,
                StackOpRecord::Pop,
            ],
        });

        change_log.mem_diff = Some(MemRecord {
            diff: vec![MemOpRecord::Write((*mem_idx, *mem_val))],
        });

        Ok(change_log)
    }
}
