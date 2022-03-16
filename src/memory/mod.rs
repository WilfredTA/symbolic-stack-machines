pub mod error;
pub mod symbolic;

use std::rc::Rc;

use crate::instructions::val::{HybridVal, Val};
use error::MemoryError;
use z3::{
    ast::{Array, Int},
    Context,
};
pub type MemoryResult<T> = Result<T, MemoryError>;
pub trait ReadOnlyMem: Sized {
    type MemVal;
    type Index;
    fn read(&self, idx: Self::Index) -> MemoryResult<Option<Self::MemVal>>;
}

pub trait WriteableMem: Sized {
    type MemVal;
    type Index;

    fn write(&self, idx: Self::Index, val: Self::MemVal) -> MemoryResult<Self>;
}

pub trait RWMem: ReadOnlyMem + WriteableMem {
    type InitArgs: Clone;
    fn init(args: Self::InitArgs) -> Self;
}

pub type MemorySlotChange<Idx, MemVal> = (Idx, MemVal, MemVal);
pub enum MemOpRecord<I, V> {
    Write(MemorySlotChange<I, V>),
}
pub struct MemRecord<M: WriteableMem> {
    diff: Vec<MemOpRecord<M::Index, M::MemVal>>,
}

impl<M> MemRecord<M>
where
    M: WriteableMem,
{
    pub fn apply(self, memory: M) -> MemoryResult<M> {
        let final_mem = self.diff.into_iter().fold(
            Ok(memory),
            |cur_mem: MemoryResult<M>, r| -> MemoryResult<M> {
                match cur_mem {
                    Ok(m) => {
                        if let MemOpRecord::Write(r) = r {
                            let idx = r.0;
                            let _old_val = r.1;
                            let new_val = r.2;
                            m.write(idx, new_val)
                        } else {
                            Ok(m)
                        }
                    }
                    Err(e) => Err(e),
                }
            },
        );
        final_mem
    }
}

pub struct BaseMemory<T> {
    inner: Vec<Val<T>>,
}
