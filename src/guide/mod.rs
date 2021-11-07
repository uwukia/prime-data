//! Module dedicated to serve as a guide, for deeper understanding of this crate
//! 
//! Hint: Start with the [introduction]

#[allow(rustdoc::invalid_rust_codeblocks)]
/// # What's the most efficient way to store raw prime numbers?
/// 
/// That's a really vague question, which includes lots of factors. One of them will always arise from
/// any question regarding "memory efficiency", which is the most efficient way to store any
/// information. That involves compression, which will be dealt with later on. Right now, we only
/// care about listing prime numbers.
/// 
/// We can also optimize data storage by, instead of storing the data itself, we store a function
/// that can generate the data. That will also be dealt with in the future. Right now, we only care
/// about storing the data itself.
/// 
/// The most straightforward way is to list the primes, perhaps as a [`u32`], and simply store that
/// in a file. It's easy to encode and decode. Unfortunately, if we decided to store the first 200
/// million primes, the file size will be ~800MB. 
/// 
/// The first optimization idea could be to notice that the first of these primes are small, and don't
/// need to be stored as a 4 byte number. Maybe 1 byte or 2 bytes suffice. The first 54 primes can
/// be stored in 1 byte, then the next 6488 can be stored in 2 bytes. We can also encode the next
/// 1,071,329 primes in 3 bytes, leaving the remaining 198,928,671 to be stored in 4 bytes.
/// Unfortunately, that only compresses ~800MB into ~775MB.
/// 
/// ## The Law of Excluded Middle
/// 
/// If you've ever studied logic, you may have come across that law. It states something really
/// intuitive, but incredibly powerful for mathematical theorems:
/// 
/// > Every statement is either true or false. Never both, never neither.
/// 
/// With this, we can determine that every integer is either prime or not. Because every time we
/// say "N is prime", that statement is either true or false. Within the context of programming,
/// we can associate the state of being prime or not as a 1 or a 0.
/// 
/// This way, we can make a huge array (or, in a more dynamic context, a vector) of bits, where
/// the position `i` will store wether the number `i` is prime or not.
/// 
/// ```text
/// 0 1 2 3 4 5 6 7 8 9 ...
/// 0 0 1 1 0 1 0 1 0 0 ...
/// ```
/// 
/// In this context, it's a little harder to ask "how much memory it takes to store the first N primes"
/// because now we're storing according to all numbers, not just primes. So we need to first ask
/// *when* will we reach the Nth prime, then divide that by 8 to get the amount of bytes it'll take.
/// 
/// The 200 millionth prime is around 4.2 billion. Which means, in order to reach there, we'll need 4.2
/// billion bits, which is ~525MB. Already a big improvement.
/// 
/// ## We're not done!
/// 
/// Hopefully you already know that 2 is the only even prime. That means, after 2, every bit in an even
/// position will be guaranteed to be a zero. That's a lot of useless information! Specifically, close to
/// half of our data is unnecessary! So we can heavily improve our system by only listing odd primes,
/// and when we want to decode our data, we hardcode an extra 2 that will be missing!
/// 
/// ```text
/// 1 3 5 7 9 11 13 ...
/// 0 1 1 1 0 1  1  ...
/// ```
/// 
/// However, we can *generalize* this idea. Obviously, 3 is the only prime divisible by 3. So after 3,
/// all positions divisible by 3 (9, 15, 29769...) will also be a guaranteed zero!
/// 
/// So we can ignore all positions that aren't divisible by either 2 or 3! First, we multiply these two 
/// to get 6. Now, we notice that for any number N, `N % 6 == k`, where k is in the range `(0..6)`. If
/// k is 0, 2, or 4, N is definitely divisible by 2. If it's 3, N is definitely divisible by 3. That means 
/// the only possible primes, after 3, are the numbers where k results in 1 or 5.
///  
/// ```text
/// 1 5 7 11 13 17 19 ...
/// 0 1 1 1  1  1  1  ...
/// ```
///  
/// How much are we improving here? Well, in the beginning, we were storing every number as a bit. Which
/// means the bit to number ratio was 100%. When we excluded even numbers, it became `50%`, because it
/// takes 1 bit to store info about two numbers (since the second one is even, we know it's not prime).
/// Once we excluded numbers divisible by 2 and 3, we went from 6 possible values of k to just 2. Meaning
/// the bit to number ratio became ~33%.
///  
/// And we can keep going. Adding 5 into the mix gives us a multiple of 30 (2*3*5). Out of the 30 possible
/// values of `N % 30`, 15 are even, 5 are odds multiple of 3, and other 2 are only multiples of 5. So we
/// have 8 possible prime candidates out of a 30 number range. ~26%.
/// 
/// However, as the range increases, we're increasing the size of the chunks of bits we need to decode.
/// For the previous example, we had to encode it in chunks of 2. One for k = 1, and the other for k = 5.
/// 
/// By adding 5 and increasing the range to 30, we have 8 possible primes, meaning the chunk size is now
/// an entire byte. As we increase the amount of primes to discard, we also increase the chunk size.
/// 
/// | Primes  | Range | bit/N | Chunk Size |
/// |---------|-------|-------|------------|
/// |  None   |  1    | 100%  | -          |
/// | 2       |  2    |  50%  | 1 bit      |
/// | 2,3     |  6    |  33%  | 2 bit      |
/// | 2,3,5   |  30   |  26%  | 8 bits     |
/// | 2,3,5,7 |  210  |  23%  | 48 bits    |
/// 
/// As you can see, when we add 7, the chunk size becomes 6 times larger, while we go from 26% to 23%.
/// We want to avoid big changes for small improvements. Not only that, but using just 2, 3, and 5 lets
/// us have exactly 1 byte of chunk size. Neat and tidy.
/// 
/// ## Recap
/// 
/// How will our data look like? We know that every byte is a chunk of prime candidates, that are either
/// prime (1) or not prime (0). Each bit represents a number N, so that when you evaluate `N % 30`, it'll
/// return some `k` that is not divisible by 2, 3 or 5. Specifically, these k values:
/// 
/// `[ 01 07 11 13 17 19 23 29 ]`
/// 
/// A byte by itself doesn't give us the full picture though. We can't extract N from k. So if we want
/// to find out the numbers in a given chunk, we need some "offset" value, such as 14. Then, we can
/// multiply the offset by 30, and we can sum each k value to that result, yielding N. Example:
/// 
/// `{ offset: 29, byte: [01111000] }`
/// 
/// First, we multiply the offset. `result = offset * 30 = 870`. Then, we notice the k values that are
/// ones (a.k.a, prime numbers) are 7, 11, 13, and 17. Summing those by 870 gives us the numbers:
/// 
/// `877, 881, 883, 887`
/// 
/// If our data isn't wrong, we officially decoded all prime numbers between 870 and 900!
/// 
/// # Where to go from here
/// 
/// Now you have all the information you need to understand what this library does. It's meant to encode
/// and decode prime numbers. It'll also (in the future) include features for analyzing and verifying
/// given data, being able to read and store files as well.
/// 
/// To learn more about chunks, read [`prime_byte`].
/// 
/// (This guide is a work in progress. It's still missing a lot of information that will be added soon.)
pub mod introduction {}

