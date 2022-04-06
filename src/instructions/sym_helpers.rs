use crate::{
    machine_eq::MachineEq,
    memory::{Mem, ReadOnlyMem, WriteableMem},
    stack::Stack,
};

use super::{bitwise::Binary, helpers, SymbolicVMInstruction, sym};

pub fn PUSH<Arg, T, S, M, SI>(x: Arg) -> SymbolicVMInstruction<S, M, SI>
where
    Arg: Into<T>,
    // TODO this shouldn't be static
    T: Clone + 'static,
    S: Stack<StackVal = T>,
    M: Mem,
{
    helpers::PUSH(x).into()
}

pub fn STOP<S, M, SI>() -> SymbolicVMInstruction<S, M, SI>
where
    S: Stack,
    M: Mem,
{
    helpers::STOP().into()
}

pub fn MSTORE<T, S, M, SI>() -> SymbolicVMInstruction<S, M, SI>
where
    T: Clone + TryInto<M::Index>,
    M: WriteableMem<MemVal = T>,
    S: Stack<StackVal = T>,
    <T as TryInto<<M as Mem>::Index>>::Error: std::fmt::Debug,
{
    helpers::MSTORE().into()
}

pub fn MLOAD<T, S, M, SI>() -> SymbolicVMInstruction<S, M, SI>
where
    T: Clone + TryInto<M::Index>,
    M: ReadOnlyMem<MemVal = T>,
    S: Stack<StackVal = T>,
    <T as TryInto<<M as Mem>::Index>>::Error: std::fmt::Debug,
{
    helpers::MLOAD().into()
}

pub fn ADD<T, S, M, SI>() -> SymbolicVMInstruction<S, M, SI>
where
    T: std::ops::Add + std::ops::Add<Output = T> + Clone,
    S: Stack<StackVal = T>,
    M: Mem,
{
    helpers::ADD().into()
}

pub fn SUB<T, S, M, SI>() -> SymbolicVMInstruction<S, M, SI>
where
    T: std::ops::Sub + std::ops::Sub<Output = T> + Clone,
    S: Stack<StackVal = T>,
    M: Mem,
{
    helpers::SUB().into()
}

pub fn ISZERO<T, S, M, SI>() -> SymbolicVMInstruction<S, M, SI>
where
    T: Binary + MachineEq,
    S: Stack<StackVal = T>,
    M: Mem,
{
    helpers::ISZERO().into()
}

pub fn JUMPI<T, S, M>() -> SymbolicVMInstruction<S, M, sym::JUMPI>
where
    T: Default + Eq + TryInto<usize>,
    S: Stack<StackVal = T>,
    M: Mem,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
{
    SymbolicVMInstruction::S(sym::JUMPI)
}
