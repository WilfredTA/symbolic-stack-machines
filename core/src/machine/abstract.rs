use crate::{instructions::{AbstractInstruction, InstructionResult, error::InstructionError, EnvExtension}, stack::Stack};
use crate::memory::{RWMem, ReadOnlyMem};
use crate::constraint::{AbstractConstraintValue, CmpType, Constraint};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AbstractMachineError {
    #[error(transparent)]
    InstructionError(#[from] InstructionError),
}

pub type MachineResult<T> = Result<T, AbstractMachineError>;
pub struct AbstractExecBranch<'mach, IType, T> 
where IType: AbstractInstruction
{
    l: &'mach AbstractMachine<IType>,
    r: &'mach AbstractMachine<IType>,
    condition: Constraint<T>
}

#[derive(Default)]
pub struct PathSummary<'mach, IType, T> 
where IType: AbstractInstruction
{
    pub reachable: Vec<AbstractExecBranch<'mach, IType, T>>,
    pub unreachable: Vec<AbstractExecBranch<'mach, IType, T>>
}
#[derive(Clone)]
pub struct AbstractMachine<IType: AbstractInstruction> {
    pub pgm: Vec<IType>,
    pub stack: IType::Stack,
    pub mem: IType::Mem,
    pub custom_env: IType::Extension,
    pub pc: usize,

}

impl<I, S, M, SVal, MIdx, MVal, Ext> AbstractMachine<I>
where
    SVal: Into<MIdx> + Into<MVal>,
    S: Stack<StackVal = SVal>,
    M: RWMem + ReadOnlyMem<MemVal = MVal, Index = MIdx>,
    I: AbstractInstruction<Stack = S, Mem = M, Extension = Ext>,
    Ext: EnvExtension
{
    pub fn exec<C: Into<Constraint<C>>>(&self) -> MachineResult<PathSummary<'_, I, C>> {
        let mut tree: Vec<AbstractExecBranch<'_, I, C>> = vec![];

        
        for i in &self.pgm {
            let ret = i.exec::<C>(&self.stack,&self.mem, &self.custom_env)?;
            

        }

        Ok(PathSummary {
            reachable: vec![],
            unreachable: vec![]
        })
    }
}
