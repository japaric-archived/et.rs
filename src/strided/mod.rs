//! "Strided" data structures

mod col;
pub mod raw;

/// Strided column vector
pub unsized type Col<T> = raw::Slice<T>;
