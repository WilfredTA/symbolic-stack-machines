use super::{MemoryResult, ReadOnlyMem, WriteableMem, RWMem, Mem};

pub trait IndexVal: Clone {}

type Writes<IV> = Vec<(IV, MemVal<IV>)>;

#[derive(Clone)]
pub struct MemVal<IV: IndexVal> {
    pub writes: Writes<IV>,
    pub read: IV,
}

pub struct SymbolicMemSymbolicIndex<IV: IndexVal> {
    writes: Writes<IV>,
}

impl<IV: IndexVal> Mem for SymbolicMemSymbolicIndex<IV> {
    type MemVal = MemVal<IV>;
    type Index = IV;
}

impl<IV: IndexVal> ReadOnlyMem for SymbolicMemSymbolicIndex<IV> {
    fn read(&self, idx: Self::Index) -> MemoryResult<Option<Self::MemVal>> {
        Ok(Some(MemVal {
            writes: self.writes.clone(),
            read: idx,
        }))
    }
}

impl<IV: IndexVal> WriteableMem for SymbolicMemSymbolicIndex<IV> {
    fn write(&self, idx: Self::Index, val: Self::MemVal) -> MemoryResult<Self> {
        let mut writes = self.writes.clone();
        writes.push((idx, val));
        Ok(Self { writes })
    }
}

impl<IV: IndexVal> SymbolicMemSymbolicIndex<IV> {
    pub fn new() -> Self {
        Self { writes: vec![] }
    }
}

impl<IV: IndexVal> RWMem for SymbolicMemSymbolicIndex<IV> {}
