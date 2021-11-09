/*!

# Guide

This guide explains and gives more details about this crate and its items.

**Hint**: Start with the [Introduction](self::introduction)

## Map/Summary:

* [**Introduction**](self::introduction)
    1. [Background](self::introduction::_1_background)
    2. [Prime Sieves](self::introduction::_2_prime_sieves)
    3. [Memory Efficiency](self::introduction::_3_memory_efficiency)
    4. [Time Efficiency](self::introduction::_4_time_efficiency)
* [**Data Structure**](self::data_structure)
    1. [Prime Bytes](self::data_structure::_1_prime_byte)
    2. [Prime Data](self::data_structure::_2_prime_data)
* [**Iterators**](self::iterators)
    1. [30-Coprime Numbers](self::iterators::_1_coprime);
    2. [Prime Numbers](self::iterators::_2_prime);
* [**Sieving**](self::sieving)
    1. [Expansion](self::sieving::_1_expansion)
* [**Estimates**](self::estimates)
    1. [Pi(n)](self::estimates::_1_bounds)
    2. [p(n)](self::estimates::_2_nth_prime)
* [**Plans for the Future**](self::future)
*/


/// # Introduction
/// 
/// This covers the basic information you need to understand the what/why/how of this library.
pub mod introduction {
    /// # Background
    /// 
    /// **Prime Numbers** are positive integers that are only divisible by 1 or themselves, excluding
    /// 1 itself. Some of the first prime numbers are {2, 3, 5, 7, ...}.
    /// 
    /// We'll refer to them as prime numbers, or primes.
    /// 
    /// Prime numbers tend to get sparser the bigger they are. 25% of numbers below 100 are prime, but
    /// when we go to 10 thousand, it becomes 12.29%.
    /// 
    /// Another important fact is that there isn't a "prime generating function" that can give you all
    /// prime numbers in some constant time complexity. The only "safe way" for us to generate every
    /// prime we want is to list them out as we go. But how?
    pub mod _1_background {}

    /// # Prime Sieves
    /// 
    /// One simple way of generating all prime numbers from 1 to N, is to first start with all the numbers
    /// from 2 to N (because 1 is not prime by definition), and for each one, verify if their only divisors
    /// are 1 and themselves. That is, for each N, verify that it is not divisible by any M where 1 < M < N.
    /// 
    /// However, that is incredibly inefficient. That's because, if N is not divisible by 2, it can't be
    /// divisible by 4, 6, or any multiple of 2. Same goes for 3, 5, and the other primes. So in order for us
    /// to see if N is prime, we should only look for primes below N.
    /// 
    /// Secondly, let's say there is an M between 1 and N that divides N. That means N is **composite**, or
    /// also called, "nonprime". Therefore, N/M = K, where K is some integer. Now let's refer to √N. By the
    /// definition of square roots, √N×√N = N = M×K. Therefore, either M or K must be less than or equal to √N.
    /// Otherwise, their product would be greater than N.
    /// 
    /// Therefore, when we want to know if N is prime, we only need to look for primes P, where 2 < P ≤ √N.
    /// 
    /// Unfortunately, that is still inefficient for generating lists of primes. It's a perfectly valid and
    /// good solution if we only care about one N, but if we want to list out a load of them, not really.
    /// 
    /// # The Sieve of Eratosthenes
    /// 
    /// Okay, can I please pause the whole formality of this documentation for a sec? Eratosthenes was a greek
    /// dude that was born like 2300 years ago, and his algorithm for generating prime number lists is *the*
    /// most common way we do it today. Whaaaaaaaa...
    /// 
    /// *Cough*. Anyway. Eratosthenes designed a simple algorithm, that starts out with all numbers from 2 to N.
    /// 
    /// ```text
    ///  2  3  4  5  6  ...
    /// [ ][ ][ ][ ][ ]
    /// ```
    /// 
    /// We start with 2, and mark it as a prime number.
    /// 
    /// ```text
    ///  2  3  4  5  6  ...
    /// [√][ ][ ][ ][ ]
    /// ```
    /// 
    /// From there, we keep adding two, crossing off everything we find in the way.
    /// 
    /// ```text
    ///  2  3  4  5  6  ...
    /// [√][ ][X][ ][X]
    /// ```
    /// 
    /// Then we keep going. We mark 3 as prime, and from there, keep adding 3. If we stumble upon anything
    /// that wasn't crossed off, we cross it off.
    /// 
    /// If we do this until we reach √N, everything that is not crossed off will be a prime number! This is
    /// because, for every prime number, we crossed anything divisible by that prime number. And since the
    /// only candidates for dividing N are at most √N, we know everything up to N that wasn't crossed off
    /// is prime.
    pub mod _2_prime_sieves {}

