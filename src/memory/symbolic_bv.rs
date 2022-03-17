use std::{rc::Rc, marker::PhantomData};

use crate::{memory::{ReadOnlyMem, WriteableMem, RWMem, memory_models::MemBitVecToBitVec}, instructions::val::Val};
use z3::{ast::{Ast, BV, Bool, Array}, Context};
use byteorder::{BigEndian, LittleEndian};

use super::MemoryResult;



// impl<'a> ReadOnlyMem for BaseMemoryBitVecIndex<'a, BV<'a>> {
//     type MemVal = BV<'a>;

//     type Index = BV<'a>;

//     fn read(&self, idx: Self::Index) -> MemoryResult<Option<Self::MemVal>> {
//         Ok(self._inner.select(&idx).as_bv())
//     }
// }

// impl<'a> WriteableMem for BaseMemoryBitVecIndex<'a, BV<'a>> {
//     type MemVal = BV<'a>;

//     type Index = BV<'a>;

//     fn write(&self, idx: Self::Index, val: Self::MemVal) -> MemoryResult<Self> {
//         Ok(Self {
//             _inner: self._inner.store(&idx, &val),
//             idx_set: PhantomData::<BV<'a>>,
//             val_set: PhantomData::<Val<BV<'a>>>,
//         })
//     }
// }

// impl<'a> RWMem for BaseMemoryBitVecIndex<'a, BV<'a>> {
//     type InitArgs = (Rc<&'a Context>, usize, usize);

//     fn init(args: Self::InitArgs) -> Self {
//         let (ctx, domain_size, range_size) = args;
//         Self {
//             _inner: Array::new_const(
//                 ctx.as_ref(),
//                 "memory",
//                 &z3::Sort::bitvector(ctx.as_ref(), domain_size as u32),
//                 &z3::Sort::bitvector(ctx.as_ref(), range_size as u32),
//             ),
//             idx_set: PhantomData::<BV<'a>>,
//             val_set: PhantomData::<Val<BV<'a>>>,
//         }
//     }
// }