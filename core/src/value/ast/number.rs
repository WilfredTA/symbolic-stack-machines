use super::SymbolId;
use enum_as_inner::EnumAsInner;

#[derive(Clone, Debug, PartialEq, Eq, EnumAsInner)]
pub enum CNumber {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

impl From<u8> for CNumber {
    fn from(v: u8) -> Self {
        CNumber::U8(v)
    }
}

impl From<u16> for CNumber {
    fn from(v: u16) -> Self {
        CNumber::U16(v)
    }
}

impl From<u32> for CNumber {
    fn from(v: u32) -> Self {
        CNumber::U32(v)
    }
}

impl From<u64> for CNumber {
    fn from(v: u64) -> Self {
        CNumber::U64(v)
    }
}

impl From<u128> for CNumber {
    fn from(v: u128) -> Self {
        CNumber::U128(v)
    }
}

impl CNumber {
    // Inner add to add the internal values
    // As opposed to std::ops::add which creates an Arith AST
    pub fn inner_add(&self, other: Self) -> Self {
        todo!()
    } 

    pub fn inner_sub(&self, other: Self) -> Self {
        todo!()
    }

    pub fn inner_div(&self, other: Self) -> Self {
        todo!()
    }

    pub fn inner_mul(&self, other: Self) -> Self {
        todo!()
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SNumber(SymbolId<CNumber>);
#[derive(Clone, Debug, PartialEq, Eq, EnumAsInner)]
pub enum Number {
    Sym(SNumber),
    Con(CNumber),
}