    /// # Memory Efficiency
    /// 
    /// As we discussed previously, primes get sparser over time. So if we start with 10 thousand primes,
    /// we know after this whole process, we'll be left with only 1229 as prime. That is, more than 80%
    /// of the memory we created went in vain.
    /// 
    /// However, there's another lingering problem that we haven't tackled. What do we do with our prime
    /// sieve after we crossed off all nonprimes? We need to store those primes somewhere.
    /// 
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
    /// to find out the numbers in a given chunk, we need some "offset" value, such as 29. Then, we can
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
    pub mod _3_memory_efficiency {}

    /// # Time Efficiency
    /// 
    /// A sieving algorithm for this data structure will consider "crossing it off as nonprime" be
    /// flipping the 1 into a 0. That is, the starter data will contain all ones, and we'll cross them
    /// off as nonprime by flipping them to zero.
    /// 
    /// With the Sieve of Eratosthenes, when we encountered a prime (2, for example), we then kept
    /// adding it by itself and crossing off those results as nonprime. (4, 6, 8, ...).
    /// 
    /// However, this data that we're defining does not store numbers divisible by 2, 3, or 5. That is,
    /// if we find some prime P, it makes no sense to "flip the 2P bit" because there is no bit in our
    /// data structure that represents 2P because 2P is divisible by 2. If we add P to that again, we
    /// get 3P, which also doesn't have a bit to it because it's divisible by 3.
    /// 
    /// The only numbers that have a bit matching their value is numbers of the form XP, where X is
    /// not divisible by 2, 3, or 5 (saying this over and over is getting repetitive. From now on,
    /// we'll call those "coprime with 30", or simply "30-coprime").
    /// 
    /// If we only iterate over 30-coprime numbers, multiply them by P, and flipping the bit matching
    /// the result to 0, we're making things 3.75x faster (30/8).
    pub mod _4_time_efficiency {}
}

/// # Data Structure
/// 
/// As discussed in the introduction, we need to develop a special data structure for efficiently storing
/// and generating prime numbers. We need to first, create an abstraction of those "bytes" that contain the
/// 8 30-coprime k-values that we mentioned. Then, our data structure will be a list (vector) of those
/// bytes.
pub mod data_structure {
    /// # Prime Bytes
    /// 
    /// A bit is a 1 or a 0. A byte is a list of 8 bits. Example: `[01101001]`
    /// 
    /// A "prime byte" is what we'll name our abstraction over k-values. Reiterating, each bit in the 
    /// prime byte will correspond to the k-value:
    /// 
    /// `[ 01 07 11 13 17 19 23 29 ]`
    /// 
    /// So, if we want to retrieve "the bit that corresponds to the k-value 13", that's the same as asking
    /// "please give me the 4th bit". We can do this by performing a right shift, then taking the result
    /// modulo 2. For instance, if we want to retrieve the third bit, we remove the last 5 bits
    /// by shifting the byte to the right by 5. Then, we take the result modulo 2 to ignore the first 2
    /// bits.
    /// 
    /// ```
    /// // the third bit  >|<  is 1
    /// let byte: u8 = 0b01101010;
    /// assert_eq!((byte >> 5) % 2, 1);
    /// ```
    /// 
    /// What if we want to extract the k values of a prime byte that are set to 1? In the previous
    /// example, we had `01101010`. If we map each bit to a boolean array, where 0 maps to false and 1
    /// maps to true, we can then zip it to the array of k values, yielding the following tuples:
    /// 
    /// `[ (false, 1), (true, 7), (true, 11), ... (false, 19), (true, 23), (false, 29) ]`
    /// 
    /// We can then filter all tuples with false, and extract the k value out of the remaining tuples.
    /// The resulting list, contains all k-values that are prime, according to the byte. Of course, we can
    /// only get their true value by determining the byte's offset.
    /// 
    /// There are [other methods](crate::PrimeByte) implemented in this library, which are all tweaked
    /// versions of what I showed above. For example, maybe we want to restrict to prime bytes where the
    /// k-values belong to some range. That's easily done by applying a second condition to the filter which
    /// is that the given range contains the k-value.
    /// 
    /// ## Visibility
    /// 
    /// I decided to keep Prime Bytes public, in case anyone else feels like developing a data structure
    /// using prime bytes. But if not, they can just simply ignore it.
    pub mod _1_prime_byte {}

