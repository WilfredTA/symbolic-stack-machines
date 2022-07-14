use super::*;
use enum_as_inner::EnumAsInner;
#[derive(Clone, Debug, PartialEq, Eq, EnumAsInner)]
pub enum Value {
    Symbolic(SSimpleVal),
    Concrete(CSimpleVal),
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

#[derive(Clone, Debug, PartialEq, Eq, EnumAsInner)]
// Symbolic Simple Val
pub enum SSimpleVal {
    SymbolicBool(SymbolId<Bool>),
    SymbolicNumber(SNumber),
    SymbolicVector(SymbolicVecc),
}
