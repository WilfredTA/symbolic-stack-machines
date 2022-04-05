use super::{ExecRecord, VMInstruction};
use crate::{memory::Mem, stack::Stack};
use std::fmt::Debug;

#[derive(Debug)]
pub struct ASSERT<T: Debug>(T);

impl<T, S, M, PC>
    VMInstruction<S, M, PC> for ASSERT<T>
where
    T: Copy + Into<PC> + Debug,
    S: Stack<StackVal = T>,
    M: Mem
{
    fn exec(
        &self,
        stack: &S,
        _memory: &M,
    ) -> super::InstructionResult<super::ExecRecord<S, M, PC>> {
        let mut change_log = ExecRecord::default();

        let constraint = stack.peek(0).unwrap().into();

        change_log.path_constraints.push(vec![constraint]);

        Ok(change_log)
    }
}
