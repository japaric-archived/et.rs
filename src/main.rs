//! Linear algebra with expression templates

//#![deny(missing_docs)]

#![feature(core)]
#![feature(filling_drop)]
#![feature(indexed_assignment)]
#![feature(scoped)]
#![feature(unboxed_closures)]
#![feature(unsized_types)]
#![feature(zero_one)]

extern crate cast;
extern crate num_cpus;

#[macro_use]
extern crate log;

use traits::UnsafeGet;

mod add;
mod col;
mod map;
mod mat;
mod mul;
mod ops;
mod raw;
mod traits;

pub mod strided;

// Example: Convert an RGB image to gray scale
mod gray;

fn main() {
    gray::main();
}

pub unsized type Col<T> = ::raw::Slice<T>;

pub unsized type Mat<T> = ::raw::Mat<T>;

// NB Scalars will always be placed in the leftmost side of a lazy sum, for example the following
// expression: `x + 1. + y + 2.` will be reduced as follows:
// - `Sum<1., x> + y + 2.`
// - `Sum<1., Sum<x, y>> + 2.`
// - `Sum<3., Sum<x, y>>`
/// Lazy addition
pub struct Add<A, B>(A, B) where
    A: UnsafeGet,
    B: UnsafeGet,
    A::Output: std::ops::Add<B::Output>;

/// Lazy function application
pub struct Map<F, M>(F, M) where
    F: Fn<(M::Output,)>,
    M: UnsafeGet;

// NB Same as above, the scalars will be placed in the leftmost side of a lazy product
/// Lazy element-wise multiplication
pub struct Mul<A, B>(A, B) where
    A: UnsafeGet,
    B: UnsafeGet,
    A::Output: std::ops::Mul<B::Output>;

