pub mod error;
pub mod val;
use crate::constraint::Constraint;
use crate::environment::EnvExtension;
use crate::environment::EnvExtensionRecord;
use crate::memory::*;
use crate::stack::*;
use error::InstructionError;
use z3::ast::Bool;

pub type InstructionResult<T> = Result<T, InstructionError>;

pub struct ExecRecord<'a, S, M>
where
    M: WriteableMem,
    S: Stack,
{
    pub stack_diff: Option<StackRecord<S>>,
    pub mem_diff: Option<MemRecord<M>>,
    // Each inner vec represents a new path in the program
    pub path_constraints: Vec<Vec<Bool<'a>>>,
    pub pc_change: Option<usize>,
    pub halt: bool,
}

pub trait VMInstruction<'a> {
    type ValStack: Stack;
    type Mem: RWMem;

    fn exec(
        &self,
        stack: &Self::ValStack,
        memory: &Self::Mem,
    ) -> InstructionResult<ExecRecord<'a, Self::ValStack, Self::Mem>>;
}

pub struct AbstractExecRecord<S, M, Ext, C>
where
    S: Stack,
    M: Mem,
    Ext: EnvExtensionRecord,
{
    pub stack_diff: Option<StackRecord<S>>,
    pub mem_diff: Option<MemRecord<M>>,
    pub ext_diff: Option<Ext>,
    pub pc_change: Option<usize>,
    pub halt: bool,
    pub constraints: Option<Vec<Constraint<C>>>,
}

pub type ConcreteAbstractExecRecord<S, M, Ext> = AbstractExecRecord<S, M, Ext, ()>;

impl<S, M, Ext, C> Default for AbstractExecRecord<S, M, Ext, C>
where
    S: Stack,
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

pub trait AbstractInstruction<S, M, Extension, StepResult>
where
    S: Stack,
    M: Mem,
    Extension: EnvExtension,
{
    fn exec(&self, stack: &S, mem: &M, ext: &Extension) -> InstructionResult<StepResult>;
}
