use std::ops::Mul;

use traits::UnsafeGet;

// `x.sin() * 2.`
impl<F, A, B> Mul<B> for ::Map<F, A> where
    A: UnsafeGet,
    F: Fn<(A::Output,)>,
    B: Mul<F::Output> + UnsafeGet<Output=B>,
{
    type Output = ::Mul<B, ::Map<F, A>>;

    fn mul(self, rhs: B) -> ::Mul<B, ::Map<F, A>> {
        ::Mul(rhs, self)
    }
}
