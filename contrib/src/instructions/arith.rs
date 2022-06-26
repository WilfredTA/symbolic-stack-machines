use symbolic_stack_machines_core::{
    environment::EnvExtension,
    instructions::{
        AbstractExecRecord, AbstractInstruction, InstructionResult,
    },
    memory::Mem,
    stack::{Stack, StackOpRecord, StackRecord},
};

pub struct ADD;

impl<T, S, M, Extension, C>
    AbstractInstruction<
        S,
        M,
        Extension,
        AbstractExecRecord<S, M, Extension::DiffRecordType, C>,
    > for ADD
where
    T: std::ops::Add + std::ops::Add<Output = T> + Clone,
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

        let op_1: T = stack.peek(0).unwrap();
        let op_2: T = stack.peek(1).unwrap();
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

pub struct SUB;

impl<T, S, M, Extension, C>
    AbstractInstruction<
        S,
        M,
        Extension,
        AbstractExecRecord<S, M, Extension::DiffRecordType, C>,
    > for SUB
where
    T: std::ops::Sub + std::ops::Sub<Output = T> + Clone,
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

        let op_1: T = stack.peek(0).unwrap();
        let op_2: T = stack.peek(1).unwrap();
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
