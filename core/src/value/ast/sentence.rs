use super::*;


// ------------- COMPOUND VALUES --------------
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Sentence {
    BinOp {
        a: Val<Sentence>,
        b: Val<Sentence>,
        op: BinOp
    },
    UnaryOp {
        a: Val<Sentence>,
        op: UnaryOp
    },
    TernaryOp {
        a: Val<Sentence>,
        b: Val<Sentence>,
        c: Val<Sentence>,
        op: TernaryOp,
    },
    Basic(Value),
    
}



#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TernaryOp {
    Ite,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinOp {
    Plus,
    Minus,
    Div,
    Mul,
    Mod,
    // Comparison
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    // Binary Ops
    BitOr,
    BitAnd,
    BitXor,
    LShift,
    RShift,

}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
    BitNot
}
