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

benchmark_group!(benches, rand_u32_xoshiro);
benchmark_main!(benches);
