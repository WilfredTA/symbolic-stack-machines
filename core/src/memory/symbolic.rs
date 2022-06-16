use crate::memory::{MemoryResult, ReadOnlyMem, WriteableMem};
use z3::ast::{Array, Int};

use super::Mem;
#[derive(Clone, Debug)]
pub struct BaseSymbolicMem<'a> {
    inner: Array<'a>,
}

impl<'a> Mem for BaseSymbolicMem<'a> {
    type MemVal = Int<'a>;
    type Index = Int<'a>;
}

impl<'a> ReadOnlyMem for BaseSymbolicMem<'a> {
    fn read(&self, idx: Self::Index) -> MemoryResult<Option<Self::MemVal>> {
        Ok(self.inner.select(&idx).as_int())
    }
}

impl<'a> WriteableMem for BaseSymbolicMem<'a> {
    fn write(&self, idx: Self::Index, val: Self::MemVal) -> MemoryResult<Self> {
        Ok(Self {
            inner: self.inner.store(&idx, &val),
        })
    }
}
