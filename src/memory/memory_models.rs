use crate::instructions::val::{HybridVal, SymbolicBytes, Val};
use std::marker::PhantomData;
use std::rc::Rc;
use z3::ast::{Array, Ast, Int, BV};
use z3::{Context, FuncDecl};

use super::{RWMem, ReadOnlyMem, WriteableMem};
#[derive(Debug, Clone)]
pub struct BaseMemoryConcreteIndex<T> {
    pub _inner: Vec<Val<T>>,
    pub(crate) idx_set: PhantomData<usize>,
    pub(crate) val_set: PhantomData<Val<T>>,
}
#[derive(Debug, Clone)]
pub struct BaseMemorySymbolicArray<'a, I, T> {
    pub _inner: Array<'a>,
    pub(crate) idx_set: PhantomData<I>,
    pub(crate) val_set: PhantomData<Val<T>>,
}
#[derive(Debug)]
pub struct BaseMemorySymbolicUF<'a, I, T> {
    pub _inner: FuncDecl<'a>,
    pub(crate) idx_set: PhantomData<I>,
    pub(crate) val_set: PhantomData<Val<T>>,
}

// pub struct BaseMemoryBitVecIndex<'a, T> {
//     pub _inner: Array<'a>,
//     pub(crate) idx_set: PhantomData<BV<'a>>,
//     pub(crate) val_set: PhantomData<Val<T>>,
// }

pub type MemConcreteToBitVec<'a> = BaseMemoryConcreteIndex<BV<'a>>;
pub type MemConcreteToInt<'a> = BaseMemoryConcreteIndex<Int<'a>>;
pub type MemBitVecToBitVec<'a> = BaseMemorySymbolicArray<'a, BV<'a>, BV<'a>>;
pub type MemIntToInt<'a> = BaseMemorySymbolicArray<'a, Int<'a>, Int<'a>>;
pub type MemBitVecToAny<'a, T> = BaseMemorySymbolicArray<'a, BV<'a>, T>;
pub type MemBitVecToInt<'a> = MemBitVecToAny<'a, Int<'a>>;

impl<'a, I> ReadOnlyMem for BaseMemorySymbolicArray<'a, I, BV<'a>>
where
    I: z3::ast::Ast<'a>,
{
    type MemVal = BV<'a>;

    type Index = I;

    fn read(&self, idx: Self::Index) -> super::MemoryResult<Option<Self::MemVal>> {
        Ok(self._inner.select(&idx).as_bv())
    }
}

impl<'a, I> WriteableMem for BaseMemorySymbolicArray<'a, I, BV<'a>>
where
    I: z3::ast::Ast<'a>,
{
    type MemVal = BV<'a>;

    type Index = I;

    fn write(&self, idx: Self::Index, val: Self::MemVal) -> super::MemoryResult<Self> {
        Ok(Self {
            _inner: self._inner.store(&idx, &val),
            idx_set: PhantomData::<I>,
            val_set: PhantomData::<Val<BV<'a>>>,
        })
    }
}

impl<'a, I> RWMem for BaseMemorySymbolicArray<'a, I, BV<'a>>
where
    I: z3::ast::Ast<'a> + Clone,
{
    type InitArgs = (Rc<&'a Context>, I, usize);

    fn init(args: Self::InitArgs) -> Self {
        let (ctx, domain, range_size) = args;
        Self {
            _inner: Array::new_const(
                ctx.as_ref(),
                "memory",
                &I::get_sort(&domain),
                &z3::Sort::bitvector(ctx.as_ref(), range_size as u32),
            ),
            idx_set: PhantomData::<I>,
            val_set: PhantomData::<Val<BV<'a>>>,
        }
    }
}

impl<'a> ReadOnlyMem for MemIntToInt<'a> {
    type MemVal = Int<'a>;

    type Index = Int<'a>;

    fn read(&self, idx: Self::Index) -> super::MemoryResult<Option<Self::MemVal>> {
        Ok(self._inner.select(&idx).as_int())
    }
}

impl<'a> WriteableMem for MemIntToInt<'a> {
    type MemVal = Int<'a>;

    type Index = Int<'a>;

    fn write(&self, idx: Self::Index, val: Self::MemVal) -> super::MemoryResult<Self> {
        Ok(Self {
            _inner: self._inner.store(&idx, &val),
            idx_set: PhantomData::<Int<'a>>,
            val_set: PhantomData::<Val<Int<'a>>>,
        })
    }
}

impl<'a> RWMem for MemIntToInt<'a> {
    type InitArgs = Rc<&'a Context>;

    fn init(args: Self::InitArgs) -> Self {
        Self {
            _inner: Array::new_const(
                args.as_ref(),
                "memory",
                &z3::Sort::int(&args),
                &z3::Sort::int(&args),
            ),
            idx_set: PhantomData::<Int<'a>>,
            val_set: PhantomData::<Val<Int<'a>>>,
        }
    }
}
