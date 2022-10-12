use crate::value::Sentence;

// TODO(will) - ultimately this will be replaced with the more general value type
// that implements an ast
#[derive(Clone, Default)]
pub struct MemVal(pub Sentence);

#[derive(Clone, Copy)]
pub struct ByteVal(u8);
// impl Into<u8> for MemVal {
//     fn into(self) -> u8 {
//         self.0
//     }
// }

// impl From<u8> for MemVal {
//     fn from(x: u8) -> Self {
//         MemVal(x)
//     }
// }
// impl From<u64> for MemVal {
//     fn from(v: u64) -> Self {
//         Self(v as u8)
//     }
// }

pub struct Word {
    size: usize,
}