#![allow(unknown_lints)]

#[macro_use]
extern crate bencher;
extern crate xoshiro;

use std::mem::size_of;
use bencher::{black_box, Bencher};
use xoshiro::Xoshiro128;

macro_rules! make_bench_u32 {
    ($name:ident, $rng:path) => {
        fn $name(b: &mut Bencher) {
            type Rng = $rng;
            let mut rng = Rng::from_seed([1, 2, 3, 4]);
            b.iter(|| {
                for _ in 0..10 {
                    black_box(rng.next());
                }
            });
            b.bytes = size_of::<u32>() as u64;
        }
    }
}

make_bench_u32!(rand_u32_xoshiro, Xoshiro128);
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
      target_feature = "avx2"))]
make_bench_u32!(rand_u32_xoshiro_simd, xoshiro::avx2::Xoshiro128);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
      target_feature = "avx2"))]
benchmark_group!(benches, rand_u32_xoshiro, rand_u32_xoshiro_simd);
#[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"),
      target_feature = "avx2")))]
benchmark_group!(benches, rand_u32_xoshiro);
benchmark_main!(benches);
