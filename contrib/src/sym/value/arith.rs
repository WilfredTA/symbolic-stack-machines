use std::rc::Rc;

use super::val::Value;

impl<T> std::ops::Add for Value<T>
where
    T: std::ops::Add + std::ops::Add<Output = T>,
{
    type Output = Value<T>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::C(x), Value::C(y)) => Self::C(x + y),
            (x, y) => Self::Add(Rc::new(x), Rc::new(y))
        }
    }
}

impl<T> std::ops::Sub for Value<T>
where
    T: std::ops::Sub + std::ops::Sub<Output = T>,
{
    type Output = Value<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::C(x), Value::C(y)) => Self::C(x - y),
            (x, y) => Self::Sub(Rc::new(x), Rc::new(y))
        }
    }
}
