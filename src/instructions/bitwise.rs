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

impl<T, S, M, PC> VMInstruction<S, M, PC>
    for ISZERO
where
    T: Eq + Binary,
    S: Stack<StackVal = T>,
    M: Mem
{
    fn exec(
        &self,
        stack: &S,
        _memory: &M,
    ) -> super::InstructionResult<super::ExecRecord<S, M, PC>> {
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
