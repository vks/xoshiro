/// Initialize a RNG from a `u64` seed using `SplitMix64`.
macro_rules! from_splitmix {
    ($seed:expr) => { {
        let mut rng = ::SplitMix64::from_seed_u64($seed);
        Self::from_rng(&mut rng).unwrap()
    } }
}

/// Perform the ** operation used by some RNGs from the xoshiro family.
macro_rules! starstar {
    ($x:expr) => {
        $x.wrapping_mul(5).rotate_left(7).wrapping_mul(9)
    }
}
