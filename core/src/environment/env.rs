use super::record::EnvRecord;


pub trait Env: Clone {
    type RecordType;
    fn apply(&self, record: Self::RecordType) -> Self;
}

#[derive(Clone)]
pub struct DefaultEnv {

}

impl Env for DefaultEnv {
    type RecordType = ();

    fn apply(&self, record: Self::RecordType) -> Self {
        self.clone()
    }
}