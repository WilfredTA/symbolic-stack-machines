use crate::{
    memory::{Mem, ReadOnlyMem, WriteableMem},
    stack::Stack, instructions::{DynConcreteVMInstruction, Binary}, vals::MachineEq,
};

use super::{arith, bitwise, misc};

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
pub fn STOP<S, M>() -> DynConcreteVMInstruction<S, M>
where
    S: Stack,
    M: Mem,
{
    Box::new(misc::STOP)
}

#[allow(non_snake_case)]
pub fn MSTORE<T, S, M>() -> DynConcreteVMInstruction<S, M>
where
    T: Clone,
    S: Stack<StackVal = T>,
    M: WriteableMem<Index = T, MemVal = T>,
{
    Box::new(misc::MSTORE)
}

#[allow(non_snake_case)]
pub fn MLOAD<T, S, M>() -> DynConcreteVMInstruction<S, M>
where
    T: Default + Clone,
    S: Stack<StackVal = T>,
    M: ReadOnlyMem<Index = T, MemVal = T>,
{
    Box::new(misc::MLOAD)
}

#[allow(non_snake_case)]
pub fn ADD<T, S, M>() -> DynConcreteVMInstruction<S, M>
where
    T: std::ops::Add + std::ops::Add<Output = T> + Clone + std::fmt::Debug,
    S: Stack<StackVal = T>,
    M: Mem,
{
    Box::new(arith::ADD)
}

#[allow(non_snake_case)]
pub fn SUB<T, S, M>() -> DynConcreteVMInstruction<S, M>
where
    T: std::ops::Sub + std::ops::Sub<Output = T> + Clone + std::fmt::Debug,
    S: Stack<StackVal = T>,
    M: Mem,
{
    Box::new(arith::SUB)
}

#[allow(non_snake_case)]
pub fn ISZERO<T, S, M>() -> DynConcreteVMInstruction<S, M>
where
    T: Binary + MachineEq,
    S: Stack<StackVal = T>,
    M: Mem,
{
    Box::new(bitwise::ISZERO)
}

#[allow(non_snake_case)]
pub fn JUMPI<T, S, M>() -> DynConcreteVMInstruction<S, M>
where
    T: Binary + Eq + TryInto<usize>,
    S: Stack<StackVal = T>,
    M: Mem,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
{
    Box::new(misc::JUMPI)
}
