use super::SymbolId;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vecc(pub Vec<u8>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolicVecc(pub SymbolId<Vecc>);

impl<T> From<T> for Vecc
where T: Into<Vec<u8>>
{
    fn from(v: T) -> Self {
        Vecc(v.into())
    }
}
