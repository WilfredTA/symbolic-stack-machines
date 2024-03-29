use super::*;

// ------------- COMPOUND VALUES --------------
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Sentence {
    BinOp {
        a: Val<Sentence>,
        b: Val<Sentence>,
        op: BinOp,
    },
    UnaryOp {
        a: Val<Sentence>,
        op: UnaryOp,
    },
    TernaryOp {
        a: Val<Sentence>,
        b: Val<Sentence>,
        c: Val<Sentence>,
        op: TernaryOp,
    },
    Basic(Value),
}


impl Sentence {
    pub fn is_bin_op(&self) -> bool {
        if let Self::BinOp { a, b, op } = self {
            true
        } else {
            false
        }
    }
  
    pub fn is_unary_op(&self) -> bool {
        if let Self::UnaryOp { a, op } = self {
            true
        } else {
            false
        }
    }

    pub fn is_ternary_op(&self) -> bool {
        if let Self::TernaryOp { a, b, c, op } = self {
            true
        } else {
            false
        }
    }
}
impl Default for Sentence {
    fn default() -> Self {
        Self::Basic(Default::default())
    }
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
    BitNot,
}
