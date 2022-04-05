use symbolic_stack_machines::{
    instructions::helpers::{ADD, PUSH, SUB},
    machine::SymbolicIntMachine,
    memory::symbolic_concrete_index::MemConcreteIntToSymbolicInt,
    stack::SymbolicIntStack,
    symbolic_int::{Inner, SymbolicInt, SYM, C},
};

#[test]
fn test_basic() {
    let stack = SymbolicIntStack::new();
    let mem = MemConcreteIntToSymbolicInt::new();
    let machine = SymbolicIntMachine::new(stack, mem);

    let pgm = vec![PUSH(1), PUSH(2), PUSH(3), ADD(), SUB()];

    assert_eq!(machine.run(&pgm), Option::Some(4.into()))
}

#[test]
fn test_basic_symbolic_1() {
    let stack = SymbolicIntStack::new();
    let mem = MemConcreteIntToSymbolicInt::new();
    let machine = SymbolicIntMachine::new(stack, mem);

    let pgm = vec![PUSH(1), PUSH(SYM()), ADD()];

    assert_eq!(
        machine.run(&pgm),
        SymbolicInt::S(Inner::Add(SYM().into(), C(1))).into()
    )
}

#[test]
fn test_basic_symbolic_2() {
    let stack = SymbolicIntStack::new();
    let mem = MemConcreteIntToSymbolicInt::new();
    let machine = SymbolicIntMachine::new(stack, mem);

    let pgm = vec![
        PUSH(1),
        PUSH(2),
        PUSH(SymbolicInt::S(Inner::Sym)),
        ADD(),
        SUB(),
    ];

    let expected = 
        SymbolicInt::S(
            Inner::Sub(
                SymbolicInt::S(
                    Inner::Add(
                        SYM().into(),
                        C(2)
                    )
                ).into(),
                C(1)
            )
        ).into();

    assert_eq!(
        machine.run(&pgm), 
        expected
    )
}
