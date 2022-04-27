use crate::solvers::Z3Value;

use super::SymbolicInt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolicIntRV(pub Option<SymbolicInt>);

impl<'ctx> Z3Value<'ctx, Option<i64>> for SymbolicIntRV {
    type SolveSym = z3::ast::Int<'ctx>;

    fn z3_value(
        &self,
        ctx: &'ctx z3::Context,
        solve: &mut Vec<(String, Box<Self::SolveSym>)>,
    ) -> Self::SolveSym {
        self.0.clone().unwrap().z3_int(ctx, solve)
    }

    fn as_concrete(sym: Self::SolveSym) -> Option<i64> {
        // Unwrap and rewrap because the return value option
        // is not for if `as_i64` succeeds
        Some(sym.as_i64().unwrap())
    }
}

impl From<Option<SymbolicInt>> for SymbolicIntRV {
    fn from(x: Option<SymbolicInt>) -> Self {
        Self(x)
    }
}