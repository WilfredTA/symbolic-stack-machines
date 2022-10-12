use symbolic_stack_machines_core::value::{Sentence, Value, CSimpleVal, CNumber};

pub mod arith;
pub mod bitwise;
pub mod misc;
pub mod risc;

pub const ZERO: Sentence = Sentence::Basic(Value::Concrete(CSimpleVal::Number(CNumber::U8(0))));
pub const ONE: Sentence = Sentence::Basic(Value::Concrete(CSimpleVal::Number(CNumber::U8(1))));