use std::ops::RangeInclusive;
use crate::{PrimeData, PrimeByte, data::{error::*, utils::{Divisible, ContainsRange}}};

/// Struct that iterates over prime numbers from some data.
/// 
/// To learn more, read the [guide](crate::guide::iterators::_2_prime).
/// 
/// Iterator takes a reference to [PrimeData](crate::PrimeData), therefore, it cannot
/// outlive the given data.
/// 
/// # Examples
/// 
/// ```
/// use prime_data::{PrimeData, PrimeIter};
/// let data = PrimeData::new();
/// let mut iter = PrimeIter::new(&data, 3..=27).unwrap();
/// 
/// assert_eq!(iter.next(), Some(3));
/// assert_eq!(iter.next(), Some(5));
/// assert_eq!(iter.next(), Some(7));
/// assert_eq!(iter.next(), Some(11));
/// assert_eq!(iter.next(), Some(13));
/// assert_eq!(iter.next(), Some(17));
/// assert_eq!(iter.next(), Some(19));
/// assert_eq!(iter.next(), Some(23));
/// assert_eq!(iter.next(), None);
/// ```
pub struct PrimeIter<'a> {
    data: &'a [PrimeByte],
    primes: Option<Vec<u64>>,
    current: (u64, usize),
    data_offset: u64,
    stop_at: u64,
}

impl<'a> PrimeIter<'a> {
    /// Creates an iterator over some [PrimeData](crate::PrimeData) within a given range.
    /// 
    /// Returns a [NotEnoughData](crate::error::ErrorType::NotEnoughData) error if the given range
    /// is not contained in the PrimeData's range.
    pub fn new(prime_data: &'a PrimeData, range: RangeInclusive<u64>) -> PrimeResult<Self> {
        if let Err(out_of_bounds) = prime_data.range.contains_range(&range) {

            let error = PrimeError {
                context: ErrorContext { action: ErrorAction::Reading, source: ErrorSource::PrimeData },
                error: ErrorType::NotEnoughData(out_of_bounds)
            };

            return Err(error)
        }

        let (range_start, stop_at) = range.into_inner();

        let original_offset = prime_data.offset();
        let data_start = range_start.div_floor(30) as usize - original_offset;
        let data_end = stop_at.div_ceil(30) as usize - original_offset;

        let data = &prime_data.data[data_start..data_end];
        let data_offset = (data_start + original_offset) as u64;
        let mut current = (0u64, 0usize);

        let primes = loop {

            if let Some(byte) = data.get(current.0 as usize) {
                let byte_primes = if current.0 == 0 {
                    let range_offset = data_offset + current.0;
                    let start = (range_start - 30 * range_offset) as u8;
                    byte.as_primes_in_range(range_offset, start..=(start + 30))
                } else {
                    byte.as_primes(data_offset + current.0)
                };
                if byte_primes.len() > 0 {
                    // PrimeData does not store the primes {2, 3, 5}, so if the range includes any
                    // of those, we need to manually add them to the first vector
                    let byte_primes = vec![2u64, 3u64, 5u64].into_iter()
                    .filter(|&x| x >= range_start)
                    .chain(byte_primes.into_iter())
                    .collect();

                    break Some(byte_primes);
                }

                current.0 += 1;
            } else {
                break None;
            }

        };

        Ok(Self { data, primes, current, data_offset, stop_at })
    }
}

impl<'a> Iterator for PrimeIter<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(vector) = &self.primes {

            let current_prime = vector[self.current.1];
            if current_prime > self.stop_at { return None }

            if self.current.1 + 1 < vector.len() {
                self.current.1 += 1;
            } else {
                self.current.1 = 0;
                self.current.0 += 1;

                self.primes = loop {

                    if let Some(byte) = self.data.get(self.current.0 as usize) {
                        let byte_primes = byte.as_primes(self.data_offset + self.current.0);
        
                        if byte_primes.len() > 0 {
                            break Some(byte_primes);
                        }
        
                        self.current.0 += 1;
                    } else {
                        break None;
                    }
        
                };
            }

            Some(current_prime)

        } else {
            None
        }
    }
}