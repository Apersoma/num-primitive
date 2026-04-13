mod signed_nonzero;
mod unsigned_nonzero;

pub use self::unsigned_nonzero::NonZeroPrimitiveUnsigned;
pub use self::signed_nonzero::NonZeroPrimitiveSigned;

use crate::{PrimitiveInteger, PrimitiveError, PrimitiveUnsigned};
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
/// 
/// # Examples
///
/// ```
/// use core::ops::NonZero;
/// use num_primitive::NonZeroPrimitiveInteger;
///
/// fn ceil_log2<T: NonZeroPrimitiveInteger>(x: <T>) -> u32 {
///     
/// }
/// 
/// assert_eq!(gcd::<NonZero<u8>>(48, 18), (2, 12));
/// ```
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

/*
/// A trait
pub trait NonZeroPrimitiveSigned: NonZeroPrimitiveInteger<Zeroable: PrimitiveSigned>  {
    /// The unsigned nonzero type with the same size as this.
    type Unsigned: NonZeroPrimitiveUnsigned;

    /// Computes the absolute value of `self`.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn abs(self) -> Self;

    /// Checked absolute value. Computes the absolute value of `self`, returning `None`
    /// if it overflows.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_abs(self) -> Option<Self>;

    /// Saturating absolute value. Computes the absolute value of `self`, saturating to 
    /// `Self::MAX` when it would overflow.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_abs(self) -> Self;

    /// Overflowing absolute value. Computes the absolute value of `self`, returning a 
    /// tuple of the result and a bool indicating if the operation overflowed.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_abs(self) -> (Self, bool);

    /// Overflowing absolute value. Computes the absolute value of `self`, wrapping when 
    /// `self == Self::MIN`.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn wrapping_abs(self) -> Self;

    // /// Computes the absolute value of `Self` without any wrapping or panicking.
    // #[must_use = "this returns the result of the operation, without modifying the original"]
    // fn unsigned_abs(self) -> Self::Unsigned;

    /// Returns true if `self` is positive and `false` if it is negative.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn is_positive(self) -> bool;

    /// Returns true if `self` is positive and `false` if it is negative.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn is_negative(self) -> bool;

    /// Checked negation. Compute `-self`, returning `None` if it overflows.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_neg(self) -> Self;

    /// Checked negation. Compute `-self`, returning a tuple of the result and a bool 
    /// indicating if the operation overflowed.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_neg(self) -> Self;

    /// Checked negation. Compute `-self`, saturating to `Self::MAX` when it would overflow.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_neg(self) -> Self;

    // /// Returns the bit pattern of `self` reinterpreted as an unsigned integer of the same size.
    // #[must_use = "this returns the result of the operation, without modifying the original"]
    // fn cast_unsigned(self) -> Self::Unsigned;
}
*/

macro_rules! impl_nonzero {
    (@uint $($t:ty),+) => {
        impl_nonzero!($($t, <$t as PrimitiveUnsigned>::Signed),+);
    };
    ($($t:ty),+) => {$(
        impl NonZeroPrimitiveInteger for NonZero<$t> {
            type Zeroable = $t;
            forward! {
                fn new(n: Self::Zeroable) -> Option<Self>;
            }
            forward! {
                fn get(self) -> Self::Zeroable;
                fn leading_zeros(self) -> u32;
                fn trailing_zeros(self) -> u32;
                fn checked_mul(self, rhs: Self) -> Option<Self>;
                fn checked_pow(self, exponent: u32) -> Option<Self>;
                fn saturating_pow(self, exponent: u32) -> Self;
                fn count_ones(self) -> NonZero<u32>;
            }
            forward! {
                unsafe fn new_unchecked(n: Self::Zeroable) -> Self;
            }
            // not forwarding because core's docs are inaccurate
            // It specifies that it can only saturate to `Self::MAX` 
            // even on signed types.
            fn saturating_mul(self, rhs: Self) -> Self {
                self.saturating_mul(rhs)
            }
        }
    )+};
}
impl_nonzero!(@uint u8, u16, u32, u64, u128, usize);