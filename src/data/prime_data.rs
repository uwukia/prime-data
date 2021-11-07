use std::{ops::RangeInclusive, cmp};
use super::{PrimeByte, PrimeIter, CoprimeIter, error::*, utils::{IntSqrt, ContainsRange, Divisible}};

/// An abstraction over storing prime numbers
/// 
/// Stores whether a number is prime or not, as a bit being one or zero. Each bit corresponds
/// to some number that is coprime with 30. This is to avoid trivial nonprimes such as even
/// numbers, or multiples of 3 or 5. In every integer range [30k, 30(k+1)) there are 8 of
/// those numbers. If we take them modulo 30, we'll have a set that in this library, I call
/// [k-values](crate::data::K_VALUES).
/// 
/// To learn more about this struct and this library as a whole, read
/// [the guide](crate::guide).
/// 
/// # Fields
/// 
/// * Data - The bit data of wether numbers are prime or not
/// * Range - What range of integers this data has about primes
/// 
/// They're both private fields for the sake of abstraction. The Range can be accessed through
/// the [`PrimeData::range`] method. The data itself, as it is merely a huge collection of
/// bits, cannot be accessed raw. However, all of the basic information you could need to
/// reach (such as counting the amount of prime numbers, verify if a number is prime or not,
/// or even iterate over its prime numbers) can be done through its other methods.
/// 
/// # Creating PrimeData
/// 
/// There are two basic ways of generating prime number data
/// 
/// 1. [`PrimeData::generate`]
/// 
/// 2. [`PrimeData::expand`]
/// 
/// If you already have some PrimeData that has all primes until some number N, and you want to gather
/// some new PrimeData that ranges up to N², it's more efficient to use the data you already have to
/// "expand" into the new one you want.
/// 
/// In fact, the [generator](PrimeData::generate) method is just an abstraction over the expand function.
/// What it does is, it calls the [`PrimeData::new`] method to generate prime numbers below 30, then
/// expands it into bigger and bigger data until it hits `sqrt(N)`, where N is the upper bound of the
/// data you're trying to generate.
pub struct PrimeData {
    pub(crate) data: Vec<PrimeByte>,
    pub(crate) range: RangeInclusive<u64>,
}

impl PrimeData {
  // methods for data generation

    /// Creates a new piece of "starter data", which are all the primes below 30
    /// 
    /// If you wish to create some starter data that gets primes below some number bigger than 30,
    /// see [`PrimeData::generate`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeData;
    /// let new = PrimeData::new();
    /// let gen = PrimeData::generate(0..=30);
    /// 
    /// assert_eq!(       new.range(), gen.range());
    /// assert_eq!(new.count_primes(), gen.count_primes());
    /// assert_eq!(  new.is_prime(29), gen.is_prime(29));
    /// ```
    pub fn new() -> Self {
        Self {
            data: vec![(0b01111111).into()],
            range: (0..=30),
        }
    }

    /// Generates PrimeData with all prime numbers between the given range
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeData;
    /// let data = PrimeData::generate(0..=100);
    /// 
    /// assert!( data.is_prime(2));
    /// assert!(!data.is_prime(49));
    /// 
    /// assert_eq!(data.range(), (0, 100));
    /// assert_eq!(data.count_primes(), 25);
    /// ```
    pub fn generate(range: RangeInclusive<u64>) -> Self {
        let (start, end) = range.into_inner();

        if start > end {

        }

        if end <= 900 {
            PrimeData::new().expand(start..=end)
        } else {
            let sqrt_end = end.sqrt_floor();
            PrimeData::generate(0..=sqrt_end).expand(start..=end)
        }
    }

  // methods for iteration

