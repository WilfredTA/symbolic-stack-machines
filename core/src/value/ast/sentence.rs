use super::*;


// ------------- COMPOUND VALUES --------------
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Sentence {
    Arith(Arith),
    Bool(BoolF),
    Val(Value)
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Arith {
    Plus(Val<Sentence>, Val<Sentence>),
    Minus(Val<Sentence>, Val<Sentence>),
    Div(Val<Sentence>, Val<Sentence>),
    Mul(Val<Sentence>, Val<Sentence>),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoolF {
    Eq(Val<Sentence>, Val<Sentence>),
    Neq(Val<Sentence>, Val<Sentence>),
    Lt(Val<Sentence>, Val<Sentence>),
    Gt(Val<Sentence>, Val<Sentence>),
    Lte(Val<Sentence>, Val<Sentence>),
    Gte(Val<Sentence>, Val<Sentence>)
}

// TODO: Bit vec operations