/// # Prime Chunks
/// 
/// The [Prime Byte](crate::data::PrimeByte) struct is an abstraction over the prime chunks we
/// talked about in the [`introduction`].
/// 
/// To make sure we're on the same page, everytime I talk about "k values", I'll be talking about
/// the numbers between 0 and 30 that aren't divisible by 2, 3, and 5. You can always refer to
/// [the defined const](crate::data::K_VALUES) if you wish.
/// 
/// ## The Bits
/// 
/// If we want to get the third bit of a prime chunk, we'll have to refer to methods involving the
/// standard library's [`u8`]. The easiest way to do so, is by performing a right shift, then
/// taking the result modulo 2. For instance, if we want the third bit, we remove the last 5 bits
/// by shifting the byte to the right by 5. Then, we take the result modulo 2 to ignore the first 2
/// bits.
/// 
/// ```
/// // the third bit  >|<  is 1
/// let byte: u8 = 0b01101010;
/// assert_eq!((byte >> 5) % 2, 1);
/// ```
/// 
/// ## The Primes
/// 
/// What if we want to extract the k values of a prime chunk that are set to prime? In the previous
/// example, we had `01101010`. If we map each bit to a boolean array, where 0 maps to false and 1
/// maps to true, we can then zip it to the array of k values, yielding the following tuples:
/// 
/// `[ (false, 1), (true, 7), (true, 11), ... (false, 19), (true, 23), (false, 29) ]`
/// 
/// We can then filter all tuples with false, and extract the k value out of the remaining tuples.
/// The resulting list, contains all k-values that are prime, according to the byte. Of course, we can
/// only get their true value by determining the byte's offset.
/// 
/// ## Count Primes
/// 
/// It's inefficient to use the previous method, then taking the list's length to calculate how many
/// primes a given chunk has. Considering the amount of ones in a byte represents the amount of primes
/// it has, we can simply [count the ones](u8::count_ones) using the standard library. Same can be
/// done to [count nonprimes](u8::count_zeros).
/// 
/// ## Primes in Range
/// 
/// As we'll see in the [Prime Data guide](data), it's useful to consider the byte's primes within
/// a more restricted range, instead of the default (0..30). 
/// 
/// That can be easily done in the filtering step of retrieving the primes. On top of filtering out
/// the tuples with `false`, we can also filter out k values that fall out of a given range.
pub mod prime_byte {}

