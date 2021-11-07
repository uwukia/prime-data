use prime_data::PrimeData;
mod data;

// The datasets [SMALL | MEDIUM | BIG]_DATASET consist of truples:
//     ( n, pi(n), isprime(n) )
//     max => refers to the entry in the given dataset that has the biggest n
// 
// 
// Test Function Name: [data]_[test]
//
// [data] => {
//     sml = SMALL_DATASET,
//     med = MEDUM_DATASET,
//     big = BIG_DATASET,
// }
// 
// [test] => {
//     genpix = { for each n: assert_eq![ generate(0..=n).count_primes(), pi(n)      ] },
//     genisp = { for each n: assert_eq![ generate(0..=n).is_prime(n)   , isprime(n) ] }
//     cmnpix = { common_data = generate(0..=max); for each n: assert_eq![ common_data.count_primes_in_range(0..=n), pi(n) ] },
//     cmnisp = { common_data = generate(0..=max); for each n: assert_eq![ common_data.is_prime(n),             isprime(n) ] },
//     cmnrng = { 
//         common_data = generate(0..=max);
//         for each n for each m: { count = pi(m) - pi(n) + isprime(n); assert_eq![ common_data.count_primes_in_range(n..=m), count ] }
//     },
//     genrng = { 
//         for each n for each m: { count = pi(m) - pi(n) + isprime(n); data_count = generate(n..=m).count_primes();
//             assert_eq![ data_count, count ]
//         }
//     },
// }

#[test]
fn sml_cmnpix() { functions::cmnpix(data::SMALL_DATASET, 1_000) }
#[test]
fn sml_genpix() { functions::genpix(data::SMALL_DATASET) }
#[test]
fn med_cmnpix() { functions::cmnpix(data::MEDIUM_DATASET, 10_000) }
#[test]
fn med_genpix() { functions::genpix(data::MEDIUM_DATASET) }
#[test]
fn big_cmnpix() { functions::cmnpix(data::BIG_DATASET, 1_000_000) }
#[test]
fn big_genpix() { functions::genpix(data::BIG_DATASET) }

#[test]
fn sml_cmnisp() { functions::cmnisp(data::SMALL_DATASET, 1_000) }
#[test]
fn sml_genisp() { functions::genisp(data::SMALL_DATASET) }
#[test]
fn med_cmnisp() { functions::cmnisp(data::MEDIUM_DATASET, 10_000) }
#[test]
fn med_genisp() { functions::genisp(data::MEDIUM_DATASET) }
#[test]
fn big_cmnisp() { functions::cmnisp(data::BIG_DATASET, 1_000_000) }
#[test]
fn big_genisp() { functions::genisp(data::BIG_DATASET) }

#[test]
fn sml_cmnrng() { functions::cmnrng(data::SMALL_DATASET, 1_000) }
#[test]
fn sml_genrng() { functions::genrng(data::SMALL_DATASET) }
#[test]
fn med_cmnrng() { functions::cmnrng(data::MEDIUM_DATASET, 10_000) }
#[test]
fn med_genrng() { functions::genrng(data::MEDIUM_DATASET) }
#[test]
fn big_cmnrng() { functions::cmnrng(data::BIG_DATASET, 1_000_000) }
#[test]
#[ignore]
fn big_genrng() { functions::genrng(data::BIG_DATASET) } // this test takes ~2 minutes 

mod functions {

    use super::PrimeData;

    pub fn cmnpix(dataset: [(u64, u64, u64); 100], max: u64) {

        let data = PrimeData::generate(0..=(max + 1));

        for &(n, pi_n, _) in dataset.iter() {
            assert_eq!(data.count_primes_in_range(0..=n), pi_n);
        }
    }
    pub fn genpix(dataset: [(u64, u64, u64); 100]) {
        for &(n, pi_n, _) in dataset.iter() {
            assert_eq!(PrimeData::generate(0..=n).count_primes(), pi_n);
        }
    }

    pub fn cmnisp(dataset: [(u64, u64, u64); 100], max: u64) {

        let data = PrimeData::generate(0..=(max + 1));

        for &(n, _, is_prime) in dataset.iter() {
            assert_eq!(if data.is_prime(n) { 1u64 } else { 0u64 }, is_prime);
        }
    }
    pub fn genisp(dataset: [(u64, u64, u64); 100]) {
        for &(n, _, is_prime) in dataset.iter() {
            assert_eq!(if PrimeData::generate(0..=n).is_prime(n) { 1u64 } else { 0u64 }, is_prime);
        }
    }

    pub fn cmnrng(dataset: [(u64, u64, u64); 100], max: u64) {

        let data = PrimeData::generate(0..=(max+1));

        for &(n, pi_n, is_prime) in dataset.iter() {
            for &(m, pi_m, _) in dataset.iter() {
                let count = if n > m { 0u64 } else { pi_m - pi_n + is_prime };
                let data_count = data.count_primes_in_range(n..=m);
                assert_eq!(data_count, count);
            }
        }
    }
    pub fn genrng(dataset: [(u64, u64, u64); 100]) {
        for &(n, pi_n, is_prime) in dataset.iter() {
            for &(m, pi_m, _) in dataset.iter() {
                let count = if n > m { 0u64 } else { pi_m - pi_n + is_prime };
                let data_count = PrimeData::generate(n..=m).count_primes();
                assert_eq!(data_count, count);
            }
        }
    }
}