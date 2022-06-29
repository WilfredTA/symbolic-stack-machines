use super::*;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Symbolic(SSimpleVal),
    Concrete(CSimpleVal),
}

// Concrete Simple Val
// Operations on this builds AST
// whereas operations on its inner types simple
// dispatches to its inner type's concrete type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CSimpleVal {
    Boolean(Bool),
    Number(CNumber),
    Vector(Vecc),
}

#[derive(Clone, Debug, PartialEq, Eq)]
// Symbolic Simple Val
pub enum SSimpleVal {
    SymbolicNumber(SNumber),
    SymbolicVector(SymbolicVecc),
}
