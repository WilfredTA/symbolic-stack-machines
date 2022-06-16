use super::{Mem, RWMem, ReadOnlyMem, WriteableMem};

pub type BaseMemoryConcreteUint64 = BaseMemoryConcreteIndex<u64>;

#[derive(Debug, Clone)]
pub struct BaseMemoryConcreteIndex<T>(Vec<T>);

impl<T> BaseMemoryConcreteIndex<T> {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl<MemVal: Clone> Mem for BaseMemoryConcreteIndex<MemVal> {
    type MemVal = MemVal;
    type Index = usize;
}

impl<T: Clone> ReadOnlyMem for BaseMemoryConcreteIndex<T> {
    fn read(&self, idx: Self::Index) -> super::MemoryResult<Option<Self::MemVal>> {
        Ok(self.0.get(idx).map(|v| v.clone()))
    }
}

impl<T: Clone> WriteableMem for BaseMemoryConcreteIndex<T> {
    fn write(&self, idx: Self::Index, val: Self::MemVal) -> super::MemoryResult<Self> {
        let mut inner = self.0.clone();
        inner[idx as usize] = val;
        Ok(Self(inner))
    }
}

impl<T: Clone> RWMem for BaseMemoryConcreteIndex<T> {}
