//! Module dedicated to factorizing numbers
use super::{PrimeData, utils::IntSqrt};
use std::collections::HashMap;

/// Retrieves every factor of x
/// 
/// *This function is only available with the `factors` feature enabled.*
/// 
/// A factor is any number that divides x. Therefore, it includes 1 and x itself.
/// 
/// This function is simply an abstraction over creating the [`Factorization`] struct
/// and calling [`Factorization::all_factors`].
/// 
/// **Note**: The returned vector will have length 1, if and only if x is 1. It will have
/// length 2 if and only if x is prime. However, it's much faster to verify if a number
/// is prime using [PrimeData](`crate::PrimeData`).
/// 
/// # Examples
/// 
/// ```
/// use prime_data::all_factors_of;
/// 
/// assert_eq!(all_factors_of(1), vec![1]);
/// assert_eq!(all_factors_of(3), vec![1, 3]);
/// assert_eq!(all_factors_of(6), vec![1, 2, 3, 6]);
/// ```
pub fn all_factors_of(x: u64) -> Vec<u64> {
    Factorization::from(x).all_factors()
}

/// Represents some number into its prime-factorized form
/// 
/// *This struct is only available with the `factors` feature enabled.*
pub struct Factorization {
    data: HashMap<u64, u32>
}

impl Factorization {
    /// Converts some factorization into the original number without consuming itself
    /// 
    /// # Examples
    ///
    /// ```
    /// use prime_data::Factorization;
    /// 
    /// assert_eq!(1, Factorization::from(1).as_u64());
    /// assert_eq!(12, Factorization::from(12).as_u64());
    /// assert_eq!(29375346, Factorization::from(29375346).as_u64());
    /// ```
    pub fn as_u64(&self) -> u64 {
        self.data.iter()
        .fold(1u64, |acc, (&prime, &amount)| acc * prime.pow(amount))
    }

    /// Retrieves the factorization as a tuple (prime, amount)
    /// 
    /// Note that if the vector is empty, it means the original number is 1.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::Factorization;
    /// 
    /// // 43560 = 2^3 * 3^2 * 5 * 11^2
    /// let factorization = Factorization::from(43560);
    /// assert_eq!(
    ///     factorization.as_tuples(),
    ///     vec![(2, 3), (3, 2), (5, 1), (11, 2)]
    /// );
    /// ```
    pub fn as_tuples(&self) -> Vec<(u64, u32)> {
        let mut vec: Vec<(u64, u32)> = self.data.iter().map(|(&p, &c)| (p, c)).collect();
        vec.sort_by(|a, b| a.0.cmp(&(b.0)));

        vec
    }

    /// Retrieves all possible factors of the factorized number
    /// 
    /// It does so by multiplying every possible combination of its prime factors.
    /// 
    /// **Note**: Includes 1 and itself. Therefore, the minimal length for this vector
    /// is 1, happening when the original numbers is 1, then length 2, if and only if
    /// the original number is prime.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use prime_data::Factorization;
    /// 
    /// let thirty = Factorization::from(30);
    /// assert_eq!(
    ///     thirty.all_factors(),
    ///     vec![1, 2, 3, 5, 6, 10, 15, 30]
    /// );
    /// ```
    pub fn all_factors(&self) -> Vec<u64> {
        let tuples = self.as_tuples();

        let mut vector = Self::factor_combos(&tuples);
        vector.sort();

        vector
    }
}

// private methods
impl Factorization {
    pub(crate) fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub(crate) fn add_factor(&mut self, factor: u64) {
        if let Some(amount) = self.data.get_mut(&factor) {
            *amount += 1;
        } else {
            self.data.insert(factor, 1);
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn factor_combos(slice: &[(u64, u32)]) -> Vec<u64> {
        if slice.len() == 0 {
            vec![1]
        } else {
            let inner_combos = Self::factor_combos(&slice[1..]);
            let (this_prime, amount) = slice[0];

            let mut combinations = Vec::new();
            for pow in 0..=amount {
                let new_factor = this_prime.pow(pow);
                combinations.append(&mut (inner_combos.iter().map(|&x| new_factor * x).collect()));
            }

            combinations
        }
    }
}

impl From<u64> for Factorization {
    fn from(number: u64) -> Factorization {
        let prime_data = PrimeData::generate(0..=(number.sqrt_floor()));

        prime_data.factorize(number)
    }
}