/// # Prime Data
/// 
/// This is the struct dedicated to storing prime numbers using the encoding and decoding techniques
/// discussed in the [`introduction`].
/// 
/// The struct contains two fields. `data` and `range`.
/// 
/// ## `data: Vec<PrimeByte>`
/// 
/// The data is straightforward. It's simply a vector of [prime bytes](crate::data::PrimeByte). However,
/// let's say we want to store the primes up to 1 million. We can't exactly divide that into our data
/// because `1_000_000 % 30 = 10`. So we'll end up using 33_334 bytes, but that last byte will not
/// contain the full information about primes in the range (999_990, 1_000_020). 
/// 
/// Therefore, this data would produce undefined behaviour if we tried to retrieve the primes in that
/// last byte of data. At least for k values above 10. Therefore, it's important to also store a...
/// 
/// ## `range: RangeInclusive<u64>`
/// 
/// An inclusive range will determine what are the exact bounds of the given data. For example, if
/// we had only one byte of data, and the range was (33..=58), we can infer that the byte of data
/// represents prime candidates from (30..60), however, the first bit is not part of the data, as it
/// represents 31. Same goes for the last bit of data, which represents 59. Trying to access those
/// bits should result in an error.
/// 
/// The range start is especially important because it also determines the byte offset we discussed
/// in the introduction. The offset is basically the range start divided by 30. If the range starts
/// at 0, 20, or anything below 30, the first byte corresponds to (0..30). As soon as the range starts
/// at 30, this means the first byte corresponds to (30..60) instead.
/// 
/// The range will also be an important factor for expanding data. But in order to expand data, we need
/// to understand how to [iterate over them](crate::guide::data_iteration).
pub mod data {}

/// # Iterating over Primes and Coprimes
/// 
/// There will be methods to create iterators over prime data structs, including implementing the
/// [`IntoIterator`] trait itself.
/// 
/// However, we'll also need to iterate over non-primes when we talk about data expansion. Specifically,
/// we'll need to iterate over all numbers coprime with 30, even if they're not necessarily prime.
/// 
/// ## Iterating over 30-Coprime numbers
/// 
/// "30-Coprime number" is just an alternative, shorter way to say "number that is coprime with 30".
/// 
/// This refers to the CoprimeIter struct, which starts at a given offset and k-value.
/// 
/// The struct will have only two fields. One is a tuple with current offset and current index. The
/// other will be a bound, for when the iterator should stop. For every iteration, it'll try to
/// increment the current index. The index refers to the [k-values](crate::data::K_VALUES), so if
/// the index goes beyond 7, it should be reset to zero, and instead we'd increment the offset.
/// 
/// The resulting iteration will simply yield `30 * offset + K_VALUES[index]`. However, if that
/// result goes above the iterator's bound, it'll just return None.
/// 
/// ## Iterating over prime numbers
/// 
/// Prime numbers on the other way, require actual Prime Data, which is a little harder to iterate
/// over. It refers to the PrimeIter struct, and it's made up of a slice reference of a given
/// prime data. This means any iterator created from some prime data cannot live longer than the
/// data itself.
///
/// Along with the slice, the iterator will include a range (so that it knows from where
/// to start and end iterating), a tuple with an offset and index (just like the other iterator),
/// except now, the index will refer to a prime byte. More specifically, the vector yielded from the
/// prime byte.
/// 
/// A more hands down explanation, using actual examples on both prime data and prime iteration can be
/// found in the [`prime_iter` submodule](self::data_iteration::prime_iter).
pub mod data_iteration {

