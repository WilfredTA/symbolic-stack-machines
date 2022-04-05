use symbolic_stack_machines::{
    instructions::{
        arith::{ADD, SUB},
        misc::{MLOAD, MSTORE, PUSH},
        VMInstruction,
    },
    machine::{ConcreteIntMachine, Program},
    memory::symbolic_concrete_index::MemIntToInt,
    stack::IntStack,
};

#[test]
fn test_basic() {
    let stack = IntStack::new();
    let mem = MemIntToInt::new();
    let machine = ConcreteIntMachine::new(stack, mem);

    let pgm: Program<Box<dyn VMInstruction<_, _, _>>> = vec![
        Box::new(PUSH(1)),
        Box::new(PUSH(2)),
        Box::new(PUSH(3)),
        Box::new(ADD),
        Box::new(SUB),
    ];

    assert_eq!(machine.run(&pgm), Option::Some(4))
}

#[test]
fn test_basic_mem() {
    let stack = IntStack::new();
    let mem = MemIntToInt::new();
    let machine = ConcreteIntMachine::new(stack, mem);

    let pgm: Program<Box<dyn VMInstruction<_, _, _>>> = vec![
        Box::new(PUSH(1)),
        Box::new(PUSH(0)),
        Box::new(MSTORE),
        Box::new(PUSH(0)),
        Box::new(MLOAD),
    ];

    assert_eq!(machine.run(&pgm), Option::Some(1))
}

#[test]
fn test_basic_jumpi() {}

#[test]
fn test_basic_multi_jumpi() {}
