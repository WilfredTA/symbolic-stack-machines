use crate::value::ast::*;

pub struct Interpreter {
    pgm: Sentence
}



impl Visitor<CSimpleVal> for Interpreter {
    fn visit_sentence(&mut self, s: &Sentence) -> CSimpleVal {
        match s {
            Sentence::Arith(a) => self.visit_arith(a),
            Sentence::Bool(b) => self.visit_boolf(b),
            Sentence::Val(v) => self.visit_val(v)
        }
    }

    fn visit_arith(&mut self, s: &Arith) -> CSimpleVal {
        match s {
            Arith::Plus(l, r) => {
                let l = self.visit_sentence(l.inner().as_ref());
                let r = self.visit_sentence(r.inner().as_ref());
                match (l, r) {
                    (CSimpleVal::Number(n), CSimpleVal::Number(m)) => {
                        let new_num = n.inner_add(m);
                        CSimpleVal::Number(new_num)
                    },
                    _ => panic!("Error must add two numbers")
               
                }
            },
            Arith::Minus(l, r) => {
                let l = self.visit_sentence(l.inner().as_ref());
                let r = self.visit_sentence(r.inner().as_ref());

                match (l, r) {
                    (CSimpleVal::Number(n), CSimpleVal::Number(m)) => {
                        let new_num = n.inner_sub(m);
                        CSimpleVal::Number(new_num)
                    },
                    _ => panic!("Error must add two numbers")
               
                }
            },
            Arith::Div(l, r) => {
                let l = self.visit_sentence(l.inner().as_ref());
                let r = self.visit_sentence(r.inner().as_ref());

                match (l, r) {
                    (CSimpleVal::Number(n), CSimpleVal::Number(m)) => {
                        let new_num = n.inner_div(m);
                        CSimpleVal::Number(new_num)
                    },
                    _ => panic!("Error must add two numbers")
               
                }
            },
            Arith::Mul(l, r) => {
                let l = self.visit_sentence(l.inner().as_ref());
                let r = self.visit_sentence(r.inner().as_ref());

                match (l, r) {
                    (CSimpleVal::Number(n), CSimpleVal::Number(m)) => {
                        let new_num = n.inner_mul(m);
                        CSimpleVal::Number(new_num)
                    },
                    _ => panic!("Error must add two numbers")
               
                }
            },
        }
    }

