use std::collections::HashMap;

use symbolic_stack_machines_contrib::{sym::{z3::Z3Solver, C, S}};
use symbolic_stack_machines_core::solver::{Solver, Constrain};

#[test]
fn test_z3_constraints() {
    let mut cfg = z3::Config::default();
    cfg.set_model_generation(true);
    let ctx = z3::Context::new(&cfg);

    let solver = Z3Solver::new(ctx);

    let actual = solver.solve(&vec![
        (S("X") + C(30)).assert(C(50))
    ]).unwrap();

    let expected = HashMap::from([
        ("X".to_string(), 20),
    ]);

    assert_eq!(actual, expected);
}
