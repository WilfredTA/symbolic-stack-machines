use super::{Irreducible, Simplifiable};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GroundValue {
    Concrete(ConcreteInnerValue),
    Symbolic(SymbolicInnerValue),
    Boolean(bool),
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
impl From<ConcreteInnerValue> for GroundValue {
    fn from(c: ConcreteInnerValue) -> Self {
        GroundValue::Concrete(c)
    }
}

impl From<SymbolicInnerValue> for GroundValue {
    fn from(c: SymbolicInnerValue) -> Self {
        GroundValue::Symbolic(c)
    }
}
impl From<&ConcreteInnerValue> for GroundValue {
    fn from(c: &ConcreteInnerValue) -> Self {
        GroundValue::Concrete(c.clone())
    }
}

impl From<&SymbolicInnerValue> for GroundValue {
    fn from(c: &SymbolicInnerValue) -> Self {
        GroundValue::Symbolic(c.clone())
    }
}
impl Irreducible for GroundValue {}
