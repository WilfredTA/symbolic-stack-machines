use symbolic_stack_machines_core::{
    environment::EnvExtension,
    instructions::{
        AbstractExecRecord, AbstractInstruction, InstructionResult,
    },
    memory::Mem,
    stack::{Stack, StackOpRecord, StackRecord},
};

// The MachineEq trait is necessary because the type of equality used in the machine
// may be different than the type of equality used outside of the machine, and
// `PartialEq` types the result of == as boolean.
//
// Using `MachineEq` lets us use our own separate predicate type which in the case
// where the machine holds symbolic values can in example be a symbolic boolean.
pub trait MachineEq {
    type Pred;

    fn machine_eq(&self, other: &Self) -> Self::Pred;
    fn machine_ite(pred: Self::Pred, then: Self, xelse: Self) -> Self;
}

pub struct ISZERO;

impl<T, S, M, Extension, C>
    AbstractInstruction<
        S,
        M,
        Extension,
        AbstractExecRecord<S, M, Extension::DiffRecordType, C>,
    > for ISZERO
where
    T: From<u8> + MachineEq,
    S: Stack<StackVal = T>,
    M: Mem,
    Extension: EnvExtension,
{
    fn exec(
        &self,
        stack: &S,
        _memory: &M,
        _ext: &Extension,
    ) -> InstructionResult<AbstractExecRecord<S, M, Extension::DiffRecordType, C>> {
        let mut change_log = AbstractExecRecord::default();

        let op: T = stack.peek(0).unwrap();

        let zero = T::from(0);
        let one = T::from(1);

        let rv = T::machine_ite(op.machine_eq(&zero), one, zero);

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Pop(op), StackOpRecord::Push(rv)],
        });

        Ok(change_log)
    }
}
