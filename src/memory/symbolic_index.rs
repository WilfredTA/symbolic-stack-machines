use super::{MemoryResult, ReadOnlyMem, WriteableMem, RWMem, Mem};

pub trait IndexVal: Clone + std::fmt::Debug {}

type Writes<IV> = Vec<(IV, MemVal<IV>)>;

#[derive(Clone, Debug)]
pub struct MemVal<IV: IndexVal> {
    pub writes: Writes<IV>,
    pub read: IV,
}

#[derive(Debug)]
pub struct MemSymbolicIndex<IV: IndexVal> {
    writes: Writes<IV>,
}

impl<IV: IndexVal> Mem for MemSymbolicIndex<IV> {
    type MemVal = MemVal<IV>;
    type Index = IV;
}

impl<IV: IndexVal> ReadOnlyMem for MemSymbolicIndex<IV> {
    fn read(&self, idx: Self::Index) -> MemoryResult<Option<Self::MemVal>> {
        Ok(Some(MemVal {
            writes: self.writes.clone(),
            read: idx,
        }))
    }
}

impl<IV: IndexVal> WriteableMem for MemSymbolicIndex<IV> {
    fn write(&self, idx: Self::Index, val: Self::MemVal) -> MemoryResult<Self> {
        let mut writes = self.writes.clone();
        writes.push((idx, val));
        Ok(Self { writes })
    }
}

impl<IV: IndexVal> MemSymbolicIndex<IV> {
    pub fn new() -> Self {
        Self { writes: vec![] }
    }
}

impl<IV: IndexVal> RWMem for MemSymbolicIndex<IV> {}
