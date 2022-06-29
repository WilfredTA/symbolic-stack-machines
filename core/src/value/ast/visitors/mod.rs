pub mod concrete;

#[cfg(test)]
mod test {
    use super::concrete::*;
    use crate::value::ast::*;

    #[test]
    fn test_add_pgm() {
        let five = Sentence::Basic(
            Value::Concrete(
                CSimpleVal::Number(
                    CNumber::U64(5_u64)
                )
            )
        );

        let ten = Sentence::Basic(
            Value::Concrete(
                CSimpleVal::Number(
                    CNumber::U64(10_u64)
                )
            )
        );

        let twenty =  Sentence::Basic(
            Value::Concrete(
                CSimpleVal::Number(
                    CNumber::U64(20_u64)
                )
            )
        );

        let sum = Sentence::Basic(
            Value::Concrete(
                CSimpleVal::Number(
                    CNumber::U64(40_u64)
                )
            )
        );
        
        let add_a_b = Sentence::BinOp { a: Val::new(five.clone()), b: Val::new(ten), op: BinOp::Plus };
        let add_a_c = Sentence::BinOp { a: Val::new(five.clone()), b: Val::new(twenty), op: BinOp::Plus };
        let pgm = Sentence::BinOp { a: Val::new(add_a_b), b: Val::new(add_a_c), op: BinOp::Plus };

        let post_hook = |s: Sentence| -> Option<Sentence> {
            if let Sentence::BinOp { a, b, op: BinOp::Plus } = s {
                match (a.inner().as_ref(), b.inner().as_ref()) {
                    (Sentence::Basic(aa), Sentence::Basic(bb)) => {
                        let aa = aa.clone().into_concrete().ok().unwrap();
                        let bb = bb.clone().into_concrete().ok().unwrap();
                        let aa: u64 = aa.as_number().and_then(|n| n.as_u64()).unwrap().clone();
                        let bb = bb.as_number().and_then(|n| n.as_u64()).unwrap().clone();
                        let sum = aa + bb;
                        Some(
                            Sentence::Basic(Value::Concrete(CSimpleVal::Number(CNumber::U64(sum))))
                        )
                    },
                    _ => None
                }
               
            } else {
                None
            }
        };

        let final_hook = |s: Sentence| -> u64 {
            if let Sentence::Basic(v) = s {
                let val = v
                    .as_concrete()
                    .and_then(|v| {
                        v.as_number()
                    })
                    .and_then(|v| {
                        v.as_u64()
                    })
                    .unwrap()
                    .clone();
                val
            } else {
                0
            }
        };
        let pre_hook = |s: Sentence| -> Option<Sentence> {
            None
        };

        let interpreter = Interpreter {
            pgm
        };
        let result = interpreter.interpret(Box::new(pre_hook), Box::new(post_hook), final_hook);
        assert_eq!(result, 40);
    }
}