use symbolic_stack_machines::instructions::arith::{ADD, SUB};
use symbolic_stack_machines::memory::symbolic_concrete_index::MemIntToInt;
use symbolic_stack_machines::memory::ReadOnlyMem;
use symbolic_stack_machines::{instructions::*, stack::*};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Instruction<T> {
    // DONE
    Add,
    // DONE
    Sub,
    // DONE
    Push(T),

    // DONE
    Assert(T),

    // DONE
    MLOAD,

    // DONE
    MSTORE,

    // DONE
    ISZERO,

    // DONE
    JUMPI,

    // DONE
    STOP,
}

impl VMInstruction<IntStack, MemIntToInt> for Instruction<i128> {
    fn exec(
        &self,
        stack: &IntStack,
        memory: &MemIntToInt,
    ) -> InstructionResult<ExecRecord<IntStack, MemIntToInt>> {
        let mut change_log: ExecRecord<IntStack, MemIntToInt> = ExecRecord {
            stack_diff: None,
            mem_diff: None,
            // path_constraints: vec![],
            pc_change: None,
            halt: false,
        };
        match self {
            Instruction::Add => {
                let op_1 = stack.peek(0).unwrap();
                let op_2 = stack.peek(1).unwrap();
                let res = op_1 + op_2;
                change_log.stack_diff = Some(StackRecord {
                    changed: vec![
                        StackOpRecord::Pop(op_1),
                        StackOpRecord::Pop(op_2),
                        StackOpRecord::Push(res),
                    ],
                });
            }
            Instruction::Sub => {
                let op_1 = stack.peek(0).unwrap();
                let op_2 = stack.peek(1).unwrap();
                let res = op_1 - op_2;
                change_log.stack_diff = Some(StackRecord {
                    changed: vec![
                        StackOpRecord::Pop(op_1),
                        StackOpRecord::Pop(op_2),
                        StackOpRecord::Push(res),
                    ],
                });
            }
            Instruction::Push(v) => {
                change_log.stack_diff = Some(StackRecord {
                    changed: vec![StackOpRecord::Push(v.clone())],
                });
            }
            Instruction::Assert(v) => {
                todo!()
                // let stack_top = stack.peek(0).unwrap();
                // let constraint = stack_top._eq(v);
                // change_log.path_constraints.push(vec![constraint]);
            }
            Instruction::MLOAD => {
                let mem_offset = stack.peek(0).unwrap();
                let mem_offset_usize = usize::try_from(mem_offset).unwrap();
                let val = {
                    match memory.read(mem_offset_usize) {
                        Ok(val) => val.unwrap(),
                        Err(e) => {
                            panic!("Error reading from memory: {:?}", e);
                        }
                    }
                };
                change_log.stack_diff = Some(StackRecord {
                    changed: vec![StackOpRecord::Pop(mem_offset), StackOpRecord::Push(val)],
                });
            }
            Instruction::MSTORE => {
                // let mem_offset = stack.peek(0).unwrap();
                // let val = stack.peek(1).unwrap();
                // let prev_val = {
                //     match memory.read(mem_offset.clone()) {
                //         Ok(val) => val.unwrap(),
                //         Err(e) => Int::from_u64(val.get_ctx(), 0),
                //     }
                // };
                // change_log.stack_diff = Some(StackRecord {
                //     changed: vec![
                //         StackOpRecord::Pop(mem_offset.clone()),
                //         StackOpRecord::Pop(val.clone()),
                //     ],
                // });
                // change_log.mem_diff = Some(MemRecord {
                //     diff: vec![MemOpRecord::Write((mem_offset, prev_val, val))],
                // });
            }
            Instruction::ISZERO => {
                todo!()
                // let top = stack.peek(0).unwrap();
                // let zero = Int::from_u64(top.get_ctx(), 0);
                // let one = Int::from_u64(top.get_ctx(), 1);
                // let is_zero = Bool::ite(&top._eq(&zero), &one, &zero);
                // change_log.stack_diff = Some(StackRecord {
                //     changed: vec![
                //         StackOpRecord::Pop(top.clone()),
                //         StackOpRecord::Push(is_zero.clone()),
                //     ],
                // });
            }
            Instruction::JUMPI => {
                // let dest = stack.peek(0).unwrap();
                // let ctx = dest.ctx;
                // let cond = stack.peek(1).unwrap();
                // if let Some(dest) = dest.as_u64() {
                //     let zero = Int::from_u64(ctx, 0);
                //     change_log.path_constraints.push(vec![cond._eq(&zero)]);
                //     change_log
                //         .path_constraints
                //         .push(vec![Bool::not(&cond._eq(&zero))]);
                //     change_log.pc_change = Some(dest as usize);
                // }
            }
            Instruction::STOP => {
                change_log.halt = true;
            }
        };
        Ok(change_log)
    }
}
pub fn push<T>(val: T) -> Instruction<T> {
    Instruction::Push(val)
}

