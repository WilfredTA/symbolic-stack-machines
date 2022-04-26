use std::borrow::Borrow;
pub use std::rc::Rc;
#[derive(Clone)]
pub enum Constraint<V> {
    Assert(V),
    Not(Rc<Constraint<V>>),
    And(Rc<Constraint<V>>, Rc<Constraint<V>>),
    BinCmp(CmpType, Rc<Constraint<V>>, Rc<Constraint<V>>),
    Ite(Rc<Constraint<V>>, V, V)
}

impl<V> Constraint<V> 
where V: Clone
{
    fn solve<C, M>(&self) -> SatResult<M> 
    where
        C: Constrained<Model = M> + From<Self>
        
    {
        C::from(self.clone()).check()
    } 
}
#[derive(Clone)]
pub enum CmpType {
  GT,
  LT,
  GTE,
  LTE,
  EQ,
  NEQ
}

#[derive(Clone)]
pub struct AbstractConstraintValue<T>(T);
impl<T: Clone> AbstractConstraintValue<T> {
    fn new(t: T) -> Self {
        Self(t)
    }
    fn read_inner<V: From<T>>(&self) -> V {
        self.clone().0.into()
    }
}

pub enum SatResult<M> {
    Sat(M),
    Unsat
}

pub trait Constrained {
    type Model;
    
    fn check(&self) -> SatResult<Self::Model>;
    
}