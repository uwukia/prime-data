pub use prime_byte::PrimeByte;
mod prime_byte;

pub use self::prime_data::PrimeData;
mod prime_data;

pub use iterators::{CoprimeIter, PrimeIter};
mod iterators;

pub use error::{PrimeResult, PrimeError};
pub mod error;

pub mod estimate;

#[cfg(feature = "factors")]
mod factors;
#[cfg(feature = "factors")]
pub use factors::{Factorization, all_factors_of};

mod utils;

/// A list of all values `N % 30`, where N is coprime with 2, 3, and 5
/// 
/// These values are: {1, 7, 11, 13, 17, 19, 23, 29}
pub const K_VALUES: [u8; 8] = [1, 7, 11, 13, 17, 19, 23, 29];


pub use public_methods::*;
mod public_methods {

    use super::utils::IntSqrt;

    /// Verifies if `x` is a prime number
    /// 
    /// Currently, this function is an abstraction over generating prime data up to sqrt(x) then
    /// calling the [check prime](super::PrimeData::check_prime) method.
    /// 
    /// Therefore, if you need to check if lots of numbers are prime, it's heavily encouraged to
    /// [generate](super::PrimeData::generate) prime numbers then calling that method.
    /// 
    /// However, it is planned to make this function faster by using primality tests instead of 
    /// generating data. See [here](crate::guide::future).
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::is_prime;
    /// assert!( is_prime(65_537));
    /// assert!(!is_prime(4_294_967_297));
    /// ```
    pub fn is_prime(x: u64) -> bool {
        let sqrt = x.sqrt_floor();

        super::PrimeData::generate(0..=sqrt).check_prime(x)
    }

    /// Counts how many prime numbers are there less than or equal to `x`
    /// 
    /// This function is an abstraction for [generating](super::PrimeData::generate) prime numbers
    /// up to x, then calling the [count primes](super::PrimeData::count_primes) method.
    /// 
    /// If you only need an approximation, see [estimates](crate::estimate).
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::count_primes;
    /// assert_eq!(count_primes(1_000),   168);
    /// assert_eq!(count_primes(100_000), 9592);
    /// ```
    pub fn count_primes(x: u64) -> u64 {
        super::PrimeData::generate(0..=x).count_primes()
    }
}