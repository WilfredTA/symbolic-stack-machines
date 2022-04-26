use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use hex::{decode, encode};
use std::borrow::Borrow;
use std::ops::{Add, Sub};
use z3::ast::{Ast, Int, BV};
use z3::Context;
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

#[derive(Clone, Debug)]
pub struct SymbolicBytes<'a> {
    inner: BV<'a>,
    size: usize,
}

impl<'a> SymbolicBytes<'a> {
    pub fn new(size: usize, name: impl Into<String>, ctx: &'a Context) -> Self {
        let bv = BV::new_const(ctx, name.into(), size as u32);
        Self { inner: bv, size }
    }

    pub fn add_u64(&mut self, val: u64) {
        let val = BV::from_u64(self.inner.get_ctx(), val, self.size as u32);
        self.inner = self.inner.bvadd(&val);
    }

    pub fn extend(&self, other: BV<'a>) -> Self {
        Self {
            inner: self.inner.concat(&other),
            size: self.size
        }
    }
}
