mod signed_nonzero;
mod unsigned_nonzero;
mod implement;

pub use self::unsigned_nonzero::NonZeroPrimitiveUnsigned;
pub use self::signed_nonzero::NonZeroPrimitiveSigned;

use crate::{PrimitiveInteger, PrimitiveError};
use core::num::NonZero;


/// Trait for [`NonZero`].
///
/// This encapsulates trait implementations, constants, and inherent methods that are common among
/// all of the implementations of `NonZero<T>`.
///
/// See the corresponding items on the individual types for more documentation and examples.
///
/// This trait is sealed with a private trait to prevent downstream implementations, so we may
/// continue to expand along with the standard library without worrying about breaking changes for
/// implementors.
pub trait NonZeroPrimitiveInteger:
    'static
    + core::cmp::PartialEq
    + core::cmp::PartialOrd
    + core::fmt::Debug
    + core::fmt::Display
    + core::fmt::LowerExp
    + core::fmt::UpperExp
    + core::marker::Copy
    + core::marker::Send
    + core::marker::Sync
    + core::marker::Unpin
    + core::panic::RefUnwindSafe
    + core::panic::UnwindSafe
    + core::str::FromStr<Err: PrimitiveError>
    + core::cmp::Eq
    + core::cmp::Ord
    + core::hash::Hash
    + core::fmt::Binary
    + core::fmt::LowerHex
    + core::fmt::Octal
    + core::fmt::UpperHex
    + core::convert::TryFrom<NonZero<i8>, Error: PrimitiveError>
    + core::convert::TryFrom<NonZero<i16>, Error: PrimitiveError>
    + core::convert::TryFrom<NonZero<i32>, Error: PrimitiveError>
    + core::convert::TryFrom<NonZero<i64>, Error: PrimitiveError>
    + core::convert::TryFrom<NonZero<i128>, Error: PrimitiveError>
    + core::convert::TryFrom<NonZero<isize>, Error: PrimitiveError>
    + core::convert::TryFrom<NonZero<u8>, Error: PrimitiveError>
    + core::convert::TryFrom<NonZero<u16>, Error: PrimitiveError>
    + core::convert::TryFrom<NonZero<u32>, Error: PrimitiveError>
    + core::convert::TryFrom<NonZero<u64>, Error: PrimitiveError>
    + core::convert::TryFrom<NonZero<u128>, Error: PrimitiveError>
    + core::convert::TryFrom<NonZero<usize>, Error: PrimitiveError>
    + core::convert::TryInto<NonZero<i8>, Error: PrimitiveError>
    + core::convert::TryInto<NonZero<i16>, Error: PrimitiveError>
    + core::convert::TryInto<NonZero<i32>, Error: PrimitiveError>
    + core::convert::TryInto<NonZero<i64>, Error: PrimitiveError>
    + core::convert::TryInto<NonZero<i128>, Error: PrimitiveError>
    + core::convert::TryInto<NonZero<isize>, Error: PrimitiveError>
    + core::convert::TryInto<NonZero<u8>, Error: PrimitiveError>
    + core::convert::TryInto<NonZero<u16>, Error: PrimitiveError>
    + core::convert::TryInto<NonZero<u32>, Error: PrimitiveError>
    + core::convert::TryInto<NonZero<u64>, Error: PrimitiveError>
    + core::convert::TryInto<NonZero<u128>, Error: PrimitiveError>
    + core::convert::TryInto<NonZero<usize>, Error: PrimitiveError>
    + core::cmp::Ord
    + core::fmt::Binary
    + core::ops::BitOr<Self, Output=Self>
    + core::ops::BitOr<Self::Zeroable, Output=Self>
    + core::ops::BitOrAssign<Self::Zeroable>
{
    /// The integer type that this represents,
    /// For a `NonZero<T>` this is `T`.
    type Zeroable: 
        PrimitiveInteger<NonZero=Self>
        + core::ops::BitOr<Self, Output=Self>
        + From<Self>
        + TryInto<Self>;

    /// The size of this non-zero integer type in bits, equal to `Self::Zeroable::BITS`
    const BITS: u32 = Self::Zeroable::BITS;

    /// The largest value that can be represented by this non-zero integer type, equal to `Self::Zeroable::MAX`.
    const MAX: Self = unsafe { core::mem::transmute_copy::<Self::Zeroable, Self>(&Self::Zeroable::MAX) };

    /// Creates a non-zero if the given value is not zero.
    fn new(n: Self::Zeroable) -> Option<Self>;
    
    /// Creates a non-zero without checking whether the value is non-zero. 
    /// This results in undefined behavior if the value is zero.
    ///
    /// # Safety
    /// The value must not be zero.
    unsafe fn new_unchecked(n: Self::Zeroable) -> Self;

    /// Returns the contained value as a primitive type.
    fn get(self) -> Self::Zeroable;

    /// Returns the number of leading zeros in the binary representation of `self`.
    /// 
    /// On many architectures, this function can perform better than `leading_zeros()` 
    /// on `Self::Zeroable`, as special handling of zero can be avoided.
    fn leading_zeros(self) -> u32;
    
    /// Returns the number of leading zeros in the binary representation of `self`.
    /// 
    /// On many architectures, this function can perform better than `trailing_zeros()` 
    /// on the `Self::Zeroable`, as special handling of zero can be avoided.
    fn trailing_zeros(self) -> u32;

    /// Returns the number of ones in the binary representation of `self`.
    fn count_ones(self) -> NonZero<u32>;

    /// Multiplies two non-zero integers together. Checks for overflow and returns `None` 
    /// if it occurs. As a consequence, the result cannot wrap to zero.
    fn checked_mul(self, rhs: Self) -> Option<Self>;

    /// Multiplies two non-zero integers together, saturating at the numeric upper and lower 
    /// bounds instead of overflowing. As a consequence, the result cannot wrap to zero.
    fn saturating_mul(self, rhs: Self) -> Self;

    /// Raise non-zero value to an integer power. Checks for overflow and returns `None` on 
    /// overflow. As a consequence, the result cannot wrap to zero.
    fn checked_pow(self, exponent: u32) -> Option<Self>;

    /// Raise non-zero value to an integer power. Return NonZero::<i8>::MIN or NonZero::<i8>::MAX 
    /// on overflow based on the sign of the exact result.
    fn saturating_pow(self, exponent: u32) -> Self;
}