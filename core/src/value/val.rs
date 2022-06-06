use std::rc::Rc;
use super::inner::*;
use super::ground::*;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Value<T>(pub Rc<T>);
impl<T> Value<T> {
    pub fn inner(&self) -> Rc<T> {
        Rc::clone(&self.0)
    }
    pub fn new(v: T) -> Self {
        Self(Rc::new(v))
    }
}


impl Value<InnerValue> {
    pub fn is_ground_value(&self) -> bool {
        match self.0.as_ref() {
            InnerValue::ConcreteLiteral(_) => true,
            InnerValue::SymbolicLiteral(_) => true,
            InnerValue::Boolean(_) => false,
            InnerValue::Arithmetic(_) => false,
        }
    }

    // TODO(tannr): Replace this method with implementation of GroundValueConversion trait
    pub fn get_ground_value(&self) -> Option<GroundValue> {
        match self.0.as_ref() {
            InnerValue::ConcreteLiteral(v) => {
                Some(GroundValue::Concrete(v.inner().as_ref().clone()))
            }
            InnerValue::SymbolicLiteral(v) => {
                Some(GroundValue::Symbolic(v.inner().as_ref().clone()))
            }
            InnerValue::Boolean(v) => match v.inner().as_ref() {
                Boolean::ValCmp(_cmp) => None,
                Boolean::BoolFormula(_v) => None,
            },
            InnerValue::Arithmetic(_v) => None,
        }
    }
}