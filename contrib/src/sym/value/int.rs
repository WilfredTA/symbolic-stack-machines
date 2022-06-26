use z3::Context;

use super::Value;

impl Value<u64> {
    pub fn transpile_u64_z3<'ctx>(
        &self,
        ctx: &'ctx Context,
        vars: &mut Vec<z3::ast::Int<'ctx>>,
    ) -> z3::ast::Int<'ctx> {
        match self {
            Value::C(x) => z3::ast::Int::from_i64(ctx, *x as i64),
            Value::S(name) => {
                let rv = z3::ast::Int::new_const(&ctx, name.as_str());
                vars.push(rv.clone());
                rv
            }

            Value::Add(l, r) => l.transpile_u64_z3(ctx, vars) + r.transpile_u64_z3(ctx, vars),
            Value::Sub(l, r) => l.transpile_u64_z3(ctx, vars) - r.transpile_u64_z3(ctx, vars),
        }
    }
}
