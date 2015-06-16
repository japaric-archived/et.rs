use std::num::Zero;
use std::ops::Deref;
use std::{iter, mem, slice};

use cast::From;

impl<T> ::Col<T> {
    pub fn zeros(n: u32) -> Box<::Col<T>> where T: Clone + Zero {
        unsafe {
            let mut v: Vec<_> = iter::repeat(T::zero()).take(usize::from(n)).collect();

            let data = v.as_mut_ptr();
            mem::forget(v);

            mem::transmute(::raw::Slice {
                data: data,
                len: n,
            })
        }
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<T> {
        self.as_mut().iter_mut()
    }

    fn repr(&self) -> ::raw::Slice<T> {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl<T> AsMut<[T]> for ::Col<T> {
    fn as_mut(&mut self) -> &mut [T] {
        unsafe {
            let ::raw::Slice { data, len } = self.repr();

            slice::from_raw_parts_mut(data, usize::from(len))
        }
    }
}

impl<T> AsRef<[T]> for ::Col<T> {
    fn as_ref(&self) -> &[T] {
        unsafe {
            let ::raw::Slice { data, len } = self.repr();

            slice::from_raw_parts(data, usize::from(len))
        }
    }
}

impl<T> Deref for ::Col<T> {
    type Target = ::strided::Col<T>;

    fn deref(&self) -> &::strided::Col<T> {
        unsafe {
            let ::raw::Slice { data, len } = self.repr();

            mem::transmute(::strided::raw::Slice {
                data: data,
                len: len,
                stride: 1,
            })
        }
    }
}

impl<T> Drop for ::Col<T> {
    fn drop(&mut self) {
        unsafe {
            let ::raw::Slice { data, len, .. } = self.repr();

            if !data.is_null() && data as usize != mem::POST_DROP_USIZE {
                let len = usize::from(len);

                mem::drop(Vec::from_raw_parts(data, len, len))
            }
        }
    }
}

unsafe impl<T> Send for ::Col<T> where T: Send {}

unsafe impl<T> Sync for ::Col<T> where T: Sync {}
