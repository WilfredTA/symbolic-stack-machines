use z3::{ast, Config, Context, SatResult, Solver};

pub trait Z3Constraint {
    fn z3_constraint<'ctx>(&self, ctx: &'ctx Context) -> ast::Bool<'ctx>;
}

// TODO need to use model to find actual values

pub fn solve<T: Z3Constraint>(constraints: Vec<T>) -> bool {
    let mut cfg = Config::default();
    cfg.set_model_generation(true);
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    constraints
        .iter()
        .for_each(|x| solver.assert(&x.z3_constraint(&ctx)));

    solver.check() == SatResult::Sat
}
