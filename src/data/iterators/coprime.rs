use std::ops::RangeInclusive;
use crate::{K_VALUES, data::utils};

/// Struct that iterates over numbers that are coprime with 2, 3, and 5 (a.k.a 30)
/// 
/// To learn more, read the [guide](crate::guide::iterators::_1_coprime).
/// 
/// # Examples
/// 
/// ```
/// use prime_data::CoprimeIter;
/// 
/// let mut iter = CoprimeIter::new(8..=22);
/// assert_eq!(iter.next(), Some(11));
/// assert_eq!(iter.next(), Some(13));
/// assert_eq!(iter.next(), Some(17));
/// assert_eq!(iter.next(), Some(19));
/// assert_eq!(iter.next(), None);
/// ```
pub struct CoprimeIter {
    current: (u64, usize),
    stop_after: u64,
}

impl CoprimeIter {
    /// Creates a new iterator over numbers coprime with 30
    pub fn new(range: RangeInclusive<u64>) -> Self {

        let (start, end) = range.into_inner();

        let offset = start / 30;
        let value = (start % 30) as u8;
        let index = utils::unwrap_any(K_VALUES.binary_search(&value));

        Self { current: (offset, index), stop_after: end }
    }
}

impl Iterator for CoprimeIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let (offset, index) = self.current;
        let current_value = 30 * offset + (K_VALUES[index] as u64);

        if current_value > self.stop_after { return None }

        if index < 7 {
            self.current = (offset, index + 1);
        } else {
            self.current = (offset + 1, 0);
        }

        Some(current_value)
    }
}