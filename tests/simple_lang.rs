use symbolic_stack_machines::memory::{MemIntToInt, MemOpRecord, MemRecord, ReadOnlyMem};
use symbolic_stack_machines::vals::{MachineEq, SymbolicInt, SYM};
use symbolic_stack_machines::{instructions::*, machine::*, stack::*};

use std::rc::Rc;
use z3::{Config, Context};
mod common;

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

impl<'a> VMInstruction<'a> for Instruction<SymbolicInt> {
    type ValStack = BaseStack<SymbolicInt>;

    type Mem = MemIntToInt;

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
                let op_1: SymbolicInt = stack.peek(0).unwrap();
                let op_2: SymbolicInt = stack.peek(1).unwrap();
                let res = op_1.clone() + op_2.clone();
                change_log.stack_diff = Some(StackRecord {
                    changed: vec![
                        StackOpRecord::Pop(op_1),
                        StackOpRecord::Pop(op_2),
                        StackOpRecord::Push(res),
                    ],
                });
            }
            Instruction::Sub => {
                let op_1: SymbolicInt = stack.peek(0).unwrap();
                let op_2: SymbolicInt = stack.peek(1).unwrap();
                let res = op_1.clone() - op_2.clone();
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
            }
            Instruction::MLOAD => {
                let mem_offset: SymbolicInt = stack.peek(0).unwrap();
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
                let mem_offset: SymbolicInt = stack.peek(0).unwrap();
                let val: SymbolicInt = stack.peek(1).unwrap();
                let prev_val = {
                    match memory.read(mem_offset.clone()) {
                        Ok(val) => val.unwrap(),
                        Err(e) => SymbolicInt::default(),
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
                let top: SymbolicInt = stack.peek(0).unwrap();
                let is_zero = SymbolicInt::machine_ite(
                    SymbolicInt::machine_eq(&top, &0.into()),
                    1.into(),
                    0.into(),
                );
                change_log.stack_diff = Some(StackRecord {
                    changed: vec![
                        StackOpRecord::Pop(top.clone()),
                        StackOpRecord::Push(is_zero.clone()),
                    ],
                });
            }
            Instruction::JUMPI => {
                todo!()
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

    let stack: BaseStack<SymbolicInt> = BaseStack::init();
    let machine = BaseMachine::new_with_ctx(stack, Rc::new(&ctx));
    let pgm = vec![
        push(SYM("a")),
        push(3.into()),
        push(SYM("c")),
        add(),
        sub(),
        assert(4.into()),
    ];

    let _res = machine.run_sym(&pgm);
}

#[test]
fn test_jumpi() {
    let mut cfg = Config::default();
    cfg.set_model_generation(true);
    let ctx = Context::new(&cfg);

    let stack: BaseStack<SymbolicInt> = BaseStack::init();
    let machine = BaseMachine::new_with_ctx(stack, Rc::new(&ctx));
    let pgm = vec![
        push(1.into()),
        push(2.into()),
        push(3.into()),
        add(),
        sub(),
        push(4.into()),
        sub(),
        is_zero(),
        push(12.into()),
        jumpi(),
        push(100.into()),
        stop(),
        push(200.into()),
    ];

    let res = machine.run_sym(&pgm);
    let (reachable, unreachable) = res;
    let first_path_reachable_stack: &BaseStack<SymbolicInt> = &reachable.first().unwrap().0 .1;
    let first_path_unreachable_stack: &BaseStack<SymbolicInt> = &unreachable.first().unwrap().0 .1;

    assert_eq!(
        first_path_reachable_stack
            .peek::<SymbolicInt>(0)
            .unwrap(),
        200.into()
    );
    assert_eq!(
        first_path_unreachable_stack
            .peek::<SymbolicInt>(0)
            .unwrap(),
        100.into()
    );
}

#[test]
fn test_multi_jumpi() {
    let mut cfg = Config::default();
    cfg.set_model_generation(true);
    let ctx = Context::new(&cfg);

    let stack: BaseStack<SymbolicInt> = BaseStack::init();
    let machine = BaseMachine::new_with_ctx(stack, Rc::new(&ctx));
    let pgm = vec![
        push(1.into()),
        push(2.into()),
        push(3.into()),
        add(),
        sub(),
        push(3.into()),
        sub(),
        push(13.into()),
        jumpi(),
        push(100.into()),
        stop(),
        stop(),
        stop(),
        push(200.into()),
        push(201.into()),
        sub(),
        push(19.into()),
        jumpi(),
        stop(),
        push(300.into()),
        stop(),
    ];

    let _res = machine.run_sym(&pgm);
}
