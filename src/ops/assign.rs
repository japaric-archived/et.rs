use std::cmp;
use std::ops::{IndexAssign, RangeFull};
use std::thread;

use cast::From;
use num_cpus;

use traits::{Flops, Matrix, UnsafeGet};

/// If FLOPS > THRESHOLD, then use fork-join parallelism
const THRESHOLD: usize = 1_000_000;

// FIXME `Mat[..] = &Col + &Col` should be rejected at compile time not at run time
impl<T, R> IndexAssign<RangeFull, R> for ::Mat<T> where
    R: Flops + Matrix + Sync + UnsafeGet<Output=T>,
    T: Send,
{
    fn index_assign(&mut self, _: RangeFull, rhs: R) {
        unsafe {
            assert_eq!(self.size(), rhs.size());

            let nelems = self.nelems();
            let nflops = R::flops() * nelems;

            debug!("Estimated flops: {}", nflops);

            let ncols = usize::from(self.ncols());
            let ncpus = num_cpus::get();
            if nflops > THRESHOLD && ncpus > 1 {
                let nrows = self.nrows();
                let nthreads = u32::from(cmp::min(ncpus, nflops / THRESHOLD)).unwrap();

                debug!("Spinning up {} threads", nthreads);

                // rows per horizontal stripe
                let sz = (nrows - 1) / nthreads + 1;

                let rhs = &rhs;
                self.as_mut().chunks_mut(ncols * usize::from(sz)).zip(0..).map(move |(hstripe, i)| {
                    thread::scoped(move || {
                        for (row, i) in hstripe.chunks_mut(ncols).zip(i*sz..) {
                            for (dst, j) in row.iter_mut().zip(0..) {
                                *dst = rhs.unsafe_get((i, j))
                            }
                        }
                    })
                }).collect::<Vec<_>>();
            } else {
                for (row, i) in self.as_mut().chunks_mut(ncols).zip(0..) {
                    for (dst, j) in row.iter_mut().zip(0..) {
                        *dst = rhs.unsafe_get((i, j))
                    }
                }
            }
        }
    }
}

impl<T, R> IndexAssign<RangeFull, R> for ::Col<T> where
    R: Flops + Matrix + Sync + UnsafeGet<Output=T>,
    T: Send,
{
    fn index_assign(&mut self, _: RangeFull, rhs: R) {
        unsafe {
            assert_eq!(self.size(), rhs.size());

            let nelems = self.nelems();
            let nflops = R::flops() * nelems;

            debug!("Estimated flops: {}", nflops);

            let ncpus = num_cpus::get();
            if nflops > THRESHOLD && ncpus > 1 {
                let nrows = self.nrows();
                let nthreads = u32::from(cmp::min(ncpus, nflops / THRESHOLD)).unwrap();

                debug!("Spinning up {} threads", nthreads);

                // rows per horizontal stripe
                let sz = (nrows - 1) / nthreads + 1;

                let rhs = &rhs;
                self.as_mut().chunks_mut(usize::from(sz)).zip(0..).map(move |(chunk, i)| {
                    thread::scoped(move || {
                        let offset = i * sz;
                        for (dst, i) in chunk.iter_mut().zip(0..) {
                            *dst = rhs.unsafe_get((offset + i, 0))
                        }
                    })
                }).collect::<Vec<_>>();
            } else {
                for (dst, i) in self.iter_mut().zip(0..) {
                    *dst = rhs.unsafe_get((i, 0))
                }
            }
        }
    }
}
