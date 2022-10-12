use symbolic_stack_machines_core::{
    instructions::{AbstractExecRecord, AbstractInstruction, InstructionResult},
    memory::{Memory},
    stack::{Stack, StackOpRecord, StackRecord}, environment::Env,
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
    type RecordType = CpuRecord;

    fn apply(&self, record: Self::RecordType) -> Self {
        
        let CpuRecord { reg_idx, new_reg_val } = record;
        let mut new_self = self.clone();
        new_self.registers[reg_idx] = new_reg_val;
        new_self
    }
}

pub struct SW;

impl AbstractInstruction<AbstractExecRecord<CpuEnv>, CpuEnv, u64> for SW {
    fn exec(&self, stack: &Stack, mem: &Memory<u64>, env: &CpuEnv) -> InstructionResult<AbstractExecRecord<CpuEnv>> {
        todo!()
    }
}