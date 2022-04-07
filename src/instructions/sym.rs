use crate::{memory::Mem, stack::Stack};

use super::SymbolicVMInstruction;

pub trait Constrain {
    type Constraint;

    fn assert_eq(self, other: Self) -> Self::Constraint;
    fn assert_not_eq(self, other: Self) -> Self::Constraint;
}

#[derive(Debug)]
pub struct JUMPI;

impl<T, S, M, C> SymbolicVMInstruction<S, M, C> for JUMPI
where
    T: TryInto<usize> + Constrain<Constraint = C> + Default + Clone,
    S: Stack<StackVal = T> + Clone,
    M: Mem + Clone,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
{
    fn sym_exec(&self, stack: &S, memory: &M, pc: usize) -> Vec<(S, M, usize, Vec<C>)> {
        let dest = stack.peek(0).unwrap().try_into().unwrap();
        let cond = stack.peek(1).unwrap();

        let stack1 = stack.pop().unwrap().pop().unwrap();
        let memory1 = (*memory).clone();

        let stack2 = stack1.clone();
        let memory2 = (*memory).clone();

        vec![
            // State 1:
            // - pc = destination
            // - constrain: cond != 0
            (
                stack1,
                memory1,
                dest,
                vec![cond.clone().assert_not_eq(T::default())],
            ),
            // State 2
            // - pc = pc + 1
            // - constrain: cond == 0
            (stack2, memory2, pc + 1, vec![cond.assert_eq(T::default())]),
        ]
    }
}

// use super::{ExecRecord, VMInstruction};
// use crate::{memory::Mem, stack::Stack};
// use std::fmt::Debug;

// #[derive(Debug)]
// pub struct ASSERT<T: Debug>(T);

// impl<T, S, M, PC>
//     VMInstruction<S, M> for ASSERT<T>
// where
//     T: Copy + Into<PC> + Debug,
//     S: Stack<StackVal = T>,
//     M: Mem
// {
//     fn exec(
//         &self,
//         stack: &S,
//         _memory: &M,
//     ) -> InstructionResult<ExecRecord<S, M>> {
//         let mut change_log = ExecRecord::default();

//         let constraint = stack.peek(0).unwrap().into();

//         change_log.path_constraints.push(vec![constraint]);

//         Ok(change_log)
//     }
// }
