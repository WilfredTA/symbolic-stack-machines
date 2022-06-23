
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Bool {
    True,
    False
}

impl From<bool> for Bool {
    fn from(b: bool) -> Self {
        match b {
            true => Bool::True,
            false => Bool::False
        }
    }
}