use super::Simplifiable;
use super::ground::*;
use super::inner::*;
use std::rc::Rc;
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
            InnerValue::Literal(_) => true,
            InnerValue::Boolean(_) => false,
            InnerValue::Arithmetic(_) => false,
        }
    }

    // TODO(tannr): Replace this method with implementation of GroundValueConversion trait
    pub fn get_ground_value(&self) -> Option<GroundValue> {
        match self.0.as_ref() {
            InnerValue::Literal(l) => {
                match l {
                    Literal::ConcreteLiteral(c) => {
                        Some(c.inner().as_ref().into())
                    },
                    Literal::SymbolicLiteral(s) => {
                        Some(s.inner().as_ref().into())
                    }
                }
            },
            InnerValue::Boolean(v) => match v.inner().as_ref() {
                Boolean::ValCmp(_cmp) => None,
                Boolean::BoolFormula(_v) => None,
            },
            InnerValue::Arithmetic(_v) => None,
        }
    }
}

// impl Simplifiable<InnerValue> for Value<InnerValue> {
//     type GroundVal = GroundValue;

//     fn simplify(&self) -> Self::GroundVal {
//         match self.inner().as_ref() {
//             InnerValue::ConcreteLiteral(c) => todo!(),
//             InnerValue::SymbolicLiteral(s) => todo!(),
//             InnerValue::Boolean(b) => todo!(),
//             InnerValue::Arithmetic(a) => todo!(),
//         }
//         GroundValue::Boolean(true)
//     }
// }

// impl Simplifiable<ConcreteInnerValue> for Value<ConcreteInnerValue> {
//     type GroundVal = GroundValue;

//     fn simplify(&self) -> Self::GroundVal {
//         match self.inner().as_ref() {
//             ConcreteInnerValue::ConcreteU8(c) => todo!(),
//             ConcreteInnerValue::ConcreteU16(c) => todo!(),
//             ConcreteInnerValue::ConcreteU32(c) => todo!(),
//             ConcreteInnerValue::ConcreteU64(c) => todo!(),
//             ConcreteInnerValue::ConcreteU128(c) => todo!(),
//             ConcreteInnerValue::ConcreteBytes(c) => todo!(),
//         }
//         GroundValue::Boolean(true)
//     }
// }

// impl Simplifiable<SymbolicInnerValue> for Value<SymbolicInnerValue> {
//     type GroundVal = GroundValue;

//     fn simplify(&self) -> Self::GroundVal {
//         match self.inner().as_ref() {
//             SymbolicInnerValue::SymbolicU8(s) => todo!(),
//             SymbolicInnerValue::SymbolicU16(s) => todo!(),
//             SymbolicInnerValue::SymbolicU32(s) => todo!(),
//             SymbolicInnerValue::SymbolicU64(s) => todo!(),
//             SymbolicInnerValue::SymbolicI64(s) => todo!(),
//             SymbolicInnerValue::SymbolicU128(s) => todo!(),
//             SymbolicInnerValue::SymbolicBitVec(s) => todo!(),
//         }
//         GroundValue::Boolean(true)
//     }
// }

// impl Simplifiable<Boolean> for Value<Boolean> {
//     type GroundVal = GroundValue;

//     fn simplify(&self) -> Self::GroundVal {
//         match self.inner().as_ref() {
//             Boolean::ValCmp(val_cmp) => todo!(),
//             Boolean::BoolFormula(bool_f) => todo!(),
//         }
//         GroundValue::Boolean(true)
//     }
// }

// impl Simplifiable<Arithmetic> for Value<Arithmetic> {
//     type GroundVal = GroundValue;

//     fn simplify(&self) -> Self::GroundVal {
//         match self.inner().as_ref() {
//             Arithmetic::Add(l, r) => todo!(),
//             Arithmetic::Sub(l, r) => todo!(),
//             Arithmetic::Mul(l, r) => todo!(),
//             Arithmetic::Div(l, r) => todo!(),
//         }
//         GroundValue::Boolean(true)
//     }
// }