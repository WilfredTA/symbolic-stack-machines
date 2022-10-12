use crate::stack::StackVal;
use crate::value::visitors::base_interpreter::Hook;
use crate::value::Sentence;

use super::{
    config::MemoryConfig,
    record::{MemOpRecord, MemRecord},
    val::MemVal,
};

const PRE_HOOK: &'static dyn Fn(Sentence) -> Option<Sentence> = &|_s: Sentence| -> Option<Sentence> { None };
#[derive(Clone, Default)]
pub struct Memory<V: Default + Clone> {
    inner: Vec<MemVal>,
    // TODO should be a reference
    config: MemoryConfig<V>,
}

// TODO: Make this memory use MemVal as Sentence and abstracted lookup

impl<V: Default + Clone> Memory<V> {
    pub fn new(init: Vec<MemVal>, config: MemoryConfig<V>) -> Self {
        Self {
            inner: init,
            config,
        }
    }

    pub fn read_word<F>(&self, idx: StackVal, final_hook: F, post_hook: Hook) -> Option<StackVal> 
    where 
        F: Fn(Sentence) -> V,
    {
        let idx_unwrapped = self.config.stack_val_to_ptr
            .as_ref()
            .unwrap()
            .interpret(Box::new(PRE_HOOK), post_hook, final_hook);
        let idx_unwrapped = 1;
        // TODO(will): Check endianness/byte ordering
        let mut bytes: [u8; 8] = [0; 8];

        for i in 0..=7 {
            let byte: u8 = (*self.read_byte_inner(idx_unwrapped + i)?).into();
            bytes[i as usize] = byte
        }

        Some(u64::from_be_bytes(bytes).into())
    }

    pub fn read_byte(&self, idx: StackVal) -> Option<&MemVal> {
        self.read_byte_inner(Into::<usize>::into(idx))
    }

    fn read_byte_inner(&self, idx: usize) -> Option<&MemVal> {
        self.inner.get(idx)
    }

    pub fn apply(&self, r: MemRecord) -> Self {
        // TODO(will) - we should use a copy on write data structure
        let mut inner = self.inner.clone();

        for c in r.changed {
            match c {
                MemOpRecord::Write(idx, val) => {
                    // TODO(will): Check endianness/byte ordering
                    let idx_usize = Into::<usize>::into(idx);
                    let val_unwrapped = Into::<usize>::into(val).to_be_bytes();

                    for i in 0..=7 {
                        inner[idx_usize + i] = val_unwrapped[i].into();
                    }
                }
            }
        }

        Self {
            inner,
            config: self.config.clone(),
        }
    }
}
