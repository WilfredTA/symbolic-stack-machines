use super::env::Env;
pub struct EnvRecord {}

impl EnvRecord {
    pub fn create<E, R>() -> R 
    where 
        R: Default + Sized
    {
        Default::default()        
    }
}
