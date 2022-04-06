use crate::{
    stack::{Stack, StackOpRecord, StackRecord}, memory::Mem,
};

use super::{ExecRecord, VMInstruction, InstructionResult};

#[derive(Debug)]
pub struct ISZERO;

pub trait Binary: Default {
    fn one() -> Self;
    fn zero() -> Self {
        Self::default()
    }
}

impl<T, S, M> VMInstruction<S, M>
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
    ) -> InstructionResult<ExecRecord<S, M>> {
        let mut change_log = ExecRecord::default();

        let op = stack.peek(0).unwrap();

        let rv = if op == T::zero() {
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
