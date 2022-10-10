use crate::value::visitors::base_interpreter::{Interpreter, Hook, InterpreterMode};

#[derive(Default, Clone)]
pub struct MemoryConfig<V: Default> {
    pub word_size: usize,
    pub endianness: Endianness,
    pub byte_addressable: bool,
    pub alignment_restriction: bool,
    pub stack_val_to_ptr: Option<Interpreter>,
    pub memval_basic_type: V
}



#[derive(Clone)]
pub enum Endianness {
    Little,
    Big
}

impl Default for Endianness {
    fn default() -> Self {
        Self::Little
    }
}