    /// # Creating an Iterator
    /// 
    /// The struct's fields are as follows:
    /// 
    /// ```no_run
    /// # use prime_data::PrimeByte;
    /// struct PrimeIter<'a> {
    ///     data: &'a [PrimeByte],
    ///     primes: Option<Vec<u64>>,
    ///     current: (u64, usize),
    ///     data_offset: u64,
    ///     stop_at: u64,
    /// }    
    /// ```
    /// 
    /// Let's say we have some prime data. It has 7 [bytes](crate::PrimeByte) of data, with the range
    /// `(73..=262)`. This tells us the data offset is 2, because `73 / 30 = 2`. Technically, the data
    /// bytes range over (60..270), but the actual range tells us we cannot verify if 61, 67, 71, 263,
    /// or 269 are primes or not. Even though their bits are in the data, they're not valid bits because
    /// they fall out of the verified range.
    /// 
    /// If we try to create an [iterator](crate::data::PrimeIter) of primes in the range `(72..=260)`,
    /// it will send out an error, since that range is not contained in our original data's range. Even
    /// if it's trivially known that 72 is not prime, and is the only number out of the range. However,
    /// I do plan to change that in the future.
    /// 
    /// Let's say we wanted to iterate over primes in the range `(101..=240)` instead. That range is
    /// contained in the original range, so it'll not return an error. From that range, we can already
    /// determine `stop_at = 240`. 
    /// 
    /// We don't have to take a slice reference to the entire data. The first byte is the range (60..90),
    /// so we can start the iterator on the byte after, because 101 is contained in the next byte.
    /// Same applies for the ending. The 7th byte contains the range (240..270). However, since 240 is
    /// directly in between two bytes (we can argue the 6th byte contains the range (210..=240)), we can
    /// stop at the 6th byte. So we can determine `data = &prime_data[1..=6]`. 
    /// 
    /// Bytes by themselves, as we saw, cannot give you prime numbers. They need an offset. Same goes
    /// for any slice of prime bytes. We need the data's offset. Since the first byte (`data[0]`) is
    /// the range (90..120), we can determine the `data_offset = 90 / 30 = 3`
    /// 
    /// Now, we can retreive primes from the data by applying the offset. However, notice that we can't
    /// simply [get all primes](crate::data::PrimeByte::as_primes) because the range starts at 101, not
    /// 90. So we need to [restrict ourselves](crate::PrimeByte::as_primes_in_range) to the range
    /// (101..=120). The result of that will be `primes`. The reason why it needs to be an option is that
    /// depending on how the iterator is, it'll eventually be a None value. That'll be explained soon.
    /// 
    /// 
    /// The only things that will change throughout the iteration is `primes` and `current`. For the
    /// starter values of the iterator, `current = (0, 0)`.
    /// 
    /// **Note**: When returning primes, there's a chance it could be empty. Meaning there are no primes
    /// in the given range. That's why it's important to, instead of simply retrieving one vector, it's
    /// necessary to use a while loop that'll loop through primes until it finds a non-empty one. That
    /// loop will end up incrementing `current.0` each time.
    ///
    /// # Retrieving `next()`
    /// 
    /// Let's say we're at some step of the iterator. When calling the next function, the first thing we
    /// do is check if `self.primes` is [None](Option::None) and return that if so. Otherwise, we retrieve
    /// a prime number using `primes[self.current.1]`. This will never panic because we won't allow the
    /// iterator to reach the given vector's length.
    /// 
    /// If not, we gotta look for the next prime. First, we check if we hit the ending of the current
    /// vector. If we did, we'll increment `self.current.0` to reach out for the next piece of data.
    /// If that is equal to the data's length, it means we hit the actual end of the whole data. That's
    /// where we set `self.primes = None` and return the last prime number we got previously.
    /// 
    /// Of course, if we didn't hit the vector's length, we just increment the index and move on. If we
    /// did, but we didn't hit the data's length, we'll just grab the next byte and retrieve its
    /// prime numbers using [as_primes](crate::PrimeByte::as_primes). The offset that should be given
    /// as a parameter is `self.data_offset + self.current.0`.
    pub mod prime_iter {}
}

