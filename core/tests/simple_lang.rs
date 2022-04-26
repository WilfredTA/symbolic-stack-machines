use symbolic_stack_machines_core::memory::{MemOpRecord, MemRecord, ReadOnlyMem};
use symbolic_stack_machines_core::{instructions::*, machine::*, memory::memory_models::*, stack::*};

use std::rc::Rc;
use z3::ast::{Ast, Bool, Int};
use z3::{Config, Context};
mod common;

use common::{z3_int, z3_int_var};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Instruction<T> {
    Add,
    Sub,
    Push(T),
    Assert(T),
    MLOAD,
    MSTORE,
    ISZERO,
    JUMPI,
    STOP,
}

impl<'a> VMInstruction<'a> for Instruction<Int<'a>> {
    type ValStack = BaseStack<Int<'a>>;

    type Mem = MemIntToInt<'a>;

    fn exec(
        &self,
        stack: &Self::ValStack,
        memory: &Self::Mem,
    ) -> InstructionResult<ExecRecord<'a, Self::ValStack, Self::Mem>> {
        let mut change_log: ExecRecord<'a, Self::ValStack, Self::Mem> = ExecRecord {
            stack_diff: None,
            mem_diff: None,
            path_constraints: vec![],
            pc_change: None,
            halt: false,
        };
        match self {
            Instruction::Add => {
                let op_1 = stack.peek(0).unwrap();
                let op_2 = stack.peek(1).unwrap();
                let res = &op_1 + &op_2;
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
                let res = &op_1 - &op_2;
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
                let stack_top = stack.peek::<Int<'a>>(0).unwrap();
                let constraint = stack_top._eq(v);
                change_log.path_constraints.push(vec![constraint]);
            }
            Instruction::MLOAD => {
                let mem_offset = stack.peek::<Int<'a>>(0).unwrap();
                let val = {
                    match memory.read(mem_offset.clone()) {
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
                let mem_offset = stack.peek::<Int<'a>>(0).unwrap();
                let val = stack.peek::<Int<'a>>(1).unwrap();
                let prev_val = {
                    match memory.read(mem_offset.clone()) {
                        Ok(val) => val.unwrap(),
                        Err(e) => Int::from_u64(val.get_ctx(), 0),
                    }
                };
                change_log.stack_diff = Some(StackRecord {
                    changed: vec![
                        StackOpRecord::Pop(mem_offset.clone()),
                        StackOpRecord::Pop(val.clone()),
                    ],
                });
                change_log.mem_diff = Some(MemRecord {
                    diff: vec![MemOpRecord::Write((mem_offset, prev_val, val))],
                });
            }
            Instruction::ISZERO => {
                let top = stack.peek::<Int<'a>>(0).unwrap();
                let zero = Int::from_u64(top.get_ctx(), 0);
                let one = Int::from_u64(top.get_ctx(), 1);
                let is_zero = Bool::ite(&top._eq(&zero), &one, &zero);
                change_log.stack_diff = Some(StackRecord {
                    changed: vec![
                        StackOpRecord::Pop(top.clone()),
                        StackOpRecord::Push(is_zero.clone()),
                    ],
                });
            }
            Instruction::JUMPI => {
                let dest = stack.peek::<Int<'a>>(0).unwrap();
                let ctx = dest.ctx;
                let cond = stack.peek::<Int<'a>>(1).unwrap();
                if let Some(dest) = dest.as_u64() {
                    let zero = Int::from_u64(ctx, 0);
                    change_log.path_constraints.push(vec![cond._eq(&zero)]);
                    change_log
                        .path_constraints
                        .push(vec![Bool::not(&cond._eq(&zero))]);
                    change_log.pc_change = Some(dest as usize);
                }
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

#[test]
fn test_basic_sym_mem() {
    let mut cfg = Config::default();
    cfg.set_model_generation(true);
    let ctx = Context::new(&cfg);

    let stack: BaseStack<Int> = BaseStack::init();
    let machine = BaseMachine::new_with_ctx(stack, Rc::new(&ctx));
    let pgm = vec![
        push(z3_int_var("a", &ctx)),
        push(z3_int(3, &ctx)),
        push(z3_int_var("c", &ctx)),
        add(),
        sub(),
        assert(z3_int(4, &ctx)),
    ];

    let _res = machine.run_sym(&pgm);
}

#[test]
fn test_jumpi() {
    let mut cfg = Config::default();
    cfg.set_model_generation(true);
    let ctx = Context::new(&cfg);

    let stack: BaseStack<Int> = BaseStack::init();
    let machine = BaseMachine::new_with_ctx(stack, Rc::new(&ctx));
    let pgm = vec![
        push(z3_int(1, &ctx)),
        push(z3_int(2, &ctx)),
        push(z3_int(3, &ctx)),
        add(),
        sub(),
        push(z3_int(4, &ctx)),
        sub(),
        is_zero(),
        push(z3_int(12, &ctx)),
        jumpi(),
        push(z3_int(100, &ctx)),
        stop(),
        push(z3_int(200, &ctx)),
    ];

    let res = machine.run_sym(&pgm);
    let (reachable, unreachable) = res;
    let first_path_reachable_stack: &BaseStack<Int> = &reachable.first().unwrap().0 .1;
    let first_path_unreachable_stack: &BaseStack<Int> = &unreachable.first().unwrap().0 .1;

    assert_eq!(
        first_path_reachable_stack
            .peek::<Int>(0)
            .unwrap()
            .as_u64()
            .unwrap(),
        200
    );
    assert_eq!(
        first_path_unreachable_stack
            .peek::<Int>(0)
            .unwrap()
            .as_u64()
            .unwrap(),
        100
    );
}

#[test]
fn test_multi_jumpi() {
    let mut cfg = Config::default();
    cfg.set_model_generation(true);
    let ctx = Context::new(&cfg);

    let stack: BaseStack<Int> = BaseStack::init();
    let machine = BaseMachine::new_with_ctx(stack, Rc::new(&ctx));
    let pgm = vec![
        push(z3_int(1, &ctx)),
        push(z3_int(2, &ctx)),
        push(z3_int(3, &ctx)),
        add(),
        sub(),
        push(z3_int(3, &ctx)),
        sub(),
        push(z3_int(13, &ctx)),
        jumpi(),
        push(z3_int(100, &ctx)),
        stop(),
        stop(),
        stop(),
        push(z3_int(200, &ctx)),
        push(z3_int(201, &ctx)),
        sub(),
        push(z3_int(19, &ctx)),
        jumpi(),
        stop(),
        push(z3_int(300, &ctx)),
        stop(),
    ];

    let _res = machine.run_sym(&pgm);
}
