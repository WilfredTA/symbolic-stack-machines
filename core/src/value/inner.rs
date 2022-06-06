use super::ground::*;
use super::val::*;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InnerValue {
    ConcreteLiteral(Value<ConcreteInnerValue>),
    SymbolicLiteral(Value<SymbolicInnerValue>),
    Boolean(Value<Boolean>),
    Arithmetic(Value<Arithmetic>),
    // Binary
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
    Add(Value<InnerValue>, Value<InnerValue>),
    Sub(Value<InnerValue>, Value<InnerValue>),
    Mul(Value<InnerValue>, Value<InnerValue>),
    Div(Value<InnerValue>, Value<InnerValue>),
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
    Ite(Value<Boolean>, Value<InnerValue>, Value<InnerValue>),
    Assert(Value<InnerValue>),
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
        InnerValue::Boolean(Value(Rc::new(Boolean::ValCmp(ValCmp::Eq(
            Value(Rc::new(self.clone())),
            Value(Rc::new(other.clone())),
        )))))
    }

    pub fn _neq(&self, other: InnerValue) -> Self {
        InnerValue::Boolean(Value(Rc::new(Boolean::ValCmp(ValCmp::Neq(
            Value(Rc::new(self.clone())),
            Value(Rc::new(other.clone())),
        )))))
    }

    pub fn _gt(&self, other: InnerValue) -> Self {
        InnerValue::Boolean(Value(Rc::new(Boolean::ValCmp(ValCmp::Gt(
            Value(Rc::new(self.clone())),
            Value(Rc::new(other.clone())),
        )))))
    }

    pub fn _gte(&self, other: InnerValue) -> Self {
        InnerValue::Boolean(Value(Rc::new(Boolean::ValCmp(ValCmp::Gte(
            Value(Rc::new(self.clone())),
            Value(Rc::new(other.clone())),
        )))))
    }

    pub fn _lt(&self, other: InnerValue) -> Self {
        InnerValue::Boolean(Value(Rc::new(Boolean::ValCmp(ValCmp::Lt(
            Value(Rc::new(self.clone())),
            Value(Rc::new(other.clone())),
        )))))
    }

    pub fn _lte(&self, other: InnerValue) -> Self {
        InnerValue::Boolean(Value(Rc::new(Boolean::ValCmp(ValCmp::Lte(
            Value(Rc::new(self.clone())),
            Value(Rc::new(other.clone())),
        )))))
    }
}




impl std::ops::Add for InnerValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Arithmetic(Value(Rc::new(Arithmetic::Add(
            Value(Rc::new(self.clone())),
            Value(Rc::new(rhs.clone())),
        ))))
    }
}

impl std::ops::Sub for InnerValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Arithmetic(Value(Rc::new(Arithmetic::Sub(
            Value(Rc::new(self.clone())),
            Value(Rc::new(rhs.clone())),
        ))))
    }
}

impl std::ops::Mul for InnerValue {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Arithmetic(Value(Rc::new(Arithmetic::Mul(
            Value(Rc::new(self.clone())),
            Value(Rc::new(rhs.clone())),
        ))))
    }
}

impl std::ops::Div for InnerValue {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Arithmetic(Value(Rc::new(Arithmetic::Div(
            Value(Rc::new(self.clone())),
            Value(Rc::new(rhs.clone())),
        ))))
    }
}



impl From<u8> for InnerValue {
    fn from(v: u8) -> Self {
        InnerValue::ConcreteLiteral(Value(Rc::new(ConcreteInnerValue::ConcreteU8(v))))
    }
}

impl From<u16> for InnerValue {
    fn from(v: u16) -> Self {
        InnerValue::ConcreteLiteral(Value(Rc::new(ConcreteInnerValue::ConcreteU16(v))))
    }
}

impl From<u32> for InnerValue {
    fn from(v: u32) -> Self {
        InnerValue::ConcreteLiteral(Value(Rc::new(ConcreteInnerValue::ConcreteU32(v))))
    }
}

impl From<u64> for InnerValue {
    fn from(v: u64) -> Self {
        InnerValue::ConcreteLiteral(Value(Rc::new(ConcreteInnerValue::ConcreteU64(v))))
    }
}

impl From<u128> for InnerValue {
    fn from(v: u128) -> Self {
        InnerValue::ConcreteLiteral(Value(Rc::new(ConcreteInnerValue::ConcreteU128(v))))
    }
}