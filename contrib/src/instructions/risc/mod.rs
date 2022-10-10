use symbolic_stack_machines_core::{
    environment::Env,
    instructions::{AbstractExecRecord, AbstractInstruction, InstructionResult},
    memory::Memory,
    stack::{Stack, StackOpRecord, StackRecord},
};

type RegisterValue = [u8; 4];
#[derive(Clone)]
pub struct CpuEnv {
    pub registers: [RegisterValue; 32],
}

impl CpuEnv {

}


pub struct CpuRecord {
    pub reg_idx: usize,
    pub new_reg_val: [u8; 4],
}

impl Env for CpuEnv {
    type RecordType = Vec<CpuRecord>;

    fn apply(&self, record: Self::RecordType) -> Self {
        todo!()
    }
}

pub struct SW;

impl AbstractInstruction<AbstractExecRecord> for SW {
    fn exec(&self, stack: &Stack, mem: &Memory, env: &Env) -> InstructionResult<AbstractExecRecord> {
        
    }
}