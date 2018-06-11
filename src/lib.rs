extern crate byteorder;
extern crate rand_core;

mod splitmix64;
mod xoshiro128starstar;
mod xoshiro256starstar;

pub use splitmix64::SplitMix64;
pub use xoshiro128starstar::Xoshiro128StarStar;
pub use xoshiro256starstar::Xoshiro256StarStar;
