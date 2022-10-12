use symbolic_stack_machines_core::{
    environment::{Env, DefaultEnv},
    instructions::{AbstractExecRecord, AbstractInstruction, InstructionResult},
    memory::{Memory, MemVal},
    stack::{Stack, StackOpRecord, StackRecord, StackVal}, value::{Sentence, CSimpleVal, CNumber, Value},
    
};

use super::{ZERO, ONE};


pub struct ISZERO;

impl AbstractInstruction<AbstractExecRecord<DefaultEnv>, DefaultEnv, MemVal> for ISZERO {
    fn exec(
        &self,
        stack: &Stack,
        _memory: &Memory<MemVal>,
        _ext: &DefaultEnv,
    ) -> InstructionResult<AbstractExecRecord<DefaultEnv>> {
        let mut change_log = AbstractExecRecord::default();

        let op = stack.peek(0).unwrap();

        let rv = op._eq(&StackVal::from(ZERO)).ite(StackVal::from(ONE), StackVal::from(ZERO));

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Pop, StackOpRecord::Push(rv)],
        });

        Ok(change_log)
    }
}
