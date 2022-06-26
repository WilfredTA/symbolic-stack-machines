use symbolic_stack_machines_core::{
    environment::EnvExtension,
    instructions::{AbstractExecRecord, AbstractInstruction, InstructionResult},
    memory::Mem,
    stack::{Stack, StackOpRecord, StackRecord}, solver::Constrain,
};

pub struct ASSERT<T>(pub T);

impl<T, S, M, Extension, C>
    AbstractInstruction<S, M, Extension, AbstractExecRecord<S, M, Extension::DiffRecordType, C>>
    for ASSERT<T>
where
    T: Clone + Constrain<C>,
    S: Stack<StackVal = T>,
    M: Mem,
    Extension: EnvExtension,
{
    fn exec(
        &self,
        stack: &S,
        _mem: &M,
        _ext: &Extension,
    ) -> InstructionResult<AbstractExecRecord<S, M, Extension::DiffRecordType, C>> {
        let mut change_log = AbstractExecRecord::default();

        let op: T = stack.peek(0).unwrap();

        change_log.stack_diff = Some(StackRecord {
            changed: vec![StackOpRecord::Pop(op.clone())],
        });

        change_log.constraints = Some(vec![op.assert(self.0.clone())]);

        Ok(change_log)
    }
}
