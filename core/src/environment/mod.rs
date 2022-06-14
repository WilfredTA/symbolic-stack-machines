pub trait EnvExtension: Clone {
    type InnerValue;
    type ErrorType: std::fmt::Debug;
    type IndexType;
    type DiffRecordType: EnvExtensionRecord;

    fn write<V: Into<Self::InnerValue>>(&self, v: V) -> Result<Self, Self::ErrorType>
    where
        Self: Sized;
    fn read<I: Into<Self::IndexType>>(&self, idx: I) -> Result<Self::InnerValue, Self::ErrorType>;
}

pub trait EnvExtensionRecord: Sized {
    fn apply<E: EnvExtension>(&self, env: E) -> Result<E, E::ErrorType>;
}
