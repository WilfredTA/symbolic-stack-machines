use symbolic_stack_machines_contrib::{instructions::{arith::*, misc::*}, sym::z3::Z3Solver};
use symbolic_stack_machines_core::{
    environment::{EnvExtension, EnvExtensionRecord},
    machine::{
        inner_interpreter::ConcreteInnerInterpreter,
        outer_interpreter::{ConcreteOuterInterpreter, OuterInterpreter, SymbolicOuterInterpreter},
        r#abstract::AbstractMachine,
    },
    memory::memory_models::BaseMemoryConcreteUint64,
    stack::{BaseStack, Stack},
};

use self::simple_lang::SimpleLang;

pub mod simple_lang {
    use symbolic_stack_machines_contrib::instructions::sym::ASSERT;
    use symbolic_stack_machines_core::{
        environment::EnvExtension,
        instructions::{AbstractInstruction, AbstractExecRecord},
        memory::Mem,
        stack::Stack, solver::Constrain,
    };

    use super::*;
    pub enum SimpleLang<T> {
        Add(ADD),
        Sub(SUB),
        Push(PUSH<T>),
        Assert(ASSERT<T>)
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

    impl<T, S, M, Extension, C>
        AbstractInstruction<
            S,
            M,
            Extension,
            AbstractExecRecord<S, M, Extension::DiffRecordType, C>,
        > for SimpleLang<T>
    where
        T: std::ops::Add
            + std::ops::Add<Output = T>
            + Clone
            + std::ops::Sub
            + std::ops::Sub<Output = T>
            + std::fmt::Debug
            + Constrain<C>,
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
            AbstractExecRecord<S, M, Extension::DiffRecordType, C>,
        > {
            match self {
                Self::Add(a) => a.exec(stack, mem, ext),
                Self::Sub(s) => s.exec(stack, mem, ext),
                Self::Push(p) => p.exec(stack, mem, ext),
                Self::Assert(a) => a.exec(stack, mem, ext)
            }
        }
    }
}

#[derive(Clone)]
pub struct DummyExtEnv {}

impl EnvExtension for DummyExtEnv {
    type InnerValue = Option<()>;

    type ErrorType = Option<()>;

    type IndexType = Option<()>;

    type DiffRecordType = DummyExtEnvRecord;

    fn write<V: Into<Self::InnerValue>>(&self, _v: V) -> Result<Self, Self::ErrorType>
    where
        Self: Sized,
    {
        todo!()
    }

    fn read<I: Into<Self::IndexType>>(&self, _idx: I) -> Result<Self::InnerValue, Self::ErrorType> {
        todo!()
    }
}

pub struct DummyExtEnvRecord {}

impl EnvExtensionRecord for DummyExtEnvRecord {
    fn apply<E: EnvExtension>(&self, env: E) -> Result<E, E::ErrorType> {
        Ok(env)
    }
}

pub fn run_concrete_machine<T>(pgm: Vec<SimpleLang<T>>) -> Option<T>
where
    T: Clone
        + std::ops::Add
        + std::ops::Add<Output = T>
        + Clone
        + std::ops::Sub
        + std::ops::Sub<Output = T>
        + std::fmt::Debug,
{
    let custom_env = DummyExtEnv {};
    let pc = Some(0);
    let mem = BaseMemoryConcreteUint64::new();
    let stack = BaseStack::init();
    let machine = AbstractMachine {
        stack,
        mem,
        custom_env,
        pc,
        pgm: &pgm,
    };
    let inner_interpreter = Box::new(ConcreteInnerInterpreter {});
    let outer_interpreter = ConcreteOuterInterpreter { inner_interpreter };

    outer_interpreter.run(machine).unwrap().stack.peek(0)
}

pub fn run_symbolic_machine<T>(pgm: Vec<SimpleLang<T>>)
{
    let mut cfg = z3::Config::default();
    cfg.set_model_generation(true);
    let ctx = z3::Context::new(&cfg);

    let custom_env = DummyExtEnv {};
    let pc = Some(0);
    let mem = BaseMemoryConcreteUint64::new();
    let stack = BaseStack::init();
    let machine = AbstractMachine {
        stack,
        mem,
        custom_env,
        pc,
        pgm: &pgm,
    };
    let inner_interpreter = Box::new(ConcreteInnerInterpreter {});
    let solver = Box::new(Z3Solver{ ctx });
    let outer_interpreter = SymbolicOuterInterpreter { inner_interpreter, solver };

    let x = outer_interpreter.run(machine);

    todo!()
}