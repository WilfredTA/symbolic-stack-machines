use z3::{
    ast::{Ast, Int},
    Context,
};

pub fn z3_int<'a>(i: u64, ctxt: &'a Context) -> z3::ast::Int<'a> {
    Int::from_u64(&ctxt, i)
}

pub fn z3_int_var<'a>(i: &str, ctxt: &'a Context) -> z3::ast::Int<'a> {
    Int::new_const(&ctxt, i)
}
