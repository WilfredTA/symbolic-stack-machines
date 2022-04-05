use symbolic_stack_machines::{
    instructions::helpers::{ADD, PUSH, SUB},
    memory::symbolic_concrete_index::MemConcreteIntToSymbolicInt,
    stack::SymbolicIntStack, machine::SymbolicIntMachine,
};

#[test]
fn test_basic() {
    let stack = SymbolicIntStack::new();
    let mem = MemConcreteIntToSymbolicInt::new();
    let machine = SymbolicIntMachine::new(stack, mem);

    let pgm = vec![PUSH(1), PUSH(2), PUSH(3), ADD(), SUB()];

    let rv = machine.run(&pgm);

    dbg!(rv);
}
