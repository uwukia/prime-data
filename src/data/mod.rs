pub use prime_byte::PrimeByte;
mod prime_byte;

pub use self::prime_data::PrimeData;
mod prime_data;

pub use iterators::{CoprimeIter, PrimeIter};
mod iterators;

pub use error::{PrimeResult, PrimeError};
pub mod error;

pub mod estimate;

mod utils;

/// A list of all values `N % 30`, where N is coprime with 2, 3, and 5
/// 
/// These values are: {1, 7, 11, 13, 17, 19, 23, 29}
pub const K_VALUES: [u8; 8] = [1, 7, 11, 13, 17, 19, 23, 29];