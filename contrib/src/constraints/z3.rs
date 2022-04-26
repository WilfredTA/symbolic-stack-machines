use symbolic_stack_machines_core::constraint::*;
use z3::ast::Bool;
use z3::{Solver as Z3InnerSolver, SatResult as Z3SatResult, Model};



pub struct Z3Solver<'a> {
    inner: Z3InnerSolver<'a>,
    ctx: z3::Context
}

impl<'a> Constrained for Z3Solver<'a> {
    type Model = Model<'a>;

    fn check(&self) -> SatResult<Self::Model> {
        todo!()
    }
}

impl Solver for Z3Solver<'_> {
    fn assert<V>(&mut self, constraint: &Constraint<V>) {
      todo!()// self.inner.assert(constraint.0)
    }

    fn solve<V>(&self) -> SatResult<Self::Model> {
        todo!()
    }
}