use std::ops::RangeInclusive;

pub use arithmetic::*;
mod arithmetic;

pub trait ContainsRange {
    type RangeDifference;
    fn contains_range(&self, range: &Self) -> Result<(), Self::RangeDifference>;

    fn bounds(&self) -> (u64, u64);
}

impl ContainsRange for RangeInclusive<u64> {
    type RangeDifference = RangeInclusive<u64>;

    fn contains_range(&self, range: &RangeInclusive<u64>) -> Result<(), Self::RangeDifference> {      
        let (self_start, self_end)   = (*self.start(), *self.end());
        let (other_start, other_end) = (*range.start(), *range.end());

        

        if self_start > other_start {
            return Err(RangeInclusive::new(other_start, self_start - 1))
        }

        if other_end > self_end {
            return Err(RangeInclusive::new(self_end + 1, other_end))
        }

        Ok(())
    }

    fn bounds(&self) -> (u64, u64) {
        (*(self.start()), *(self.end()))
    }
}

// same thing as std::result::Result::into_ok_or_err
// but it's currently unstable for some reason 
pub fn unwrap_any<T>(result: Result<T, T>) -> T {
    match result {
        Ok(something) => something,
        Err(something) => something,
    }
}