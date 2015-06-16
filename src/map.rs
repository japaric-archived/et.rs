use traits::{Flops, Matrix, UnsafeGet};

impl<F, M> Flops for ::Map<F, M> where
    F: Fn<(M::Output,)>,
    M: Flops + Matrix + UnsafeGet,
{
    #[inline(always)]
    fn flops() -> usize {
        M::flops() + 1
    }
}

impl<F, M> Matrix for ::Map<F, M> where
    F: Fn<(M::Output,)>,
    M: Matrix + UnsafeGet,
{
    #[inline(always)]
    fn nrows(&self) -> u32 {
        M::nrows(&self.1)
    }

    #[inline(always)]
    fn ncols(&self) -> u32 {
        M::nrows(&self.1)
    }

    #[inline(always)]
    fn nelems(&self) -> usize {
        M::nelems(&self.1)
    }

    #[inline(always)]
    fn size(&self) -> (u32, u32) {
        M::size(&self.1)
    }
}

impl<F, M> UnsafeGet for ::Map<F, M> where
    F: Fn<(M::Output,)>,
    M: UnsafeGet,
{
    type Output = F::Output;

    #[inline(always)]
    unsafe fn unsafe_get(&self, i: (u32, u32)) -> F::Output {
        self.0(self.1.unsafe_get(i))
    }
}
