use super::{Mem, MemoryResult, RWMem, ReadOnlyMem, WriteableMem};

pub trait MemVal: Default + Clone {}

#[derive(Clone, Debug)]
pub struct SymbolicMemConcreteIndex<MV: MemVal> {
    inner: Vec<MV>,
}

impl<MV: MemVal> Mem for SymbolicMemConcreteIndex<MV> {
    type MemVal = MV;
    type Index = usize;
}

impl<MV: MemVal> ReadOnlyMem for SymbolicMemConcreteIndex<MV> {
    fn read(&self, idx: Self::Index) -> MemoryResult<Option<Self::MemVal>> {
        Ok(Some(
            self.inner
                .get(idx)
                .map(|x| (*x).clone())
                .unwrap_or_default(),
        ))
    }
}

impl<MV: MemVal> WriteableMem for SymbolicMemConcreteIndex<MV> {
    fn write(&self, idx: Self::Index, val: Self::MemVal) -> MemoryResult<Self> {
        let mut x = Self {
            inner: self.inner.clone(),
        };
        x.inner[idx] = val;
        Ok(x)
    }
}

impl<MV: MemVal> SymbolicMemConcreteIndex<MV> {
    pub fn new() -> Self {
        Self { inner: vec![] }
    }
}

impl<MV: MemVal> RWMem for SymbolicMemConcreteIndex<MV> {}

type Int = i128;
impl MemVal for Int {}
pub type MemIntToInt = SymbolicMemConcreteIndex<Int>;
