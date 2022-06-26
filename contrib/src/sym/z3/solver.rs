use std::collections::HashMap;

use symbolic_stack_machines_core::solver::Solver;
use z3::{ast::Ast, Context};

use crate::sym::value::Assertion;

pub struct Z3Solver {
    ctx: Context,
}

impl Z3Solver {
    pub fn new(ctx: Context) -> Self {
        Z3Solver { ctx }
    }
}

impl Solver<Assertion<u64>, u64> for Z3Solver {
    fn solve(&self, assertions: &Vec<Assertion<u64>>) -> Option<HashMap<String, u64>> {
        let solver = z3::Solver::new(&self.ctx);

        let mut vars = vec![];

        assertions.into_iter().for_each(|assertion| {
            let constraint = assertion
                .0
                .transpile_u64_z3(&self.ctx, &mut vars)
                ._eq(&assertion.1.transpile_u64_z3(&self.ctx, &mut vars));
            solver.assert(&constraint);
        });

        match solver.check() {
            z3::SatResult::Unsat => None,
            z3::SatResult::Unknown => None,
            z3::SatResult::Sat => {
                let model = solver.get_model().unwrap();

                let mut rv = HashMap::new();

                vars.into_iter().for_each(|var| {
                    let solved = model.eval(&var, true).unwrap();
                    let concrete = solved.as_u64().unwrap();
                    rv.insert(var.to_string(), concrete);
                });

                Some(rv)
            }
        }
    }
}
