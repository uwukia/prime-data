use crate::data::utils::Logarithm;
use std::ops::RangeInclusive;

/// Approximates the "size" of the nth prime number.
/// 
/// I may or may not have
/// [stolen this from Wikipedia](https://en.wikipedia.org/wiki/Prime_number_theorem#Approximations_for_the_nth_prime_number)
pub fn nth_prime_approximation(n: u64) -> u64 {

    match n {
        0 => panic!("Tried to get the zeroth prime!"),
        1 => return 2,
        2 => return 3,
        3 => return 5,
        _ => {}
    };

    let x = n as f64;
    let logn = x.ln();
    let loglogn = logn.ln();
    let log2n = logn * logn;
    let log2logn = loglogn * loglogn;

    let term1 = (loglogn - 2.0) / logn;
    let term2 = (log2logn - 6.0*loglogn + 11.0) / (2.0*log2n);

    let approximation = x * (logn + loglogn - 1.0 + term1 - term2);

    approximation as u64
}

/// Returns a range that contains the nth prime number
/// 
/// This is possible due to the fact that [`nth_prime_approximation`] converges to the actual nth prime as n grows
/// over time, so we can ensure the error is at most some value epsilon.
pub fn nth_prime_bounds(n: u64) -> RangeInclusive<u64> {

    let log = Logarithm::log10(n);

    if log < 4 {
        0..=104723
    } else {
        let approximation = nth_prime_approximation(n);
        let two: f64 = 2.0;
        let relative_epsilon: f64 = match log {
            4 => two.powi(-7),
            5 => two.powi(-10),
            6 => two.powi(-11),
            7 => two.powi(-13),
            _ => two.powi(-14),
        };

        let epsilon = (approximation as f64 * relative_epsilon) as u64;

        (approximation - epsilon)..=(approximation + epsilon)
    }
}