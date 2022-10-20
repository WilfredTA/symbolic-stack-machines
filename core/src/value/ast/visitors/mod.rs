pub mod base_interpreter;

#[cfg(test)]
mod test {
    use super::base_interpreter::*;
    use crate::value::ast::*;
    use rhai::{Engine, EvalAltResult, Scope, Dynamic};

    // TO DO: Need to add methods to Sentence & AST types to transform their data in Rhai arrays in order for a hook like this
    // to work: https://github.com/rhaiscript/rhai/blob/a0e272750abe066f7cc7813848782fa6aba42fed/tests/switch.rs#L194
    // Obviously, need to replace match with 'switch' etc as well
    const HOOK: &str = r###"
    fn script_hook(pgm) {

       if pgm.is_bin_op()
        {
            let a = pgm
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
                    return Some(Sentence::Basic(Value::Concrete(CSimpleVal::Number(
                        CNumber::U64(sum),
                    ))));
                }
                _ => {
                    return None;
                },
            }
        } else {
            return None;
        }
    }

    script_hook(pgm)
    "###;

    fn hook(pgm: Sentence) -> Option<Sentence> {
        if let Sentence::BinOp {
            a,
            b,
            op: BinOp::Plus,
        } = pgm
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
    }

    fn final_hook(pgm: Sentence) -> u64 {
        if let Sentence::Basic(v) = pgm {
            let val = *v
                .as_concrete()
                .and_then(|v| v.as_number())
                .and_then(|v| v.as_u64())
                .unwrap();
            val
        } else {
            0
        }
    }
    struct TestInterpreter<'a> {
        engine: Engine,
        scope: Scope<'a>,
    }


    impl TestInterpreter<'_> {
        pub fn new(pgm: Sentence) -> Self {
            let mut engine = Engine::new();
            let mut scope = Scope::new();
            scope.push("pgm", pgm);
            engine.register_type_with_name::<Sentence>("Sentence");
            engine.register_type_with_name::<BinOp>("BinOp");
            engine.register_fn("hook", hook);
            engine.register_fn("final_hook", final_hook);
            
            Self {engine, scope}
        }
    }
    impl Evaluate for TestInterpreter<'_> {
        type FinalType = u64;

        fn typ_name(&self) -> String {
            "u64".to_string()
        }

        fn interpreter_hook(&self) -> String {
          // HOOK.to_string()
          "hook(pgm)".to_string()
        }

        fn post_process_fn(&self) -> String {
            "final_hook(pgm_final)".to_string()
        }

        fn engine(&self) -> &rhai::Engine {
            &self.engine
        }

        fn scope(&self) -> rhai::Scope {
            self.scope.clone()
        }
    }


    #[test]
    fn test_add_pgm_rhai_hook_interpreter() {
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

        let interpreter = TestInterpreter::new(pgm.clone());
        let result = interpreter.interpret(pgm);
        println!("Final pgm is: {:?}", interpreter.scope.get_value::<Sentence>("pgm_final"));
        assert_eq!(result, 40);

    }

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
