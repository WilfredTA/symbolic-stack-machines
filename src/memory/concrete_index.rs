use crate::vals::{ConcreteInt, SymbolicInt};

use super::{Mem, MemoryResult, RWMem, ReadOnlyMem, WriteableMem};
use std::{fmt::Debug, marker::PhantomData};

pub trait MemVal: Default + Clone + Debug {}
pub trait IndexVal: Debug + Clone + TryInto<usize> {}

#[derive(Clone, Debug)]
pub struct MemConcreteIndex<IV: IndexVal, MV: MemVal> {
    inner: Vec<MV>,
    indices: PhantomData<IV>,
}

impl<IV: IndexVal, MV: MemVal> Mem for MemConcreteIndex<IV, MV> {
    type Index = IV;
    type MemVal = MV;
}

impl<IV: IndexVal, MV: MemVal> ReadOnlyMem for MemConcreteIndex<IV, MV>
where
    <IV as TryInto<usize>>::Error: Debug,
{
    fn read(&self, idx: Self::Index) -> MemoryResult<Option<Self::MemVal>> {
        Ok(Some(
            self.inner
                .get(idx.try_into().unwrap())
                .map(|x| (*x).clone())
                .unwrap_or_default(),
        ))
    }
}

impl<IV: IndexVal, MV: MemVal> WriteableMem for MemConcreteIndex<IV, MV>
where
    <IV as TryInto<usize>>::Error: Debug,
{
    fn write(&self, idx: Self::Index, val: Self::MemVal) -> MemoryResult<Self> {
        let mut x = Self {
            inner: self.inner.clone(),
            indices: PhantomData,
        };

        let min_len = idx.clone().try_into().unwrap() + 1;

        if x.inner.len() < min_len {
            x.inner.resize(min_len, Self::MemVal::default());
        }

        x.inner[idx.try_into().unwrap()] = val;

        Ok(x)
    }
}

impl<IV: IndexVal, MV: MemVal> MemConcreteIndex<IV, MV> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            inner: vec![],
            indices: PhantomData,
        }
    }
}

impl<IV: IndexVal, MV: MemVal> RWMem for MemConcreteIndex<IV, MV> where
    <IV as TryInto<usize>>::Error: Debug
{
}

pub type MemConcreteIntToConcreteInt = MemConcreteIndex<ConcreteInt, ConcreteInt>;
pub type MemConcreteIntToSymbolicInt = MemConcreteIndex<ConcreteInt, SymbolicInt>;
