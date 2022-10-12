use symbolic_stack_machines_core::{
    environment::{Env, DefaultEnv},
    instructions::{AbstractExecRecord, AbstractInstruction, InstructionResult},
    memory::{Memory, MemVal},
    stack::{Stack, StackOpRecord, StackRecord, StackVal},
};

pub struct ADD;

impl AbstractInstruction<AbstractExecRecord<DefaultEnv>, DefaultEnv, StackVal> for ADD {
    fn exec(
        &self,
        stack: &Stack,
        _mem: &Memory<StackVal>,
        _ext: &DefaultEnv,
    ) -> InstructionResult<AbstractExecRecord<DefaultEnv>> {
        let mut change_log = AbstractExecRecord::default();

        let op_1 = stack.peek(0).unwrap().clone();
        let op_2 = stack.peek(1).unwrap().clone();
        let res = op_1 + op_2;

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

impl AbstractInstruction<AbstractExecRecord<DefaultEnv>, DefaultEnv, StackVal> for SUB {
    fn exec(
        &self,
        stack: &Stack,
        _mem: &Memory<StackVal>,
        _ext: &DefaultEnv,
    ) -> InstructionResult<AbstractExecRecord<DefaultEnv>> {
        let mut change_log = AbstractExecRecord::default();

        let op_1 = stack.peek(0).unwrap().clone();
        let op_2 = stack.peek(1).unwrap().clone();
        let res = op_1 - op_2;

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
