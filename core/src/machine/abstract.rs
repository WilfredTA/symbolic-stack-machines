use crate::{
    environment::{EnvExtension, EnvExtensionRecord},
    memory::{Mem, MemRecord, WriteableMem},
    stack::{Stack, StackRecord},
};

#[derive(Clone)]
pub struct AbstractMachine<'a, S, M, E, I>
where
    S: Stack,
    M: Mem,
    E: EnvExtension,
{
    pub stack: S,
    pub mem: M,
    pub custom_env: E,
    pub pc: Option<usize>,
    pub pgm: &'a [I],
}
// NOTE(will): For some reason, calling `.clone` directly on
// `AbstractMachine` requires that `I` implement `Clone`. `I` is behind
// a reference and shouldn't have to implement clone in order to clone
// `AbstractMachine`
impl<'a, S, M, E, I> AbstractMachine<'a, S, M, E, I>
where
    S: Stack,
    M: WriteableMem,
    E: EnvExtension,
{
    pub fn xclone(&self) -> Self {
        AbstractMachine {
            stack: self.stack.clone(),
            mem: self.mem.clone(),
            custom_env: self.custom_env.clone(),
            pc: self.pc.clone(),
            pgm: self.pgm,
        }
    }

    pub fn apply(
        self,
        stack_diff: Option<StackRecord<S>>,
        mem_diff: Option<MemRecord<M>>,
        ext_diff: Option<E::DiffRecordType>,
        pc_change: Option<usize>,
        halt: bool,
    ) -> Self {
        let mut stack = self.stack;
        let mut mem = self.mem;
        let mut ext = self.custom_env;

        stack = {
            if let Some(stack_diff) = stack_diff {
                stack_diff.apply(stack).unwrap()
            } else {
                stack
            }
        };

        mem = {
            if let Some(mem_diff) = mem_diff {
                mem_diff.apply(mem).unwrap()
            } else {
                mem
            }
        };

        ext = {
            if let Some(ext_diff) = ext_diff {
                ext_diff.apply(ext).unwrap()
            } else {
                ext
            }
        };

        let pc = if halt {
            None
        } else {
            match pc_change {
                Some(pc_change) => Some(pc_change),
                None => Some(self.pc.unwrap() + 1),
            }
        };

        AbstractMachine {
            stack,
            mem,
            custom_env: ext,
            pc,
            pgm: self.pgm,
        }
    }
}

impl<'a, S, M, E, I> AbstractMachine<'a, S, M, E, I>
where
    S: Stack,
    M: Mem,
    E: EnvExtension,
{
    pub fn can_continue(&self) -> bool {
        match self.pc {
            Some(pc) => pc < self.pgm.len(),
            None => false,
        }
    }
}
