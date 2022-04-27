use std::ops::Not;

use crate::{
    instructions::{symbolic::SymbolicVMInstruction, Constrain},
    memory::Mem,
    stack::Stack,
};

#[derive(Debug)]
pub struct JUMPI;

impl<T, S, M, C> SymbolicVMInstruction<S, M, C> for JUMPI
where
    T: TryInto<usize> + Constrain<Constraint = C> + Default + Clone,
    S: Stack<StackVal = T> + Clone,
    M: Mem + Clone,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
    // TODO this should have already specified by `T: Constrain<Constraint = C>`
    C: Not + Not<Output = C>
{
    fn sym_exec(&self, stack: &S, memory: &M, pc: usize) -> Vec<(S, M, usize, Vec<C>)> {
        let dest: T = stack.peek(0).unwrap();
        let dest = dest.try_into().unwrap();
        let cond: T = stack.peek(1).unwrap();

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
                vec![cond.clone().assert_eq(T::default()).not()],
            ),
            // State 2
            // - pc = pc + 1
            // - constrain: cond == 0
            (stack2, memory2, pc + 1, vec![cond.assert_eq(T::default())]),
        ]
    }
}
