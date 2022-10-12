// TODO(will) - ultimately this will be replaced with a more general value
// that implements an AST, etc...
use crate::value::{Sentence, CNumber, CSimpleVal, SNumber, SSimpleVal, Value, Number, Val, BinOp, TernaryOp};
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct StackVal(pub Sentence);

// pub static ZERO: StackVal = StackVal(CNumber::U64(0));
// pub static ONE: StackVal = StackVal(CNumber::U64(1));

// static FALSE: StackVal = ZERO;
// static TRUE: StackVal = ONE;

impl From<u64> for StackVal {
    fn from(x: u64) -> Self {
        Self(Sentence::Basic(Value::Concrete(CSimpleVal::Number(CNumber::U64(x)))))
    }
}

impl From<Sentence> for StackVal {
    fn from(s: Sentence) -> Self {
        Self(s)
    }
}

impl From<usize> for StackVal {
    fn from(x: usize) -> Self {
        Self::from(x as u64)
    }
}

impl From<StackVal> for usize {
    fn from(x: StackVal) -> Self {
        todo!()
    }
}

// impl Into<u64> for StackVal {
//     fn into(self) -> u64 {
//         self.0
//     }
// }

// impl Into<usize> for StackVal {
//     fn into(self) -> usize {
//         self.0 as usize
//     }
// }

impl StackVal {
    pub fn _eq(&self, other: &Self) -> Self {
        StackVal(Sentence::BinOp { a: Val::new(self.0.clone()), b: Val::new(other.0.clone()), op: BinOp::Eq })
    }

    pub fn ite(&self, then: Self, xelse: Self) -> Self {
        let a = self.0.clone();
        let b = then.0.clone();
        StackVal(Sentence::TernaryOp { 
            a: Val::new(a), 
            b: Val::new(b), 
            c: Val::new(xelse.0), 
            op: TernaryOp::Ite }
        )
       
    }
}

impl std::ops::Add for StackVal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        StackVal(Sentence::BinOp {
            a: Val::new(self.0),
            b: Val::new(rhs.0),
            op: BinOp::Plus,
        })
    }
}

impl std::ops::Sub for StackVal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        StackVal(Sentence::BinOp {
            a: Val::new(self.0),
            b: Val::new(rhs.0),
            op: BinOp::Minus,
        })
    }
}
