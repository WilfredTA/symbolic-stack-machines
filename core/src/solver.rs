use std::collections::HashMap;

pub trait Constrain<C> {
    fn assert(self, other: Self) -> C;
}

pub trait Solver<Assertion, Result> {
    fn solve(&self, assertions: &Vec<Assertion>) -> Option<HashMap<String, Result>>;
}
