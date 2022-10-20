use std::rc::Rc;
mod number;
pub use number::*;
mod simple_type;
pub use simple_type::*;
mod boolean;
pub use boolean::*;
mod vecc;
pub use vecc::*;
mod sentence;
pub use sentence::*;
pub mod visitors;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolId<T>(pub String, pub Option<T>);

impl<T> SymbolId<T> {
    pub fn new(item: Option<T>) -> Self {
        Self (uuid::Uuid::new_v4().to_string(), item)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Val<T>(pub Rc<T>);
impl<T> Val<T> {
    pub fn inner(&self) -> Rc<T> {
        Rc::clone(&self.0)
    }
    pub fn new(v: T) -> Self {
        Self(Rc::new(v))
    }
}

pub trait Visitor<T> {
    fn visit_sentence(&mut self, s: &Sentence) -> T;

    fn visit_bin_op(&mut self, a: &Sentence, b: &Sentence) -> T;

    fn visit_unary_op(&mut self, s: &Sentence) -> T;

    fn visit_ternary_op(&mut self, a: &Sentence, b: &Sentence, c: &Sentence) -> T;

    fn visit_val(&mut self, s: &Value) -> T;
}
