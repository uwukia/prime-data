# Prime Data

This library was originally meant to be a simple way of working with prime numbers
and storing prime numbers in a fast and memory-efficient way.


When you import this crate into your project, you will have, by default, access to the following tools:

Let's say you want to know what are all the prime numbers between 100 and 200:

```rust
let primes = prime_data::PrimeData::generate(100..=200)
```

With that, you can do some cool things with it:

```rust
// You can iterate over all those prime numbers
for prime in primes.iter_all() {
    println!("{} is a prime number!", prime);
}

// Therefore, you can collect them into a vector
println!("{:?}", primes.iter_all().collect::<Vec<_>>());

// You don't have to iterate through all of them. Maybe you just want primes within some range
// Naturally, you can collect those into a vector too
for prime in primes.iter(161..=179) {
    println!("{p} is a prime number and 161 <= {p} <= 179", p = prime);
}

// If you just want to know how many there are
println!("There are {} primes between 100 and 200.", primes.count_primes());

// ...within a range
println!("There are {} primes between 161 and 179.", primes.count_primes_in_range(161..=179));
```

However, there are some handy public methods, that abstract you from the need of generating any data:

```rust
// You can verify if a number is prime
println!("2027 is {}", if prime_data::is_prime(2_027) { "prime!" } else { "not prime!" });

// You can count the amount of primes from 1 to some bound
println!("There are {} primes from 1 to 1024", prime_data::count_primes(1024));
```

# Features

You'll get more functionality by including features in the dependency:

## `"factors"`

The **factors** feature includes a struct and a public method for factorizing numbers.

```rust
# #[cfg(feature = "factors")] {
// from some prime data...
let data = prime_data::PrimeData::generate(0..=12);

// you can factorize numbers
let factors = data.factorize(120);

// from that, you can retrieve the original number
println!("The prime factorization of {} is:", factors.as_u64());
// retrieve the prime factors as tuples, where each tuple is of the form
// (prime, amount) meaning prime.pow(amount)
println!("{:?}", factors.as_tuples());

// or alternatively, you can retrieve a list of all factors
let all_factors = factors.all_factors();
println!("All factors are: {:?}", all_factors);
println!("120 has {} factors in total.", all_factors.len());

// you can also convert a u64 into its factorization
let factorized_44 = prime_data::Factorization::from(44);
println!("The factors of 44 are {:?}", factorized_44);

// finally, if you only need to list a number's factors once:
println!("The factors of 490 are {:?}", prime_data::all_factors_of(490));
# }