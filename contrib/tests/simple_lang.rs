// Simple lang, but with each AbstractInstruction defined
// on each individual singleton instruction

use symbolic_stack_machines_core::environment::{Env, DefaultEnv};
use symbolic_stack_machines_core::machine::{
    inner_interpreter::ConcreteInnerInterpreter, r#abstract::AbstractMachine,
};
use symbolic_stack_machines_core::memory::Memory;
use symbolic_stack_machines_core::{
    machine::outer_interpreter::{ConcreteOuterInterpreter, OuterInterpreter},
    stack::{Stack, StackVal},
};
mod common;
use common::simple_lang::*;

#[test]
fn test_abstract_machine() {
    let pgm = vec![push(15), push(5), push(5), push(5), add(), add(), sub()];
    let env = DefaultEnv{};
    let pc = Some(0);
    let mem = Memory::default();
    let stack = Stack::default();
    let machine = AbstractMachine {
        stack,
        mem,
        env,
        pc,
        pgm: &pgm,
    };
    let inner_interpreter = Box::new(ConcreteInnerInterpreter {});
    let outer_interpreter = ConcreteOuterInterpreter { inner_interpreter };

    let res = outer_interpreter
        .run(machine)
        .unwrap();
    let res = res
        .stack
        .peek(0)
        .unwrap();

    assert_eq!(res.clone(), StackVal::from(0_u64));
}

#[test]
fn test_symbolic_compute() {
    
}