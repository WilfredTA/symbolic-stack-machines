use symbolic_stack_machines::{
    instructions::sym_helpers::{ADD, ISZERO, JUMPI, PUSH, STOP, SUB},
    machine::{
        run_sym_machine, SymbolicIntMachine, SymbolicIntMachineInnerConcrete,
        SymbolicMachineOutput, SymbolicProgram,
    },
    memory::MemConcreteIntToSymbolicInt,
    solvers::Z3Solver,
    stack::SymbolicIntStack,
    vals::{SymbolicIntConstraint, SymbolicIntRV, SYM},
};

fn test_helper(
    pgm: SymbolicProgram<SymbolicIntStack, MemConcreteIntToSymbolicInt, SymbolicIntConstraint>,

    expected: Vec<SymbolicMachineOutput<SymbolicIntRV, i64, Option<i64>>>,
) {
    let stack = SymbolicIntStack::new();
    let mem = MemConcreteIntToSymbolicInt::new();
    let concrete_machine = SymbolicIntMachineInnerConcrete::new(stack, mem, &pgm, None);
    let machine = SymbolicIntMachine::new(concrete_machine, None);

    let mut cfg = z3::Config::default();
    cfg.set_model_generation(true);
    let ctx = z3::Context::new(&cfg);

    let solver = Z3Solver::new(&ctx);

    let actual = run_sym_machine(machine, solver);

    assert_eq!(actual, expected)
}

#[test]
fn test_basic() {
    test_helper(
        vec![PUSH(1), PUSH(2), PUSH(3), ADD(), SUB()],
        vec![SymbolicMachineOutput {
            symbolic: Some(4.into()).into(),
            concrete: Some(4),
            model: vec![],
            additional_model: vec![],
        }],
    );
}

#[test]
fn test_basic_symbolic_1() {
    test_helper(
        vec![PUSH(1), PUSH(SYM("a".into())), ADD()],
        vec![SymbolicMachineOutput {
            symbolic: Some(SYM("a".into()) + 1.into()).into(),
            concrete: Some(1),
            model: vec![],
            additional_model: vec![("a".into(), 0.into())],
        }],
    )
}

#[test]
fn test_basic_symbolic_2() {
    test_helper(
        vec![PUSH(1), PUSH(2), PUSH(SYM("a".into())), ADD(), SUB()],
        vec![SymbolicMachineOutput {
            symbolic: Some(SYM("a".into()) + 2.into() - 1.into()).into(),
            concrete: Some(1),
            model: vec![],
            additional_model: vec![("a".into(), 0.into())],
        }],
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
                symbolic: Some(200.into()).into(),
                concrete: Some(200),
                additional_model: vec![],
                model: vec![("a".into(), 3.into())],
            },
            SymbolicMachineOutput {
                symbolic: Some(100.into()).into(),
                concrete: Some(100),
                additional_model: vec![],
                model: vec![("a".into(), 4.into())],
            },
        ],
    )
}