    /// Tries to create an iterator over the given range
    /// 
    /// Will try to iterate over all primes in the given range, returning an error if the
    /// given range falls out of the targeted PrimeData range.
    /// 
    /// See [`PrimeData::iter`].
    pub fn try_iter<'a>(&'a self, range: RangeInclusive<u64>) -> PrimeResult<PrimeIter<'a>> {
        PrimeIter::new(self, range)
    }

    /// Creates an iterator over the given range
    /// 
    /// Will iterate over all primes in the given range, if you wish to return an error instead of
    /// panicking, see [`PrimeData::try_iter`].
    /// 
    /// If you wish to iterate over all primes in the given range, see [`PrimeData::iter_all`].
    /// 
    /// # Panics
    /// 
    /// Panics if the given range falls out of the targeted PrimeData range.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeData;
    /// let data = PrimeData::generate(0..=1000);
    /// let mut iter = data.iter(472..=491);
    /// 
    /// assert_eq!(iter.next(), Some(479));
    /// assert_eq!(iter.next(), Some(487));
    /// assert_eq!(iter.next(), Some(491));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///     
    /// ```
    /// # use prime_data::PrimeData;
    /// # let data = PrimeData::generate(0..=1000);
    /// // fun fact: the biggest gap between to primes up to 1000 is 20,
    /// // and it happens between 887 and 907
    /// let mut iter = data.iter(888..=906);
    /// 
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter<'a>(&'a self, range: RangeInclusive<u64>) -> PrimeIter<'a> {
        self.try_iter(range).unwrap()
    }

    /// Iterates over all prime numbers in the given data.
    /// 
    /// This is useful if you wish to collect all of the data's primes into a vector.
    /// 
    /// **Warning**: PrimeData are meant to be really condensed, so when you extract raw prime vectors
    /// from it, it could grow in size up to 8 times as itself. So collect this iterator carefully!
    /// 
    /// If you wish to iterate over primes within some range, see [`PrimeData::iter`]
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeData;
    /// let data = PrimeData::generate(472..=491);
    /// let mut iter = data.iter_all();
    /// 
    /// assert_eq!(iter.next(), Some(479));
    /// assert_eq!(iter.next(), Some(487));
    /// assert_eq!(iter.next(), Some(491));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_all<'a>(&'a self) -> PrimeIter<'a> {
        self.iter(self.range.clone())
    }

  // methods for expansion/generation

    /// Tries to expand the current PrimeData into more PrimeData
    /// 
    /// The expanded data will contain all primes in `range`
    /// 
    /// Returns an error if the current PrimeData does not have enough data
    /// to retrieve primes in the given range.
    /// 
    /// See [`PrimeData::expand`].
    pub fn try_expand(&self, range: RangeInclusive<u64>) -> PrimeResult<Self> {

        let (start, end) = range.bounds();
        let end_sqrt = end.sqrt_floor();

        if let Err(missing_range) = self.range.contains_range(&(7..=end_sqrt)) {

            let error = PrimeError {
                context: ErrorContext { action: ErrorAction::Modifying, source: ErrorSource::PrimeData },
                error: ErrorType::NotEnoughData(missing_range)
            };

            return Err(error)
        }

        let mut expanded_data = Self::create_empty(range);
        if expanded_data.is_empty() { return Ok(expanded_data) }

        for prime in self.iter(7..=end_sqrt) {
            let lower_bound = cmp::max(start.div_ceil(prime), 7);
            let upper_bound = end.div_floor(prime);
            for multiplier in CoprimeIter::new(lower_bound..=upper_bound) {
                let composite_number = prime * multiplier;
                expanded_data.set_nonprime(composite_number).unwrap();
            }
        }

        Ok(expanded_data)
    }

    /// Expands the current PrimeData into more PrimeData
    /// 
    /// The expanded data will contain all primes in `range`
    /// 
    /// If you wish to return an error instead of panicking, see [`PrimeData::try_expand`]
    /// 
    /// # Panics
    ///
    /// Panics if the given PrimeData does not have enough data to expand into the given range.
    /// 
    /// More specifically, if you wish to expand it to some prime data with range (X..=Y), the
    /// current PrimeData must have a range from 7 to √Y or more.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeData;
    /// 
    /// let starter_data = PrimeData::new();
    /// let expanded_data = starter_data.expand(59..=90);
    /// 
    /// assert_eq!(
    ///     expanded_data.iter_all().collect::<Vec<u64>>(),
    ///     vec![59, 61, 67, 71, 73, 79, 83, 89]
    /// );
    /// ```
    pub fn expand(&self, range: RangeInclusive<u64>) -> Self {
        self.try_expand(range).unwrap()
    }

  // general methods

    /// Destructures the PrimeData range into (start, end)
    /// 
    /// **Note**: The range is inclusive.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeData;
    /// let expansion = PrimeData::generate(173..=244);
    /// assert_eq!(expansion.range(), (173, 244));
    /// ```
    pub fn range(&self) -> (u64, u64) {
        (*(self.range.start()), *(self.range.end()))
    }

    /// Retrieves the PrimeData offset
    /// 
    /// PrimeData stores its raw data based on its range. The data starts at ⌊ range.start / 30 ⌋
    /// 
    /// To know more, read the [guide](crate::guide::data).
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeData;
    /// let data = PrimeData::new();
    /// 
    /// assert_eq!(data.expand(29..=100).offset(), 0);
    /// assert_eq!(data.expand(30..=100).offset(), 1);
    /// assert_eq!(data.expand(59..=100).offset(), 1);
    /// ```
    pub fn offset(&self) -> usize {
        *(self.range.start()) as usize / 30
    }

    /// Tries to verify if the given number is prime
    /// 
    /// See [`PrimeData::is_prime`]
    pub fn try_is_prime(&self, x: u64) -> PrimeResult<bool> {
        if self.range.contains(&x) && !self.is_empty() {

            if (x == 2) || (x == 3) || (x == 5) { return Ok(true) }
            if x.divisible_by(30) { return Ok(false) }

            let index = self.data_index_that_contains(x).unwrap();
            Ok(self.data[index].is_prime((x % 30) as u8))
        } else {
            let error = PrimeError {
                context: ErrorContext { action: ErrorAction::Reading, source: ErrorSource::PrimeData },
                error: ErrorType::OutOfBounds(x)
            };

            return Err(error)
        }
    }

    /// Verifies if the given number is prime
    /// 
    /// If you want to avoid panicking and error handle, see [`PrimeData::try_is_prime`].
    /// 
    /// # Panics
    /// 
    /// Panics if it falls out of the data range
    /// 
    /// ```
    /// use prime_data::PrimeData;
    /// let data = PrimeData::generate(0..=900);
    /// 
    /// assert!( data.is_prime(727));
    /// assert!(!data.is_prime(781));
    /// ```
    /// 
    /// 907 is a prime number, but it's not in the range (0..=900).
    /// ```should_panic
    /// # use prime_data::PrimeData;
    /// # let data = PrimeData::generate(0..=900);
    /// assert!(data.is_prime(907));
    /// ```
    pub fn is_prime(&self, x: u64) -> bool {
        self.try_is_prime(x).unwrap()
    }

    /// Tries to count the amount of prime numbers in a given range
    /// 
    /// Returns an error if the range goes out of the data's range. Keep in mind that if the range
    /// start is greater than the range end, Rust interprets that as an empty range, and this function
    /// will return zero.
    /// 
    /// See [`PrimeData::count_primes_in_range`].
    pub fn try_count_primes_in_range(&self, range: RangeInclusive<u64>) -> PrimeResult<u64> {
        if let Err(missing_range) = self.range.contains_range(&range) {
            let error = PrimeError {
                context: ErrorContext { action: ErrorAction::Modifying, source: ErrorSource::PrimeData },
                error: ErrorType::NotEnoughData(missing_range)
            };

            return Err(error)
        }

        if self.is_empty() { return Ok(0) }

        // primedata does not take 2, 3, and 5 into account
        let missing_primes = [2, 3, 5].iter().filter(|x| range.contains(x)).count() as u64;

        let (start, end) = range.into_inner();

        use cmp::Ordering;
        match start.cmp(&end) {
            // start > end implies range.is_empty()
            Ordering::Greater => { Ok(0) },
            // start = end implies range is a singular value
            // fun fact: in maths we call this a degenerate interval
            Ordering::Equal => { if self.is_prime(start) { Ok(1) } else { Ok(0) } },
            // most of the time, start < end
            Ordering::Less => {

                let start_index = self.data_index_that_contains(start).unwrap();
                let end_index = self.data_index_that_contains(end).unwrap();

                let start_mod = (start % 30) as u8;
                let end_mod = (end % 30) as u8;

                if start.divisible_by(30) && end.divisible_by(30) {
                    let end_index = start_index + ((end - start) as usize / 30);

                    let prime_count = self.data[start_index..end_index].iter()
                    .fold(0u64, |acc, cur| acc + cur.count_primes());

                    Ok(missing_primes + prime_count)
                } else if start.divisible_by(30) {
                    let prime_count = self.data[start_index..end_index].iter()
                    .fold(0u64, |acc, cur| acc + cur.count_primes());

                    let last_primes = self.data[end_index]
                    .count_primes_in_range(0..=end_mod);

                    Ok(missing_primes + prime_count + last_primes)
                } else if end.divisible_by(30) {
                    let first_primes = self.data[start_index]
                    .count_primes_in_range(start_mod..=30);

                    if start_index == end_index || start + 30 > end {
                        Ok(missing_primes + first_primes)
                    } else {
                        let end_index = if end == *(self.range.end()) { end_index } else { end_index - 1 };

                        let prime_count = self.data[(start_index+1)..=end_index].iter()
                        .fold(0u64, |acc, cur| acc + cur.count_primes());

                        Ok(missing_primes + first_primes + prime_count)
                    }
                } else {
                    // from here, we know (start % 30 != 0 != end % 30)
                    if start_index == end_index {
                        let prime_range = self.data[start_index]
                        .count_primes_in_range(start_mod..=end_mod);

                        Ok(missing_primes + prime_range)
                    } else {
                        let first_primes = self.data[start_index]
                        .count_primes_in_range(start_mod..=30);

                        let prime_count = self.data[(start_index+1)..end_index].iter()
                        .fold(0u64, |acc, cur| acc + cur.count_primes());

                        let last_primes = self.data[end_index]
                        .count_primes_in_range(0..=end_mod);

                        Ok(missing_primes + first_primes + prime_count + last_primes)
                    }
                }
            }
        }
    }

    /// Counts the amount of prime numbers in a given range
    /// 
    /// Keep in mind that if the range start is greater than the range end, Rust interprets that as an
    /// empty range, and this function will return zero. 
    /// 
    /// If you wish for it to return an error instead of panic, see [`PrimeData::try_count_primes_in_range`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeData;
    /// let data = PrimeData::new();
    /// 
    /// assert_eq!(data.count_primes_in_range(0..=30), 10);
    /// assert_eq!(data.count_primes_in_range(2..=7),   4);
    /// assert_eq!(data.count_primes_in_range(2..=2),   1);
    /// assert_eq!(data.count_primes_in_range(24..=26), 0);
    /// assert_eq!(data.count_primes_in_range(30..=0),  0);
    /// ```
    pub fn count_primes_in_range(&self, range: RangeInclusive<u64>) -> u64 {
        self.try_count_primes_in_range(range).unwrap()
    }

    /// Counts the amount of prime numbers in the entire data
    /// 
    /// If you wish to only count primes within some range, see [`PrimeData::count_primes_in_range`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeData;
    /// let below_30   = PrimeData::new();
    /// let below_100  = below_30.expand(0..=100);
    /// let below_1000 = below_100.expand(0..=1000);
    /// 
    /// assert_eq!(below_30.count_primes(), 10);
    /// assert_eq!(below_100.count_primes(), 25);
    /// assert_eq!(below_1000.count_primes(), 168);
    /// ```
    pub fn count_primes(&self) -> u64 {
        self.count_primes_in_range(self.range.clone())
    }

    /// Verifies if the data is empty.
    /// 
    /// This will guaranteed to happen if the range start is greated than range end.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeData;
    /// 
    /// assert!(!PrimeData::generate(17..=71).is_empty());
    /// assert!(!PrimeData::generate(29..=29).is_empty());
    /// assert!( PrimeData::generate(31..=30).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        let (start, end) = self.range();
        start > end // || self.data.len() == 0
    }
}

