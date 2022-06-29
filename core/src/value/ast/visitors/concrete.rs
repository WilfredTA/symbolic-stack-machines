use crate::value::ast::*;


pub enum InterpreterMode {
    Symbolic,
    Concrete
}
pub struct Interpreter {
    pgm: Sentence,
    mode: InterpreterMode,
}

// Post hook is called after the AST has been processed
// Pre hook is called at each node in the AST prior to potentially increased traversal.
// If pre_hook returns None, then processing continues as normal. If it returns Some(Sentence), then the sentence returned
// is used instead of the sentence that would result from further processing.
// Final is only called at the end
impl Interpreter {
    pub fn interpret<T, Hook, Final>(&self, pre_hook: Hook, post_hook: Hook, final_hook: Final) -> T
    where 
        Hook: Fn(Sentence) -> Option<Sentence>,
        Final: Fn(Sentence) -> T,
    {
       if let Some(s) = pre_hook(self.pgm.clone()) {
           let ret = match self.exec_hook(&post_hook, &s) {
               Some(sen) => sen,
               None => s
           };
           final_hook(ret)
       } else {
           let sentence = self.process_sentence::<Hook>(&self.pgm, &pre_hook, &post_hook);
           let ret = match self.exec_hook(&post_hook, &sentence) {
                Some(sen) => sen,
                None => sentence
            };
            final_hook(ret)
       }
      
    }


    fn exec_hook(&self, hook: &impl Fn(Sentence) -> Option<Sentence>, on: &Sentence) -> Option<Sentence> {
        hook(on.clone())
    }


    fn process_sentence<Hook>(&self, s: &Sentence, pre_hook: &Hook, post_hook: &Hook) -> Sentence 
    where 
        Hook: Fn(Sentence) -> Option<Sentence>,
    {
  
       let res =  match self.exec_hook(pre_hook, s) {
            None => {
                match s {
                    Sentence::BinOp { a, b, op } => {
                       self.process_bin_op::<Hook>(&a.inner(), &b.inner(), op.clone(), pre_hook, post_hook)
                       
                    }
                    Sentence::UnaryOp { a, op } => todo!(),
                    Sentence::TernaryOp { a, b, c, op } => todo!(),
                    Sentence::Basic(v) => s.clone(),
                }
            },
            Some(s) => s
        };

        // We only need to exec post_hook here , since every node in the tree is encapsulated by a sentence, so each subtree traversal
        // will result in a call to process_sentence
        match self.exec_hook(post_hook, &res) {
            Some(s) => s,
            None => res
        }
        
    } 

    fn get_basic_val(s: &Sentence) -> Option<Value> {
        if let Sentence::Basic(v) = s {
            Some(v.clone())
        } else {
            None
        }
    }



    fn process_bin_op<Hook>(&self, a: &Sentence, b: &Sentence, op: BinOp, pre_hook: &Hook, post_hook: &Hook) -> Sentence 
    where 
        Hook: Fn(Sentence) -> Option<Sentence>,
    {
   
        match op {
            BinOp::Plus => {

                let aa = match self.exec_hook(pre_hook, a) {
                  None => self.process_sentence::<Hook>(a, pre_hook, post_hook),
                  Some(s) => s  
                };
                let bb = match self.exec_hook(pre_hook, b) {
                    None => self.process_sentence::<Hook>(b, pre_hook, post_hook),
                    Some(s) => s  
                };

                
                Sentence::BinOp { a: Val::new(aa), b: Val::new(bb), op: BinOp::Plus }
                // These should always return Some(Value) since the above calls should reduce them
                

                // The below could be the skeleton of a post_hook that adds numbers together to
                // simplify the BinOp to a single BasicValue
                
                //let aa_basic = Interpreter::get_basic_val(&aa).unwrap();
                //let bb_basic = Interpreter::get_basic_val(&bb).unwrap();
                // match (aa_basic, bb_basic) {
                //     (Value::Symbolic(x), Value::Symbolic(y)) => {
                //         todo!()
                //     },
                //     (Value::Symbolic(x), Value::Concrete(y)) => {
                //         todo!()
                //     },
                //     (Value::Concrete(x), Value::Symbolic(y)) => {
                //         todo!()
                //     },
                //     (Value::Concrete(x), Value::Concrete(y)) => {
                //         todo!()
                //     },
                // }

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
