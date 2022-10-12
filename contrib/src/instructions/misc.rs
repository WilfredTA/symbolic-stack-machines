use symbolic_stack_machines_core::{
    environment::{Env, DefaultEnv},
    instructions::{AbstractExecRecord, AbstractInstruction, InstructionResult},
    memory::{Memory, MemVal, MemRecord, MemOpRecord},
    stack::{Stack, StackOpRecord, StackRecord, StackVal}, 
    value::{Sentence, CSimpleVal, CNumber, Value, visitors::ADDER_POST_HOOK},
    
};
use super::{ONE, ZERO};
pub struct PUSH(pub StackVal);

impl AbstractInstruction<AbstractExecRecord<DefaultEnv>, DefaultEnv, StackVal> for PUSH {
    fn exec(
        &self,
        stack: &Stack,
        _memory: &Memory<StackVal>,
        _ext: &DefaultEnv,
    ) -> InstructionResult<AbstractExecRecord<DefaultEnv>> {
        let mut change_log = AbstractExecRecord::default();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Push(self.0.clone())],
        });

        Ok(change_log)
    }
}

pub struct STOP;

impl AbstractInstruction<AbstractExecRecord<DefaultEnv>, DefaultEnv, MemVal> for STOP {
    fn exec(
        &self,
        stack: &Stack,
        _memory: &Memory<MemVal>,
        _ext: &DefaultEnv,
    ) -> InstructionResult<AbstractExecRecord<DefaultEnv>> {
        let mut change_log = AbstractExecRecord::default();

        change_log.halt = true;

        Ok(change_log)
    }
}

pub struct JUMPI;

impl AbstractInstruction<AbstractExecRecord<DefaultEnv>, DefaultEnv, MemVal> for JUMPI {
    fn exec(
        &self,
        stack: &Stack,
        _memory: &Memory<MemVal>,
        _ext: &DefaultEnv,
    ) -> InstructionResult<AbstractExecRecord<DefaultEnv>> {
        let mut change_log = AbstractExecRecord::default();

        let dest = stack.peek(0).unwrap();
        let cond = stack.peek(1).unwrap();

        if *cond != ZERO.into() {
            let x = Into::<usize>::into(dest.clone());
            change_log.pc_change = Some(x);
        }

        Ok(change_log)
    }
}

const FINAL_HOOK: &'static dyn Fn(Sentence) -> MemVal = &|s: Sentence| -> MemVal {
    if let Sentence::Basic(v) = s {
        let val = *v
            .as_concrete()
            .and_then(|v| v.as_number())
            .and_then(|v| v.as_u64())
            .unwrap();
        val.into()
    } else {
        0_u8.into()
    }
};
pub struct MLOAD;

impl AbstractInstruction<AbstractExecRecord<DefaultEnv>, DefaultEnv, MemVal> for MLOAD {
    fn exec(
        &self,
        stack: &Stack,
        memory: &Memory<MemVal>,
        _ext: &DefaultEnv,
    ) -> InstructionResult<AbstractExecRecord<DefaultEnv>> {
        let mut change_log = AbstractExecRecord::default();

        let mem_idx = stack.peek(0).unwrap();
        let mem_val = memory.read_word(mem_idx.clone(), FINAL_HOOK, Box::new(ADDER_POST_HOOK)).unwrap();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Pop, StackOpRecord::Push(mem_val)],
        });

        Ok(change_log)
    }
}

pub struct MSTORE;

impl AbstractInstruction<AbstractExecRecord<DefaultEnv>, DefaultEnv, MemVal> for MSTORE {
    fn exec(
        &self,
        stack: &Stack,
        _memory: &Memory<MemVal>,
        _ext: &DefaultEnv,
    ) -> InstructionResult<AbstractExecRecord<DefaultEnv>> {
        let mut change_log = AbstractExecRecord::default();

        let mem_idx = stack.peek(0).unwrap();
        let mem_val = stack.peek(1).unwrap();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Pop, StackOpRecord::Pop],
        });

        change_log.mem_diff = Some(MemRecord {
            changed: vec![MemOpRecord::Write(mem_idx.clone(), mem_val.clone())],
        });

        Ok(change_log)
    }
}
