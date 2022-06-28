pub mod error;
use crate::constraint::Constraint;
use crate::environment::EnvExtension;
use crate::environment::EnvExtensionRecord;
use crate::memory::*;
use crate::stack::*;
use error::InstructionError;

pub type InstructionResult<T> = Result<T, InstructionError>;

pub struct AbstractExecRecord<M, Ext, C>
where
    M: Mem,
    Ext: EnvExtensionRecord,
{
    pub stack_diff: Option<StackRecord>,
    pub mem_diff: Option<MemRecord<M>>,
    pub ext_diff: Option<Ext>,
    pub pc_change: Option<usize>,
    pub halt: bool,
    pub constraints: Option<Vec<Constraint<C>>>,
}

pub type ConcreteAbstractExecRecord<M, Ext> = AbstractExecRecord<M, Ext, ()>;

impl<M, Ext, C> Default for AbstractExecRecord<M, Ext, C>
where
    M: Mem,
    Ext: EnvExtensionRecord,
{
    fn default() -> Self {
        Self {
            stack_diff: None,
            mem_diff: None,
            ext_diff: None,
            pc_change: None,
            halt: false,
            constraints: None,
        }
    }
}

pub trait AbstractInstruction<M, Extension, StepResult>
where
    M: Mem,
    Extension: EnvExtension,
{
    fn exec(&self, stack: &Stack, mem: &M, ext: &Extension) -> InstructionResult<StepResult>;
}