    /// # Prime Data
    /// 
    /// Onto the actual data structure for storing prime numbers.
    /// 
    /// The structure contains two fields. `data` and `range`.
    /// 
    /// ## `data: Vec<PrimeByte>`
    /// 
    /// The data is straightforward. It's simply a vector of [prime bytes](crate::data::PrimeByte). However,
    /// let's say we want to store the primes up to 1 million. We can't exactly divide that into our data
    /// because `1_000_000 % 30 = 10`. So we'll end up using 33_334 bytes, but that last byte will not
    /// contain the full information about primes in the range (999_990, 1_000_020). 
    /// 
    /// Therefore, this data would produce undefined behaviour if we tried to retrieve the primes in that
    /// last byte of data. In order for us to rigorously make sure to return an error when it happens,
    /// we need to store a range.
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
    /// to understand how to [iterate over them](crate::guide::iterators).
    pub mod _2_prime_data {}
}

/// # Iterators
pub mod iterators {
    /// # Iterating over 30-Coprime Numbers
    /// 
    /// As discussed in the introduction, if we want to create a prime sieve using our data structure, we
    /// need to iterate over numbers coprime with 30, to multiply them by some prime, and retrieve the
    /// bit that corresponds to that result to flip it to zero.
    /// 
    /// This is quite straightforward to do. We start with some offset and some value between 0 and 7,
    /// which is a "bit position" that corresponds to a k-value. Calling `next()` will increase that
    /// bit by one (yielding the next k-value), or if that goes above 7, we reset it to 0 and increase
    /// the offset by 1. We stop once `30*offset + k` is greater than some bound.
    /// 
    /// If you need more intricate details, you can see my implementation [here](crate::CoprimeIter).
    pub mod _1_coprime {}

    /// # Iterating over Prime Numbers
    /// 
    /// The prime sieve requires us to iterate over primes, so that we then create a nested iterator
    /// that will iterate over 30-coprime numbers. Those primes will come from an already known data.
    /// 
    /// This is much trickier than simply iterating over 30-Coprimes so I'll include the details of
    /// my implementation below.
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
    /// I could change that in the [future](super::future).
    /// 
    /// Let's say we wanted to iterate over primes in the range `(101..=240)` instead. That range is
    /// contained in the original range, so it'll not return an error. From that range, we can already
    /// determine `stop_at = 240`. 
    /// 
    /// We don't have to take a slice reference to the entire data. The first byte is the range (60..90),
    /// so we can start the iterator on the byte after, because 101 is contained in the next byte.
    /// Same applies for the ending. The 7th byte contains the range (240..270). However, since 240 is
    /// directly in between two bytes (we can argue the 6th byte contains the range (210..=240)), we can
    /// stop at the 6th byte. So we can determine `data = &prime_data[1..=6]`. If the range end was 241
    /// instead, then we would need the slice end to be at 7, not 6.
    /// 
    /// Bytes by themselves, as we saw, cannot give you prime numbers. They need an offset. Same goes
    /// for any slice of prime bytes. We need the data's offset. Since the first byte (`data[0]`) is
    /// the range (90..120), we can determine the `data_offset = 90 / 30 = 3`
    /// 
    /// Now, we can retreive primes from the data by applying the offset. However, notice that we can't
    /// simply [get all primes](crate::data::PrimeByte::as_primes) because the range starts at 101, not
    /// 90. So we need to [restrict ourselves](crate::PrimeByte::as_primes_in_range) to the range
    /// (101..=120). The result of that will be the field`primes`. The reason why it needs to be an
    /// option is that there are two situations in which we won't be able to retrieve that vector.
    /// 
    /// 1. If the range has no primes, the given vector will be empty. This means if we try to access
    /// `self.primes[self.current.1]`, that will panic. Therefore, as a safety measure, we store it
    /// as an option and set it to None when empty.
    /// 
    /// 2. If we're iterating through the data, we could encounter a prime byte that has no primes.
    /// This means, when we iterate through the data, we need to keep incrememting `current.0` until
    /// we hit a prime byte that is non empty. However, there's a chance we increase it to the point where
    /// we hit the end of our slice. We return it as None and move on.
    /// 
    /// In both situations, that's equivalent of "ending the iterator". From that, we will always
    /// yield None.
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
    pub mod _2_prime {}
}

