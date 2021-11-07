#![allow(missing_docs)]
//! Error Handling

use std::{fmt, ops::RangeInclusive};

/// Result abstraction for public methods that return some result
pub type PrimeResult<T> = Result<T, PrimeError>;

/// This Error Struct is what will be returned with every function in this crate that yields an error
pub struct PrimeError {
    pub context: ErrorContext,
    pub error: ErrorType,
}

impl std::error::Error for PrimeError {}

impl fmt::Display for PrimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}\n -> {}", self.context, self.error)
    }
}

impl fmt::Debug for PrimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// * **NotEnoughData** - This error happens when someone tries to access some range of numbers that 
/// PrimeData does not have information about. The range it stores is the difference between the range given
/// and the data's range.
/// 
/// * **OutOfBounds** - Same concept, except it doesn't need to be some range. If you have some set of values
/// {1, 3, 4} and try to access the number 2, it's in the range but not in the set's bounds.
pub enum ErrorType {
    NotEnoughData(RangeInclusive<u64>),
    OutOfBounds(u64),
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotEnoughData(range) => write!(f, "Cannot access any data in the given range: {:?}", range),
            Self::OutOfBounds(num) => write!(f, "Cannot access the given number: {}", num),
        }
    }
}

/// Some errors can happen in more than one context, such as trying to access some number outside of a
/// range. This will give better context to what happened, for example, when unwrapping some function that
/// could return an error, and reading the error message.
pub struct ErrorContext {
    pub action: ErrorAction,
    pub source: ErrorSource,
}

pub enum ErrorAction {
    Reading,
    Modifying,
    Generating,
}
pub enum ErrorSource {
    PrimeByte,
    PrimeData,
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An error occurred when trying to {} {}", self.action, self.source)
    }
}

impl fmt::Display for ErrorAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reading => write!(f, "read"),
            Self::Modifying => write!(f, "modify"),
            Self::Generating => write!(f, "generate"),
        }
    }
}

impl fmt::Display for ErrorSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PrimeByte => write!(f, "PrimeByte"),
            Self::PrimeData => write!(f, "PrimeData"),
        }
    }
}