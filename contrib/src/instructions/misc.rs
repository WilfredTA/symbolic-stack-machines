use symbolic_stack_machines_core::{
    environment::Env,
    instructions::{AbstractExecRecord, AbstractInstruction, InstructionResult},
    memory::{MemOpRecord, MemRecord, Memory},
    stack::{Stack, StackOpRecord, StackRecord, StackVal},
};
use super::ZERO;

pub struct PUSH(pub StackVal);

impl AbstractInstruction<AbstractExecRecord> for PUSH {
    fn exec(
        &self,
        _stack: &Stack,
        _memory: &Memory,
        _ext: &Env,
    ) -> InstructionResult<AbstractExecRecord> {
        let mut change_log = AbstractExecRecord::default();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Push(self.0.clone())],
        });

        Ok(change_log)
    }
}

pub struct STOP;

impl AbstractInstruction<AbstractExecRecord> for STOP {
    fn exec(
        &self,
        _stack: &Stack,
        _memory: &Memory,
        _ext: &Env,
    ) -> InstructionResult<AbstractExecRecord> {
        let mut change_log = AbstractExecRecord::default();

        change_log.halt = true;

        Ok(change_log)
    }
}

pub struct JUMPI;

impl AbstractInstruction<AbstractExecRecord> for JUMPI {
    fn exec(
        &self,
        stack: &Stack,
        _memory: &Memory,
        _ext: &Env,
    ) -> InstructionResult<AbstractExecRecord> {
        let mut change_log = AbstractExecRecord::default();

        let dest = stack.peek(0).unwrap();
        let cond = stack.peek(1).unwrap();

        if *cond != StackVal(ZERO) {
            let x = Into::<usize>::into(dest.clone());
            change_log.pc_change = Some(x);
        }

        Ok(change_log)
    }
}

pub struct MLOAD;

impl AbstractInstruction<AbstractExecRecord> for MLOAD {
    fn exec(
        &self,
        stack: &Stack,
        memory: &Memory,
        _ext: &Env,
    ) -> InstructionResult<AbstractExecRecord> {
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

impl AbstractInstruction<AbstractExecRecord> for MSTORE {
    fn exec(
        &self,
        stack: &Stack,
        _memory: &Memory,
        _ext: &Env,
    ) -> InstructionResult<AbstractExecRecord> {
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
