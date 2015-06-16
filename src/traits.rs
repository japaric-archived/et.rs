use cast::From;

/// A rough measurement of the (floating point) operations per element that will be required to
/// evaluate this lazy expression
pub trait Flops {
    /// Returns the number of (floating point) operations per element required to evaluate this
    /// expression
    fn flops() -> usize;
}

impl<'a, T: ?Sized> Flops for &'a T where T: Flops {
    #[inline(always)]
    fn flops() -> usize {
        T::flops()
    }
}

/// A matrix, a rectangular array arranged in rows and columns
pub trait Matrix {
    /// Returns the number of rows of this matrix
    #[inline(always)]
    fn nrows(&self) -> u32 {
        self.size().0
    }

    /// Returns the number of columns of this matrix
    #[inline(always)]
    fn ncols(&self) -> u32 {
        self.size().1
    }

    /// Returns the number of elements in this matrix
    #[inline(always)]
    fn nelems(&self) -> usize {
        let (nrows, ncols) = self.size();
        usize::from(nrows) * usize::from(ncols)
    }

    /// Returns the size of this matrix
    #[inline(always)]
    fn size(&self) -> (u32, u32) {
        (self.nrows(), self.ncols())
    }
}

impl<'a, M: ?Sized> Matrix for &'a M where M: Matrix {
    #[inline(always)]
    fn nrows(&self) -> u32 {
        M::nrows(*self)
    }

    #[inline(always)]
    fn ncols(&self) -> u32 {
        M::ncols(*self)
    }

    #[inline(always)]
    fn nelems(&self) -> usize {
        M::nelems(*self)
    }

    #[inline(always)]
    fn size(&self) -> (u32, u32) {
        M::size(*self)
    }
}

// FIXME this should be a sealed trait
/// Gets an element of the collection, without performing bounds checks
pub trait UnsafeGet {
    /// The element
    type Output;

    /// Returns the element at the coordinates `(i, j)`
    unsafe fn unsafe_get(&self, (u32, u32)) -> Self::Output;
}

impl<'a, T: ?Sized> UnsafeGet for &'a T where T: UnsafeGet {
    type Output = T::Output;

    #[inline(always)]
    unsafe fn unsafe_get(&self, (i, j): (u32, u32)) -> T::Output {
        T::unsafe_get(*self, (i, j))
    }
}

macro_rules! scalar {
    ($($ty:ty),+) => {
        $(
            impl Flops for $ty {
                #[inline(always)]
                fn flops() -> usize {
                    0
                }
            }

            impl UnsafeGet for $ty {
                type Output = $ty;

                #[inline(always)]
                unsafe fn unsafe_get(&self, _: (u32, u32)) -> $ty {
                    *self
                }
            }
         )+
    }
}

scalar!(f32, f64, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
