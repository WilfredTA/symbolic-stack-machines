use std::ops::{Add, Sub, Mul, Div, Shl, Shr, BitOr, BitAnd, BitXor, Neg, Not};
use std::cmp::{PartialEq, Eq, Ord, PartialOrd};
pub trait Simplifiable {
    type GroundVal;
    fn simplify(&self) -> Self::GroundVal;
}



pub trait Arithmetic: Add + Sub + Mul + Div + Sized {}

pub trait Signed: Arithmetic + Neg {}

pub trait Binary: Shl + Shr + BitOr + BitAnd + BitXor + Not + Sized {}

pub trait Comparable: PartialEq + Eq + PartialOrd + Ord + Sized {}
pub trait Abstracted: Simplifiable + Arithmetic + Binary + Comparable {

}