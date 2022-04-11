use symbolic_stack_machines::{
    instructions::sym_helpers::{ADD, ISZERO, JUMPI, PUSH, STOP, SUB},
    machine::{
        run_sym_machine, SymbolicIntMachine, SymbolicIntMachineInnerConcrete, SymbolicProgram,
    },
    memory::MemConcreteIntToSymbolicInt,
    stack::SymbolicIntStack,
    vals::{SymbolicInt, SymbolicIntConstraint, SYM},
};

fn test_helper(
    pgm: SymbolicProgram<SymbolicIntStack, MemConcreteIntToSymbolicInt, SymbolicIntConstraint>,

    expected: Vec<Option<SymbolicInt>>,
) {
    let stack = SymbolicIntStack::new();
    let mem = MemConcreteIntToSymbolicInt::new();
    let concrete_machine = SymbolicIntMachineInnerConcrete::new(stack, mem, &pgm, None);
    let machine = SymbolicIntMachine::new(concrete_machine, None);

    let actual = run_sym_machine(machine);

    assert_eq!(actual, expected)
}

#[test]
fn test_basic() {
    test_helper(
        vec![PUSH(1), PUSH(2), PUSH(3), ADD(), SUB()],
        vec![Some(4.into())],
    );
}

#[test]
fn test_basic_symbolic_1() {
    test_helper(
        vec![PUSH(1), PUSH(SYM("a".into())), ADD()],
        vec![Some(SYM("a".into()) + 1.into())],
    )
}

#[test]
fn test_basic_symbolic_2() {
    test_helper(
        vec![PUSH(1), PUSH(2), PUSH(SYM("a".into())), ADD(), SUB()],
        vec![Some(SYM("a".into()) + 2.into() - 1.into())],
    )
}

#[test]
fn test_jumpi() {
    test_helper(
        vec![
            PUSH(1),
            PUSH(2),
            PUSH(SYM("a".into())),
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
        ],
        vec![Some(200.into()), Some(100.into())],
    )
}
