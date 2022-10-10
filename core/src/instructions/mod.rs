pub mod error;
use crate::constraint::Constraint;
use crate::environment::Env;
use crate::environment::EnvRecord;
use crate::memory::*;
use crate::stack::*;
use error::InstructionError;

pub type InstructionResult<T> = Result<T, InstructionError>;

pub struct AbstractExecRecord<E: Env> {
    pub stack_diff: Option<StackRecord>,
    pub mem_diff: Option<MemRecord>,
    pub env_diff: Option<E::RecordType>,
    pub pc_change: Option<usize>,
    pub halt: bool,
    pub constraints: Option<Vec<Constraint>>,
}

impl<E: Env> Default for AbstractExecRecord<E>{
    fn default() -> Self {
        Self {
            stack_diff: None,
            mem_diff: None,
            env_diff: None,
            pc_change: None,
            halt: false,
            constraints: None,
        }
    }
}

pub trait AbstractInstruction<StepResult, E: Env, V: Default + Clone> {
    fn exec(&self, stack: &Stack, mem: &Memory<V>, env: &E) -> InstructionResult<StepResult>;
}
