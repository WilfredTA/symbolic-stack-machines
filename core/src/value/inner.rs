use super::Simplifiable;
use super::ground::*;
use super::val::*;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InnerValue {
    Literal(Literal),
    Boolean(Value<Boolean>),
    Arithmetic(Value<Arithmetic>),
    // Binary
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Literal {
    ConcreteLiteral(Value<ConcreteInnerValue>),
    SymbolicLiteral(Value<SymbolicInnerValue>),
}

impl From<GroundValue> for Literal {
    fn from(v: GroundValue) -> Self {
        match v {
            GroundValue::Concrete(c) => Literal::ConcreteLiteral(Value::new(c)),
            GroundValue::Symbolic(s) => Literal::ConcreteLiteral(Value::new(s)),
            GroundValue::Boolean(_) => todo!(),
        }
    }
}
// impl InnerValue {
//     pub fn unwrap(&self) -> Rc<dyn Any> {
//         match self {
//             InnerValue::ConcreteLiteral(v) => Rc::new(v.clone()),
//             InnerValue::SymbolicLiteral(v) => Rc::new(v.clone()),
//             InnerValue::Boolean(v) => Rc::new(v.clone()),
//             InnerValue::Arithmetic(v) => Rc::new(v.clone()),
//         }
//     }
// }

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Arithmetic {
    Add(InnerValue, InnerValue),
    Sub(InnerValue, InnerValue),
    Mul(InnerValue, InnerValue),
    Div(InnerValue, InnerValue),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Boolean {
    ValCmp(ValCmp),
    BoolFormula(BoolFormula),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoolFormula {
    True,
    False,
    Or(Value<Boolean>, Value<Boolean>),
    And(Value<Boolean>, Value<Boolean>),
    Not(Value<Boolean>, Value<Boolean>),
    Ite(Value<Boolean>, InnerValue, InnerValue),
    Assert(InnerValue),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValCmp {
    Gt(Value<InnerValue>, Value<InnerValue>),
    Gte(Value<InnerValue>, Value<InnerValue>),
    Lt(Value<InnerValue>, Value<InnerValue>),
    Lte(Value<InnerValue>, Value<InnerValue>),
    Eq(Value<InnerValue>, Value<InnerValue>),
    Neq(Value<InnerValue>, Value<InnerValue>),
}

impl InnerValue {
    pub fn _eq(&self, other: InnerValue) -> Self {
        InnerValue::Boolean(
            Value::new(
                Boolean::ValCmp(
                    ValCmp::Eq(
                        Value::new(self.clone()),
                        Value::new(other.clone())
                    )
                )
            )
        )
        
    }

    pub fn _neq(&self, other: InnerValue) -> Self {
        InnerValue::Boolean(
            Value::new(
                Boolean::ValCmp(
                    ValCmp::Neq(
                        Value::new(self.clone()),
                        Value::new(other.clone())
                    )
                )
            )
        )
    }

    pub fn _gt(&self, other: InnerValue) -> Self {
        InnerValue::Boolean(
            Value::new(
                Boolean::ValCmp(
                    ValCmp::Gt(
                        Value::new(self.clone()),
                        Value::new(other.clone())
                    )
                )
            )
        )
    }

    pub fn _gte(&self, other: InnerValue) -> Self {
        InnerValue::Boolean(
            Value::new(
                Boolean::ValCmp(
                    ValCmp::Gte(
                        Value::new(self.clone()),
                        Value::new(other.clone())
                    )
                )
            )
        )
    }

    pub fn _lt(&self, other: InnerValue) -> Self {
        InnerValue::Boolean(
            Value::new(
                Boolean::ValCmp(
                    ValCmp::Lt(
                        Value::new(self.clone()),
                        Value::new(other.clone())
                    )
                )
            )
        )
    }

    pub fn _lte(&self, other: InnerValue) -> Self {
        InnerValue::Boolean(
            Value::new(
                Boolean::ValCmp(
                    ValCmp::Lte(
                        Value::new(self.clone()),
                        Value::new(other.clone())
                    )
                )
            )
        )
    }
}

impl std::ops::Sub for InnerValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Arithmetic(Value::new(Arithmetic::Sub(
            self.clone(),
            rhs.clone())
        ))
    }
}

impl std::ops::Mul for InnerValue {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Arithmetic(Value::new(Arithmetic::Mul(
            self.clone(),
            rhs.clone())
        ))
    }
}

impl std::ops::Div for InnerValue {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Arithmetic(Value::new(Arithmetic::Div(
            self.clone(),
            rhs.clone())
        ))
    }
}

// impl<T> From<T> for InnerValue
// where T: Into<GroundValue> {
//     fn from(v: T) -> Self {
        
//     }
// }

// impl From<u8> for InnerValue {
//     fn from(v: u8) -> Self {
//         InnerValue::ConcreteLiteral(Value::new(ConcreteInnerValue::ConcreteU8(v))))
//     }
// }

