use crate::instructions::val::Val;
use std::marker::PhantomData;
use std::rc::Rc;
use z3::ast::{Array, Ast, Int, BV};
use z3::{Context, FuncDecl};

use super::{Mem, RWMem, ReadOnlyMem, WriteableMem};
#[derive(Debug, Clone)]
pub struct BaseMemoryConcreteIndex<T> {
    pub _inner: Vec<Val<T>>,
    pub(crate) idx_set: PhantomData<usize>,
    pub(crate) val_set: PhantomData<Val<T>>,
}

impl<T> BaseMemoryConcreteIndex<T> {
    pub fn new() -> Self {
        Self {
            _inner: vec![],
            idx_set: PhantomData::<usize>,
            val_set: PhantomData::<Val<T>>,
        }
    }
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
pub type BaseMemoryConcreteUint64 = BaseMemoryConcreteIndex<u64>;

impl<'a, I> Mem for BaseMemorySymbolicArray<'a, I, BV<'a>>
where
    I: z3::ast::Ast<'a> + Clone,
{
    type MemVal = BV<'a>;

    type Index = I;
}

impl<'a, I> ReadOnlyMem for BaseMemorySymbolicArray<'a, I, BV<'a>>
where
    I: z3::ast::Ast<'a> + Clone,
{
    fn read(&self, idx: Self::Index) -> super::MemoryResult<Option<Self::MemVal>> {
        Ok(self._inner.select(&idx).as_bv())
    }
}

impl<'a, I> WriteableMem for BaseMemorySymbolicArray<'a, I, BV<'a>>
where
    I: z3::ast::Ast<'a> + Clone,
{
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

impl<'a> Mem for BaseMemorySymbolicArray<'a, u64, u64> {
    type MemVal = u64;
    type Index = u64;
}

impl<'a> ReadOnlyMem for BaseMemorySymbolicArray<'a, u64, u64> {
    fn read(&self, idx: Self::Index) -> super::MemoryResult<Option<Self::MemVal>> {
        Ok(self
            ._inner
            .select(&Int::from_u64(self._inner.get_ctx(), idx))
            .as_int()
            .unwrap()
            .as_u64())
    }
}

impl<'a> WriteableMem for BaseMemorySymbolicArray<'a, u64, u64> {
    fn write(&self, idx: Self::Index, val: Self::MemVal) -> super::MemoryResult<Self> {
        Ok(Self {
            _inner: self._inner.store(
                &Int::from_u64(self._inner.get_ctx(), idx),
                &Int::from_u64(self._inner.get_ctx(), val),
            ),
            idx_set: PhantomData::<u64>,
            val_set: PhantomData::<Val<u64>>,
        })
    }
}

impl<'a> RWMem for BaseMemorySymbolicArray<'a, u64, u64> {
    type InitArgs = (Rc<&'a Context>, u64, usize);

    fn init(args: Self::InitArgs) -> Self {
        let (ctx, _domain, _range_size) = args;
        Self {
            _inner: Array::new_const(
                ctx.as_ref(),
                "memory",
                &z3::Sort::int(ctx.as_ref()),
                &z3::Sort::int(ctx.as_ref()),
            ),
            idx_set: PhantomData::<u64>,
            val_set: PhantomData::<Val<u64>>,
        }
    }
}

impl<'a> Mem for MemIntToInt<'a> {
    type MemVal = Int<'a>;
    type Index = Int<'a>;
}

impl<'a> ReadOnlyMem for MemIntToInt<'a> {
    fn read(&self, idx: Self::Index) -> super::MemoryResult<Option<Self::MemVal>> {
        Ok(self._inner.select(&idx).as_int())
    }
}

impl<'a> WriteableMem for MemIntToInt<'a> {
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

impl Mem for BaseMemoryConcreteUint64 {
    type MemVal = u64;
    type Index = u64;
}

impl ReadOnlyMem for BaseMemoryConcreteUint64 {
    fn read(&self, idx: Self::Index) -> super::MemoryResult<Option<Self::MemVal>> {
        Ok(self._inner.get(idx as usize).map(|v| v.0))
    }
}

impl WriteableMem for BaseMemoryConcreteUint64 {
    fn write(&self, idx: Self::Index, val: Self::MemVal) -> super::MemoryResult<Self> {
        let mut inner = self._inner.clone();
        inner[idx as usize] = Val(val);
        Ok(Self {
            _inner: inner,
            idx_set: PhantomData::<usize>,
            val_set: PhantomData::<Val<u64>>,
        })
    }
}

impl RWMem for BaseMemoryConcreteUint64 {
    type InitArgs = ();

    fn init(_args: Self::InitArgs) -> Self {
        Self {
            _inner: vec![],
            idx_set: PhantomData::<usize>,
            val_set: PhantomData::<Val<u64>>,
        }
    }
}
