use symbolic_stack_machines_core::constraint::*;
use symbolic_stack_machines_core::value::AbstractInt;
use z3::ast::{Ast, Bool, Int};
use z3::{Config, Context, Model, SatResult as Z3SatResult, Solver as Z3InnerSolver};

pub type ValInt = AbstractInt;

pub struct Z3SolverBuilder {
    ctx: Option<Context>,
}

impl<'a> Z3SolverBuilder {
    pub fn new() -> Self {
        Self { ctx: None }
    }

    pub fn ctx(mut self, cfg: Option<Config>) -> Self {
        self.ctx = Some(Context::new(&cfg.unwrap_or_default()));
        self
    }

    pub fn build<T>(self) -> Z3Solver<'a, T> {
        Z3Solver {
            inner: None,
            constraints: vec![],
            ctx: self.ctx.unwrap(),
        }
    }
}
pub struct Z3Solver<'a, T> {
    inner: Option<Z3InnerSolver<'a>>,
    constraints: Vec<Constraint<T>>,
    ctx: Context,
}

impl<'a, T> Z3Solver<'a, T> {
    pub fn inner(&self) -> &Z3InnerSolver<'a> {
        assert!(self.inner.is_some());
        self.inner.as_ref().unwrap()
    }
    pub fn get_ctx(&self) -> &'a Context {
        &self.inner().get_context()
    }

    pub fn get_constraints(&self) -> &Vec<Constraint<T>> {
        &self.constraints
    }

    pub fn set_solver(&'a mut self) {
        self.inner = Some(Z3InnerSolver::new(&self.ctx));
    }
}

pub fn z3_int<'a>(i: u64, ctxt: &'a Context) -> z3::ast::Int<'a> {
    Int::from_u64(&ctxt, i)
}

pub fn z3_int_var<'a>(i: &str, ctxt: &'a Context) -> z3::ast::Int<'a> {
    Int::new_const(&ctxt, i)
}

impl<'a, T> Constrained for Z3Solver<'a, T> {
    type Model = Model<'a>;

    fn check(&self) -> SatResult<Self::Model> {
        todo!()
    }
}

impl<'a> Solver<ValInt, Bool<'a>, Int<'a>> for Z3Solver<'a, ValInt> {
    fn solve(&self) -> SatResult<Self::Model> {
        match self.inner().check() {
            Z3SatResult::Sat => SatResult::Sat(self.inner().get_model().unwrap()),
            Z3SatResult::Unsat => SatResult::Unsat,
            Z3SatResult::Unknown => SatResult::Unknown,
        }
    }

    fn generic_assert(&mut self, constraint: &Constraint<ValInt>) {
        self.inner().assert(&self.transpile(constraint));
    }
}

impl<'a> Solver<u64, Bool<'a>, Int<'a>> for Z3Solver<'a, u64> {
    fn solve(&self) -> SatResult<Self::Model> {
        match self.inner().check() {
            Z3SatResult::Sat => SatResult::Sat(self.inner().get_model().unwrap()),
            Z3SatResult::Unsat => SatResult::Unsat,
            Z3SatResult::Unknown => SatResult::Unknown,
        }
    }

    fn generic_assert(&mut self, constraint: &Constraint<u64>) {
        self.inner().assert(&self.transpile(constraint));
    }
}

// TO DO: Generic impl

// impl<'a, T, A> Transpile<T, Bool<'a>, A> for Z3Solver<'a, T>
// where A: From<T> + Ast<'a>
// {

// }

// TODO(tannr): Impl Transpile from

impl<'a> Transpile<u64, Bool<'a>, Int<'a>> for Z3Solver<'a, u64> {
    fn val_to_ground_type(&self, v: u64) -> Int<'a> {
        z3_int(v, self.get_ctx())
    }

    fn ground_type_to_val(&self, g: Int<'a>) -> u64 {
        g.as_u64().unwrap()
    }

    fn assert(&self, c: Bool<'a>) -> Bool<'a> {
        self.inner().assert(&c);
        c
    }

    fn and(&self, l: Bool<'a>, r: Bool<'a>) -> Bool<'a> {
        z3::ast::Bool::and(l.get_ctx(), &vec![&l, &r])
    }

    fn not(&self, c: Bool<'a>) -> Bool<'a> {
        z3::ast::Bool::not(&c)
    }

    fn or(&self, l: Bool<'a>, r: Bool<'a>) -> Bool<'a> {
        z3::ast::Bool::or(l.get_ctx(), &vec![&l, &r])
    }

    fn gt(&self, l: Int<'a>, r: Int<'a>) -> Bool<'a> {
        l.gt(&r)
    }

    fn lt(&self, l: Int<'a>, r: Int<'a>) -> Bool<'a> {
        l.lt(&r)
    }

    fn lte(&self, l: Int<'a>, r: Int<'a>) -> Bool<'a> {
        l.le(&r)
    }

    fn gte(&self, l: Int<'a>, r: Int<'a>) -> Bool<'a> {
        l.ge(&r)
    }

    fn eq(&self, l: Int<'a>, r: Int<'a>) -> Bool<'a> {
        l._eq(&r)
    }

    fn neq(&self, l: Int<'a>, r: Int<'a>) -> Bool<'a> {
        self.not(self.eq(l, r))
    }

    fn true_(&self) -> Bool<'a> {
        Bool::from_bool(&self.get_ctx(), true)
    }

    fn false_(&self) -> Bool<'a> {
        Bool::from_bool(&self.get_ctx(), false)
    }
}

impl<'a> Transpile<ValInt, Bool<'a>, Int<'a>> for Z3Solver<'a, ValInt> {
    fn val_to_ground_type(&self, v: ValInt) -> Int<'a> {
        if let Some(val) = v.inner() {
            z3_int(val, self.get_ctx())
        } else {
            z3_int_var(v.id(), self.get_ctx())
        }
    }

    fn ground_type_to_val(&self, g: Int<'a>) -> ValInt {
        ValInt::new(g.as_u64().unwrap(), Some(g.to_string()))
    }

    fn assert(&self, c: Bool<'a>) -> Bool<'a> {
        self.inner().assert(&c);
        c
    }

    fn and(&self, l: Bool<'a>, r: Bool<'a>) -> Bool<'a> {
        z3::ast::Bool::and(l.get_ctx(), &vec![&l, &r])
    }

    fn not(&self, c: Bool<'a>) -> Bool<'a> {
        z3::ast::Bool::not(&c)
    }

    fn or(&self, l: Bool<'a>, r: Bool<'a>) -> Bool<'a> {
        z3::ast::Bool::or(l.get_ctx(), &vec![&l, &r])
    }

    fn gt(&self, l: Int<'a>, r: Int<'a>) -> Bool<'a> {
        l.gt(&r)
    }

    fn lt(&self, l: Int<'a>, r: Int<'a>) -> Bool<'a> {
        l.lt(&r)
    }

    fn lte(&self, l: Int<'a>, r: Int<'a>) -> Bool<'a> {
        l.le(&r)
    }

    fn gte(&self, l: Int<'a>, r: Int<'a>) -> Bool<'a> {
        l.ge(&r)
    }

    fn eq(&self, l: Int<'a>, r: Int<'a>) -> Bool<'a> {
        l._eq(&r)
    }

    fn neq(&self, l: Int<'a>, r: Int<'a>) -> Bool<'a> {
        self.not(self.eq(l, r))
    }

    fn true_(&self) -> Bool<'a> {
        Bool::from_bool(&self.get_ctx(), true)
    }

    fn false_(&self) -> Bool<'a> {
        Bool::from_bool(&self.get_ctx(), false)
    }
}
