use crate::{
    machine_eq::MachineEq,
    memory::{Mem, ReadOnlyMem, WriteableMem},
    stack::Stack,
};

use super::{bitwise::Binary, helpers, HybridVMInstruction, sym::{self, Constrain}};

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

pub fn STOP<S, M, C>() -> HybridVMInstruction<S, M, C>
where
    S: Stack,
    M: Mem,
{
    helpers::STOP().into()
}

pub fn MSTORE<T, S, M, C>() -> HybridVMInstruction<S, M, C>
where
    T: Clone + std::fmt::Debug + TryInto<M::Index>,
    M: WriteableMem<MemVal = T>,
    S: Stack<StackVal = T>,
    <T as TryInto<<M as Mem>::Index>>::Error: std::fmt::Debug,
{
    helpers::MSTORE().into()
}

pub fn MLOAD<T, S, M, C>() -> HybridVMInstruction<S, M, C>
where
    T: Clone + std::fmt::Debug + TryInto<M::Index>,
    M: ReadOnlyMem<MemVal = T>,
    S: Stack<StackVal = T>,
    <T as TryInto<<M as Mem>::Index>>::Error: std::fmt::Debug,
{
    helpers::MLOAD().into()
}

pub fn ADD<T, S, M, C>() -> HybridVMInstruction<S, M, C>
where
    T: std::ops::Add + std::fmt::Debug + std::ops::Add<Output = T> + Clone,
    S: Stack<StackVal = T>,
    M: Mem,
{
    helpers::ADD().into()
}

pub fn SUB<T, S, M, C>() -> HybridVMInstruction<S, M, C>
where
    T: std::ops::Sub + std::fmt::Debug + std::ops::Sub<Output = T> + Clone,
    S: Stack<StackVal = T>,
    M: Mem,
{
    helpers::SUB().into()
}

pub fn ISZERO<T, S, M, C>() -> HybridVMInstruction<S, M, C>
where
    T: Binary + MachineEq,
    S: Stack<StackVal = T>,
    M: Mem,
{
    helpers::ISZERO().into()
}

pub fn JUMPI<T, S, M, C>() -> HybridVMInstruction<S, M, C>
where
    T: TryInto<usize> + Constrain<Constraint = C> + Default + Clone,
    S: Stack<StackVal = T> + Clone,
    M: Mem + Clone,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
{
    HybridVMInstruction::S(Box::new(sym::JUMPI))
}