// private methods
impl PrimeData {
    // Creates "empty" data, with all bits set to one.
    // Should only be called by expansion functions.
    fn create_empty(range: RangeInclusive<u64>) -> Self {
        let data_start = (*range.start()).div_floor(30);
        let data_end   = (*range.end()).div_ceil(30);

        if data_start >= data_end {
            return Self { data: vec![], range }
        }

        let data_length = (data_end - data_start) as usize;
        let mut data = vec![PrimeByte::new(); data_length];

        // We want 1 to be set as nonprime by default
        if data_start == 0 {
            data[0].set_nonprime(1).unwrap();
        }

        Self { data, range }
    }

    fn set_nonprime(&mut self, nonprime: u64) -> PrimeResult<bool> {
        if self.range.contains(&nonprime) {
            let data_index = (nonprime / 30) as usize - self.offset();
            let k_value = (nonprime % 30) as u8;

            match self.data[data_index].set_nonprime(k_value) {
                Ok(boolean) => Ok(boolean),
                Err(()) => {
                    let error = PrimeError {
                        context: ErrorContext {
                            action: ErrorAction::Modifying,
                            source: ErrorSource::PrimeData
                        },
                        error: ErrorType::OutOfBounds(nonprime)
                    };
        
                    Err(error)
                },
            }
        } else {

            let error = PrimeError {
                context: ErrorContext { action: ErrorAction::Modifying, source: ErrorSource::PrimeData },
                error: ErrorType::OutOfBounds(nonprime)
            };

            Err(error)
        }
    }

