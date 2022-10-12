pub mod base_interpreter;
use base_interpreter::*;
use crate::value::ast::*;
pub const ADDER_POST_HOOK: &'static dyn Fn(Sentence) -> Option<Sentence> = &|s: Sentence| -> Option<Sentence> {
    if let Sentence::BinOp {
        a,
        b,
        op: BinOp::Plus,
    } = s
    {
        match (a.inner().as_ref(), b.inner().as_ref()) {
            (Sentence::Basic(aa), Sentence::Basic(bb)) => {
                let aa = aa
                    .clone()
                    .into_concrete()
                    .ok()
                    .and_then(|aa| aa.into_number().ok())
                    .and_then(|n| n.into_u64().ok())
                    .unwrap();
                let bb = bb
                    .clone()
                    .into_concrete()
                    .ok()
                    .and_then(|bb| bb.into_number().ok())
                    .and_then(|n| n.into_u64().ok())
                    .unwrap();

                let sum = aa + bb;
                Some(Sentence::Basic(Value::Concrete(CSimpleVal::Number(
                    CNumber::U64(sum),
                ))))
            }
            _ => None,
        }
    } else {
        None
    }
};

#[cfg(test)]
mod test {
    use super::base_interpreter::*;
    use crate::value::ast::*;

    #[test]
    fn test_add_pgm() {
        let five = Sentence::Basic(Value::Concrete(CSimpleVal::Number(CNumber::U64(5_u64))));

        let ten = Sentence::Basic(Value::Concrete(CSimpleVal::Number(CNumber::U64(10_u64))));

        let twenty = Sentence::Basic(Value::Concrete(CSimpleVal::Number(CNumber::U64(20_u64))));

        let _sum = Sentence::Basic(Value::Concrete(CSimpleVal::Number(CNumber::U64(40_u64))));

        let add_a_b = Sentence::BinOp {
            a: Val::new(five.clone()),
            b: Val::new(ten),
            op: BinOp::Plus,
        };
        let add_a_c = Sentence::BinOp {
            a: Val::new(five),
            b: Val::new(twenty),
            op: BinOp::Plus,
        };
        let pgm = Sentence::BinOp {
            a: Val::new(add_a_b),
            b: Val::new(add_a_c),
            op: BinOp::Plus,
        };

        // Post hook to collapse binary addition between two literals.
        // This effectively simplifies (flattens) a tree of addition operations into a single SimpleVal
        let post_hook = |s: Sentence| -> Option<Sentence> {
            if let Sentence::BinOp {
                a,
                b,
                op: BinOp::Plus,
            } = s
            {
                match (a.inner().as_ref(), b.inner().as_ref()) {
                    (Sentence::Basic(aa), Sentence::Basic(bb)) => {
                        let aa = aa
                            .clone()
                            .into_concrete()
                            .ok()
                            .and_then(|aa| aa.into_number().ok())
                            .and_then(|n| n.into_u64().ok())
                            .unwrap();
                        let bb = bb
                            .clone()
                            .into_concrete()
                            .ok()
                            .and_then(|bb| bb.into_number().ok())
                            .and_then(|n| n.into_u64().ok())
                            .unwrap();

                        let sum = aa + bb;
                        Some(Sentence::Basic(Value::Concrete(CSimpleVal::Number(
                            CNumber::U64(sum),
                        ))))
                    }
                    _ => None,
                }
            } else {
                None
            }
        };

        // This hook effectively transforms a final BasicVal into a u64
        let final_hook = |s: Sentence| -> u64 {
            if let Sentence::Basic(v) = s {
                let val = *v
                    .as_concrete()
                    .and_then(|v| v.as_number())
                    .and_then(|v| v.as_u64())
                    .unwrap();
                val
            } else {
                0
            }
        };
        let pre_hook = |_s: Sentence| -> Option<Sentence> { None };

        let interpreter = Interpreter { pgm };
        let result = interpreter.interpret(Box::new(pre_hook), Box::new(post_hook), final_hook);
        assert_eq!(result, 40);
    }
}
