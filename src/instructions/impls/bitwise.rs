use crate::{
    instructions::{Binary, ConcreteVMInstruction, ExecRecord, InstructionResult},
    memory::Mem,
    stack::{Stack, StackOpRecord, StackRecord},
    vals::MachineEq,
};

#[derive(Debug)]
pub struct ISZERO;

impl<T, S, M> ConcreteVMInstruction<S, M> for ISZERO
where
    T: Binary + MachineEq,
    S: Stack<StackVal = T>,
    M: Mem,
{
    fn exec(&self, stack: &S, _memory: &M) -> InstructionResult<ExecRecord<S, M>> {
        let mut change_log = ExecRecord::default();

        let op: T = stack.peek(0).unwrap();

        let rv = T::machine_ite(op.machine_eq(&T::zero()), T::one(), T::zero());

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Pop(op), StackOpRecord::Push(rv)],
        });

        Ok(change_log)
    }
}
