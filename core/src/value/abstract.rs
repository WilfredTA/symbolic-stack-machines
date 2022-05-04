


#[derive(Clone)]
pub struct AbstractValue<T> {
    symbol: String,
    val: T
}

#[derive(Clone, Default)]
pub struct AbstractInt {
    concrete: Option<u64>
}

impl AbstractInt {
    pub fn inner(&self) -> Option<u64> {
        self.concrete.clone()
    }
}

impl From<u64> for AbstractInt {
    fn from(v: u64) -> Self {
        Self {
            concrete: Some(v)
        }
    }
}
pub type Val<T> = AbstractValue<T>;

impl<T: Clone> AbstractValue<T> {
    pub fn new(val: T, symbol: String) -> Self {
        Self {symbol, val}
    }
    pub fn inner<V: From<T>>(&self) -> V {
        self.val.clone().into()
    }

    pub fn id(&self) -> &str {
        self.symbol.as_str()
    }

    pub fn set_val(&mut self, new_val: T) {
        self.val = new_val;
    }

    pub fn set_symbol(&mut self, new_symbol: String) {
        self.symbol = new_symbol;
    }
}

