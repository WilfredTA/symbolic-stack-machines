use symbolic_stack_machines::{
    instructions::{
        arith::{ADD, SUB},
        misc::{MLOAD, MSTORE, PUSH, JUMPI, STOP},
        VMInstruction, bitwise::ISZERO,
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
        Box::new(PUSH(1.into())),
        Box::new(PUSH(2.into())),
        Box::new(PUSH(3.into())),
        Box::new(ADD),
        Box::new(SUB),
    ];

    assert_eq!(machine.run(&pgm), Option::Some(4.into()))
}

#[test]
fn test_basic_mem() {
    let stack = IntStack::new();
    let mem = MemIntToInt::new();
    let machine = ConcreteIntMachine::new(stack, mem);

    let pgm: Program<Box<dyn VMInstruction<_, _, _>>> = vec![
        Box::new(PUSH(1.into())),
        Box::new(PUSH(0.into())),
        Box::new(MSTORE),
        Box::new(PUSH(0.into())),
        Box::new(MLOAD),
    ];

    assert_eq!(machine.run(&pgm), Option::Some(1.into()))
}

#[test]
fn test_basic_jumpi() {
    let stack = IntStack::new();
    let mem = MemIntToInt::new();
    let machine = ConcreteIntMachine::new(stack, mem);

    let pgm: Program<Box<dyn VMInstruction<_, _, _>>> = vec![
        Box::new(PUSH(1.into())),
        Box::new(PUSH(2.into())),
        Box::new(PUSH(3.into())),
        Box::new(ADD),
        Box::new(SUB),
        Box::new(PUSH(4.into())),
        Box::new(SUB),
        Box::new(ISZERO),
        Box::new(PUSH(12.into())),
        Box::new(JUMPI),
        Box::new(PUSH(100.into())),
        Box::new(STOP),
        Box::new(PUSH(200.into())),
    ];

    assert_eq!(machine.run(&pgm), Option::Some(200.into()))
}

#[test]
fn test_basic_multi_jumpi() {}
