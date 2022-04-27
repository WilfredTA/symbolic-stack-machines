use std::ops::Not;

use crate::{
    memory::{Mem, ReadOnlyMem, WriteableMem},
    stack::Stack, instructions::{HybridVMInstruction, Binary, Constrain}, vals::MachineEq,
};

use super::{helpers, sym};

#[allow(non_snake_case)]
pub fn PUSH<Arg, T, S, M, C>(x: Arg) -> HybridVMInstruction<S, M, C>
where
    Arg: Into<T>,
    // TODO this shouldn't be static
    T: Clone + std::fmt::Debug + 'static,
    S: Stack<StackVal = T>,
    M: Mem,
{
    helpers::PUSH(x).into()
}

#[allow(non_snake_case)]
pub fn STOP<S, M, C>() -> HybridVMInstruction<S, M, C>
where
    S: Stack,
    M: Mem,
{
    helpers::STOP().into()
}

#[allow(non_snake_case)]
pub fn MSTORE<T, S, M, C>() -> HybridVMInstruction<S, M, C>
where
    T: Clone,
    S: Stack<StackVal = T>,
    M: WriteableMem<Index = T, MemVal = T>,
{
    helpers::MSTORE().into()
}

#[allow(non_snake_case)]
pub fn MLOAD<T, S, M, C>() -> HybridVMInstruction<S, M, C>
where
    T: Default + Clone,
    S: Stack<StackVal = T>,
    M: ReadOnlyMem<Index = T, MemVal = T>,
{
    helpers::MLOAD().into()
}

#[allow(non_snake_case)]
pub fn ADD<T, S, M, C>() -> HybridVMInstruction<S, M, C>
where
    T: std::ops::Add + std::ops::Add<Output = T> + Clone + std::fmt::Debug,
    S: Stack<StackVal = T>,
    M: Mem,
{
    helpers::ADD().into()
}

#[allow(non_snake_case)]
pub fn SUB<T, S, M, C>() -> HybridVMInstruction<S, M, C>
where
    T: std::ops::Sub + std::ops::Sub<Output = T> + Clone + std::fmt::Debug,
    S: Stack<StackVal = T>,
    M: Mem,
{
    helpers::SUB().into()
}

#[allow(non_snake_case)]
pub fn ISZERO<T, S, M, C>() -> HybridVMInstruction<S, M, C>
where
    T: Binary + MachineEq,
    S: Stack<StackVal = T>,
    M: Mem,
{
    helpers::ISZERO().into()
}

#[allow(non_snake_case)]
pub fn JUMPI<T, S, M, C>() -> HybridVMInstruction<S, M, C>
where
    T: TryInto<usize> + Constrain<Constraint = C> + Default + Clone,
    S: Stack<StackVal = T> + Clone,
    M: Mem + Clone,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
    // TODO this should have already specified by `T: Constrain<Constraint = C>`
    C: Not + Not<Output = C>
{
    HybridVMInstruction::S(Box::new(sym::JUMPI))
}
