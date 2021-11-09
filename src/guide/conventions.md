# Conventions

* Panic/Error

Documenting the panicking function should read as
// See [`alternative_function`] if you wish to return an error instead of panicking.

Documenting the error function should read as
// Returns a [_error_name_](crate::error::ErrorType::_error_name_) error if _error_source_

possible errors:
// Returns a [NotEnoughData](crate::error::ErrorType::NotEnoughData) error if
// Returns an [OutOfBounds](crate::error::ErrorType::OutOfBounds) error if

possible error sources:
/// the given range falls out of the data (self) range.

* Ranges
If you wish to _something_ within a specific range, see [`alternative_function`].

* Guide References

To learn more, read the [guide](crate::guide::_where_)