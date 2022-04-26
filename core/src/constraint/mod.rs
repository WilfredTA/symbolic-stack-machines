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
        C: Solver<Model = M> + From<Self>
        
    {
        C::from(self.clone()).check()
    }

    fn assert(v: V) -> Self {
        Self::Assert(v)
    }

    fn and(self, c: Constraint<V>) -> Self {
        Self::And(Rc::new(self), Rc::new(c))
    }
    // to do rest
}

pub struct ConstraintSolver<S: Solver, V> {
    pub constraints: Vec<Constraint<V>>,
    pub solver: S
}

pub trait Solver: Constrained {
    fn assert<V>(&mut self, constraint: &Constraint<V>);
    fn solve<V>(&self) -> SatResult<Self::Model>;
}

// impl<S,V> Constrained for ConstraintSolver<S,V> {
//     type Model;

//     fn check(&self) -> SatResult<Self::Model> {
//         todo!()
//     }
// }
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