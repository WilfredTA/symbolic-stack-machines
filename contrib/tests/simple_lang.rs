// Simple lang, but with each AbstractInstruction defined
// on each individual singleton instruction

use symbolic_stack_machines_core::machine::{
    inner_interpreter::ConcreteInnerInterpreter, r#abstract::AbstractMachine,
};
use symbolic_stack_machines_core::memory::Memory;
use symbolic_stack_machines_core::{
    environment::{EnvExtension, EnvExtensionRecord},
    machine::outer_interpreter::{ConcreteOuterInterpreter, OuterInterpreter},
    stack::{Stack, StackVal},
};
mod common;
use common::simple_lang::*;

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

#[test]
fn test_abstract_machine() {
    let pgm = vec![push(15), push(5), push(5), push(5), add(), add(), sub()];
    let custom_env = DummyExtEnv {};
    let pc = Some(0);
    let mem = Memory::default();
    let stack = Stack::default();
    let machine = AbstractMachine {
        stack,
        mem,
        custom_env,
        pc,
        pgm: &pgm,
    };
    let inner_interpreter = Box::new(ConcreteInnerInterpreter {});
    let outer_interpreter = ConcreteOuterInterpreter { inner_interpreter };

    let res = *outer_interpreter
        .run(machine)
        .unwrap()
        .stack
        .peek(0)
        .unwrap();

    assert_eq!(res, StackVal::from(0));
}
