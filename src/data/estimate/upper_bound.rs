use crate::data::utils::Logarithm;

/// Estimates an upper bound for the amount of primes up to `bound`.
/// 
/// Sometimes, it's too costly to evaluate the amount of primes from 1 to N, especially in situations
/// where a "good enough" estimate would suffice.
/// 
/// This function is guaranteed to give you a value greater or equal to the actual amount of prime numbers
/// up to the given number. The relative error is also guaranteed to be `< 0.005`, but it gets better as
/// the given number goes to infinity.
pub fn upper_bound(bound: u64) -> u64 {
    if bound <= 10_000 {
        super::exact_count(bound)
    } else {
        match Logarithm::log10(bound) {
            4 => offset_x_ln_x(bound, 1.109),
            5 => offset_x_ln_x(bound, 1.100),
            _ => pierre_dusart(bound, (1.00, 2.51)),
        }
    }
}

fn offset_x_ln_x(bound: u64, offset: f64) -> u64 {

    let float = bound as f64;
    let ln_x = float.ln();

    (float / (ln_x - offset)) as u64
}

fn pierre_dusart(bound: u64, coef: (f64, f64)) -> u64 {

    let float = bound as f64;
    let ln_x = float.ln();
    let x_ln_x = float / ln_x;
    let inv_ln = ln_x.recip();
    let inv_sq = inv_ln * inv_ln;

    (x_ln_x * (1.0 + coef.0 * inv_ln + coef.1 * inv_sq)) as u64
}