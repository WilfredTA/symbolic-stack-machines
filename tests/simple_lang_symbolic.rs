use symbolic_stack_machines::{
    instructions::{
        sym_helpers::{ADD, ISZERO, JUMPI, PUSH, STOP, SUB},
        HybridVMInstruction,
    },
    machine::{run_sym_machine, symbolic::SymbolicIntMachine, BaseMachine, Program},
    memory::symbolic_concrete_index::MemConcreteIntToSymbolicInt,
    stack::SymbolicIntStack,
    symbolic_int::{SymbolicInt, SymbolicIntConstraint, SYM},
};

fn test_helper(
    pgm: Program<
        HybridVMInstruction<SymbolicIntStack, MemConcreteIntToSymbolicInt, SymbolicIntConstraint>,
    >,

    expected: Vec<SymbolicInt>,
) {
    let stack = SymbolicIntStack::new();
    let mem = MemConcreteIntToSymbolicInt::new();
    let machine = SymbolicIntMachine::new(stack, mem, &pgm, None, None);

    assert_eq!(
        run_sym_machine(machine)
            .iter()
            .map(|x| x.return_value())
            .collect::<Vec<Option<SymbolicInt>>>(),
        expected
            .into_iter()
            .map(|x| Option::Some(x))
            .collect::<Vec<Option<SymbolicInt>>>()
    )
}

#[test]
fn test_basic() {
    test_helper(
        vec![PUSH(1), PUSH(2), PUSH(3), ADD(), SUB()],
        vec![4.into()],
    );
}

#[test]
fn test_basic_symbolic_1() {
    test_helper(vec![PUSH(1), PUSH(SYM()), ADD()], vec![SYM() + 1.into()])
}

#[test]
fn test_basic_symbolic_2() {
    test_helper(
        vec![PUSH(1), PUSH(2), PUSH(SYM()), ADD(), SUB()],
        vec![SYM() + 2.into() - 1.into()],
    )
}

#[test]
fn test_jumpi() {
    test_helper(
        vec![
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
        ],
        // TODO we now need to pipe constraints into constraint solver to
        // see which of these solutions is value
        vec![200.into(), 100.into()],
    )
}
