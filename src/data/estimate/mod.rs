//! # Prime Estimates
//! 
//! This module is dedicated for estimating prime bounds
#![allow(missing_docs)]

use super::PrimeData;

pub use upper_bound::upper_bound;
mod upper_bound;

pub use nth_prime::{nth_prime_approximation, nth_prime_bounds};
mod nth_prime;

/// Evaluates the exact amount of prime numbers from 1 to N
/// 
/// This is exactly the same as creating some [PrimeData](crate::data::PrimeData) ranging from
/// 1 to N and [counting its primes](crate::data::PrimeData::count_primes)
pub fn exact_count(bound: u64) -> u64 {
    PrimeData::generate(0..=bound).count_primes()
}