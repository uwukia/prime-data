//! Module dedicated to the PrimeByte struct

use std::ops::RangeInclusive;
use super::K_VALUES;

/// A "byte of primes", a chunk of 8 bits corresponding to the 8 values in the (0..30) range
/// that are not divisible by 2, 3, or 5. Those values are also called **k-values**.
/// 
/// Click [here](crate::guide::prime_byte) to learn more
/// 
/// "k-values" are values k, such that (N % 30 = k) and N is coprime with 30. Those values
/// are listed [here](crate::data::K_VALUES).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PrimeByte {
    byte: u8,
}

impl PrimeByte {
    /// Creates a new byte, setting all k-values as prime
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeByte;
    /// let byte = PrimeByte::new();
    /// assert_eq!(u8::from(byte), 0b11111111);
    /// ```
    pub fn new() -> Self {
        Self { byte: 255 }
    }

    /// Sets one of the bits to non-prime/composite based on the k-value
    ///
    /// If the given value is not a k-value, returns an error.
    /// If the bit was already set to non-prime, returns false.
    /// Otherwise, returns true.
    /// 
    /// To understand what k-value maps to what bit, refer to the [k-values](crate::data::K_VALUES)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeByte;
    /// 
    /// let mut byte = PrimeByte::from(0b10110111);
    /// // k-value 1 corresponds to the first bit
    /// // returns Ok(true) because the first bit was indeed, a one
    /// assert!(byte.set_nonprime(1).unwrap());
    /// assert_eq!(byte.as_u8(), 0b00110111);
    /// // now, if we try to set it to non-prime again, it'll return false,
    /// // because it's already a zero
    /// assert!(!byte.set_nonprime(1).unwrap());
    /// // when given a non-k-value, returns an error
    /// assert!(byte.set_nonprime(4).is_err());
    /// ```
    pub fn set_nonprime(&mut self, k_value: u8) -> Result<bool, ()> {
        if let Ok(index) = K_VALUES.binary_search(&k_value) {

            let bit = self.byte >> (7 - index);
            let is_prime = Self::is_one(bit);

            if is_prime {
                self.byte -= 1 << (7 - index);
                Ok(true)
            } else {
                Ok(false)
            }

        } else {
            Err(())
        }
    }

    /// Converts the bits into boolean entries
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeByte;
    /// let byte = PrimeByte::from(0b10100110);
    /// assert_eq!(
    ///     byte.as_boolean_array(),
    ///     [true, false, true, false, false, true, true, false]
    /// );
    /// ```
    pub fn as_boolean_array(&self) -> [bool; 8] {

        let byte = self.byte;

        [7, 6, 5, 4, 3, 2, 1, 0]
        .map(|shr| PrimeByte::is_one(byte >> shr))
    }

    /// Retrieves all bits set to one and converts them into their respective k-values
    /// 
    /// For more information, read the [guide](crate::guide::prime_byte), or refer to the
    /// list of [k-values](crate::data::K_VALUES). 
    /// 
    /// If you wish to retrieve k-values within some range, see [`PrimeByte::as_k_values_in_range`].
    ///
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeByte;
    /// let byte = PrimeByte::from(0b10100110);
    /// assert_eq!(
    ///     byte.as_k_values(),
    ///     vec![1, 11, 19, 23]
    /// );
    /// ```
    pub fn as_k_values(&self) -> Vec<u8> {
        self.as_boolean_array().iter()
        .zip(K_VALUES.iter())
        .filter(|(&is_prime, _)| is_prime)
        .map(|(_, &k_value)| k_value)
        .collect()
    }

