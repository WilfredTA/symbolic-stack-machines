use super::*;
use enum_as_inner::EnumAsInner;

// pub trait Valued {
//     type SymbolicType;
//     type ConcreteType;

//     pub fn as_symbolic(&self) -> Self::SymbolicType;

//     pub fn as_concrete(&self) -> Self::ConcreteType;
// }

// pub struct BoolVal {
//     inner: bool
// }

// pub struct NumVal {
//     inner: Vec<u8>,
//     size: u16
// }

// pub struct VecVal {
//     inner: Vec<&dyn Valued>,

// }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnyVal {
    inner_sym: SSimpleVal,
    inner_con: CSimpleVal
}

impl Default for AnyVal {
    fn default() -> Self {
        Self { inner_sym: Default::default(), inner_con: Default::default() }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, EnumAsInner)]
pub enum Value {
    Symbolic(SSimpleVal),
    Concrete(CSimpleVal),
    Either(AnyVal)
}

impl Default for Value {
    fn default() -> Self {
        Value::Either(Default::default())
    }
}

// Concrete Simple Val
// Operations on this builds AST
// whereas operations on its inner types simple
// dispatches to its inner type's concrete type
#[derive(Clone, Debug, PartialEq, Eq, EnumAsInner)]
pub enum CSimpleVal {
    Boolean(Bool),
    Number(CNumber),
    Vector(Vecc),
}

impl Default for CSimpleVal {
    fn default() -> Self {
        Self::Boolean(Bool::True)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, EnumAsInner)]
// Symbolic Simple Val
pub enum SSimpleVal {
    SymbolicBool(SymbolId<Bool>),
    SymbolicNumber(SNumber),
    SymbolicVector(SymbolicVecc),
}

impl Default for SSimpleVal {
    fn default() -> Self {
        Self::SymbolicBool(SymbolId::new(Some(Bool::True)))
    }
}