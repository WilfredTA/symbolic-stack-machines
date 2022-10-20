use symbolic_stack_machines_core::{
    environment::Env,
    instructions::{AbstractExecRecord, AbstractInstruction, InstructionResult},
    memory::Memory,
    stack::{Stack, StackOpRecord, StackRecord},
};

pub struct ADD;

impl AbstractInstruction<AbstractExecRecord> for ADD {
    fn exec(
        &self,
        stack: &Stack,
        _mem: &Memory,
        _ext: &Env,
    ) -> InstructionResult<AbstractExecRecord> {
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

impl AbstractInstruction<AbstractExecRecord> for SUB {
    fn exec(
        &self,
        stack: &Stack,
        _mem: &Memory,
        _ext: &Env,
    ) -> InstructionResult<AbstractExecRecord> {
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