    /// Retrieves all bits set to one and converts them into their respective k-values,
    /// as long as they fall inside the inclusive range
    /// 
    /// For more information, read the [guide](crate::guide::prime_byte), or refer to the
    /// list of [k-values](crate::data::K_VALUES). 
    /// 
    /// If you wish to retrieve all k-values, see [`PrimeByte::as_k_values`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeByte;
    /// let byte = PrimeByte::from(0b10100110);
    /// assert_eq!(
    ///     byte.as_k_values_in_range(2..=23),
    ///     vec![11, 19, 23]
    /// );
    /// ```
    pub fn as_k_values_in_range(&self, range: RangeInclusive<u8>) -> Vec<u8> {
        self.as_boolean_array().iter()
        .zip(K_VALUES.iter())
        .filter(|(&is_prime, k_value)| is_prime && range.contains(k_value))
        .map(|(_, &k_value)| k_value)
        .collect()
    }

    /// Retrieves the k-values and converts them to actual prime numbers.
    ///
    /// For more information, read the [guide](crate::guide::prime_byte).
    ///
    /// Calling this function with `offset = 1` yields the same results as [`PrimeByte::as_k_values`],
    /// except that it returns a vector of [u64] instead of [u8].
    /// 
    /// If you wish to only retrieve primes within a range, see [`PrimeByte::as_primes_in_range`]
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeByte;
    /// let byte = PrimeByte::from(0b10100110);
    /// assert_eq!(
    ///     byte.as_primes(21),
    ///     vec![631, 641, 649, 653]
    /// );
    /// ```
    pub fn as_primes(&self, offset: u64) -> Vec<u64> {
        self.as_boolean_array().iter()
        .zip(K_VALUES.iter())
        .filter(|(&is_prime, _)| is_prime)
        .map(|(_, &k_value)| 30 * offset + (k_value as u64))
        .collect()
    }

    /// Retrieves the k-values, as long as they fall inside the inclusive range,
    /// and converts them to actual prime numbers
    /// 
    /// For more information, read the [guide](crate::guide::prime_byte).
    ///
    /// Calling this function with `offset = 1` yields the same results as
    /// [`PrimeByte::as_k_values_in_range`], except that it returns a vector of [u64] instead of [u8].
    /// 
    /// If you wish to retrieve all primes, see [`PrimeByte::as_primes`]
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeByte;
    /// let byte = PrimeByte::from(0b10100110);
    /// assert_eq!(
    ///     byte.as_primes_in_range(21, 2..=23),
    ///     vec![641, 649, 653]
    /// );
    /// ```
    pub fn as_primes_in_range(&self, offset: u64, range: RangeInclusive<u8>) -> Vec<u64> {
        self.as_boolean_array().iter()
        .zip(K_VALUES.iter())
        .filter(|(&is_prime, k_value)| is_prime && range.contains(k_value))
        .map(|(_, &k_value)| 30 * offset + (k_value as u64))
        .collect()
    }

    /// Overwrites the bits in this byte with the bits in the other byte, from a given position
    /// 
    /// The position should be a bit index in the range (0..=7), 0 meaning it'll overwrite the
    /// entire byte with the given one.
    /// 
    /// This function is useful for joining two data structures of Prime Bytes, as most of the time
    /// the former's end and the latter's start intersect in the middle of a prime byte. That way,
    /// we overwrite the byte at the intersection.
    /// 
    /// # Panics
    /// 
    /// Panics if `position > 7`
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeByte;
    /// let mut original = PrimeByte::from(0b00000000);
    /// let     new_bits = PrimeByte::from(0b11111111);
    /// original.overwrite_at(new_bits, 5);
    /// assert_eq!(
    ///     u8::from(original),
    ///     0b00000111
    /// );
    /// ```
    pub fn overwrite_at(&mut self, overwriting: Self, position: u8) {

        self.byte = if position == 0 {
            overwriting.byte
        } else {
            let clear_byte = (self.byte >> (8 - position)) << (8 - position);
            let new_bits = (overwriting.byte << position) >> position;
    
            clear_byte + new_bits
        }

    }