// impl From<u16> for InnerValue {
//     fn from(v: u16) -> Self {
//         InnerValue::ConcreteLiteral(Value::new(ConcreteInnerValue::ConcreteU16(v))))
//     }
// }

// impl From<u32> for InnerValue {
//     fn from(v: u32) -> Self {
//         InnerValue::ConcreteLiteral(Value::new(ConcreteInnerValue::ConcreteU32(v))))
//     }
// }

// impl From<u64> for InnerValue {
//     fn from(v: u64) -> Self {
//         InnerValue::ConcreteLiteral(Value::new(ConcreteInnerValue::ConcreteU64(v))))
//     }
// }

// impl From<u128> for InnerValue {
//     fn from(v: u128) -> Self {
//         InnerValue::ConcreteLiteral(Value::new(ConcreteInnerValue::ConcreteU128(v))))
//     }
// }

// impl From<Boolean> for InnerValue {
//     fn from(b: Boolean) -> Self {
//         Self::Boolean(b)
//     }
// }

// impl From<Arithmetic> for InnerValue {
//     fn from(a: Arithmetic) -> Self {
//         Self::Arithmetic(a)
//     }
// }

// impl From<ConcreteInnerValue> for InnerValue {
//     fn from(c: ConcreteInnerValue) -> Self {
//         Self::ConcreteLiteral(c)
//     }
// }

// impl From<SymbolicInnerValue> for InnerValue {
//     fn from(s: SymbolicInnerValue) -> Self {
//         Self::SymbolicLiteral(s)
//     }
// }

// impl From<InnerValue> for Boolean {
//     fn from(b: InnerValue) -> Self {
//         match b {
//             InnerValue::ConcreteLiteral(_) => todo!(),
//             InnerValue::SymbolicLiteral(_) => todo!(),
//             InnerValue::Boolean(_) => todo!(),
//             InnerValue::Arithmetic(_) => todo!(),
//         }
//     }
// }

// impl From<InnerValue> for Arithmetic {
//     fn from(a: InnerValue) -> Self {
     
//     }
// }

// impl From<InnerValue> for ConcreteInnerValue {
//     fn from(c: InnerValue) -> Self {
       
//     }
// }

// impl From<InnerValue> for SymbolicInnerValue {
//     fn from(s: InnerValue) -> Self {
        
//     }
// }


// impl Simplifiable<Arithmetic> for InnerValue {
//     type GroundVal = GroundValue;


//     fn simplify(&self) -> Self::GroundVal {
//         match self {
//             Arithmetic::Add(l, r) => todo!(),
//             Arithmetic::Sub(l, r) => todo!(),
//             Arithmetic::Mul(l, r) => todo!(),
//             Arithmetic::Div(l, r) => todo!(),
//         }
//         todo!()
//     }
// }

// impl Simplifiable<Boolean> for InnerValue {
//     type GroundVal = GroundValue;

   
// }
// impl Simplifiable<InnerValue> for Arithmetic {
//     type GroundVal = GroundValue;
//     fn simplify(&self) -> Self::GroundVal {
//         match self {
//             Arithmetic::Add(l, r) => todo!(),
//             Arithmetic::Sub(l, r) => todo!(),
//             Arithmetic::Mul(l, r) => todo!(),
//             Arithmetic::Div(l, r) => todo!(),
//         }
//         todo!()
//     }

// }

// impl Simplifiable<InnerValue> for Boolean {
//     type GroundVal = GroundValue;

//     fn simplify(&self) -> Self::GroundVal {
//         match self {
//             Boolean::ValCmp(val_cmp) => todo!(),
//             Boolean::BoolFormula(bool_f) => todo!(),
//         }
//         todo!()
//     }
// }

// impl Simplifiable<InnerValue> for ValCmp {
//     type GroundVal = GroundValue;

//     fn simplify(&self) -> Self::GroundVal {
//         match self {
//             ValCmp::Gt(l, r) => todo!(),
//             ValCmp::Gte(l, r) => todo!(),
//             ValCmp::Lt(l, r) => todo!(),
//             ValCmp::Lte(l, r) => todo!(),
//             ValCmp::Eq(l, r) => todo!(),
//             ValCmp::Neq(l, r) => todo!(),
//         }
//     }
// }

// impl Simplifiable<InnerValue> for BoolFormula {
//     type GroundVal = GroundValue;

//     fn simplify(&self) -> Self::GroundVal {
//         match self {
//             BoolFormula::True => todo!(),
//             BoolFormula::False => todo!(),
//             BoolFormula::Or(l, r) => todo!(),
//             BoolFormula::And(l, r) => todo!(),
//             BoolFormula::Not(l, r) => todo!(),
//             BoolFormula::Ite(c, f, t) => todo!(),
//             BoolFormula::Assert(v) => todo!(),
//         }
//     }
// }