pub fn assert<T>(val: T) -> Instruction<T> {
    Instruction::Assert(val)
}

pub fn add<T>() -> Instruction<T> {
    Instruction::Add
}

pub fn sub<T>() -> Instruction<T> {
    Instruction::Sub
}
pub fn mload<T>() -> Instruction<T> {
    Instruction::MLOAD
}

pub fn mstore<T>() -> Instruction<T> {
    Instruction::MSTORE
}

pub fn is_zero<T>() -> Instruction<T> {
    Instruction::ISZERO
}
pub fn jumpi<T>() -> Instruction<T> {
    Instruction::JUMPI
}

pub fn stop<T>() -> Instruction<T> {
    Instruction::STOP
}

// #[test]
// fn test_basic_sym_mem() {
//     let mut cfg = Config::default();
//     cfg.set_model_generation(true);
//     let ctx = Context::new(&cfg);

//     let stack = IntStack::new();
//     let mem = MemIntToInt::new();
//     let machine = BaseMachine::new_with_ctx(stack, mem);
//     let pgm = vec![
//         push(z3_int_var("a", &ctx)),
//         push(z3_int(3, &ctx)),
//         push(z3_int_var("c", &ctx)),
//         add(),
//         sub(),
//         assert(z3_int(4, &ctx)),
//     ];

//     let _res = machine.run_sym(&pgm, &ctx);
// }

// #[test]
// fn test_jumpi() {
//     let mut cfg = Config::default();
//     cfg.set_model_generation(true);
//     let ctx = Context::new(&cfg);

//     let stack = IntStack::new();
//     let mem = MemIntToInt::new();
//     let machine = BaseMachine::new_with_ctx(stack, mem);
//     let pgm = vec![
//         push(z3_int(1, &ctx)),
//         push(z3_int(2, &ctx)),
//         push(z3_int(3, &ctx)),
//         add(),
//         sub(),
//         push(z3_int(4, &ctx)),
//         sub(),
//         is_zero(),
//         push(z3_int(12, &ctx)),
//         jumpi(),
//         push(z3_int(100, &ctx)),
//         stop(),
//         push(z3_int(200, &ctx)),
//     ];

//     let res = machine.run_sym(&pgm, &ctx);
//     let (reachable, unreachable) = res;
//     let first_path_reachable_stack: &BaseStack<Int> = &reachable.first().unwrap().0 .1;
//     let first_path_unreachable_stack: &BaseStack<Int> = &unreachable.first().unwrap().0 .1;

//     assert_eq!(
//         first_path_reachable_stack
//             .peek(0)
//             .unwrap()
//             .as_u64()
//             .unwrap(),
//         200
//     );
//     assert_eq!(
//         first_path_unreachable_stack
//             .peek(0)
//             .unwrap()
//             .as_u64()
//             .unwrap(),
//         100
//     );
// }

// #[test]
// fn test_multi_jumpi() {
//     let mut cfg = Config::default();
//     cfg.set_model_generation(true);
//     let ctx = Context::new(&cfg);

//     let stack = IntStack::new();
//     let mem = MemIntToInt::new();
//     let machine = BaseMachine::new_with_ctx(stack, mem);
//     let pgm = vec![
//         push(z3_int(1, &ctx)),
//         push(z3_int(2, &ctx)),
//         push(z3_int(3, &ctx)),
//         add(),
//         sub(),
//         push(z3_int(3, &ctx)),
//         sub(),
//         push(z3_int(13, &ctx)),
//         jumpi(),
//         push(z3_int(100, &ctx)),
//         stop(),
//         stop(),
//         stop(),
//         push(z3_int(200, &ctx)),
//         push(z3_int(201, &ctx)),
//         sub(),
//         push(z3_int(19, &ctx)),
//         jumpi(),
//         stop(),
//         push(z3_int(300, &ctx)),
//         stop(),
//     ];

//     let _res = machine.run_sym(&pgm, &ctx);
// }