/// # Sieving
/// 
/// Sieving requires some already know data. Hence, PrimeData has a handy method which is a hard-coded
/// piece of data with all primes from 0 to 30. Can you guess how the prime byte looks like? If you need,
/// refer to the [k-values](crate::K_VALUES)
/// 
/// Answer: `[01111111]` (1 isn't prime)
pub mod sieving {
        /// # Sieving = Expansion
        /// 
        /// Data Expansion is essentially, grabbing an initial piece of data, generating a new piece with a wider range,
        /// then sieving it to filter out nonprime numbers, just like an Eratosthenes sieve. However, there's a very
        /// intricate reason I chose to call this an "expansion": Because it sounds cool 😎.
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
        /// We just need to use some handy `data.set_nonprime(p*c)` function to flip the bit to zero. For a
        /// safer user experience, that function is private for PrimeData, but it [is](crate::PrimeByte::set_nonprime)
        /// public for prime bytes, if you wish to create your own data struct using those.
        ///
        /// That's it. That's all there is to it. Once we iterate over all **p** values, the expanded data is a valid
        /// data of prime numbers from 30 thousand to 50 thousand!
        /// 
        /// Of course, as previously said, we cannot *join* those two bits of data together unless the original data
        /// ranges to 30 thousand. But since the iteration process does not consume data, just references it, we can
        /// expand to as many new PrimeData as we want, and join them all together once their starts and ends match.
        /// Joining two PrimeData into one has not yet been implemented, but [is planned to](super::future).
    pub mod _1_expansion {}
}

/// # Estimates
/// 
/// As discussed in the introduction, our only hope to safely generate and count prime numbers is by
/// sieving-like methods. However, sometimes, a good approximation is all we need.
pub mod estimates {
    /// # Approximating π(x)
    /// 
    /// Don't be fooled! That "π" represents a function, not the famous constant.
    /// 
    /// Mathematicians refer to π(N) as "how many primes, less than or equal to N, are there?".
    /// 
    /// We can generate some prime data from 1 to N and count its primes. But that takes a good amount
    /// of time if N is big enough. If we want to approximate it (we'll see later why this is useful),
    /// we can use a very interesting theorem.
    ///
    /// > π(x) ~ x / ln(x)
    /// 
    /// This is known as the Prime Number Theorem, and unfortunately, even the most elementary proof for
    /// this is quite the hassle for this guide. But, if you're interested, you can give it a try. Heads
    /// up, you'll find yourself reading about complex analysis (including the Riemann Zeta Function),
    /// some crazy logarithmic sums, perhaps even integrals.
    /// 
    /// I'm also not gonna deeply explain what ln(x) is. It's the inverse of an exponential function.
    /// Exponentials have the properties of growing incredibly quickly, and defined for all x values.
    /// This means that its inverse, the logarithm, will grow incredibly **slowly**, but it never stops
    /// growing. If it did, that would mean the logarithm stops at a certain bound. And that would mean
    /// the exponential wasn't defined for anything above that bound.
    /// 
    /// That's kinda like prime numbers. The further we go in the number line, the less primes we'll find,
    /// as they get sparser, but we'll never stop finding primes. So that theorem does make sense.
    /// 
    /// # Approximations
    /// 
    /// The problem here, is that ⌊ x / ln(x) ⌋ is not a great approximation for π(x). When x = 1000, the
    /// approximation is off by 13%. If x = 1 billion, it's off by 5%.
    /// 
    /// Don't get me wrong, that's okay, but 5% of 1 billion still is still 50 million. I think we need
    /// a better way to approximate things.
    /// 
    /// There's a very cool alternative, which is (x / ln(x)) × Σ ( n! / ln^n(x) )
    /// 
    /// That is, we multiply the original approximation by a sum. That sum, theoretically goes to infinity,
    /// but since we want an approximation, we can stop at a bound B. So, for each n in the range (0..=B),
    /// we evaluate ( n! / ln^n(x) ), then we sum everything.
    /// 
    /// Just to make sure everyone understands, n! refers to the factorial of n. And ln^n(x) refers to
    /// `x.ln().powi(n)` in Rust code.
    /// 
    /// However, if we want to stop at a certain bound B, we should probably tweak those n values a bit.
    /// Not the ones in ln^n(x), but the ones in n factorial.
    /// 
    /// Pierre Dusart, a french mathematician, found out that if we only take the range (0..2), it's
    /// a good approximation to use the n values: (0, 1, 2.51).
    /// 
    /// This yields the function (x / ln(x)) × (1 + 1 / ln(x) + 2.51 / ln^2(x)). Spoiler alert, at
    /// x = 1 billion, the approximation is off by 0.035%. Nice.
    /// 
    /// That function above is actually an upper bound for π(x) after ~356k. We can tweak the n values
    /// to get a nice lower bound as well. I plan to implement that in the [future](super::future).
    pub mod _1_bounds {}

