use crate::stack::StackVal;

pub struct MemRecord {
    pub changed: Vec<MemOpRecord>,
}

pub enum MemOpRecord {
    Write(StackVal, StackVal),
}
