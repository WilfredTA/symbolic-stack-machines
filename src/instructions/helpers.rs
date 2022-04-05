use crate::{stack::Stack, memory::{Mem, WriteableMem, ReadOnlyMem}};

use super::{VMInstruction, misc, arith, bitwise::{self, Binary}};

pub fn PUSH<Arg, T, S, M, PC>(x: Arg) -> Box<dyn VMInstruction<S, M, PC>>
where
    Arg: Into<T>,
    // TODO this shouldn't be static
    T: Copy + 'static,
    S: Stack<StackVal = T>,
    M: Mem,
{
    Box::new(misc::PUSH(x.into()))
}

pub fn STOP<S, M, PC>() -> Box<dyn VMInstruction<S, M, PC>>
where
    S: Stack,
    M: Mem,
{
    Box::new(misc::STOP)
}

pub fn MSTORE<T, S, M, PC>() -> Box<dyn VMInstruction<S, M, PC>>
where
    T: Copy + TryInto<M::Index>,
    M: WriteableMem<MemVal = T>,
    S: Stack<StackVal = T>,
    <T as TryInto<<M as Mem>::Index>>::Error: std::fmt::Debug,
{
    Box::new(misc::MSTORE)
}

pub fn MLOAD<T, S, M, PC>() -> Box<dyn VMInstruction<S, M, PC>>
where
    T: Copy + TryInto<M::Index>,
    M: ReadOnlyMem<MemVal = T>,
    S: Stack<StackVal = T>,
    <T as TryInto<<M as Mem>::Index>>::Error: std::fmt::Debug,
{
    Box::new(misc::MLOAD)
}

pub fn ADD<T, S, M, PC>() -> Box<dyn VMInstruction<S, M, PC>> 
where
    T: std::ops::Add + std::ops::Add<Output = T> + Copy,
    S: Stack<StackVal = T>,
    M: Mem,
{
    Box::new(arith::ADD)
}

pub fn SUB<T, S, M, PC>() -> Box<dyn VMInstruction<S, M, PC>> 
where
    T: std::ops::Sub + std::ops::Sub<Output = T> + Copy,
    S: Stack<StackVal = T>,
    M: Mem,
{
    Box::new(arith::SUB)
}

pub fn ISZERO<T, S, M, PC>() -> Box<dyn VMInstruction<S, M, PC>> 
where
    T: Eq + Binary,
    S: Stack<StackVal = T>,
    M: Mem
{
    Box::new(bitwise::ISZERO)
}

pub fn JUMPI<T, S, M, PC>() -> Box<dyn VMInstruction<S, M, PC>> 
where
    T: Default + Eq + TryInto<usize>,
    S: Stack<StackVal = T>,
    M: Mem,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
{
    Box::new(misc::JUMPI)
}