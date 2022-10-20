pub mod arith;
pub mod bitwise;
pub mod misc;

use symbolic_stack_machines_core::value::{Sentence, Value, CSimpleVal, CNumber};


pub const ZERO: Sentence = Sentence::Basic(Value::Concrete(CSimpleVal::Number(CNumber::U8(0))));
pub const ONE: Sentence = Sentence::Basic(Value::Concrete(CSimpleVal::Number(CNumber::U8(1))));