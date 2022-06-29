// TODO(will) - ultimately this will be replaced with a more general value
// that implements an AST, etc...

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct StackVal(u64);

pub static ZERO: StackVal = StackVal(0);
pub static ONE: StackVal = StackVal(1);

static FALSE: StackVal = ZERO;
static TRUE: StackVal = ONE;

impl From<u64> for StackVal {
    fn from(x: u64) -> Self {
        Self(x)
    }
}

impl Into<u64> for StackVal {
    fn into(self) -> u64 {
        self.0
    }
}

impl Into<usize> for StackVal {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl StackVal {
    pub fn _eq(&self, other: &Self) -> Self {
        if self == other {
            TRUE
        } else {
            FALSE
        }
    }

    pub fn ite(&self, then: Self, xelse: Self) -> Self {
        if *self == TRUE {
            then
        } else {
            xelse
        }
    }
}

impl std::ops::Add for StackVal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for StackVal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
