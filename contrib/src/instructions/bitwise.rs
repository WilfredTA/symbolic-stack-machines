use symbolic_stack_machines_core::{
    environment::Env,
    instructions::{
        AbstractExecRecord, AbstractInstruction, ConcreteAbstractExecRecord, InstructionResult,
    },
    memory::Memory,
    stack::{Stack, StackOpRecord, StackRecord, ONE, ZERO},
};

pub struct ISZERO;

impl AbstractInstruction<ConcreteAbstractExecRecord> for ISZERO {
    fn exec(
        &self,
        stack: &Stack,
        _memory: &Memory,
        _ext: &Env,
    ) -> InstructionResult<ConcreteAbstractExecRecord> {
        let mut change_log = AbstractExecRecord::default();

        let op = stack.peek(0).unwrap();

        let rv = op._eq(&ZERO).ite(ONE, ZERO);

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Pop, StackOpRecord::Push(rv)],
        });

        Ok(change_log)
    }
}
