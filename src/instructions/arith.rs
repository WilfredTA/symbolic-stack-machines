use crate::{
    memory::Mem,
    stack::{Stack, StackOpRecord, StackRecord},
};

use super::{ExecRecord, VMInstruction};

#[derive(Debug)]
pub struct ADD;

impl<
        T: std::ops::Add + std::ops::Add<Output = T> + Copy,
        ValStack: Stack<StackVal = T>,
        M: Mem,
        PathConstraint,
    > VMInstruction<ValStack, M, PathConstraint> for ADD
{
    fn exec(
        &self,
        stack: &ValStack,
        _memory: &M,
    ) -> super::InstructionResult<super::ExecRecord<ValStack, M, PathConstraint>> {
        let mut change_log = ExecRecord::default();

        let op_1 = stack.peek(0).unwrap();
        let op_2 = stack.peek(1).unwrap();
        let res = op_1 + op_2;

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

impl<
        T: std::ops::Sub + std::ops::Sub<Output = T> + Copy,
        S: Stack<StackVal = T>,
        M: Mem,
        PC,
    > VMInstruction<S, M, PC> for SUB
{
    fn exec(
        &self,
        stack: &S,
        _memory: &M,
    ) -> super::InstructionResult<super::ExecRecord<S, M, PC>> {
        let mut change_log = ExecRecord::default();

        let op_1 = stack.peek(0).unwrap();
        let op_2 = stack.peek(1).unwrap();
        let res = op_1 - op_2;

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
