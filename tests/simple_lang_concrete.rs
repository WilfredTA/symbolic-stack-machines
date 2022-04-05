use symbolic_stack_machines::{
    instructions::{
        arith::{ADD, SUB},
        misc::PUSH,
        VMInstruction,
    },
    machine::{Program, ConcreteIntMachine},
    memory::symbolic_concrete_index::MemIntToInt,
    stack::IntStack,
};

#[test]
fn test_basic() {
    let stack = IntStack::new();
    let mem = MemIntToInt::new();
    let machine = ConcreteIntMachine::new(stack, mem);

    let pgm: Program<Box<dyn VMInstruction<IntStack, MemIntToInt, ()>>> = vec![
        Box::new(PUSH(1)),
        Box::new(PUSH(2)),
        Box::new(PUSH(3)),
        Box::new(ADD),
        Box::new(SUB),
    ];

    assert_eq!(machine.run(&pgm), Option::Some(4))
}
