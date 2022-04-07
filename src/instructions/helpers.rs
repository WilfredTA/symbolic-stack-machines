use crate::{
    machine_eq::MachineEq,
    memory::{Mem, ReadOnlyMem, WriteableMem},
    stack::Stack,
};

use super::{
    arith,
    bitwise::{self, Binary},
    misc, DynConcreteVMInstruction,
};

pub fn PUSH<Arg, T, S, M>(x: Arg) -> DynConcreteVMInstruction<S, M>
where
    Arg: Into<T>,
    // TODO this shouldn't be static
    T: Clone + std::fmt::Debug + 'static,
    S: Stack<StackVal = T>,
    M: Mem,
{
    Box::new(misc::PUSH(x.into()))
}

pub fn STOP<S, M>() -> DynConcreteVMInstruction<S, M>
where
    S: Stack,
    M: Mem,
{
    Box::new(misc::STOP)
}

pub fn MSTORE<T, S, M>() -> DynConcreteVMInstruction<S, M>
where
    T: Clone + std::fmt::Debug + TryInto<M::Index>,
    M: WriteableMem<MemVal = T>,
    S: Stack<StackVal = T>,
    <T as TryInto<<M as Mem>::Index>>::Error: std::fmt::Debug,
{
    Box::new(misc::MSTORE)
}

pub fn MLOAD<T, S, M>() -> DynConcreteVMInstruction<S, M>
where
    T: Clone + std::fmt::Debug + TryInto<M::Index>,
    M: ReadOnlyMem<MemVal = T>,
    S: Stack<StackVal = T>,
    <T as TryInto<<M as Mem>::Index>>::Error: std::fmt::Debug,
{
    Box::new(misc::MLOAD)
}

pub fn ADD<T, S, M>() -> DynConcreteVMInstruction<S, M>
where
    T: std::ops::Add + std::ops::Add<Output = T> + Clone + std::fmt::Debug,
    S: Stack<StackVal = T>,
    M: Mem,
{
    Box::new(arith::ADD)
}

pub fn SUB<T, S, M>() -> DynConcreteVMInstruction<S, M>
where
    T: std::ops::Sub + std::ops::Sub<Output = T> + Clone + std::fmt::Debug,
    S: Stack<StackVal = T>,
    M: Mem,
{
    Box::new(arith::SUB)
}

pub fn ISZERO<T, S, M>() -> DynConcreteVMInstruction<S, M>
where
    T: Binary + MachineEq,
    S: Stack<StackVal = T>,
    M: Mem,
{
    Box::new(bitwise::ISZERO)
}

pub fn JUMPI<T, S, M>() -> DynConcreteVMInstruction<S, M>
where
    T: Default + Eq + TryInto<usize>,
    S: Stack<StackVal = T>,
    M: Mem,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
{
    Box::new(misc::JUMPI)
}

pub fn UNREACHABLE<S, M>() -> DynConcreteVMInstruction<S, M>
where
    S: Stack,
    M: Mem,
{
    Box::new(misc::UNREACHABLE)
}
