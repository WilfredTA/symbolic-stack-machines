pub mod error;
use error::StackError;

pub type StackResult<T> = Result<T, StackError>;

pub trait Stack: Sized + Clone {
    type StackVal;
    fn push<V: Into<Self::StackVal>>(&self, v: V) -> StackResult<Self>;
    fn pop(&self) -> StackResult<Self>;

    fn peek<V: From<Self::StackVal>>(&self, idx: usize) -> Option<V>;
}

#[derive(Clone)]
pub enum StackOpRecord<T> {
    Pop(T),
    Push(T),
}

#[derive(Clone)]
pub struct StackRecord<S: Stack> {
    pub changed: Vec<StackOpRecord<S::StackVal>>,
}

impl<S> StackRecord<S>
where
    S: Stack,
{
    pub fn new<V>(changes: Vec<StackOpRecord<V>>) -> Self
    where
        V: Into<S::StackVal>,
    {
        Self {
            changed: changes
                .into_iter()
                .map(|v| match v {
                    StackOpRecord::Pop(v) => StackOpRecord::Pop(v.into()),
                    StackOpRecord::Push(v) => StackOpRecord::Push(v.into()),
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn apply(self, stack: S) -> StackResult<S> {
        self.changed
            .into_iter()
            .fold(Ok(stack), |cur_stack: StackResult<S>, record| {
                match cur_stack {
                    Ok(s) => {
                        match record {
                            StackOpRecord::Pop(_v) => {
                                // Assert that pop() == v?
                                s.pop()
                            }
                            StackOpRecord::Push(v) => s.push(v),
                        }
                    }
                    Err(e) => Err(e),
                }
            })
    }
}

#[derive(Clone, Debug)]
pub struct BaseStack<T>(Vec<T>);

impl<T> BaseStack<T> {
    pub fn init() -> Self {
        Self(vec![])
    }
}

impl<T> Stack for BaseStack<T>
where
    T: Clone,
{
    type StackVal = T;
    fn push<V>(&self, val: V) -> StackResult<Self>
    where
        V: Into<Self::StackVal>,
    {
        let mut new_self = self.clone();
        new_self.0.push(val.into());
        Ok(new_self)
    }

    fn pop(&self) -> StackResult<Self> {
        let mut new_self = self.clone();
        new_self.0.pop();
        Ok(new_self)
    }

    fn peek<V>(&self, idx: usize) -> Option<V>
    where
        V: From<Self::StackVal>,
    {
        let last_idx = self.0.len() - 1;
        let get_idx = last_idx - idx;
        self.0.get(get_idx).cloned().map(|val| val.into())
    }
}
