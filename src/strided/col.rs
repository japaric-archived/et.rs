use std::mem;

use traits::{Flops, Matrix, UnsafeGet};

impl<T> ::strided::Col<T> {
    pub fn map<F>(&self, f: F) -> ::Map<F, &Self> where F: Fn<(T,)>, T: Clone {
        ::Map(f, self)
    }

    fn repr(&self) -> ::strided::raw::Slice<T> {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl<T> Flops for ::strided::Col<T> {
    fn flops() -> usize {
        0
    }
}

impl<T> Matrix for ::strided::Col<T> {
    #[inline(always)]
    fn nrows(&self) -> u32 {
        self.repr().len
    }

    #[inline(always)]
    fn ncols(&self) -> u32 {
        1
    }
}

unsafe impl<T> Send for ::strided::Col<T> where T: Send {}

unsafe impl<T> Sync for ::strided::Col<T> where T: Sync {}

impl<T> UnsafeGet for ::strided::Col<T> where T: Clone {
    type Output = T;

    #[inline(always)]
    unsafe fn unsafe_get(&self, (i, _): (u32, u32)) -> T {
        let ::strided::raw::Slice { data, stride, .. } = self.repr();

        (*data.offset(i as isize * stride as isize)).clone()
    }
}
