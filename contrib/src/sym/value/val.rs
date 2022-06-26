use std::rc::Rc;

use symbolic_stack_machines_core::solver::Constrain;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value<T> {
    C(T),
    S(String),

    // Arithmetic
    Add(Rc<Value<T>>, Rc<Value<T>>),
    Sub(Rc<Value<T>>, Rc<Value<T>>),
}

pub struct Assertion<T>(pub Value<T>, pub Value<T>);

impl<T> Value<T> {
    pub fn new(c: T) -> Self {
        Self::C(c)
    }

    pub fn new_sym(s: String) -> Self {
        Self::S(s)
    }
}

#[allow(non_snake_case)]
pub fn C<T>(c: T) -> Value<T> {
    Value::new(c)
}

#[allow(non_snake_case)]
pub fn S<T>(s: &str) -> Value<T> {
    Value::new_sym(s.into())
}

impl<T> Constrain<Assertion<T>> for Value<T> {
    fn assert(self, other: Self) -> Assertion<T> {
        Assertion(self, other)
    }
}
