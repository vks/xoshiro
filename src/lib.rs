extern crate rand_core;

mod xoshiro128starstar;
mod xoshiro256starstar;

pub use xoshiro128starstar::Xoshiro128StarStar;
pub use xoshiro256starstar::Xoshiro256StarStar;