    // Retrieves an index such that `self.data[index]` contains x
    // Returns none if x is out of `self.range`]
    // 
    // if x % 30 == 0, it'll give you the range [x, x+30], unless
    // x is equal to the range ending. this means the data does not
    // contain [x, x+30] and will instead return [x-30, x]
    fn data_index_that_contains(&self, x: u64) -> Option<usize> {

        if self.is_empty() { return None }

        if self.range.contains(&x) {
            let result = (x / 30) as usize - self.offset();
            
            if result >= self.data.len() {
                if x.divisible_by(30) {
                    Some(result - 1)
                } else {
                    panic!("Unexpected error! This should be removed once unit tested!")
                }
            } else {
                Some(result)
            }
        } else {
            None
        }
    }

    fn raw_data<'a>(&'a self) -> &'a [PrimeByte] {
        &self.data
    }
}

use std::fmt;
impl fmt::Debug for PrimeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let digit_len = digit_len(self.range().1);
        let byte_full_size = digit_len + 10;
        let bytes_per_line = bytes_per_line(byte_full_size);
        let line_len = bytes_per_line * (byte_full_size + 1) + 3;

        let title_and_range = format!("{}\n{}\n", title(line_len), range(line_len, self.range()));

        let mut data_str = String::new();
        let offset = self.offset();
        for (idx, chunk) in self.data.chunks(bytes_per_line).enumerate() {
            let outer_offset = offset + (idx * bytes_per_line);
            let mut starter = format!("# ");
            for (i, byte) in chunk.iter().enumerate() {
                let inner_offset = outer_offset + i;
                starter.push_str(&format!("{} ", print_byte(byte, inner_offset, digit_len)));
            }
            starter.push_str(&format!("{}#", " ".repeat(line_len - starter.len() - 1)));

            data_str.push_str(&format!("{}\n", starter));
        }

        let bottom = "#".repeat(line_len);
        let full_debug_string = format!("{}{}{}", title_and_range, data_str, bottom);
        write!(f, "\n{}", full_debug_string)
    }
}

