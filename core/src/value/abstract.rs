use super::inner::*;
use rand::Rng;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct AbstractValue<T> {
    symbol: Option<String>,
    val: T,
}

// To do: Transpile like so:
// InnerValue (and all sub types) implement Into<smt_val> (e.g., Booleans implement Into<z3::ast::Bool>)

impl<T> AsRef<T> for AbstractValue<T> {
    fn as_ref(&self) -> &T {
        &self.val
    }
}
impl<T: Clone> AbstractValue<T> {
    pub fn new(val: impl Into<T>, symbol: Option<String>) -> Self {
        Self {
            symbol,
            val: val.into(),
        }
    }
    pub fn inner(&self) -> T {
        self.val.clone().into()
    }

    pub fn id(&self) -> &str {
        if let Some(ref symb) = self.symbol {
            symb.as_str()
        } else {
            ""
        }
    }

    pub fn set_val(&mut self, new_val: T) {
        self.val = new_val;
    }

    pub fn set_symbol(&mut self, new_symbol: String) {
        self.symbol = Some(new_symbol);
    }
}

pub type AbstractInt = AbstractValue<Option<u64>>;

// Val is the universal value type
pub type Val = AbstractValue<InnerValue>;

// TODO(tannr): impl Mul
impl Deref for AbstractValue<InnerValue> {
    type Target = InnerValue;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl<T: Clone> From<T> for AbstractValue<T> {
    fn from(v: T) -> Self {
        AbstractValue::new(v, Some(random_val_symbol()))
    }
}

impl<T> std::ops::Add for AbstractValue<T>
where
    T: std::ops::Add + std::ops::Add<Output = T> + Clone,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let new_val: T = self.inner() + rhs.inner();
        Self::new(new_val, None)
    }
}

impl<T> std::ops::Sub for AbstractValue<T>
where
    T: std::ops::Sub + std::ops::Sub<Output = T> + Clone,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let new_val: T = self.inner() - rhs.inner();
        Self::new(new_val, None)
    }
}

impl<T> std::ops::Div for AbstractValue<T>
where
    T: std::ops::Div + std::ops::Div<Output = T> + Clone,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let new_val: T = self.inner() / rhs.inner();
        Self::new(new_val, None)
    }
}

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz";
const SYMBOL_LEN: usize = 10;

fn random_val_symbol() -> String {
    let mut rng = rand::thread_rng();

    let symbol: String = (0..SYMBOL_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    symbol
}
