use rand::Rng;
use std::{any::Any, ops::Deref, rc::Rc};
use z3::ast::BV;

#[derive(Clone, Debug)]
pub struct AbstractValue<T> {
    symbol: Option<String>,
    val: T,
}

pub trait GroundValueConversion {
    type GroundVal;
    fn to(&self) -> Self::GroundVal;
}

// To do: Transpile like so:
// InnerValue (and all sub types) implement Into<smt_val> (e.g., Booleans implement Into<z3::ast::Bool>)

impl<T> AsRef<T> for AbstractValue<T> {
    fn as_ref(&self) -> &T {
        &self.val
    }
}
impl<T: Clone> AbstractValue<T> {
    pub fn new(val: impl Into<T>, symbol: Option<String>) -> Self {
        Self {
            symbol,
            val: val.into(),
        }
    }
    pub fn inner(&self) -> T {
        self.val.clone().into()
    }

    pub fn id(&self) -> &str {
        if let Some(ref symb) = self.symbol {
            symb.as_str()
        } else {
            ""
        }
    }

    pub fn set_val(&mut self, new_val: T) {
        self.val = new_val;
    }

    pub fn set_symbol(&mut self, new_symbol: String) {
        self.symbol = Some(new_symbol);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Value<T>(pub Rc<T>);
impl<T> Value<T> {
    pub fn inner(&self) -> Rc<T> {
        Rc::clone(&self.0)
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

    pub fn get_ground_value(&self) -> Option<GroundValue> {
        match self.0.as_ref() {
            InnerValue::ConcreteLiteral(v) => {
                Some(GroundValue::Concrete(v.inner().as_ref().clone()))
            }
            InnerValue::SymbolicLiteral(v) => {
                Some(GroundValue::Symbolic(v.inner().as_ref().clone()))
            }
            InnerValue::Boolean(v) => match v.inner().as_ref() {
                Boolean::ValCmp(cmp) => None,
                Boolean::BoolFormula(v) => None,
            },
            InnerValue::Arithmetic(v) => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GroundValue {
    Concrete(ConcreteInnerValue),
    Symbolic(SymbolicInnerValue),
    Boolean(bool),
}

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConcreteInnerValue {
    ConcreteU8(u8),
    ConcreteU16(u16),
    ConcreteU32(u32),
    ConcreteU64(u64),
    ConcreteU128(u128),
    ConcreteBytes(Vec<u8>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolicInnerValue {
    SymbolicU8(u8),
    SymbolicU16(u16),
    SymbolicU32(u32),
    SymbolicU64(u64),
    SymbolicI64(i64),
    SymbolicU128(u128),
    SymbolicBitVec(Vec<u8>),
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

impl From<ConcreteInnerValue> for SymbolicInnerValue {
    fn from(v: ConcreteInnerValue) -> Self {
        match v {
            ConcreteInnerValue::ConcreteU8(v) => SymbolicInnerValue::SymbolicU8(v),
            ConcreteInnerValue::ConcreteU16(v) => SymbolicInnerValue::SymbolicU16(v),
            ConcreteInnerValue::ConcreteU32(v) => SymbolicInnerValue::SymbolicU32(v),
            ConcreteInnerValue::ConcreteU64(v) => SymbolicInnerValue::SymbolicU64(v),
            ConcreteInnerValue::ConcreteU128(v) => SymbolicInnerValue::SymbolicU128(v),
            ConcreteInnerValue::ConcreteBytes(v) => SymbolicInnerValue::SymbolicBitVec(v),
        }
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

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz";
const SYMBOL_LEN: usize = 10;

fn random_val_symbol() -> String {
    let mut rng = rand::thread_rng();

    let symbol: String = (0..SYMBOL_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    symbol
}

pub type AbstractInt = AbstractValue<Option<u64>>;
// Val is the universal value type
pub type Val = AbstractValue<InnerValue>;

impl Deref for AbstractValue<InnerValue> {
    type Target = InnerValue;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl<T: Clone> From<T> for AbstractValue<T> {
    fn from(v: T) -> Self {
        AbstractValue::new(v, Some(random_val_symbol()))
    }
}

impl<T> std::ops::Add for AbstractValue<T>
where
    T: std::ops::Add + std::ops::Add<Output = T> + Clone,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let new_val: T = self.inner() + rhs.inner();
        Self::new(new_val, None)
    }
}

impl<T> std::ops::Sub for AbstractValue<T>
where
    T: std::ops::Sub + std::ops::Sub<Output = T> + Clone,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let new_val: T = self.inner() - rhs.inner();
        Self::new(new_val, None)
    }
}

impl<T> std::ops::Div for AbstractValue<T>
where
    T: std::ops::Div + std::ops::Div<Output = T> + Clone,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let new_val: T = self.inner() / rhs.inner();
        Self::new(new_val, None)
    }
}

// impl TryFrom<AbstractValue<InnerValue>> for u64
// {
//     type Error = String;

//     fn try_from(v: AbstractValue<InnerValue>) -> Result<Self, Self::Error> {
//         let v = match v.inner() {
//             InnerValue::ConcreteLiteral(v) => {
//                 match v.0.as_ref() {
//                     ConcreteInnerValue::ConcreteU8(v) => v.to_owned() as u64,
//                     ConcreteInnerValue::ConcreteU16(v) => v.to_owned() as u64,
//                     ConcreteInnerValue::ConcreteU32(v) => v.to_owned() as u64,
//                     ConcreteInnerValue::ConcreteU64(v) => v.to_owned() as u64,
//                     ConcreteInnerValue::ConcreteU128(_v) => panic!("Cannot convert from u128 to u64"),
//                     ConcreteInnerValue::ConcreteBytes(_v) => panic!("Cannot convert from vec<u8> to u64 safely"),
//                 }
//             },
//             InnerValue::SymbolicLiteral(_) => todo!(),
//             InnerValue::Boolean(_) => todo!(),
//             InnerValue::Arithmetic(_) => todo!(),
//         };
//         Ok(v)
//     }
// }
