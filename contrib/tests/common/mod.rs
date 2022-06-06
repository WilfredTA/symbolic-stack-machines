use symbolic_stack_machines_contrib::instructions::{arith::*, misc::*};
use z3::{ast::Int, Context};
pub fn z3_int<'a>(i: u64, ctxt: &'a Context) -> z3::ast::Int<'a> {
    Int::from_u64(&ctxt, i)
}

pub fn z3_int_var<'a>(i: &str, ctxt: &'a Context) -> z3::ast::Int<'a> {
    Int::new_const(&ctxt, i)
}

pub mod simple_lang {
    use symbolic_stack_machines_core::{
        instructions::{AbstractInstruction, ConcreteAbstractExecRecord, EnvExtension},
        memory::Mem,
        stack::Stack,
    };

    use super::*;
    pub enum SimpleLang<T> {
        Add(ADD),
        Sub(SUB),
        Push(PUSH<T>),
    }

    pub fn add<T>() -> SimpleLang<T> {
        SimpleLang::Add(ADD)
    }

    pub fn sub<T>() -> SimpleLang<T> {
        SimpleLang::Sub(SUB)
    }

    pub fn push<T>(val: T) -> SimpleLang<T> {
        SimpleLang::Push(PUSH(val))
    }

    impl<T, S, M, Extension>
        AbstractInstruction<
            S,
            M,
            Extension,
            ConcreteAbstractExecRecord<S, M, Extension::DiffRecordType>,
        > for SimpleLang<T>
    where
        T: std::ops::Add
            + std::ops::Add<Output = T>
            + Clone
            + std::ops::Sub
            + std::ops::Sub<Output = T>
            + std::fmt::Debug,
        S: Stack<StackVal = T>,
        M: Mem,
        Extension: EnvExtension,
    {
        fn exec(
            &self,
            stack: &S,
            mem: &M,
            ext: &Extension,
        ) -> symbolic_stack_machines_core::instructions::InstructionResult<
            ConcreteAbstractExecRecord<S, M, Extension::DiffRecordType>,
        > {
            match self {
                Self::Add(a) => a.exec(stack, mem, ext),
                Self::Sub(s) => s.exec(stack, mem, ext),
                Self::Push(p) => p.exec(stack, mem, ext),
            }
        }
    }
}
