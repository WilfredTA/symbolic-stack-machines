// Simple lang, but with each AbstractInstruction defined
// on each individual singleton instruction

use symbolic_stack_machines_contrib::{
    constraints::z3::*,
    instructions::{Instruction, arith::*, bitwise::*, misc::*},
};

use std::marker::PhantomData;
use symbolic_stack_machines_core::{value::r#abstract::{AbstractInt, Val}, machine::{outer_interpreter::{OuterInterpreter, ConcreteOuterInterpreter}, r#abstract::AbstractMachine, inner_interpreter::ConcreteInnerInterpreter}, stack::{BaseStack, Stack}, memory::memory_models::BaseMemoryConcreteIndex};
use symbolic_stack_machines_core::{instructions::*, machine, memory::memory_models, stack};
use symbolic_stack_machines_core::{
    machine::{
        inner_interpreter::{self, InnerInterpreter},
        outer_interpreter,
    },
    memory::{memory_models::BaseMemoryConcreteUint64, MemOpRecord, MemRecord, ReadOnlyMem},
};

type SimpleLangRecord = ConcreteAbstractExecRecord<
    stack::BaseStack<u64>,
    memory_models::BaseMemoryConcreteUint64,
    DummyExtEnvRecord,
>;

type ValInt = Val<AbstractInt>;

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


type ConcreteInterpreter<'a, V> = ConcreteOuterInterpreter<'a, 
    BaseStack<V>, 
    BaseMemoryConcreteIndex<V>, 
    DummyExtEnv, 
    Instruction<V>, 
    SimpleLangRecord, 
    ConcreteMachine<'a, V>,
    ConcreteInnerInterpreter,    
>;

type ConcreteMachine<'a, V> = AbstractMachine<'a, BaseStack<V>, BaseMemoryConcreteIndex<V>, DummyExtEnv, Instruction<V>>;

#[test]
fn test_initialize_machine() {
    // Everything can use the type definition of Instruction
    type Inst = Instruction<u64>;
    let mem = BaseMemoryConcreteUint64::new();

    // SHould result in 0
    let pgm: Vec<Inst> = vec![Instruction::push(PUSH(15)), Instruction::push(PUSH(5)), Instruction::push(PUSH(5)), Instruction::push(PUSH(5)), Instruction::add(ADD), Instruction::add(ADD), Instruction::sub(SUB)];
    
    let machine: ConcreteMachine<'_, u64> = machine::r#abstract::AbstractMachine {
        stack: BaseStack::init(),
        mem,
        custom_env: DummyExtEnv {},
        pc: Some(0),
        pgm: &pgm,
    };
    let inner_interpreter: machine::inner_interpreter::ConcreteInnerInterpreter =
        machine::inner_interpreter::ConcreteInnerInterpreter {};

    let outer_interpreter: ConcreteInterpreter<'_, u64> = outer_interpreter::ConcreteOuterInterpreter::new(inner_interpreter);

    let res = outer_interpreter.run(machine).unwrap();
    let stack_top = res.stack.peek(0);
    assert_eq!(stack_top, Some(0_u64));
    
}