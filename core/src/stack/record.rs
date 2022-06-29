use super::StackVal;

#[derive(Clone)]
pub struct StackRecord {
    pub changed: Vec<StackOpRecord>,
}

#[derive(Clone)]
pub enum StackOpRecord {
    Pop,
    Push(StackVal),
}
