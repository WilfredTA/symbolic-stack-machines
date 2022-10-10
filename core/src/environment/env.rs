use super::record::EnvRecord;


pub trait Env: Clone {
    type RecordType;
    fn apply(&self, record: Self::RecordType) -> Self;
}