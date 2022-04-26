pub mod error;
pub mod val;
use crate::memory::*;
use crate::stack::*;
use crate::constraint::*;
use error::InstructionError;
use z3::ast::Bool;
use serde::{Serialize, Deserialize};

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
    M: RWMem,
    Ext: EnvExtension,
    C: Into<Constraint<C>>,
{
    pub stack_diff: Option<StackRecord<S>>,
    pub mem_diff: Option<MemRecord<M>>,
    pub ext_diff: Option<Ext>,
    pub pc_change: Option<usize>,
    pub halt: bool,
    constraints: Option<Vec<C>>,
}

pub trait EnvExtensionRecord {
    type Env: EnvExtension;
    type ErrorType;
    fn apply(&self, env: Self::Env) -> Result<Self::Env, Self::ErrorType>;
}

pub trait EnvExtension {
    type InnerValue;
    type ErrorType;
    type IndexType;
    type DiffRecordType: EnvExtensionRecord;
    fn write<V: Into<Self::InnerValue>>(&self, v: V) -> Result<Self, Self::ErrorType> where Self: Sized;

    fn read<I: Into<Self::IndexType>>(&self, idx: I) -> Result<Self::InnerValue, Self::ErrorType>;
}

pub trait AbstractInstruction {
    type Stack: Stack;
    type Mem: RWMem;
    type Extension: EnvExtension;
    type ReturnRecord;

    fn exec<C: Into<Constraint<C>>>(&self, stack: &Self::Stack, mem: &Self::Mem, ext: &Self::Extension) 
    -> InstructionResult<AbstractExecRecord<Self::Stack, Self::Mem, Self::Extension, C>>;
}