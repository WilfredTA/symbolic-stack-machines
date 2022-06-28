use symbolic_stack_machines_core::{
    environment::EnvExtension,
    instructions::{
        AbstractExecRecord, AbstractInstruction, ConcreteAbstractExecRecord, InstructionResult,
    },
    memory::Mem,
    stack::{Stack, StackOpRecord, StackRecord},
};

pub struct ADD;

impl<M, Extension>
    AbstractInstruction<
        M,
        Extension,
        ConcreteAbstractExecRecord<M, Extension::DiffRecordType>,
    > for ADD
where
    M: Mem,
    Extension: EnvExtension,
{
    fn exec(
        &self,
        stack: &Stack,
        _mem: &M,
        _ext: &Extension,
    ) -> InstructionResult<ConcreteAbstractExecRecord<M, Extension::DiffRecordType>> {
        let mut change_log = AbstractExecRecord::default();

        let op_1 = stack.peek(0).unwrap();
        let op_2 = stack.peek(1).unwrap();
        let res = op_1.clone() + op_2.clone();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![
                StackOpRecord::Pop,
                StackOpRecord::Pop,
                StackOpRecord::Push(res),
            ],
        });

        Ok(change_log)
    }
}

pub struct SUB;

impl<M, Extension>
    AbstractInstruction<
        M,
        Extension,
        ConcreteAbstractExecRecord<M, Extension::DiffRecordType>,
    > for SUB
where
    M: Mem,
    Extension: EnvExtension,
{
    fn exec(
        &self,
        stack: &Stack,
        _mem: &M,
        _ext: &Extension,
    ) -> InstructionResult<ConcreteAbstractExecRecord<M, Extension::DiffRecordType>> {
        let mut change_log = AbstractExecRecord::default();

        let op_1 = stack.peek(0).unwrap();
        let op_2 = stack.peek(1).unwrap();
        let res = op_1.clone() - op_2.clone();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![
                StackOpRecord::Pop,
                StackOpRecord::Pop,
                StackOpRecord::Push(res),
            ],
        });

        Ok(change_log)
    }
}
