// TODO(will) - ultimately this will be replaced with the more general value type
// that implements an ast

#[derive(Clone, Copy)]
pub struct MemVal(u8);

impl Into<u8> for MemVal {
    fn into(self) -> u8 {
        self.0
    }
}

impl From<u8> for MemVal {
    fn from(x: u8) -> Self {
        MemVal(x)
    }
}
