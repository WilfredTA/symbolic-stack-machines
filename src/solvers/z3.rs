use z3::{ast, Config, Context, SatResult, Solver};

use crate::symbolic_int::SymbolicInt;

// TODO should be abstract over more than z3:ast::Int
pub trait Z3Constraint {
    fn z3_constraint<'ctx>(
        &self,
        ctx: &'ctx Context,
        solve: &mut Vec<(String, z3::ast::Int<'ctx>)>,
    ) -> ast::Bool<'ctx>;
}

// TODO should be abstract over more than i64

pub fn solve<T: Z3Constraint>(
    constraints: Vec<T>,
    additional: Option<SymbolicInt>,
) -> Option<(Vec<(String, i64)>, Option<i64>)> {
    let mut cfg = Config::default();
    cfg.set_model_generation(true);

    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let mut solve = vec![];

    constraints.iter().for_each(|x| {
        let constraint = x.z3_constraint(&ctx, &mut solve);

        solver.assert(&constraint);
    });

    if solver.check() != SatResult::Sat {
        return None;
    }

    let model = solver.get_model().unwrap();

    let rv = additional.map(|x| {
        x.z3_int(&ctx, &mut solve)
    });

    let xrv = rv.map(|x| model.eval(&x, true).unwrap().as_i64().unwrap());

    let syms = solve
        .into_iter()
        .map(|(sym, ast)| {
            let concrete = model.eval(&ast, true).unwrap().as_i64().unwrap();
            (sym, concrete)
        })
        .collect();

    Option::Some((syms, xrv))
}
