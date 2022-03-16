pub mod instructions;
pub mod machine;
pub mod memory;
pub mod stack;

use instructions::*;
use machine::*;
use memory::symbolic::BaseSymbolicMem;
use stack::*;
use z3::ast::{Ast, Int};

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

    type Mem = BaseSymbolicMem<'a>;

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
                let stack_top = stack.peek(0).unwrap();
                let constraint = stack_top._eq(v);
                change_log.path_constraints.push(vec![constraint]);
            }
            Instruction::MLOAD => todo!(),
            Instruction::MSTORE => todo!(),
            Instruction::ISZERO => todo!(),
            Instruction::JUMPI => todo!(),
            Instruction::STOP => todo!(),
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

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use z3::{
        ast::{Array, Int},
        Config, Context, FuncDecl, SatResult,
    };

    use super::*;

    fn z3_int<'a>(i: u64, ctxt: &'a Context) -> z3::ast::Int<'a> {
        Int::from_u64(&ctxt, i)
    }

    fn z3_int_var<'a>(i: &str, ctxt: &'a Context) -> z3::ast::Int<'a> {
        Int::new_const(&ctxt, i)
    }

    #[test]
    fn test_basic_sym_mem() {
        let cfg = Config::default();
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

        let res = machine.run_sym(&pgm);
        println!("{:?}", res);
        assert_eq!(res.0, SatResult::Sat)
    }
}
