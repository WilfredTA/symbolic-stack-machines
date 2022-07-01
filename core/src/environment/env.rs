use super::record::EnvRecord;

#[derive(Clone)]
pub struct Env {}

impl Env {
    pub fn apply(&self, _r: EnvRecord) -> Self {
        self.clone()
    }
}
