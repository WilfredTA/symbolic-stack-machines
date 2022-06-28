use symbolic_stack_machines_core::{
    environment::EnvExtension,
    instructions::{
        AbstractExecRecord, AbstractInstruction, ConcreteAbstractExecRecord, InstructionResult,
    },
    memory::Mem,
    stack::{Stack, StackOpRecord, StackRecord, ZERO, ONE},
};

pub struct ISZERO;

impl<M, Extension>
    AbstractInstruction<
        M,
        Extension,
        ConcreteAbstractExecRecord<M, Extension::DiffRecordType>,
    > for ISZERO
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

        let op = stack.peek(0).unwrap();

        let rv = op._eq(&ZERO).ite(ONE, ZERO);

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Pop, StackOpRecord::Push(rv)],
        });

        Ok(change_log)
    }
}
