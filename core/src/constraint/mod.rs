use std::borrow::Borrow;
pub use std::rc::Rc;


#[derive(Clone)]
pub enum Constraint<V> {
    True,
    False,
    Assert(Node<V>),
    Not(Node<V>),
    And(Node<V>, Node<V>),
    Or(Node<V>, Node<V>),
    BinCmp(CmpType<V>),
    Ite(Rc<Constraint<V>>, V, V)
}
#[derive(Clone)]
pub enum Node<V> {
    Simple(V),
    Compound(Rc<Constraint<V>>),
}

impl<G> AsRef<Constraint<G>> for Constraint<G> {
    fn as_ref(&self) -> &Constraint<G> {
        self
    }
} 



impl<V: Clone> Node<V> {
    pub fn new_simple(v: V) -> Self {
        Self::Simple(v)
    }

    pub fn new_compound(c: Constraint<V>) -> Self {
        Self::Compound(Rc::new(c))
    }

    pub fn unwrap(&self) -> V {
        match self {
            Self::Simple(v) => v.clone(),
            Self::Compound(_) => {
                panic!("Cannot unwrap compound type");
            }
        }
    }
}





pub trait Transpile<V: Clone, Ast, G> {

    fn val_to_ground_type(&self, v: V) -> G;
    fn ground_type_to_val(&self, g: G) -> V;
    fn assert(&self, c: Ast) -> Ast;

    fn and(&self, l: Ast, r: Ast) -> Ast;

    fn not(&self, c: Ast) -> Ast;

    fn or(&self, l: Ast, r: Ast) -> Ast;

    fn gt(&self, l: G, r: G) -> Ast;

    fn lt(&self, l: G, r: G) -> Ast;

    fn lte(&self, l: G, r: G) -> Ast;

    fn gte(&self, l: G, r: G) -> Ast;

    fn eq(&self, l: G, r: G) -> Ast;

    fn neq(&self, l: G, r: G) -> Ast;

    fn true_(&self) -> Ast;
    fn false_(&self) -> Ast;

    fn transpile(&self, constraint: impl AsRef<Constraint<V>>) -> Ast {
        match constraint.as_ref() {
            Constraint::Assert(c) => {
                if let Node::Compound(constraint) = c {
                    self.transpile(constraint)
                } else {
                    panic!("Cannot assert a ground value")
                }
            },
            Constraint::Not(c) => {
                if let Node::Compound(constraint) = c {
                    self.transpile(constraint)
                } else {
                    panic!("Cannot logically operate a ground value")
                }
            },
            Constraint::And(l, r) => {
                let l = {
                    if let Node::Compound(c) = l {
                        self.transpile(c)
                    } else {
                        panic!("Cannot logically operate a ground value")
                    }
                };

                let r = {
                    if let Node::Compound(c) = r {
                        self.transpile(c)
                    } else {
                        panic!("Cannot logically operate a ground value")
                    }
                };

                self.and(l, r)
            },
            Constraint::Or(l, r) => {
                let l = {
                    if let Node::Compound(c) = l {
                        self.transpile(c)
                    } else {
                        panic!("Cannot logically operate a ground value")
                    }
                };

                let r = {
                    if let Node::Compound(c) = r {
                        self.transpile(c)
                    } else {
                        panic!("Cannot logically operate a ground value")
                    }
                };

                self.or(l, r)
            },
            Constraint::BinCmp(cmp) => {
                match cmp {
                    CmpType::GT(l, r) => {
                        self.gt(self.val_to_ground_type(l.unwrap()), self.val_to_ground_type(r.unwrap()))
                    },
                    CmpType::LT(l, r) => {
                        self.lt(self.val_to_ground_type(l.unwrap()), self.val_to_ground_type(r.unwrap()))
                    },
                    CmpType::GTE(l, r) => {
                        self.gte(self.val_to_ground_type(l.unwrap()), self.val_to_ground_type(r.unwrap()))
                    },
                    CmpType::LTE(l, r) => {
                        self.lte(self.val_to_ground_type(l.unwrap()), self.val_to_ground_type(r.unwrap()))
                    },
                    CmpType::EQ(l, r) => {
                        self.eq(self.val_to_ground_type(l.unwrap()), self.val_to_ground_type(r.unwrap()))
                    },
                    CmpType::NEQ(l, r) => {
                        self.neq(self.val_to_ground_type(l.unwrap()), self.val_to_ground_type(r.unwrap()))
                    },
                }
            },
            Constraint::Ite(_, _, _) => todo!(),
            Constraint::True => {
                self.assert(self.true_())
            },
            Constraint::False => {
                self.assert(self.false_())
            },
        }
    }
}

impl<V> Constraint<V> 
where V: Clone
{

    fn assert(c: Constraint<V>) -> Self {
        Self::Assert(Node::new_compound(c))
    }

    fn and(self, c: Constraint<V>) -> Self {
        Self::And(Node::new_compound(self), Node::new_compound(c))
    }

    fn not(self, c: Constraint<V>) -> Self {
        Self::Not(Node::new_compound(c))
    }

    fn or(self, c: Constraint<V>) -> Self {
        Self::Or(Node::new_compound(self), Node::new_compound(c))
    }

    fn gt(l: V, r: V) -> Self {
        Self::BinCmp(CmpType::GT(Node::new_simple(l), Node::new_simple(r)))
    }

    fn lt(l: V, r: V) -> Self {
        Self::BinCmp(CmpType::LT(Node::new_simple(l), Node::new_simple(r)))
    }

    fn eq(l: V, r: V) -> Self {
        Self::BinCmp(CmpType::EQ(Node::new_simple(l), Node::new_simple(r)))
    }

    fn lte(l: V, r: V) -> Self {
        Self::BinCmp(CmpType::LTE(Node::new_simple(l), Node::new_simple(r)))
    }

    fn gte(l: V, r: V) -> Self {
        Self::BinCmp(CmpType::GTE(Node::new_simple(l), Node::new_simple(r)))
    }
    // to do rest
}

// pub struct ConstraintSolver<S: Solver, V> {
//     pub constraints: Vec<Constraint<V>>,
//     pub solver: S
// }

pub trait Solver<V, Ast, G>: Constrained + Transpile<V, Ast, G> 
where V: Clone
{
    fn generic_assert(&mut self, constraint: &Constraint<V>);
    fn solve(&self) -> SatResult<Self::Model>;
}

#[derive(Clone)]
pub enum CmpType<V> {
  GT(Node<V>, Node<V>),
  LT(Node<V>, Node<V>),
  GTE(Node<V>, Node<V>),
  LTE(Node<V>, Node<V>),
  EQ(Node<V>, Node<V>),
  NEQ(Node<V>, Node<V>),
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
    Unsat,
    Unknown
}

pub trait Constrained {
    type Model;
    
    fn check(&self) -> SatResult<Self::Model>;
    
}