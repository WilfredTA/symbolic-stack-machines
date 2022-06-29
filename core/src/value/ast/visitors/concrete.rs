use crate::value::ast::*;


pub enum InterpreterMode {
    Symbolic,
    Concrete
}
pub struct Interpreter {
    pgm: Sentence,
    mode: InterpreterMode,
}

// Post hook is called once after all processing is finished
// Pre hook is called at each node in the AST prior to potentially increased traversal.
// If pre_hook returns None, then processing continues as normal. If it returns Some(Sentence), then the sentence returned
// is used instead of the sentence that would result from further processing.
impl Interpreter {
    pub fn interpret<T, Pre, Post>(&self, pre_hook: Pre, post_hook: Post) -> T
    where 
        Pre: Fn(Sentence) -> Option<Sentence>,
        Post: Fn(Sentence) -> T,
    {
       let sentence = self.process_sentence(&self.pgm);
       post_hook(sentence)
    }

    pub fn visit(&self) -> Sentence {
        self.process_sentence(&self.pgm)
    }
    
    fn process_sentence(&self, s: &Sentence) -> Sentence {
        match s {
            Sentence::BinOp { a, b, op } => {
                self.process_bin_op(&a.inner(), &b.inner(), op.clone())
            }
            Sentence::UnaryOp { a, op } => todo!(),
            Sentence::TernaryOp { a, b, c, op } => todo!(),
            Sentence::Basic(v) => s.clone(),
        }
    } 

    fn get_basic_val(s: &Sentence) -> Option<Value> {
        if let Sentence::Basic(v) = s {
            Some(v.clone())
        } else {
            None
        }
    }
    fn process_bin_op(&self, a: &Sentence, b: &Sentence, op: BinOp) -> Sentence {
        match op {
            BinOp::Plus => {
                let aa = self.process_sentence(a);
                let bb = self.process_sentence(b);
                // These should always return Some(Value) since the above calls should reduce them
                let aa_basic = Interpreter::get_basic_val(&aa).unwrap();
                let bb_basic = Interpreter::get_basic_val(&bb).unwrap();
                match (aa_basic, bb_basic) {
                    (Value::Symbolic(x), Value::Symbolic(y)) => {
                        todo!()
                    },
                    (Value::Symbolic(x), Value::Concrete(y)) => {
                        todo!()
                    },
                    (Value::Concrete(x), Value::Symbolic(y)) => {
                        todo!()
                    },
                    (Value::Concrete(x), Value::Concrete(y)) => {
                        todo!()
                    },
                }

            },
            BinOp::Minus => todo!(),
            BinOp::Div => todo!(),
            BinOp::Mul => todo!(),
            BinOp::Mod => todo!(),
            BinOp::Eq => todo!(),
            BinOp::Neq => todo!(),
            BinOp::Lt => todo!(),
            BinOp::Lte => todo!(),
            BinOp::Gt => todo!(),
            BinOp::Gte => todo!(),
            BinOp::BitOr => todo!(),
            BinOp::BitAnd => todo!(),
            BinOp::BitXor => todo!(),
            BinOp::LShift => todo!(),
            BinOp::RShift => todo!(),
        }
    }

    
}
