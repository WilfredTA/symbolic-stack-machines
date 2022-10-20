use crate::value::Sentence;
use crate::value::visitors::base_interpreter::{Interpreter, AdHocInterpreter, Hook};
use crate::stack::{StackVal};
use rhai::{Engine, EvalAltResult};


#[derive(Default, Clone)]
pub struct MemoryConfig {
    pub word_size: u16,
}

impl MemoryConfig {

    pub fn stack_val_to_mem_addr<MemAddr, H>(&self, s: StackVal, type_converter: String) -> MemAddr
    where
        H: Fn(Sentence) -> MemAddr
    {
        // let post = self.stack_mem_addr;
        // let val = AdHocInterpreter::interpret(post, type_converter, s.0.clone());
        todo!()
    }
}
