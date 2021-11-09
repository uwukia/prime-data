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
/// The simplest one. Just give that function a range and it'll return a PrimeData instance with all
/// primes that lie in the range you gave.
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
/// data you're trying to generate. Finally, it does one last expansin from `sqrt(N)` to `N`.
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
    /// Returns a [NotEnoughData](crate::error::ErrorType::NotEnoughData) error if the given range
    /// falls out of the data (self) range.
    /// 
    /// See [`PrimeData::iter`].
    pub fn try_iter<'a>(&'a self, range: RangeInclusive<u64>) -> PrimeResult<PrimeIter<'a>> {
        PrimeIter::new(self, range)
    }

    /// Creates an iterator over the given range
    /// 
    /// Will iterate over all primes in the given range. See [`Self::try_iter`]
    /// if you wish to return an error instead of panicking.
    /// 
    /// If you wish to iterate over all primes within the given range, see [`PrimeData::iter_all`].
    /// 
    /// # Panics
    /// 
    /// Panics if the given range falls out of the data (self) range.
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
    /// If you wish to iterate over primes within a specific range, see [`PrimeData::iter`]
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
    /// Returns a [NotEnoughData](crate::error::ErrorType::NotEnoughData) error if
    /// the given range falls out of the data (self) range.
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
    /// The expanded data will contain all primes in the given range.
    /// 
    /// See [`PrimeData::try_expand`] if you wish to return an error instead of panicking.
    /// 
    /// # Panics
    ///
    /// Panics if the given PrimeData does not have enough data to expand into the given range.
    /// 
    /// More specifically, if you wish to expand it to some prime data with range (X..=Y), the
    /// PrimeData (self) must have a range from 7 to √Y or more.
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
    /// To learn more, read the [guide](crate::guide::data_structure::_2_prime_data)
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
    /// Returns an [OutOfBounds](crate::error::ErrorType::OutOfBounds) error if
    /// the data range does not contain x.
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
    /// **Note**: If your data starts below 7, it's heavily recommended to use the
    /// [`check_primes`](crate::PrimeData::check_prime) method instead. This is because
    /// this method requires the given parameter to lie within the data range. But the other only
    /// requires that the data range includes `7..=sqrt(parameter)`.
    /// 
    /// See [`PrimeData::try_is_prime`] if you wish to return an error instead of panicking.
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
    /// Returns a [NotEnoughData](crate::error::ErrorType::NotEnoughData) error if
    /// the given range falls out of the data (self) range.
    /// 
    /// See [`PrimeData::count_primes_in_range`].
    pub fn try_count_primes_in_range(&self, range: RangeInclusive<u64>) -> PrimeResult<u64> {
        if let Err(missing_range) = self.range.contains_range(&range) {
            let error = PrimeError {
                context: ErrorContext { action: ErrorAction::Reading, source: ErrorSource::PrimeData },
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
    /// See [`PrimeData::try_count_primes_in_range`] if you wish to return an error instead of panicking.
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
    /// If you wish to only count primes within a specific range, see [`PrimeData::count_primes_in_range`].
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
    /// Returns `true` if and only if the the range end is greater than the range start.
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

    /// Tries to find the nth prime using the given data
    /// 
    /// Returns a [NotEnoughData](crate::error::ErrorType::NotEnoughData) error in two situations:
    /// 
    /// * The data starts anywhere after 7: This function requires that we count all primes up to
    /// some bound, so we need the range to start at the beginning. Anywhere `<= 7` suffices.
    /// * The data doesn't have n primes: Naturally, if we want the 1000th prime, we can't retrieve
    /// it if the data only has 999.
    /// 
    /// Returns an [OutOfBounds](crate::error::ErrorType::OutOfBounds) error if `nth` is zero.
    /// 
    /// See [`Self::nth_prime`]
    pub fn try_nth_prime(&self, nth: u64) -> PrimeResult<u64> {

        match nth {
            0 => {
                let error = PrimeError {
                    context: ErrorContext { action: ErrorAction::Reading, source: ErrorSource::PrimeData },
                    error: ErrorType::OutOfBounds(nth)
                };
    
                return Err(error)
            },
            1 => return Ok(2),
            2 => return Ok(3),
            3 => return Ok(5),
            _ => {}
        }

        let total_primes = self.count_primes();

        if let Err(missing_range) = self.range.contains_range(&(7..=total_primes)) {

            let error = PrimeError {
                context: ErrorContext { action: ErrorAction::Reading, source: ErrorSource::PrimeData },
                error: ErrorType::NotEnoughData(missing_range)
            };

            return Err(error)
        }

        let (unchecked_start, unchecked_end) = super::estimate::nth_prime_bounds(nth).into_inner();
        let start = std::cmp::max(7, unchecked_start);
        let end = std::cmp::min(*(self.range.end()), unchecked_end);

        let offset = 3 + self.count_primes_in_range(7..=start) - (if self.is_prime(start) { 1 } else { 0 });

        Ok(self.iter(start..=end).nth((nth - offset - 1) as usize).unwrap())
    }

    /// Retrieves the nth prime number from some data
    /// 
    /// If we call "nth prime number" as p(n), we have that p(1) = 2, because 2 is the first prime
    /// number. p(2) = 3, and so on. Therefore, the "zeroth" prime number is not defined.
    /// 
    /// See [`Self::try_nth_prime`] if you wish to return an error instead of panicking.
    /// 
    /// # Panics
    /// 
    /// Panics in the following situations:
    /// 
    /// * `nth` is zero
    /// * PrimeData (self) starts after 7
    /// * PrimeData (self) ends before `nth`
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeData;
    /// let data = PrimeData::generate(7..=105_000);
    /// 
    /// assert_eq!(data.nth_prime(1), 2);
    /// assert_eq!(data.nth_prime(4), 7);
    /// assert_eq!(data.nth_prime(19), 67);
    /// assert_eq!(data.nth_prime(10001), 104743);
    /// ```
    pub fn nth_prime(&self, nth: u64) -> u64 {
        self.try_nth_prime(nth).unwrap()
    }

    /// Tries to verify if the given number is prime
    /// 
    /// Returns a [NotEnoughData](crate::error::ErrorType::NotEnoughData) error if both are true:
    /// 
    /// * The data range does not include x
    /// * The data range does not contain the range between 7 and √x
    /// 
    /// See [`Self::check_prime`]
    pub fn try_check_prime(&self, x: u64) -> PrimeResult<bool> {
        if self.range.contains(&x) {
            self.try_is_prime(x)
        } else {

            let sqrt = x.sqrt_floor();

            if let Err(missing_range) = self.range.contains_range(&(7..=sqrt)) {

                let error = PrimeError {
                    context: ErrorContext { action: ErrorAction::Reading, source: ErrorSource::PrimeData },
                    error: ErrorType::NotEnoughData(missing_range)
                };
    
                return Err(error)
            }

            if (x == 2) || (x == 3) || (x == 5) { return Ok(true) }
            if x.divisible_by(30) { return Ok(false) }

            for prime in self.iter(7..=sqrt) {
                if x.divisible_by(prime) { return Ok(false) }
            }

            Ok(true)
        }
    }

    /// Verifies if the given number is prime
    /// 
    /// If we have information of all prime numbers between 0 and √x, we can verify if x is prime
    /// by checking if none of those primes divide x. Therefore, when calling this function, if x
    /// lies inside the data, it's O(1), but if the data has information about all primes up to √x,
    /// it's O(n).
    /// 
    /// See [`Self::try_check_prime`] if you wish to return an error instead of panicking.
    /// 
    /// # Panics
    /// 
    /// Panics if both are true:
    /// 
    /// * The data range does not include x
    /// * The data range does not contain the range between 7 and √x
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeData;
    /// 
    /// let data = PrimeData::generate(7..=35);
    /// 
    /// assert!( data.check_prime(7));  // O(1)
    /// assert!(!data.check_prime(35)); // O(1)
    /// assert!( data.check_prime(1123));  // O(n)
    /// assert!(!data.check_prime(1225));  // O(n)
    /// ```
    pub fn check_prime(&self, x: u64) -> bool {
        self.try_check_prime(x).unwrap()
    }

    /// Tries to factorize the given number into prime factors.
    /// 
    /// Returns a [NotEnoughData](crate::error::ErrorType::NotEnoughData) error if
    /// the data (self) range does not contain the range `2..=sqrt(x)`.
    /// 
    /// See [`Self::factorize`]
    #[cfg(feature = "factors")]
    pub fn try_factorize(&self, x: u64) -> PrimeResult<super::Factorization> {

        let mut number = x;
        let sqrt = x.sqrt_floor();

        if let Err(missing_range) = self.range.contains_range(&(2..=sqrt)) {
            let error = PrimeError {
                context: ErrorContext { action: ErrorAction::Reading, source: ErrorSource::PrimeData },
                error: ErrorType::NotEnoughData(missing_range)
            };

            return Err(error)
        }

        let mut factorization = super::Factorization::new();

        for prime in self.iter(2..=sqrt) {
            while number % prime == 0 {
                let other_factor = number / prime;
                factorization.add_factor(prime);
                number /= prime;
            }
        }

        if number > 1 {
            factorization.add_factor(number);
        }

        Ok(factorization)
    }

    /// Factorizes the given number into prime factors.
    /// 
    /// See [`Self::try_factorize`] if you wish to return an error instead of panicking.
    /// 
    /// # Panics
    /// 
    /// Panics if the data (self) range does not contain the range `2..=sqrt(x)`.
    /// 
    /// # Examples
    /// 
    /// This function returns a [Factorization](super::Factorization) struct. Read
    /// to the struct's documentation for its methods and examples.
    #[cfg(feature = "factors")]
    pub fn factorize(&self, x: u64) -> super::Factorization {
        self.try_factorize(x).unwrap()
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
mod tests {}