    /// # Approximating p(n)
    /// 
    /// p(n) refers to the "nth prime number". For example, as we discussed, the first prime numbers are
    /// {2, 3, 5, 7, ...}. So p(1) = 2, p(2) = 3, p(4) = 7, and so on. p(0) is undefined. I know, I know,
    /// within programming we commonly start at zero, but this is mathematician land. And that's how
    /// mathematicians chose to do this.
    /// 
    /// Anyway, if we approximate π(1 billion) ~ 50865514, this means there are roughly 50 million primes
    /// under 1 billion. If we listed them out in a vector, the last entry would be very close to 1 billion
    /// and its index would close to 50865514. See what we're doing here?
    /// 
    /// We can flip the question around. We can instead say that the 50865514th prime is ~ 1 billion.
    /// It's actually quite accurate, the real answer is 1,000,373,273.
    /// 
    /// However, I found myself fond of a [formula](https://en.wikipedia.org/wiki/Prime_number_theorem#Approximations_for_the_nth_prime_number)
    /// I found on Wikipedia.
    /// 
    /// What's fun is that, with an approximation and a known error bound for that approximation, we can
    /// create a lower and upper bound for where the nth prime number lies. Then, we can simply generate
    /// data, count the primes from 1 to the lower bound, then look for nth prime between those bounds.
    pub mod _2_nth_prime {}
}

/// # Plans for the Future
/// 
/// This section isn't really a "guide", and more of a list of things I plan to add in the future, to my
/// library. 
/// 
/// * High Priority = Expect to see it in the next versions
/// * Medium Priority = I will do it eventually.
/// * Low Priority = I could do it, but I don't see why would I.
/// 
/// # High Priority
/// 
/// ## Faster `is_prime(x)`
/// 
/// Currently, this function creates prime data up to the square root of `x`, then checking if none of the
/// generated primes divide x. Unfortunately, this is way slower than it needs to be. Primality tests
/// like Fermat or, the one I intend to implement, the Miller-Rabin test, are way faster.
/// 
/// ## More Rigorous Tests
/// 
/// Currently I'm running integration tests to make sure the data generation is working properly.
/// But most of the other methods PrimeData and PrimeByte have have not been thoroughly tested.
/// 
/// # Medium Priority
/// 
/// ## Add a `lower_bound()` method
/// 
/// Currently, the [estimate](crate::estimate) module only has an [`upper_bound`](crate::estimate::upper_bound)
/// method.
/// 
/// ## Better approximatin function for π(x)
/// 
/// Currently, π(x) approximations are never off by more than 0.5%. They're actually super good for values
/// greater than [`u32::MAX`], but below that, I'm probably better off using combinatorial methods.
/// 
/// # Low Priority
/// 
/// ## Smarter Structs
/// 
/// If your data ranges from 73 to 144, you can't retrieve primes from 72 to 144, even if it's obvious
/// that 72 isn't prime. I could implement a check to see if the values that lie outside of the data range
/// are "obviously prime" (by checking if they're not coprime with 30).
pub mod future {}