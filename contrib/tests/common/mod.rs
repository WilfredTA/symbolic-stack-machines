use symbolic_stack_machines_contrib::instructions::{arith::*, misc::*};

pub mod simple_lang {
    use symbolic_stack_machines_core::{
        environment::{Env, DefaultEnv},
        instructions::{AbstractExecRecord, AbstractInstruction},
        memory::{Memory, MemVal},
        stack::{Stack, StackVal, StackRecord, StackOpRecord},
    };

    use super::*;
    pub enum SimpleLang {
        Add(ADD),
        Sub(SUB),
        Push(PUSH),
    }

    pub fn add() -> SimpleLang {
        SimpleLang::Add(ADD)
    }

    pub fn sub() -> SimpleLang {
        SimpleLang::Sub(SUB)
    }

    pub fn push(val: u64) -> SimpleLang {
        SimpleLang::Push(PUSH(val.into()))
    }

    impl AbstractInstruction<AbstractExecRecord<DefaultEnv>, DefaultEnv, StackVal> for SimpleLang {
        fn exec(
            &self,
            stack: &Stack,
            mem: &Memory<StackVal>,
            ext: &DefaultEnv,
        ) -> symbolic_stack_machines_core::instructions::InstructionResult<AbstractExecRecord<DefaultEnv>>
        {
            match self {
                Self::Add(a) => a.exec(stack, mem, ext),
                Self::Sub(s) => s.exec(stack, mem, ext),
                Self::Push(p) => p.exec(stack, mem, ext),
            }
        }
    }
}
