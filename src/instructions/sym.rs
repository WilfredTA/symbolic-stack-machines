use crate::{stack::Stack, memory::Mem};
use super::{VMInstruction, ExecRecord};

pub struct ASSERT<T>(T);

impl<
        T: Copy + Into<PathConstraint>,
        ValStack: Stack<StackVal = T>,
        M: Mem,
        PathConstraint
    > VMInstruction<ValStack, M, PathConstraint> for ASSERT<T>
{
    fn exec(
        &self,
        stack: &ValStack,
        _memory: &M,
    ) -> super::InstructionResult<super::ExecRecord<ValStack, M, PathConstraint>> {
        let mut change_log = ExecRecord::default();

        let constraint = stack.peek(0).unwrap().into();

        change_log.path_constraints.push(vec![constraint]);

        Ok(change_log)
    }
}
