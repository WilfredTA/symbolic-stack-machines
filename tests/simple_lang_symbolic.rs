use symbolic_stack_machines::{
    instructions::{
        sym_helpers::{ADD, ISZERO, JUMPI, PUSH, STOP, SUB},
        HybridVMInstruction,
    },
    machine::{
        run_sym_machine, symbolic::SymbolicIntMachine, BaseMachine, Program, SymbolicMachineOutput,
    },
    memory::symbolic_concrete_index::MemConcreteIntToSymbolicInt,
    stack::SymbolicIntStack,
    symbolic_int::{SymbolicInt, SymbolicIntConstraint, SYM},
};

fn test_helper(
    pgm: Program<
        HybridVMInstruction<SymbolicIntStack, MemConcreteIntToSymbolicInt, SymbolicIntConstraint>,
    >,

    expected: Vec<SymbolicMachineOutput>,
) {
    let stack = SymbolicIntStack::new();
    let mem = MemConcreteIntToSymbolicInt::new();
    let machine = SymbolicIntMachine::new(stack, mem, &pgm, None, None);

    let actual = run_sym_machine(machine);

    assert_eq!(actual, expected)
}

#[test]
fn test_basic() {
    test_helper(
        vec![PUSH(1), PUSH(2), PUSH(3), ADD(), SUB()],
        vec![
            SymbolicMachineOutput {
                concrete: Some(4),
                symbolic: Some(4.into()),
                model: vec![]
            }
        ]
    );
}

#[test]
fn test_basic_symbolic_1() {
    test_helper(
        vec![PUSH(1), PUSH(SYM("a".into())), ADD()],
        vec![
            SymbolicMachineOutput {
                concrete: Some(1),
                symbolic: Some(SYM("a".into()) + 1.into()),
                model: vec![("a".into(), 0)]
            }
        ]
    )
}

#[test]
fn test_basic_symbolic_2() {
    test_helper(
        vec![PUSH(1), PUSH(2), PUSH(SYM("a".into())), ADD(), SUB()],
        vec![
            SymbolicMachineOutput {
                symbolic: Some(SYM("a".into()) + 2.into() - 1.into()),
                concrete: Some(1),
                model: vec![("a".into(), 0)],
            }
        ]
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
        vec![
            SymbolicMachineOutput {
                concrete: Some(200),
                symbolic: Some(200.into()),
                model: vec![("a".into(), 3)]
            },
            SymbolicMachineOutput {
                concrete: Some(100),
                symbolic: Some(100.into()),
                model: vec![("a".into(), 4)]
            }
        ],
    )
}
