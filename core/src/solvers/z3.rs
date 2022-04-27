use std::ops::Deref;

use z3::{ast, Context, SatResult};

use super::{Solver, SolverOutput};

pub struct Z3Solver<'ctx> {
    ctx: &'ctx Context,
}

impl<'ctx> Z3Solver<'ctx> {
    pub fn new(ctx: &'ctx Context) -> Self {
        Self { ctx }
    }
}

pub trait Z3Constraint<'ctx, Output> {
    type SolveSym: z3::ast::Ast<'ctx>;

    fn z3_constraint(
        &self,
        ctx: &'ctx Context,
        solve: &mut Vec<(String, Box<Self::SolveSym>)>,
    ) -> ast::Bool<'ctx>;

    fn as_concrete(sym: Self::SolveSym) -> Output;
}

pub trait Z3Value<'ctx, Output> {
    type SolveSym: z3::ast::Ast<'ctx>;

    fn z3_value(
        &self,
        ctx: &'ctx Context,
        solve: &mut Vec<(String, Box<Self::SolveSym>)>,
    ) -> Self::SolveSym;
    fn as_concrete(sym: Self::SolveSym) -> Output;
}

impl<'ctx, Constraint, AdditionalValue, ModelOutput, AdditionalValueOutput>
    Solver<Constraint, AdditionalValue, ModelOutput, AdditionalValueOutput> for Z3Solver<'ctx>
where
    Constraint: Z3Constraint<'ctx, ModelOutput>,
    AdditionalValue: Z3Value<'ctx, AdditionalValueOutput>,
{
    fn solve(
        &self,
        constraints: &Vec<Constraint>,
        additional: Vec<AdditionalValue>,
    ) -> Option<SolverOutput<ModelOutput, AdditionalValueOutput>> {
        let solver = z3::Solver::new(self.ctx);

        let mut constraints_solve = vec![];
        let mut additional_values_solve = vec![];

        constraints.iter().for_each(|x| {
            let constraint = x.z3_constraint(self.ctx, &mut constraints_solve);
            solver.assert(&constraint);
        });

        if solver.check() != SatResult::Sat {
            return None;
        };

        let model: z3::Model = solver.get_model().unwrap();

        let additional: Vec<Option<AdditionalValueOutput>> = additional
            .into_iter()
            .map(|x| {
                let sym = model.eval(&x.z3_value(self.ctx, &mut additional_values_solve), true);
                let concrete = sym.map(|x| AdditionalValue::as_concrete(x));
                concrete
            })
            .collect();

        let xmodel: Vec<(String, ModelOutput)> = constraints_solve
            .into_iter()
            .map(|(str, ast)| {
                let sym = model.eval(ast.deref(), true).unwrap();
                let concrete = Constraint::as_concrete(sym);
                (str, concrete)
            })
            .collect();

        let additional_model = additional_values_solve
            .into_iter()
            .map(|(str, ast)| {
                let sym = model.eval(ast.deref(), true).unwrap();
                let concrete = AdditionalValue::as_concrete(sym);
                (str, concrete)
            })
            .collect();

        Some(SolverOutput {
            model: xmodel,
            additional_model,
            additional,
        })
    }
}
