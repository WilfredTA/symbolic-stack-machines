use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Shl, Shr, Sub};
pub trait Simplifiable {
    type GroundVal;
    fn simplify(&self) -> Self::GroundVal;
}

pub trait Arith: Add + Sub + Mul + Div + Sized {}

pub trait Signed: Arith + Neg {}

pub trait Binary: Shl + Shr + BitOr + BitAnd + BitXor + Not + Sized {}

pub trait Comparable: PartialEq + Eq + PartialOrd + Ord + Sized {}
pub trait Abstracted: Simplifiable + Arith + Binary + Comparable {}
