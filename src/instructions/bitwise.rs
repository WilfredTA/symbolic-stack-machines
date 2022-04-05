use crate::{
    stack::{Stack, StackOpRecord, StackRecord}, memory::Mem,
};

use super::{ExecRecord, VMInstruction};

#[derive(Debug)]
pub struct ISZERO;

pub trait Binary: Default {
    fn one() -> Self;
    fn zero() -> Self {
        Self::default()
    }
}

impl<T: Eq + Binary, ValStack: Stack<StackVal = T>, M, PathConstraint> VMInstruction<ValStack, M, PathConstraint>
    for ISZERO
where
    M: Mem
{
    fn exec(
        &self,
        stack: &ValStack,
        _memory: &M,
    ) -> super::InstructionResult<super::ExecRecord<ValStack, M, PathConstraint>> {
        let mut change_log = ExecRecord::default();

        let op = stack.peek(0).unwrap();

        let rv = if op == T::default() {
            T::one()
        } else {
            T::zero()
        };

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Pop(op), StackOpRecord::Push(rv)],
        });

        Ok(change_log)
    }
}
