use std::collections::HashMap;

use crate::{stack::Stack, memory::WriteableMem, environment::EnvExtension};

use super::r#abstract::AbstractMachine;

pub type SingleBranch<'a, S, M, E, I, C> = (AbstractMachine<'a, S, M, E, I>, Vec<C>);

pub type AbstractExecBranch<'a, S, M, E, I, C> = Vec<SingleBranch<'a, S, M, E, I, C>>;

pub struct SymbolicMachineResults<'a, S, M, E, I, C, SolverResult> 
where
    S: Stack,
    M: WriteableMem,
    E: EnvExtension,
{
    pub reachable: Vec<ReachableSymbolicMachineResult<'a, S, M, E, I, C, SolverResult>>,
    pub unreachable: Vec<UnreachableSymbolicMachineResult<'a, S, M, E, I, C>>,
}

pub struct ReachableSymbolicMachineResult<'a, S, M, E, I, C, SolverResult>
where
    S: Stack,
    M: WriteableMem,
    E: EnvExtension,
{
    pub machine: AbstractMachine<'a, S, M, E, I>,
    pub constraints: Vec<C>,
    pub model: HashMap<String, SolverResult>,
}

pub struct UnreachableSymbolicMachineResult<'a, S, M, E, I, C>
where
    S: Stack,
    M: WriteableMem,
    E: EnvExtension,
{
    pub machine: AbstractMachine<'a, S, M, E, I>,
    pub constraints: Vec<C>,
}
