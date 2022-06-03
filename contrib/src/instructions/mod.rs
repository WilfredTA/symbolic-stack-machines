pub mod arith;
pub mod bitwise;
pub mod misc;
use symbolic_stack_machines_core::{
    instructions::{
        AbstractExecRecord, AbstractInstruction, ConcreteAbstractExecRecord, EnvExtension,
        InstructionResult,
    },
    memory::Mem,
    stack::{Stack, StackOpRecord, StackRecord},
};

pub enum Instruction<V> {
    add(arith::ADD),
    sub(arith::SUB),
    push(misc::PUSH<V>)
}


impl<T, S, M, Extension>
    AbstractInstruction<
        S,
        M,
        Extension,
        ConcreteAbstractExecRecord<S, M, Extension::DiffRecordType>,
    > for Instruction<T>
where
    T: std::ops::Add + std::ops::Add<Output = T> + std::ops::Sub + std::ops::Sub<Output = T> + Clone + std::fmt::Debug,
    S: Stack<StackVal = T>,
    M: Mem,
    Extension: EnvExtension,
{
    fn exec(&self, stack: &S, mem: &M, ext: &Extension) -> InstructionResult<ConcreteAbstractExecRecord<S, M, Extension::DiffRecordType>> {
        match self {
            Instruction::add(a) => a.exec(stack, mem, ext),
            Instruction::sub(s) => s.exec(stack, mem, ext),
            Instruction::push(p) => p.exec(stack, mem, ext)
        }
    }
}