/// # Discovering more Primes
///
/// Prime Data only trusts itself. So if you want to create a list of prime numbers in some range,
/// that'll require you to know all the primes below the square root of the given range's end.
///
/// This means, if you want a list of all prime numbers up to 900, you need to already know the
/// prime numbers below 30.
///
/// Prime Data will have a starter function, that'll give you the primes below 30. But from that,
/// you can expand as much as you want. From 30 to 900, then from 900 to 810 thousand. After that,
/// you cannot expand too much at a time, for memory safety. You will be able to bypass the limit
/// by calling an "unsafe" counterpart of the expansion method.
///
/// If you have a prime data that ranges from 0 to 1000, you can expand this data into any prime data
/// as long as the range end is less than or equal to 1 million. It's perfectly acceptable to expand
/// that into prime numbers in (234094..392400). However, sometimes, when we expand some data into
/// other data, we want to then be able to concatenate them into only one piece of data.
/// 
/// ## Joining Data 
///
/// Concatenation between data is only allowed if the range start of the second one is equal to or
/// one more the range ending of the first one. For extra safety, it will not allow you to concatenate
/// data in (30..=80) with (82..=100), even if it's "known" that 81 is not a prime.
/// 
/// It is allowed (and sometimes encouraged, if working with prime data from two different sources)
/// to join two overlapping data. If you do so, for example with (30..=80) and (50..=100), it'll
/// join them as if you were joining (30..=80) with (81..=100), but on top of that, it'll also verify
/// if the bits in the range (50..=80) match in both data. I'll return an error if they don't.
pub mod data_expansion {

    /// # Prime Sieves
    /// 
    /// A sieve, in its most general meaning, is a tool for filtering out unwanted particles in a
    /// group, letting only the stuff we want to pass.
    /// 
    /// Hopefully you've heard of the [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes),
    /// a very powerful way of generating prime number lists. Of course, it's straightforward to make
    /// its algorithm when we're given a raw list of numbers. But here, we have to tackle it in a
    /// different way.
    /// 
    /// For instance, the first 3 steps of the sieve, which is marking out all the numbers that are
    /// multiples of 2, 3, and 5, are unnecessary here. We already did it implicitly. 
    /// 
    /// So the first step is 7. In a "normal" algorithm, we'd then get the square of 7, which is 49, and from there,
    /// we mark every multiple of 7 as non-prime. That is, we run through the indexes `49, 56, 63, 70, ...` and
    /// set their "primality" to false. Here, not only that's not as straightforward to do, but it's also something
    /// unnecessary. Because we'd be running into so many multiples of 2, 3, and 5. 
    /// 
    /// That's why [CoprimeIter](crate::CoprimeIter) is important. It'll iterate through the integers, except
    /// skipping all of these multiples. That way, for each number that's iterated, we multiply it by whatever prime
    /// we're at, and we'll be sure our data will have a bit corresponding to that number. And we can make sure to
    /// set it to zero. Very efficient.
    /// 
    /// # Sieving = Expansion
    /// 
    /// Data Expansion is essentially, grabbing an initial piece of data, generating a new piece with a wider range,
    /// then sieving it to filter out nonprime numbers, just like an Eratosthenes sieve. However, there's a very
    /// intricate reason I chose to call this an "expansion". Because it sounds cool.
    /// 
    /// Anyway, if we have some piece of data and we want to expand it to the prime numbers from 30 thousand to 50
    /// thousand, we first need to check if the original piece of data can expand that much. `isqrt(50_000) = 223`,
    /// so as long as the original data goes from 1 to 223 or more, we're set.
    /// 
    /// If so, the first thing we do is create a starter data. The range is, naturally, `30_000..=50_000`, the data
    /// size will be `ceil(50_000 / 30) - floor(30_000 / 30) = 667` and the entire data is filled with ones.
    /// 
    /// The next step is so iterate over prime numbers from 1 to 223 (I mean, we can start anywhere from 0 to 7,
    /// same results). We know how to do that. Each iteration yields a prime number **p**. For each **p**, we need
    /// to calculate the biggest number, that when multiplied by it, will be less than or equal to 50 thousand.
    /// If we consider the first prime in the iteration, 7, that is `floor(50_000 / 7) = 7142`.
    /// 
    /// From that, we can now iterate over numbers coprime with 30 (not just primes) that are between 1 and 7142
    /// (again, it doesn't have to be 1, anything from 0 to 7 works). Each iteration will yield a **c**.
    /// 
    /// Then, all we have to do is calculate **pc** and the result of that multiplication is not a prime number!
    /// All we have to do now is use some handy `data.set_nonprime(p*c)` function to flip the bit to zero. For a
    /// safer user experience, that function is private for PrimeData, but it [is](crate::PrimeByte::set_nonprime)
    /// public for prime bytes, if you wish to create your own data struct using those.
    ///
    /// That's it. That's all there is to it. Once we iterate over all **p** values, the expanded data is a valid
    /// data of prime numbers from 30 thousand to 50 thousand!
    /// 
    /// Of course, as previously said, we cannot *join* those two bits of data together unless the original data
    /// ranges to 30 thousand. But since the iteration process does not consume data, just references it, we can
    /// expand to as many new PrimeData as we want, and join them all together once their starts and ends match.
    pub mod expanding_data {}
}