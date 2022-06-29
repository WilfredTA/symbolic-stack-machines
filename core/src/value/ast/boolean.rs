
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

impl From<Bool> for bool {
    fn from(b: Bool) -> Self {
        match b {
            Bool::True => true,
            Bool::False => false 
        }
    }
}