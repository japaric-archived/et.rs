use std::ops::{Add, Mul};

use traits::{Matrix, UnsafeGet};

// `a * b + c * d`
impl<A, B, C, D, E, F> Add<::Mul<D, E>> for ::Mul<A, B> where
    A: UnsafeGet,
    B: Matrix + UnsafeGet,
    A::Output: Mul<B::Output, Output=C>,
    D: UnsafeGet,
    E: Matrix + UnsafeGet,
    D::Output: Mul<E::Output, Output=F>,
    C: Add<F>,
{
    type Output = ::Add<::Mul<A, B>, ::Mul<D, E>>;

    fn add(self, rhs: ::Mul<D, E>) -> Self::Output {
        assert_eq!(self.size(), rhs.size());

        ::Add(self, rhs)
    }
}

// `a + b + c * d`
impl<A, B, C, D, E, F> Add<::Mul<C, D>> for ::Add<A, B> where
    A: UnsafeGet,
    B: Matrix + UnsafeGet,
    A::Output: Add<B::Output> + Add<F>,
    C: UnsafeGet,
    D: Matrix + UnsafeGet,
    C::Output: Mul<D::Output, Output=E>,
    B::Output: Add<E, Output=F>,
{
    type Output = ::Add<A, ::Add<B, ::Mul<C, D>>>;

    fn add(self, rhs: ::Mul<C, D>) -> Self::Output {
        assert_eq!(self.size(), rhs.size());

        ::Add(self.0, ::Add(self.1, rhs))
    }
}
