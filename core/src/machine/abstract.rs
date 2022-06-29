use crate::{
    environment::{EnvExtension, EnvExtensionRecord},
    memory::{Memory, MemRecord},
    stack::{Stack, StackRecord},
};

#[derive(Clone)]
pub struct AbstractMachine<'a, E, I>
where
    E: EnvExtension,
{
    pub stack: Stack,
    pub mem: Memory,
    pub custom_env: E,
    pub pc: Option<usize>,
    pub pgm: &'a [I],
}
// NOTE(will): For some reason, calling `.clone` directly on
// `AbstractMachine` requires that `I` implement `Clone`. `I` is behind
// a reference and shouldn't have to implement clone in order to clone
// `AbstractMachine`
impl<'a, E, I> AbstractMachine<'a, E, I>
where
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
        stack_diff: Option<StackRecord>,
        mem_diff: Option<MemRecord>,
        ext_diff: Option<E::DiffRecordType>,
        pc_change: Option<usize>,
        halt: bool,
    ) -> Self {
        let mut stack = self.stack;
        let mut mem = self.mem;
        let mut ext = self.custom_env;

        stack = {
            if let Some(stack_diff) = stack_diff {
                stack.apply(stack_diff)
            } else {
                stack
            }
        };

        mem = {
            if let Some(mem_diff) = mem_diff {
                mem.apply(mem_diff)
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

impl<'a, E, I> AbstractMachine<'a, E, I>
where
    E: EnvExtension,
{
    pub fn can_continue(&self) -> bool {
        match self.pc {
            Some(pc) => pc < self.pgm.len(),
            None => false,
        }
    }
}