    /// Vefifies if the given `x` is prime, based on the [k-values](crate::data::K_VALUES)
    /// 
    /// **Warning**: It will return false for 2, 3, and 5. So take care of those cases
    /// before you call this function.
    /// 
    /// **Second Warning**: Will return false for values above 29. So always make sure to
    /// apply modulo 30 when calling this function.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeByte;
    /// let byte = PrimeByte::from(0b00101000);
    /// assert!( byte.is_prime(11));
    /// assert!(!byte.is_prime(13));
    /// assert!( byte.is_prime(17));
    /// ```
    pub fn is_prime(&self, x: u8) -> bool {
        if let Some(thing) = self.as_boolean_array().iter()
            .zip(K_VALUES.iter())
            .find(|(_, &k_value)| k_value == x)
        {
            *thing.0
        } else {
            false
        }
    }

    /// Counts the number of primes (a.k.a the number of ones) it has.
    /// 
    /// If you wish to count primes in some range, see [`PrimeByte::count_primes_in_range`]
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeByte;
    /// let byte = PrimeByte::from(0b11010111);
    /// assert_eq!(byte.count_primes(), 6);
    /// ```
    pub fn count_primes(&self) -> u64 {
        self.byte.count_ones() as u64
    }

    /// Counts the number of primes it has as long as their k-values fall within the range
    /// 
    /// If you wish to count all primes, see [`PrimeByte::count_primes`]
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeByte;
    /// let byte = PrimeByte::from(0b11010111);
    /// assert_eq!(byte.count_primes_in_range(0..=30), 6);
    /// assert_eq!(byte.count_primes_in_range(2..=30), 5);
    /// assert_eq!(byte.count_primes_in_range(8..=12), 0);
    /// ```
    pub fn count_primes_in_range(&self, range: RangeInclusive<u8>) -> u64 {
        self.as_boolean_array().iter()
        .zip(K_VALUES.iter())
        .filter(|(&is_prime, k_value)| is_prime && range.contains(k_value))
        .count() as u64
    }

    /// Converts byte into a u8
    /// 
    /// This has the same effect as calling `u8::from()`. It's meant to be an alternative way
    /// of converting into a u8, having the byte be written on the left instead of right.
    /// 
    /// Technically, only requires a reference when `u8::from` consumes it, however, PrimeByte
    /// implements [`Copy`], so there's no difference.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeByte;
    /// let byte = PrimeByte::new();
    /// assert_eq!(byte.as_u8(), u8::from(byte))
    /// ```
    pub fn as_u8(&self) -> u8 {
        self.byte
    }

    /// Compares two bytes to see if their bits match in the given k-value range
    /// 
    /// Range is expected to be within (0..=30), but it will not return an error or panic, if given
    /// anything above 30. If you pass it some range like (30..=199), since none of the bits fall in
    /// that range, none will be compared, and hence the function will trivially return true.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::PrimeByte;
    /// let byte  = PrimeByte::from(0b10111010);
    /// let other = PrimeByte::from(0b10110111);
    /// assert!( byte.matches_in_range(other, 0..=14));
    /// assert!( byte.matches_in_range(other, 23..=23));
    /// assert!(!byte.matches_in_range(other, 16..=17));
    /// assert!( byte.matches_in_range(other, 30..=255));
    /// ```
    pub fn matches_in_range(&self, other: PrimeByte, range: RangeInclusive<u8>) -> bool {
        self.as_boolean_array().iter()
        .zip(other.as_boolean_array().iter())
        .zip(K_VALUES.iter())
        .filter(|(_, k_value)| range.contains(k_value))
        .fold(true, |acc, (cur, _)| acc && (cur.0 == cur.1))
    }

    /// Returns whether the last bit of `bit` is a one
    fn is_one(bit: u8) -> bool {
        bit % 2 == 1
    }
}

impl From<u8> for PrimeByte {
    fn from(byte: u8) -> PrimeByte {
        PrimeByte { byte }
    }
}

impl From<PrimeByte> for u8 {
    fn from(byte: PrimeByte) -> u8 {
        byte.byte
    }
}

use std::fmt;
impl fmt::Display for PrimeByte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{:08b}|", self.byte)
    }
}

impl fmt::Debug for PrimeByte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}