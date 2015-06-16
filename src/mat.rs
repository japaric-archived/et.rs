use std::num::{One, Zero};
use std::ops::{Index, RangeFull};
use std::{fmt, iter, mem, slice};

use cast::From;

use traits::{Flops, Matrix, UnsafeGet};

impl<T> ::Mat<T> {
    /// Creates a matrix where each element is initialized to `elem`
    pub fn from_elem((nrows, ncols): (u32, u32), elem: T) -> Box<::Mat<T>> where T: Clone {
        let n = usize::from(nrows).checked_mul(usize::from(ncols)).unwrap();
        let mut v: Vec<_> = iter::repeat(elem).take(n).collect();

        let data = v.as_mut_ptr();
        mem::forget(v);

        unsafe {
            mem::transmute(::raw::Mat { data: data, nrows: nrows, ncols: ncols })
        }
    }

    /// Creates a matrix where each element is initialized using the function `f`
    pub fn from_fn<F>((nrows, ncols): (u32, u32), mut f: F) -> Box<::Mat<T>> where
        F: FnMut((u32, u32)) -> T,
    {
        let n = usize::from(nrows).checked_mul(usize::from(ncols)).unwrap();
        let mut v = Vec::with_capacity(n);

        for i in 0..nrows {
            for j in 0..ncols {
                v.push(f((i, j)))
            }
        }

        let data = v.as_mut_ptr();
        mem::forget(v);

        unsafe {
            mem::transmute(::raw::Mat { data: data, nrows: nrows, ncols: ncols })
        }
    }

    /// Reshapes an slice into a matrix with dimensions `(nrows, ncols)`
    pub fn reshape(slice: &[T], (nrows, ncols): (u32, u32)) -> &::Mat<T> {
        unsafe {
            assert_eq!(slice.len(), usize::from(nrows) * usize::from(ncols));

            mem::transmute(::raw::Mat {
                data: slice.as_ptr() as *mut T,
                nrows: nrows,
                ncols: ncols,
            })
        }
    }

    /// Lazily maps this matrix
    pub fn map<F>(&self, f: F) -> ::Map<F, &::Mat<T>> where
        F: Fn<(T,)>,
        T: Clone,
    {
        ::Map(f, self)
    }

    /// Creates a matrix filled with ones
    pub fn ones((nrows, ncols): (u32, u32)) -> Box<::Mat<T>> where T: Clone + One {
        ::Mat::from_elem((nrows, ncols), T::one())
    }

    /// Returns the raw representation of this matrix
    pub fn repr(&self) -> ::raw::Mat<T> {
        unsafe {
            mem::transmute(self)
        }
    }

    /// Creates a matrix filled with zeros
    pub fn zeros((nrows, ncols): (u32, u32)) -> Box<::Mat<T>> where T: Clone + Zero {
        ::Mat::from_elem((nrows, ncols), T::zero())
    }
}

impl<T> AsMut<[T]> for ::Mat<T> {
    fn as_mut(&mut self) -> &mut [T] {
        unsafe {
            let ::raw::Mat { data, nrows, ncols } = self.repr();
            let len = nrows as usize * ncols as usize;
            slice::from_raw_parts_mut(data, len)
        }
    }
}

impl<T> AsRef<[T]> for ::Mat<T> {
    fn as_ref(&self) -> &[T] {
        unsafe {
            let ::raw::Mat { data, nrows, ncols } = self.repr();
            let len = nrows as usize * ncols as usize;
            slice::from_raw_parts(data, len)
        }
    }
}

impl<T> Drop for ::Mat<T> {
    fn drop(&mut self) {
        unsafe {
            let ::raw::Mat { data, nrows, ncols } = self.repr();

            if !data.is_null() && data as usize != mem::POST_DROP_USIZE {
                let len = nrows as usize * ncols as usize;

                mem::drop(Vec::from_raw_parts(data, len, len))
            }
        }
    }
}

impl<T> fmt::Debug for ::Mat<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut is_first = true;

        for row in self.as_ref().chunks(self.ncols() as usize) {
            if is_first {
                is_first = false;
            } else {
                try!(f.write_str("\n"));
            }

            try!(write!(f, "{:?}", row));
        }

        Ok(())
    }
}

impl<T> Flops for ::Mat<T> {
    fn flops() -> usize {
        0
    }
}

impl<T> Index<(RangeFull, u32)> for ::Mat<T> {
    type Output = ::strided::Col<T>;

    fn index(&self, (_, c): (RangeFull, u32)) -> &::strided::Col<T> {
        unsafe {
            let ::raw::Mat { data, nrows, ncols } = self.repr();

            assert!(c < ncols);

            mem::transmute(::strided::raw::Slice {
                data: data.offset(c as isize),
                len: nrows,
                stride: ncols,
            })
        }
    }
}

impl<T> Matrix for ::Mat<T> {
    #[inline(always)]
    fn nrows(&self) -> u32 {
        self.repr().nrows
    }

    #[inline(always)]
    fn ncols(&self) -> u32 {
        self.repr().ncols
    }

    #[inline(always)]
    fn size(&self) -> (u32, u32) {
        let ::raw::Mat { nrows, ncols, .. } = self.repr();
        (nrows, ncols)
    }
}

unsafe impl<T> Send for ::Mat<T> where T: Send {}

unsafe impl<T> Sync for ::Mat<T> where T: Sync {}

impl<T> UnsafeGet for ::Mat<T> where T: Clone {
    type Output = T;

    #[inline(always)]
    unsafe fn unsafe_get(&self, (i, j): (u32, u32)) -> T {
        let ::raw::Mat { data, ncols, .. } = self.repr();
        (*data.offset((i * ncols + j) as isize)).clone()
    }
}
