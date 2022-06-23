use super::*;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Symbolic(CSimpleVal),
    Concrete(SSimpleVal),
}

// Concrete Simple Val
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
