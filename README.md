# `et.rs`

Experiments with expression templates

**WARNING** Right now this crate only works in the (unofficial) [edge] channel, which contains
features that are still in the RFC phase.

[edge]: https://github.com/japaric/rusty-edge

This crate implements contiguous/strided matrices/vectors where all their arithmetic operations
are lazily evaluated and return proxies like `Add(A, B)` or `Mul(A, B)`. Those proxies also
implement lazy arithmetic, such that expression like `a + b * c` return `Add(A, Mul(B, C))`.

The indexed assignment operator (`y[..] = a + b * c`) is used for evaluating the RHS and directly
storing the result in the LHS (`y`) without incurring in temporary allocations (at least for level
1 BLAS like operations). During evaluation, the complex operation on the RHS is performed in a
**single memory pass**, and its **implicitly parallelized** for "big" inputs.

## [API docs]

**NOTE** Unsized types like `Mat<T>` are (wrongly) rendered as empty enums `enum Mat<T> {}` by rustdoc
because I haven't got around to add proper rendering support.

[API docs]: http://japaric.github.io/et.rs/et/

## Example: Convert an RGB image to grayscale

Source code: [gray.rs]
Inputs: `r`, `g` and `b` are the red, green and blue channels of the image.
Output: `gray` will contain the transformed grayscale image, this vector is uninitialized before
the operation.

[gray.rs]: https://github.com/japaric/et.rs/blob/master/gray.rs

Using this crate the transformation can be written in a single statement.

``` rust
// gray: &Col<u8>
// r, g, b: &::strided::Col<u8>
gray[..] = {
    r.map(|x| x as f32) * 0.2126 +
    g.map(|x| x as f32) * 0.7152 +
    b.map(|x| x as f32) * 0.0722
}.map(|x| x as u8);
```

This statement expands into a "parallel for loop" that looks like this (pseudo-code):

```
parfor i in 0..len
    gray[i] = u8(f32(r[i]) * 0.2126 + f32(g[i]) * 0.7152 * f32(b[i]) * 0.0722)
done
```

For a 1920x1080 image, the execution time is around 3.6 ms on my machine.

For comparison this is how the same operation would be performed using [linalg], where the
arithmetic operations directly map to BLAS calls.

[linalg]: https://github.com/japaric/linalg.rs/tree/ng

``` rust
// r, g, b, gray: &Mat<f32>
gray[..] = 0.;        // scopy
*gray += r * 0.2126;  // saxpy
*gray += g * 0.7152;  // saxpy
*gray += b * 0.0722;  // saxpy
```

For a 1920x1080 image, execution time is around 4.2 ms on my machine.

Even though this version performs 4 memory passes over the output matrix, the execution time is
close to the ET version. The caveat is that this version needs to operate on `f32` matrices and the
input/output images are `u8` matrices, so additional expensive conversions would be required in
this case, which results in far worse overall execution time.

## Areas to explore

- Given that Rust, AFAIK, doesn't perform any alias analysis. Could that mean that the performance
  observed here has potential to improve? It would be interesting to compare the assembly generated
  by Rust vs the assembly generated by a C program that uses the `restrict` keyword or vs a Fortran
  program.

- For simplicity, the current implementation relies heavily on indexing. Could this be hampering
  auto-vectorization? In particular, 2D indexing on "contiguous" matrices may hide the fact that
  the matrix is just a slice stored in contiguous memory.

# License

et.rs is dual licensed under the Apache 2.0 license and the MIT license.

See LICENSE-APACHE and LICENSE-MIT for more details.