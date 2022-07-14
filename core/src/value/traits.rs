use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Shl, Shr, Sub};
// Generic over the output type
// For each traversal, there is an associated inner type
pub trait Simplifiable<T> {
    type GroundVal: Irreducible;
    fn simplify(&self) -> Self::GroundVal;
}

pub trait Irreducible {}
pub trait Arith: Add + Sub + Mul + Div + Sized {}

pub trait Signed: Arith + Neg {}

pub trait Binary: Shl + Shr + BitOr + BitAnd + BitXor + Not + Sized {}

pub trait Comparable: PartialEq + Eq + PartialOrd + Ord + Sized {}
pub trait Abstracted: Arith + Binary + Comparable {}
