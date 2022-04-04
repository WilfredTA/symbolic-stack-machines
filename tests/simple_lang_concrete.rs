use symbolic_stack_machines::{stack::IntStack, memory::symbolic_concrete_index::MemIntToInt, machine::{BaseMachine, Program}, instructions::{misc::PUSH, arith::{ADD, SUB}, VMInstruction}};

#[test]
fn test_basic() {
    let stack = IntStack::new();
    let mem = MemIntToInt::new();
    let machine = BaseMachine::new(stack, mem);

    let pgm: Program<dyn VMInstruction<IntStack, MemIntToInt, ()>> = vec![
        PUSH(1),
        PUSH(2),
        PUSH(3),
        ADD,
        SUB,
    ];
}
