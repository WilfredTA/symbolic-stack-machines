use std::rc::Rc;
use rand::Rng;
use z3::ast::BV;
#[derive(Clone)]
pub struct AbstractValue<T> {
    symbol: String,
    val: T,
}

// To do: Transpile like so:
// InnerValue (and all sub types) implement Into<smt_val> (e.g., Booleans implement Into<z3::ast::Bool>)

impl<T: Clone> AbstractValue<T> {
    pub fn new(val: T, symbol: String) -> Self {
        Self { symbol, val }
    }
    pub fn inner<V: From<T>>(&self) -> V {
        self.val.clone().into()
    }

    pub fn id(&self) -> &str {
        self.symbol.as_str()
    }

    pub fn set_val(&mut self, new_val: T) {
        self.val = new_val;
    }

    pub fn set_symbol(&mut self, new_symbol: String) {
        self.symbol = new_symbol;
    }
}

// TODO(tannr): Impl arithmetic ops, comparison ops, equality ops for inner value
// Do it in such a way that if values are symbolic, then a `Constraint` tree is produced that can be passed to Transpile::transpile()



#[derive(Clone)]
pub struct Value<T>(pub Rc<T>);

#[derive(Clone)]
pub enum InnerValue {
    ConcreteLiteral(Value<ConcreteInnerValue>),
    SymbolicLiteral(Value<SymbolicInnerValue>),
    Boolean(Boolean),
    Arithmetic(Arithmetic),
    // Binary
}



#[derive(Clone)]
pub enum Arithmetic {
    Add(Value<InnerValue>, Value<InnerValue>),
    Sub(Value<InnerValue>, Value<InnerValue>),
    Mul(Value<InnerValue>, Value<InnerValue>),
    Div(Value<InnerValue>, Value<InnerValue>),
}

#[derive(Clone)]
pub enum Boolean {
    ValCmp(ValCmp),
    BoolFormula(BoolFormula)
}

#[derive(Clone)]
pub enum BoolFormula {
    True,
    False,
    Or(Value<Boolean>, Value<Boolean>),
    And(Value<Boolean>, Value<Boolean>),
    Not(Value<Boolean>, Value<Boolean>),
    Ite(Value<Boolean>, Value<InnerValue>, Value<InnerValue>),
    Assert(Value<InnerValue>),
}

#[derive(Clone)]
pub enum ValCmp {
    Gt(Value<InnerValue>, Value<InnerValue>),
    Gte(Value<InnerValue>, Value<InnerValue>),
    Lt(Value<InnerValue>, Value<InnerValue>),
    Lte(Value<InnerValue>, Value<InnerValue>),
    Eq(Value<InnerValue>, Value<InnerValue>),
    Neq(Value<InnerValue>, Value<InnerValue>),
}

#[derive(Clone, Debug)]
pub enum ConcreteInnerValue {
    ConcreteU8(u8),
    ConcreteU16(u16),
    ConcreteU32(u32),
    ConcreteU64(u64),
    ConcreteU128(u128),
    ConcreteBytes(Vec<u8>),
}

#[derive(Clone, Debug)]
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
        Self::Arithmetic(Arithmetic::Add(
                Value(Rc::new(self.clone())), Value(Rc::new(rhs.clone()))
            )
        )
    }
}

impl std::ops::Sub for InnerValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Arithmetic(Arithmetic::Sub(
                Value(Rc::new(self.clone())), Value(Rc::new(rhs.clone()))
            )
        )
    }
}

impl std::ops::Mul for InnerValue {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Arithmetic(Arithmetic::Mul(
            Value(Rc::new(self.clone())), Value(Rc::new(rhs.clone()))
        )
    )
    }
}

impl std::ops::Div for InnerValue {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Arithmetic(Arithmetic::Div(
            Value(Rc::new(self.clone())), Value(Rc::new(rhs.clone()))
        )
    )
    }
}

impl InnerValue {
    pub fn _eq(&self, other: InnerValue) -> Self {
        InnerValue::Boolean(
            Boolean::ValCmp(
                ValCmp::Eq(
                    Value(Rc::new(self.clone())), Value(Rc::new(other.clone()))
                )
            )
        )
    }

    pub fn _neq(&self, other: InnerValue) -> Self {
        InnerValue::Boolean(
            Boolean::ValCmp(
                ValCmp::Neq(
                    Value(Rc::new(self.clone())), Value(Rc::new(other.clone()))
                )
            )
        )
    }

    pub fn _gt(&self, other: InnerValue) -> Self {
        InnerValue::Boolean(
            Boolean::ValCmp(
                ValCmp::Gt(
                    Value(Rc::new(self.clone())), Value(Rc::new(other.clone()))
                )
            )
        )
    }

    pub fn _gte(&self, other: InnerValue) -> Self {
        InnerValue::Boolean(
            Boolean::ValCmp(
                ValCmp::Gte(
                    Value(Rc::new(self.clone())), Value(Rc::new(other.clone()))
                )
            )
        )
    }

    pub fn _lt(&self, other: InnerValue) -> Self {
        InnerValue::Boolean(
            Boolean::ValCmp(
                ValCmp::Lt(
                    Value(Rc::new(self.clone())), Value(Rc::new(other.clone()))
                )
            )
        )
    }

    pub fn _lte(&self, other: InnerValue) -> Self {
        InnerValue::Boolean(
            Boolean::ValCmp(
                ValCmp::Lte(
                    Value(Rc::new(self.clone())), Value(Rc::new(other.clone()))
                )
            )
        )
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



// Convenient Exports

// #[derive(Clone, Default)]
// pub struct AbstractInt {
//     concrete: Option<u64>,
// }

// impl AbstractInt {
//     pub fn inner(&self) -> Option<u64> {
//         self.concrete.clone()
//     }
// }

// impl From<u64> for AbstractInt {
//     fn from(v: u64) -> Self {
//         Self { concrete: Some(v) }
//     }
// }

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


impl<T: Clone> From<T> for AbstractValue<T> {
    fn from(v: T) -> Self {
        AbstractValue::new(v, random_val_symbol())
    }
}