    fn visit_boolf(&mut self, s: &BoolF) -> CSimpleVal {
       match s {
        BoolF::Eq(l, r) => {
            let l = self.visit_sentence(l.inner().as_ref());
            let r = self.visit_sentence(r.inner().as_ref());
            match (l, r) {
                (CSimpleVal::Boolean(l), CSimpleVal::Boolean(r)) => todo!(),
                (CSimpleVal::Boolean(l), CSimpleVal::Number(r)) => todo!(),
                (CSimpleVal::Boolean(l), CSimpleVal::Vector(r)) => todo!(),
                (CSimpleVal::Number(l), CSimpleVal::Boolean(r)) => todo!(),
                (CSimpleVal::Number(l), CSimpleVal::Number(r)) => todo!(),
                (CSimpleVal::Number(l), CSimpleVal::Vector(r)) => todo!(),
                (CSimpleVal::Vector(l), CSimpleVal::Boolean(r)) => todo!(),
                (CSimpleVal::Vector(l), CSimpleVal::Number(r)) => todo!(),
                (CSimpleVal::Vector(l), CSimpleVal::Vector(r)) => todo!(),
            }
        },
        BoolF::Neq(l, r) => {
            let l = self.visit_sentence(l.inner().as_ref());
            let r = self.visit_sentence(r.inner().as_ref());
            match (l, r) {
                (CSimpleVal::Boolean(l), CSimpleVal::Boolean(r)) => todo!(),
                (CSimpleVal::Boolean(l), CSimpleVal::Number(r)) => todo!(),
                (CSimpleVal::Boolean(l), CSimpleVal::Vector(r)) => todo!(),
                (CSimpleVal::Number(l), CSimpleVal::Boolean(r)) => todo!(),
                (CSimpleVal::Number(l), CSimpleVal::Number(r)) => todo!(),
                (CSimpleVal::Number(l), CSimpleVal::Vector(r)) => todo!(),
                (CSimpleVal::Vector(l), CSimpleVal::Boolean(r)) => todo!(),
                (CSimpleVal::Vector(l), CSimpleVal::Number(r)) => todo!(),
                (CSimpleVal::Vector(l), CSimpleVal::Vector(r)) => todo!(),
            }
        },
        BoolF::Lt(l, r) => {
            let l = self.visit_sentence(l.inner().as_ref());
            let r = self.visit_sentence(r.inner().as_ref());
            match (l, r) {
                (CSimpleVal::Boolean(l), CSimpleVal::Boolean(r)) => todo!(),
                (CSimpleVal::Boolean(l), CSimpleVal::Number(r)) => todo!(),
                (CSimpleVal::Boolean(l), CSimpleVal::Vector(r)) => todo!(),
                (CSimpleVal::Number(l), CSimpleVal::Boolean(r)) => todo!(),
                (CSimpleVal::Number(l), CSimpleVal::Number(r)) => todo!(),
                (CSimpleVal::Number(l), CSimpleVal::Vector(r)) => todo!(),
                (CSimpleVal::Vector(l), CSimpleVal::Boolean(r)) => todo!(),
                (CSimpleVal::Vector(l), CSimpleVal::Number(r)) => todo!(),
                (CSimpleVal::Vector(l), CSimpleVal::Vector(r)) => todo!(),
            }
        },
        BoolF::Gt(l, r) => {
            let l = self.visit_sentence(l.inner().as_ref());
            let r = self.visit_sentence(r.inner().as_ref());
            match (l, r) {
                (CSimpleVal::Boolean(l), CSimpleVal::Boolean(r)) => todo!(),
                (CSimpleVal::Boolean(l), CSimpleVal::Number(r)) => todo!(),
                (CSimpleVal::Boolean(l), CSimpleVal::Vector(r)) => todo!(),
                (CSimpleVal::Number(l), CSimpleVal::Boolean(r)) => todo!(),
                (CSimpleVal::Number(l), CSimpleVal::Number(r)) => todo!(),
                (CSimpleVal::Number(l), CSimpleVal::Vector(r)) => todo!(),
                (CSimpleVal::Vector(l), CSimpleVal::Boolean(r)) => todo!(),
                (CSimpleVal::Vector(l), CSimpleVal::Number(r)) => todo!(),
                (CSimpleVal::Vector(l), CSimpleVal::Vector(r)) => todo!(),
            }
        },
        BoolF::Lte(l, r) => {
            let l = self.visit_sentence(l.inner().as_ref());
            let r = self.visit_sentence(r.inner().as_ref());
            match (l, r) {
                (CSimpleVal::Boolean(l), CSimpleVal::Boolean(r)) => todo!(),
                (CSimpleVal::Boolean(l), CSimpleVal::Number(r)) => todo!(),
                (CSimpleVal::Boolean(l), CSimpleVal::Vector(r)) => todo!(),
                (CSimpleVal::Number(l), CSimpleVal::Boolean(r)) => todo!(),
                (CSimpleVal::Number(l), CSimpleVal::Number(r)) => todo!(),
                (CSimpleVal::Number(l), CSimpleVal::Vector(r)) => todo!(),
                (CSimpleVal::Vector(l), CSimpleVal::Boolean(r)) => todo!(),
                (CSimpleVal::Vector(l), CSimpleVal::Number(r)) => todo!(),
                (CSimpleVal::Vector(l), CSimpleVal::Vector(r)) => todo!(),
            }
        },
        BoolF::Gte(l, r) => {
            let l = self.visit_sentence(l.inner().as_ref());
            let r = self.visit_sentence(r.inner().as_ref());
            match (l, r) {
                (CSimpleVal::Boolean(l), CSimpleVal::Boolean(r)) => todo!(),
                (CSimpleVal::Boolean(l), CSimpleVal::Number(r)) => todo!(),
                (CSimpleVal::Boolean(l), CSimpleVal::Vector(r)) => todo!(),
                (CSimpleVal::Number(l), CSimpleVal::Boolean(r)) => todo!(),
                (CSimpleVal::Number(l), CSimpleVal::Number(r)) => todo!(),
                (CSimpleVal::Number(l), CSimpleVal::Vector(r)) => todo!(),
                (CSimpleVal::Vector(l), CSimpleVal::Boolean(r)) => todo!(),
                (CSimpleVal::Vector(l), CSimpleVal::Number(r)) => todo!(),
                (CSimpleVal::Vector(l), CSimpleVal::Vector(r)) => todo!(),
            }
        },
    }
    }
    fn visit_val(&mut self, s: &Value) -> CSimpleVal {
        todo!()
    }
}