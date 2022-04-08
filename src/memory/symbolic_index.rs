use crate::vals::SymbolicInt;

use super::{Mem, MemoryResult, RWMem, ReadOnlyMem, WriteableMem};

pub trait IndexVal: Clone + std::fmt::Debug {}
pub trait MemVal<IV>: Clone + std::fmt::Debug {
    fn read(writes: Vec<(IV, Self)>, read: IV) -> Self;
}

#[derive(Clone, Debug)]
pub struct MemSymbolicIndex<IV, MV>
where
    IV: IndexVal,
    MV: MemVal<IV>,
{
    writes: Vec<(IV, MV)>,
}

impl<IV, MV> Mem for MemSymbolicIndex<IV, MV>
where
    IV: IndexVal,
    MV: MemVal<IV>,
{
    type MemVal = MV;
    type Index = IV;
}

impl<IV, MV> ReadOnlyMem for MemSymbolicIndex<IV, MV>
where
    IV: IndexVal,
    MV: MemVal<IV>,
{
    fn read(&self, idx: Self::Index) -> MemoryResult<Option<Self::MemVal>> {
        Ok(Some(MV::read(self.writes.clone(), idx)))
    }
}

impl<IV, MV> WriteableMem for MemSymbolicIndex<IV, MV>
where
    IV: IndexVal,
    MV: MemVal<IV>,
{
    fn write(&self, idx: Self::Index, val: Self::MemVal) -> MemoryResult<Self> {
        let mut writes = self.writes.clone();
        writes.push((idx, val));
        Ok(Self { writes })
    }
}

impl<IV, MV> MemSymbolicIndex<IV, MV>
where
    IV: IndexVal,
    MV: MemVal<IV>,
{
    pub fn new() -> Self {
        Self { writes: vec![] }
    }
}

impl<IV, MV> RWMem for MemSymbolicIndex<IV, MV>
where
    IV: IndexVal,
    MV: MemVal<IV>,
{
}

pub type MemIntToInt = MemSymbolicIndex<SymbolicInt, SymbolicInt>;
