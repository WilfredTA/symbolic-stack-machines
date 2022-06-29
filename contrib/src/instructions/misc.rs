use symbolic_stack_machines_core::{
    environment::EnvExtension,
    instructions::{
        AbstractExecRecord, AbstractInstruction, ConcreteAbstractExecRecord, InstructionResult,
    },
    memory::{Memory, MemOpRecord, MemRecord},
    stack::{Stack, StackOpRecord, StackRecord, StackVal, ZERO},
};

pub struct PUSH(pub StackVal);

impl<Extension>
    AbstractInstruction<Extension, ConcreteAbstractExecRecord<Extension::DiffRecordType>>
    for PUSH
where
    Extension: EnvExtension,
{
    fn exec(
        &self,
        _stack: &Stack,
        _memory: &Memory,
        _ext: &Extension,
    ) -> InstructionResult<ConcreteAbstractExecRecord<Extension::DiffRecordType>> {
        let mut change_log = AbstractExecRecord::default();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Push(self.0.clone())],
        });

        Ok(change_log)
    }
}

pub struct STOP;

impl<Extension>
    AbstractInstruction<Extension, ConcreteAbstractExecRecord<Extension::DiffRecordType>>
    for STOP
where
    Extension: EnvExtension,
{
    fn exec(
        &self,
        _stack: &Stack,
        _memory: &Memory,
        _ext: &Extension,
    ) -> InstructionResult<ConcreteAbstractExecRecord<Extension::DiffRecordType>> {
        let mut change_log = AbstractExecRecord::default();

        change_log.halt = true;

        Ok(change_log)
    }
}

pub struct JUMPI;

impl<Extension>
    AbstractInstruction<Extension, ConcreteAbstractExecRecord<Extension::DiffRecordType>>
    for JUMPI
where
    Extension: EnvExtension,
{
    fn exec(
        &self,
        stack: &Stack,
        _memory: &Memory,
        _ext: &Extension,
    ) -> InstructionResult<ConcreteAbstractExecRecord<Extension::DiffRecordType>> {
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

impl<Extension>
    AbstractInstruction<Extension, ConcreteAbstractExecRecord<Extension::DiffRecordType>>
    for MLOAD
where
    Extension: EnvExtension,
{
    fn exec(
        &self,
        stack: &Stack,
        memory: &Memory,
        _ext: &Extension,
    ) -> InstructionResult<ConcreteAbstractExecRecord<Extension::DiffRecordType>> {
        let mut change_log = AbstractExecRecord::default();

        let mem_idx = stack.peek(0).unwrap();
        let mem_val = memory.read_word(mem_idx.clone()).unwrap();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Pop, StackOpRecord::Push(mem_val)],
        });

        Ok(change_log)
    }
}

pub struct MSTORE;

impl<Extension>
    AbstractInstruction<Extension, ConcreteAbstractExecRecord<Extension::DiffRecordType>>
    for MSTORE
where
    Extension: EnvExtension,
{
    fn exec(
        &self,
        stack: &Stack,
        _memory: &Memory,
        _ext: &Extension,
    ) -> InstructionResult<ConcreteAbstractExecRecord<Extension::DiffRecordType>> {
        let mut change_log = AbstractExecRecord::default();

        let mem_idx = stack.peek(0).unwrap();
        let mem_val = stack.peek(1).unwrap();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Pop, StackOpRecord::Pop],
        });

        change_log.mem_diff = Some(MemRecord {
            changed: vec![MemOpRecord::Write(*mem_idx, *mem_val)],
        });

        Ok(change_log)
    }
}
