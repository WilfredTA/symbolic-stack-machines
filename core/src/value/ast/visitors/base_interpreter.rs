use crate::value::ast::*;

pub enum InterpreterMode {
    Symbolic,
    Concrete,
}

#[derive(Clone)]
pub struct Interpreter {
    pub pgm: Sentence,
}



pub type Hook = Box<dyn Fn(Sentence) -> Option<Sentence>>;
// Post hook is called after the AST has been processed
// Pre hook is called at each node in the AST prior to potentially increased traversal.
// If pre_hook returns None, then processing continues as normal. If it returns Some(Sentence), then the sentence returned
// is used instead of the sentence that would result from further processing.
// Final is only called at the end
impl Interpreter {
    pub fn interpret<T, Final>(&self, pre: Hook, post: Hook, final_hook: Final) -> T
    where
        Final: Fn(Sentence) -> T,
    {
        if let Some(s) = pre(self.pgm.clone()) {
            let ret = match self.exec_hook(&post, &s) {
                Some(sen) => sen,
                None => s,
            };
            final_hook(ret)
        } else {
            let sentence = self.process_sentence(&self.pgm, &pre, &post);
            let ret = match self.exec_hook(&post, &sentence) {
                Some(sen) => sen,
                None => sentence,
            };
            final_hook(ret)
        }
    }

    fn exec_hook(
        &self,
        hook: &impl Fn(Sentence) -> Option<Sentence>,
        on: &Sentence,
    ) -> Option<Sentence> {
        hook(on.clone())
    }

    fn process_sentence(&self, s: &Sentence, pre_hook: &Hook, post_hook: &Hook) -> Sentence {
        let res = match self.exec_hook(pre_hook, s) {
            None => match s {
                Sentence::BinOp { a, b, op } => {
                    self.process_bin_op(&a.inner(), &b.inner(), op.clone(), pre_hook, post_hook)
                }
                Sentence::UnaryOp { a, op } => {
                    self.process_unary_op(&a.inner(), op.clone(), pre_hook, post_hook)
                }
                Sentence::TernaryOp { a, b, c, op } => self.process_ternary_op(
                    &a.inner(),
                    &b.inner(),
                    &c.inner(),
                    op.clone(),
                    pre_hook,
                    post_hook,
                ),
                Sentence::Basic(_v) => s.clone(),
                Sentence::Nothing => {
                    s.clone()
                }
            },
            Some(s) => s,
        };

        // We only need to exec post_hook here , since every node in the tree is encapsulated by a sentence, so each subtree traversal
        // will result in a call to process_sentence
        match self.exec_hook(post_hook, &res) {
            Some(s) => s,
            None => res,
        }
    }

    fn get_basic_val(s: &Sentence) -> Option<Value> {
        if let Sentence::Basic(v) = s {
            Some(v.clone())
        } else {
            None
        }
    }

    fn process_bin_op(
        &self,
        a: &Sentence,
        b: &Sentence,
        op: BinOp,
        pre_hook: &Hook,
        post_hook: &Hook,
    ) -> Sentence {
        let aa = match self.exec_hook(pre_hook, a) {
            None => self.process_sentence(a, pre_hook, post_hook),
            Some(s) => s,
        };
        let bb = match self.exec_hook(pre_hook, b) {
            None => self.process_sentence(b, pre_hook, post_hook),
            Some(s) => s,
        };

        Sentence::BinOp {
            a: Val::new(aa),
            b: Val::new(bb),
            op,
        }
    }

    fn process_unary_op(
        &self,
        a: &Sentence,
        op: UnaryOp,
        pre_hook: &Hook,
        post_hook: &Hook,
    ) -> Sentence {
        let aa = match self.exec_hook(pre_hook, a) {
            None => self.process_sentence(a, pre_hook, post_hook),
            Some(s) => s,
        };
        Sentence::UnaryOp {
            a: Val::new(aa),
            op,
        }
    }

    fn process_ternary_op(
        &self,
        a: &Sentence,
        b: &Sentence,
        c: &Sentence,
        op: TernaryOp,
        pre_hook: &Hook,
        post_hook: &Hook,
    ) -> Sentence {
        let aa = match self.exec_hook(pre_hook, a) {
            None => self.process_sentence(a, pre_hook, post_hook),
            Some(s) => s,
        };
        let bb = match self.exec_hook(pre_hook, b) {
            None => self.process_sentence(b, pre_hook, post_hook),
            Some(s) => s,
        };
        let cc = match self.exec_hook(pre_hook, c) {
            None => self.process_sentence(b, pre_hook, post_hook),
            Some(s) => s,
        };

        Sentence::TernaryOp {
            a: Val::new(aa),
            b: Val::new(bb),
            c: Val::new(cc),
            op,
        }
    }
}
