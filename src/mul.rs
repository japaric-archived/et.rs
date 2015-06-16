use std::ops::Mul;

use traits::{Flops, Matrix, UnsafeGet};

impl<A, B> Flops for ::Mul<A, B> where
    A: Flops + UnsafeGet,
    B: Flops + UnsafeGet,
    A::Output: Mul<B::Output>,
{
    #[inline(always)]
    fn flops() -> usize {
        A::flops() + B::flops() + 1
    }
}

impl<A, B, C> Matrix for ::Mul<A, B> where
    A: UnsafeGet,
    B: Matrix + UnsafeGet,
    A::Output: Mul<B::Output, Output=C>,
{
    #[inline(always)]
    fn nrows(&self) -> u32 {
        B::nrows(&self.1)
    }

    #[inline(always)]
    fn ncols(&self) -> u32 {
        B::ncols(&self.1)
    }

    #[inline(always)]
    fn nelems(&self) -> usize {
        B::nelems(&self.1)
    }

    #[inline(always)]
    fn size(&self) -> (u32, u32) {
        B::size(&self.1)
    }
}

impl<A, B, C> UnsafeGet for ::Mul<A, B> where
    A: UnsafeGet,
    B: UnsafeGet,
    A::Output: Mul<B::Output, Output=C>,
{
    type Output = C;

    #[inline(always)]
    unsafe fn unsafe_get(&self, i: (u32, u32)) -> C {
        self.0.unsafe_get(i) * self.1.unsafe_get(i)
    }
}
