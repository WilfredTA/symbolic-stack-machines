// Simple lang, but with each AbstractInstruction defined
// on each individual singleton instruction

use std::rc::Rc;

use symbolic_stack_machines_contrib::{
    constraints::z3::*,
    instructions::{arith::*, bitwise::*, misc::*},
};

use symbolic_stack_machines_core::memory::{
    memory_models::BaseMemoryConcreteUint64, MemOpRecord, MemRecord, ReadOnlyMem,
};
use symbolic_stack_machines_core::{instructions::*, machine, memory::memory_models, stack};
use symbolic_stack_machines_core::{
    machine::{
        inner_interpreter::ConcreteInnerInterpreter,
        outer_interpreter::{ConcreteOuterInterpreter, OuterInterpreter},
        r#abstract::AbstractMachine,
    },
    memory::memory_models::BaseMemoryConcreteIndex,
    stack::{BaseStack, Stack},
    value::r#abstract::*,
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

// type ConcreteInterpreter<'a, V> = ConcreteOuterInterpreter<'a,
//     BaseStack<V>,
//     BaseMemoryConcreteIndex<V>,
//     DummyExtEnv,
//     Instruction<V>,
//     SimpleLangRecord,
//     ConcreteMachine<'a, V>,
//     ConcreteInnerInterpreter,
// >;

// type ConcreteMachine<'a, V> = AbstractMachine<'a, BaseStack<V>, BaseMemoryConcreteIndex<V>, DummyExtEnv, Instruction<V>>;

#[test]
fn test_abstract_machine() {
    let pgm = vec![push(15_u64), push(5), push(5), push(5), add(), add(), sub()];
    let custom_env = DummyExtEnv {};
    let pc = Some(0);
    let mem = BaseMemoryConcreteUint64::new();
    let stack = BaseStack::<u64>::init();
    let machine = AbstractMachine {
        stack,
        mem,
        custom_env,
        pc,
        pgm: &pgm,
    };
    let inner_interpreter = Box::new(ConcreteInnerInterpreter {});
    let outer_interpreter = ConcreteOuterInterpreter { inner_interpreter };

    let res: Option<u64> = outer_interpreter.run(machine).unwrap().stack.peek(0);

    assert_eq!(res, Some(0))
}

#[test]
fn test_abstract_arithmetic() {
    // Program is Vec of AbstractVal<InnerVal>
    let pgm: Vec<SimpleLang<AbstractValue<InnerValue>>> = vec![
        push(InnerValue::from(30_u64).into()),
        push(InnerValue::from(20_u64).into()),
        add(),
    ];
    let pc = Some(0);
    let custom_env = DummyExtEnv {};
    let mem = BaseMemoryConcreteUint64::new();
    let stack = BaseStack::<Val>::init();
    let machine = AbstractMachine {
        stack,
        mem,
        custom_env,
        pc,
        pgm: &pgm,
    };
    let inner_interpreter = Box::new(ConcreteInnerInterpreter {});
    let outer_interpreter = ConcreteOuterInterpreter { inner_interpreter };

    let res: Option<Val> = outer_interpreter.run(machine).unwrap().stack.peek(0);
    println!("{:?}", res);
    assert!(
        res.unwrap().inner()
            == InnerValue::Arithmetic(Value(Rc::new(Arithmetic::Add(
                Value(Rc::new(InnerValue::ConcreteLiteral(Value(Rc::new(
                    ConcreteInnerValue::ConcreteU64(20)
                ))))),
                Value(Rc::new(InnerValue::ConcreteLiteral(Value(Rc::new(
                    ConcreteInnerValue::ConcreteU64(30)
                )))))
            ))))
    );
}