// debug stuff
    fn digit_len(max: u64) -> usize {
        format!("{}", max).len()
    }

    fn print_byte(byte: &PrimeByte, offset: usize, digit_len: usize) -> String {
        format!("{:>width$}{}", offset * 30, byte, width = digit_len)
    }

    fn bytes_per_line(byte_full_size: usize) -> usize {
        128 / byte_full_size
    }

    fn title(line_len: usize) -> String {
        let title = "### PRIME DATA ";
        let filler = "#".repeat(line_len - title.len());
        format!("{}{}", title, filler)
    }

    fn range(line_len: usize, range: (u64, u64)) -> String {
        let (start, end) = range;
        let left = format!("Range: ({} -> {})", start, end);
        let right = " ".repeat(line_len - left.len() - 4);
        format!("# {}{} #", left, right)
    }

#[cfg(test)]
mod tests {
    use super::PrimeData;

    #[test]
    fn gen_billion() {
        let size: u64 = 1u64 << 32;
        let data = PrimeData::generate(0..=size);
        // let data1 = PrimeData::generate(0..=size);
        // let data2 = PrimeData::generate(size..=(2*size));
        println!("{:?}", analyze_bits(&data));
        println!("{}", data.count_primes());

        assert_eq!(data.range(), (1, size));
    }

    fn analyze_bits(data: &PrimeData) -> Vec<(usize, u64)> {

        let mut bit_count = [0u64; 256];

        for byte in data.raw_data().iter() {
            bit_count[byte.as_u8() as usize] += 1;
        }

        let mut vec: Vec<(usize, u64)> = bit_count.into_iter().enumerate().collect();
        vec.sort_by(|a, b| a.1.cmp(&b.1).reverse());

        vec
    }
}