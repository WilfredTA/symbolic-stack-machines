use z3::ast::Ast;

use crate::solvers::Z3Constraint;

use super::{SymbolicIntConstraint, SymbolicInt, symbolic_int::{Inner, EqCheck}};

impl<'ctx> Z3Constraint<'ctx, i64> for SymbolicIntConstraint {
    type SolveSym = z3::ast::Int<'ctx>;

    fn z3_constraint(
        &self,
        ctx: &'ctx z3::Context,
        solve: &mut Vec<(String, Box<Self::SolveSym>)>,
    ) -> z3::ast::Bool<'ctx> {
        match self {
            SymbolicIntConstraint::Eq(l, r) => l.z3_int(ctx, solve)._eq(&r.z3_int(ctx, solve)),
            SymbolicIntConstraint::Not(x) => {
                x.z3_constraint(ctx, solve).not()
            }
        }
    }

    fn as_concrete(sym: Self::SolveSym) -> i64 {
        sym.as_i64().unwrap()
    }
}

impl SymbolicInt {
    pub fn z3_int<'ctx>(
        &self,
        ctx: &'ctx z3::Context,
        solve: &mut Vec<(String, Box<z3::ast::Int<'ctx>>)>,
    ) -> z3::ast::Int<'ctx> {
        match self {
            SymbolicInt::C(i) => z3::ast::Int::from_i64(ctx, *i as i64),
            SymbolicInt::S(i) => i.z3_int(ctx, solve),
        }
    }
}

impl Inner {
    pub fn z3_int<'ctx>(
        &self,
        ctx: &'ctx z3::Context,
        solve: &mut Vec<(String, Box<z3::ast::Int<'ctx>>)>,
    ) -> z3::ast::Int<'ctx> {
        match self {
            Inner::Sym(name) => {
                let rv = z3::ast::Int::new_const(ctx, z3::Symbol::from(name.as_str()));

                // TODO(will) - ideally, we would be able to store a direct reference 
                // for both and wouldn't have to clone
                solve.push((name.clone(), Box::new(rv.clone())));

                rv
            }
            Inner::Add(l, r) => l.z3_int(ctx, solve) + r.z3_int(ctx, solve),
            Inner::Sub(l, r) => l.z3_int(ctx, solve) - r.z3_int(ctx, solve),
            Inner::Ite(pred, then, xelse) => pred
                .z3_bool(ctx, solve)
                .ite(&then.z3_int(ctx, solve), &xelse.z3_int(ctx, solve)),

            // TODO(will) - implement
            Inner::RW(_) => todo!(),
        }
    }
}

impl EqCheck {
    pub fn z3_bool<'ctx>(
        &self,
        ctx: &'ctx z3::Context,
        solve: &mut Vec<(String, Box<z3::ast::Int<'ctx>>)>,
    ) -> z3::ast::Bool<'ctx> {
        self.l.z3_int(ctx, solve)._eq(&self.r.z3_int(ctx, solve))
    }
}
