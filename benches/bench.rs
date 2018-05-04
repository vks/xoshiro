#![allow(unknown_lints)]

#[macro_use]
extern crate bencher;
extern crate xoshiro;

use std::mem::size_of;
use bencher::{black_box, Bencher};
use xoshiro::{Xoshiro128, Xoshiro128SIMD};

macro_rules! make_bench_u32 {
    ($name:ident, $rng:ident) => {
        fn $name(b: &mut Bencher) {
            let mut rng = $rng::from_seed([1, 2, 3, 4]);
            b.iter(|| {
                black_box(rng.next());
            });
            b.bytes = size_of::<u32>() as u64;
        }
    }
}

make_bench_u32!(rand_u32_xoshiro, Xoshiro128);
make_bench_u32!(rand_u32_xoshiro_simd, Xoshiro128SIMD);

benchmark_group!(benches, rand_u32_xoshiro, rand_u32_xoshiro_simd);
benchmark_main!(benches);
