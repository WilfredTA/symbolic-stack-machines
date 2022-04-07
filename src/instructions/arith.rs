use crate::{
    memory::Mem,
    stack::{Stack, StackOpRecord, StackRecord},
};

use super::{ExecRecord, ConcreteVMInstruction, InstructionResult};

#[derive(Debug)]
pub struct ADD;

impl<T, S, M> ConcreteVMInstruction<S, M> for ADD
where
    T: std::ops::Add + std::ops::Add<Output = T> + Clone,
    S: Stack<StackVal = T>,
    M: Mem,
{
    fn exec(
        &self,
        stack: &S,
        _memory: &M,
    ) -> InstructionResult<ExecRecord<S, M>> {
        let mut change_log = ExecRecord::default();

        let op_1 = stack.peek(0).unwrap();
        let op_2 = stack.peek(1).unwrap();
        let res = op_1.clone() + op_2.clone();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![
                StackOpRecord::Pop(op_1),
                StackOpRecord::Pop(op_2),
                StackOpRecord::Push(res),
            ],
        });

        Ok(change_log)
    }
}

#[derive(Debug)]
pub struct SUB;

impl<T, S, M>
    ConcreteVMInstruction<S, M> for SUB
where
    T: std::ops::Sub + std::ops::Sub<Output = T> + Clone,
    S: Stack<StackVal = T>,
    M: Mem,
{
    fn exec(
        &self,
        stack: &S,
        _memory: &M,
    ) -> InstructionResult<ExecRecord<S, M>> {
        let mut change_log = ExecRecord::default();

        let op_1 = stack.peek(0).unwrap();
        let op_2 = stack.peek(1).unwrap();
        let res = op_1.clone() - op_2.clone();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![
                StackOpRecord::Pop(op_1),
                StackOpRecord::Pop(op_2),
                StackOpRecord::Push(res),
            ],
        });

        Ok(change_log)
    }
}
