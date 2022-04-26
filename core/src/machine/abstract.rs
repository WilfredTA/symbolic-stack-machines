use crate::{instructions::{AbstractInstruction, InstructionResult, error::InstructionError, EnvExtension, EnvExtensionRecord}, stack::Stack, constraint::{Solver, self, SatResult}};
use crate::memory::{RWMem, ReadOnlyMem};
use crate::constraint::{AbstractConstraintValue, CmpType, Constraint};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AbstractMachineError {
    #[error(transparent)]
    InstructionError(#[from] InstructionError),
}

pub type MachineResult<T> = Result<T, AbstractMachineError>;
pub struct AbstractExecBranch<IType, T> 
where IType: AbstractInstruction
{
    pub l: Option<AbstractMachine<IType>>,
    pub r: Option<AbstractMachine<IType>>,
    pub l_constraints: Vec<Constraint<T>>,
    pub r_constraints: Vec<Constraint<T>>,
}

impl<IType, T> AbstractExecBranch<IType, T>
where IType: AbstractInstruction
{
    pub fn flatten(self) -> (Option<SingleBranch<IType, T>>, Option<SingleBranch<IType, T>>) {
        let left = {
            if let Some(l) = self.l {
                Some((l, self.l_constraints))
            } else {
                None
            }
        };
        let right = {
            if let Some(r) = self.r {
                Some((r, self.r_constraints))
            } else {
                None
            }
        };
        (left, right)
    }
}

impl<I,T> From<SingleBranch<I, T>> for AbstractExecBranch<I, T> 
where I: AbstractInstruction
{
    fn from(b: SingleBranch<I, T>) -> Self {
        Self {
            l: Some(b.0),
            r: None,
            l_constraints: b.1,
            r_constraints: vec![],
        }
    }
}

pub type SingleBranch<I: AbstractInstruction, T> = (AbstractMachine<I>, Vec<Constraint<T>>);


#[derive(Default)]
pub struct PathSummary<IType, T, M> 
where IType: AbstractInstruction
{
    pub reachable: Vec<(SingleBranch<IType, T>, SatResult<M>)>,
    pub unreachable: Vec<(SingleBranch<IType, T>, SatResult<M>)>
}
#[derive(Clone)]
pub struct AbstractMachine<IType: AbstractInstruction> {
    pub pgm: Vec<IType>,
    pub stack: IType::Stack,
    pub mem: IType::Mem,
    pub custom_env: IType::Extension,
    pub pc: usize,

}

impl<I, S, M, SVal, MIdx, MVal, Ext> AbstractMachine<I>
where
    SVal: Into<MIdx> + Into<MVal>,
    S: Stack<StackVal = SVal> + Clone,
    M: RWMem + ReadOnlyMem<MemVal = MVal, Index = MIdx> + Clone,
    Ext: EnvExtension + Clone,
    I: AbstractInstruction<Stack = S, Mem = M, Extension = Ext> + Clone,
{

   
    pub fn new(stack: S, mem: M, custom_env: Ext, pgm: Vec<I>, pc: usize) -> Self {
        Self {
            pgm,
            stack,
            mem,
            custom_env,
            pc,
        }
    }
    pub fn exec<C: Into<Constraint<C>> + Clone, CS: Solver>(&self, solver: Option<CS>) -> MachineResult<PathSummary<I, C, CS::Model>> {
       

        let execute = |pc: usize, pgm: &Vec<I>, mut stack:S, mut mem: M, mut ext: Ext | -> AbstractExecBranch< I, C> {
            
            for i in &pgm[pc..] {
                let ret = i.exec::<C>(&self.stack,&self.mem, &self.custom_env).unwrap();
                if ret.halt || pc == pgm.len() {
                    return AbstractExecBranch {
                        l: None,
                        r: None,
                        l_constraints: vec![],
                        r_constraints: vec![]
                    };
                }

                stack =  {
                    if let Some(stack_diff) = ret.stack_diff {
                        stack_diff.apply(stack).unwrap()
                    } else {
                        stack
                    }
                };

                mem = {
                    if let Some(mem_diff) = ret.mem_diff {
                        mem_diff.apply(mem).unwrap()
                    } else {
                        mem
                    }
                };

                ext = {
                    if let Some(ext_diff) = ret.ext_diff {
                        ext_diff.apply(ext).unwrap() 
                    } else {
                        ext
                    }
                };

                let mut continuation_branch = AbstractExecBranch {
                    l: Some(Self::new(stack.clone(), mem.clone(), ext.clone(), pgm.clone(), pc + 1)),
                    r: None,
                    l_constraints: vec![],
                    r_constraints: vec![]
                };

                if let Some(constraints) = ret.constraints {
                    if let Some(c) = constraints.first() {
                        continuation_branch.l_constraints = c.clone();
                    }
    
                    if let Some(c) = constraints.get(1) {
                        continuation_branch.r_constraints = c.clone();
                        // Can unrwap pc_change on right side since it will be a jump of some form
                        continuation_branch.r = Some(Self::new(stack.clone(), mem.clone(), ext.clone(), pgm.clone(), ret.pc_change.unwrap()));
                    }
                }

                return continuation_branch;

            }
            AbstractExecBranch {
                l: None,
                r: None,
              
                l_constraints:vec![],
                r_constraints: vec![],
            }
        };
        let mut trace_tree: Vec<AbstractExecBranch< I, C>> = vec![];
        let init_branch: AbstractExecBranch<I, C> = AbstractExecBranch {
            l: Some(self.clone()),
            r: None,
          
            l_constraints:vec![],
            r_constraints: vec![],
        };
        trace_tree.push(init_branch);
        let mut leaves: Vec<SingleBranch<I, C>> = vec![];

        loop {
            let start_branch = trace_tree.pop();
            if let Some(start_branch) = start_branch {
                if let Some(mach) = start_branch.l {
                    let AbstractMachine { pgm, stack, mem, custom_env, pc } = &mach;
                    let mut constraints = start_branch.l_constraints;
                    let branches = execute(pc.clone(), pgm, stack.clone(), mem.clone(), custom_env.clone());
                    match branches.flatten() {
                        (None, None) => {
                            leaves.push((mach, constraints));
                        },
                        (None, Some(_)) => {
                            panic!("This should never happen");
                        },
                        (Some(b), None) => {
                            constraints.extend(b.1);
                            trace_tree.push((b.0, constraints.clone()).into());
                        },
                        (Some(l), Some(r)) => {
                            let mut r_constraints = constraints.clone();
                            r_constraints.extend(r.1);
                            constraints.extend(l.1);
                            trace_tree.push(AbstractExecBranch {
                                l: Some(l.0),
                                r: Some(r.0),
                                l_constraints: constraints.clone(),
                                r_constraints
                            });
                        },
                    }
                }
            } else {
                break;
            }
        }
                

        let mut summary = PathSummary {
            reachable: vec![],
            unreachable: vec![]
        };
        if let Some(mut solver) = solver {
            for leaf in leaves {
           
                let constraints = &leaf.1;
                for constraint in constraints {
                    solver.assert(constraint);
                }
                let sat = solver.solve::<C>();
                if let SatResult::Sat(m) = sat {
                    summary.reachable.push((leaf, SatResult::Sat(m)));
                } else {
                    summary.unreachable.push((leaf, SatResult::Unsat));
                }
            }
        }

        Ok(summary)
    }
}
