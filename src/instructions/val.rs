use std::borrow::Borrow;
use std::ops::{Add, Sub};
use z3::ast::Int;
#[derive(Debug, Clone)]
pub struct Val<T>(pub T);

impl<T> From<T> for Val<T> {
    fn from(v: T) -> Self {
        Self(v)
    }
}

impl<T> AsRef<T> for Val<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Borrow<T> for Val<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> Add for Val<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T> Sub for Val<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

pub enum HybridInner<'a> {
    C(u64),
    S(Box<Int<'a>>),
}
impl Add for HybridInner<'_> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            HybridInner::C(c) => {
                if let Self::C(r) = rhs {
                    return Self::C(c + r);
                } else {
                    panic!("Cannot add concrete and symbolic value together");
                }
            }
            HybridInner::S(s) => {
                if let Self::S(r) = rhs {
                    return Self::S(Box::new(s.add(r.as_ref())));
                } else {
                    panic!("Cannot add concrete and symbolic value together.");
                }
            }
        }
    }
}
impl Sub for HybridInner<'_> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            HybridInner::C(c) => {
                if let Self::C(r) = rhs {
                    return Self::C(c - r);
                } else {
                    panic!("Cannot add concrete and symbolic value together");
                }
            }
            HybridInner::S(s) => {
                if let Self::S(r) = rhs {
                    return Self::S(Box::new(s.sub(r.as_ref())));
                } else {
                    panic!("Cannot add concrete and symbolic value together.");
                }
            }
        }
    }
}

pub type HybridVal<'a> = Val<HybridInner<'a>>;
