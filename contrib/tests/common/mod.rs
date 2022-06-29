use symbolic_stack_machines_contrib::instructions::{arith::*, misc::*};

pub mod simple_lang {
    use symbolic_stack_machines_core::{
        environment::EnvExtension,
        instructions::{AbstractInstruction, ConcreteAbstractExecRecord},
        memory::Memory,
        stack::Stack,
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

    impl<Extension>
        AbstractInstruction<Extension, ConcreteAbstractExecRecord<Extension::DiffRecordType>>
        for SimpleLang
    where
        Extension: EnvExtension,
    {
        fn exec(
            &self,
            stack: &Stack,
            mem: &Memory,
            ext: &Extension,
        ) -> symbolic_stack_machines_core::instructions::InstructionResult<
            ConcreteAbstractExecRecord<Extension::DiffRecordType>,
        > {
            match self {
                Self::Add(a) => a.exec(stack, mem, ext),
                Self::Sub(s) => s.exec(stack, mem, ext),
                Self::Push(p) => p.exec(stack, mem, ext),
            }
        }
    }
}
