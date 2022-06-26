// Simple lang, but with each AbstractInstruction defined
// on each individual singleton instruction

use symbolic_stack_machines_contrib::sym::{Value, C, S};
mod common;
use common::{run_concrete_machine, simple_lang::*};

#[test]
fn test_abstract_machine() {
    let pgm = vec![push(15_u64), push(5), push(5), push(5), add(), add(), sub()];
    let res = run_concrete_machine(pgm);
    assert_eq!(res, Some(0))
}

#[test]
fn test_abstract_arithmetic_all_concrete_values() {
    let pgm = vec![push(C(30)), push(C(20)), add()];
    let res = run_concrete_machine(pgm);
    assert_eq!(res.unwrap(), C(50));
}

#[test]
fn test_abstract_arithmetic_symbolic_values() {
    let pgm = vec![push(C(30_u64)), push(S("X")), add()];
    let res = run_concrete_machine(pgm);
    assert_eq!(res.unwrap(), Value::Add(S("X").into(), C(30).into()));
}
