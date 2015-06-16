use std::num::Zero;
use std::ops::{Deref, DerefMut};
use std::{iter, mem, slice};

use traits::{Matrix, UnsafeGet};

impl<T> ::Slice<T> {
    pub fn map<F>(&self, f: F) -> ::Map<F, &::Slice<T>> where F: Fn<(T,)>, T: Clone {
        ::Map(f, self)
    }

    pub fn zeros(n: usize) -> Box<::Slice<T>> where T: Clone + Zero {
        unsafe {
            mem::transmute(iter::repeat(T::zero()).take(n).collect::<Vec<_>>().into_boxed_slice())
        }
    }
}

impl<T> Deref for ::Slice<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.0
    }
}

impl<T> DerefMut for ::Slice<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.0
    }
}

impl<T> UnsafeGet for ::Slice<T> where T: Clone {
    type Output = T;

    unsafe fn unsafe_get(&self, (i, _): (u32, u32)) -> T {
        (*self.0.as_ptr().offset(i as isize)).clone()
    }
}
