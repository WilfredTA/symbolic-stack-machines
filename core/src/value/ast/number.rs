use super::SymbolId;
use enum_as_inner::EnumAsInner;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NumberError {
    #[error("Cannot convert {0:?} to {1:?}")]
    Convert(CNumber, CNumber),
}
#[derive(Clone, Debug, PartialEq, Eq, EnumAsInner, PartialOrd, Ord)]
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

impl TryFrom<CNumber> for u8 {
    type Error = NumberError;

    fn try_from(value: CNumber) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<CNumber> for u16 {
    type Error = NumberError;

    fn try_from(value: CNumber) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<CNumber> for u32 {
    type Error = NumberError;

    fn try_from(value: CNumber) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<CNumber> for u64 {
    type Error = NumberError;

    fn try_from(value: CNumber) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<CNumber> for u128 {
    fn from(_: CNumber) -> Self {
        todo!()
    }
}

impl std::ops::Add for CNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Sub for CNumber {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Mul for CNumber {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Div for CNumber {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
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
