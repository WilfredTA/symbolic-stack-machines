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

use crate::{
    memory::Mem, stack::Stack,
};

use super::SymbolicVMInstruction;

pub struct JUMPI;

impl<S, M> SymbolicVMInstruction<S, M> for JUMPI
where
    S: Stack,
    M: Mem,
{
    fn sym_exec(&self, stack: &S, memory: &M, pc: usize) -> Vec<(S, M, usize)> {
        todo!()
    }
}
