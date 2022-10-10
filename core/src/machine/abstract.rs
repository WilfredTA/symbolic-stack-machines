use crate::{
    environment::{Env, EnvRecord},
    memory::{MemRecord, Memory},
    stack::{Stack, StackRecord},
};

#[derive(Clone)]
pub struct AbstractMachine<'a, I, E: Env, V: Default + Clone> {
    pub stack: Stack,
    pub mem: Memory<V>,
    pub env: E,
    pub pc: Option<usize>,
    pub pgm: &'a [I],
}
// NOTE(will): For some reason, calling `.clone` directly on
// `AbstractMachine` requires that `I` implement `Clone`. `I` is behind
// a reference and shouldn't have to implement clone in order to clone
// `AbstractMachine`
impl<'a, I, E: Env, V: Default + Clone> AbstractMachine<'a, I, E, V> {
    pub fn xclone(&self) -> Self {
        AbstractMachine {
            stack: self.stack.clone(),
            mem: self.mem.clone(),
            env: self.env.clone(),
            pc: self.pc,
            pgm: self.pgm,
        }
    }

    pub fn apply<ER>(
        self,
        stack_diff: Option<StackRecord>,
        mem_diff: Option<MemRecord>,
        env_diff: Option<ER>,
        pc_change: Option<usize>,
        halt: bool,
    ) -> Self 
    where E: Env<RecordType = ER>
    {
        let mut stack = self.stack;
        let mut mem = self.mem;
        let mut env = self.env;

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

        env = {
            if let Some(env_diff) = env_diff {
                env.apply(env_diff)
            } else {
                env
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
            env,
            pc,
            pgm: self.pgm,
        }
    }
}

impl<'a, I, E: Env, V: Default + Clone> AbstractMachine<'a, I, E, V> {
    pub fn can_continue(&self) -> bool {
        match self.pc {
            Some(pc) => pc < self.pgm.len(),
            None => false,
        }
    }
}
