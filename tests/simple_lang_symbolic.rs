use symbolic_stack_machines::{
    instructions::{
        sym,
        sym_helpers::{ADD, ISZERO, JUMPI, PUSH, STOP, SUB}, SymbolicVMInstruction,
    },
    machine::{run_machine, symbolic::SymbolicIntMachine, Program},
    memory::symbolic_concrete_index::MemConcreteIntToSymbolicInt,
    stack::SymbolicIntStack,
    symbolic_int::{SymbolicInt, SYM},
};

fn test_helper(
    pgm: Program<SymbolicVMInstruction<SymbolicIntStack, MemConcreteIntToSymbolicInt, sym::JUMPI>>,
    expected: SymbolicInt,
) {
    let stack = SymbolicIntStack::new();
    let mem = MemConcreteIntToSymbolicInt::new();
    let machine = SymbolicIntMachine::new(stack, mem, &pgm);

    assert_eq!(run_machine(machine), Option::Some(expected.into()))
}

#[test]
fn test_basic() {
    test_helper(vec![PUSH(1), PUSH(2), PUSH(3), ADD(), SUB()], 4.into());
}

#[test]
fn test_basic_symbolic_1() {
    test_helper(vec![PUSH(1), PUSH(SYM()), ADD()], SYM() + 1.into())
}

#[test]
fn test_basic_symbolic_2() {
    test_helper(
        vec![PUSH(1), PUSH(2), PUSH(SYM()), ADD(), SUB()],
        SYM() + 2.into() - 1.into(),
    )
}

#[test]
fn test_jumpi() {
    let stack = SymbolicIntStack::new();
    let mem = MemConcreteIntToSymbolicInt::new();

    // TODO HERE
    let pgm = vec![
        PUSH(1),
        PUSH(2),
        PUSH(SYM()),
        ADD(),
        SUB(),
        PUSH(4),
        SUB(),
        ISZERO(),
        PUSH(12),
        JUMPI(),
        PUSH(100),
        STOP(),
        PUSH(200),
    ];

    let machine = SymbolicIntMachine::new(stack, mem, &pgm);

    let rv = run_machine(machine);

    dbg!(rv);
}
