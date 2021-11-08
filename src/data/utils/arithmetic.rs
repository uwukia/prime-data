pub trait Number: Copy + Sized + Ord {}
impl Number for u64 {}

pub trait Logarithm: Number {
    fn log2(self) -> Self;
    fn log10(self) -> Self;
}
impl Logarithm for u64 {
    fn log2(self) -> u64 {
        (self as f64).log2() as u64
    }

    fn log10(self) -> u64 {
        (self as f64).log10() as u64
    }
}

pub trait Divisible: Number {
    fn div_floor(self, other: Self) -> Self;
    fn div_ceil(self, other: Self) -> Self;

    fn divisible_by(self, other: Self) -> bool {
        self.div_floor(other) == self.div_ceil(other)
    }
}
impl Divisible for u64 {
    fn div_floor(self, other: u64) -> u64 {
        self / other
    }

    fn div_ceil(self, other: u64) -> u64 {
        let div_floor = self / other;

        if div_floor * other == self {
            div_floor
        } else {
            div_floor + 1
        }
    }
}

pub trait Increment: Number {
    fn increment(self) -> Self;
}
impl Increment for u64 {
    fn increment(self) -> Self { self + 1 }
}

pub trait IntSqrt: Increment {
    fn isqrt(self) -> Result<Self, Self>;

    fn sqrt_floor(self) -> Self {
        match self.isqrt() {
            Ok(value) => value,
            Err(value) => value,
        }
    }

    fn sqrt_ceil(self) -> Self {
        match self.isqrt() {
            Ok(value) => value,
            Err(value) => value.increment(),
        }
    }

    fn is_square(self) -> bool {
        self.isqrt().is_ok()
    }
}
impl IntSqrt for u64 {
    fn isqrt(self) -> Result<Self, Self> {
        let sqrt = if self < (1 << 52) {
            (self as f64).sqrt() as u64
        } else {
            // when a number is >= 2^52, because of floating point
            // error, we need to "fix" it, treating it as a first
            // guess for the actual sqrt
            let first_guess = (self as f64).sqrt() as u64;

            // apply one step of the babylonian method
            (first_guess + (self / first_guess)) >> 1
        };

        if self == sqrt * sqrt {
            Ok(sqrt)
        } else {
            Err(sqrt)
        }
    }
}