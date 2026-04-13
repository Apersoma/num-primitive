use core::convert::Infallible;
use core::num::NonZero;
use core::ops::{Neg, Div, DivAssign, Rem, RemAssign};

use crate::{NonZeroPrimitiveInteger, NonZeroPrimitiveUnsigned, PrimitiveSigned, PrimitiveUnsigned};

/// A trait for [`NonZero`] of a signed integer.
/// 
/// This encapsulates trait implementations, constants, and inherent methods that are common among
/// all of the implementations of `NonZero<T> where T: PrimitiveSigned`.
///
/// See the corresponding items on the individual types for more documentation and examples.
///
/// This trait is sealed with a private trait to prevent downstream implementations, so we may
/// continue to expand along with the standard library without worrying about breaking changes for
/// implementors.
/// 
/// 
pub trait NonZeroPrimitiveSigned: 
    NonZeroPrimitiveInteger<Zeroable: PrimitiveSigned> 
    + TryFrom<NonZero<i8>, Error=Infallible>
    + Neg<Output=Self>
where Self::Zeroable: PrimitiveSigned,
    <Self::Unsigned as NonZeroPrimitiveInteger>::Zeroable: 
        Div<Self::Unsigned, Output=<Self::Unsigned as NonZeroPrimitiveInteger>::Zeroable>
        + DivAssign<Self::Unsigned>
        + Rem<Self::Unsigned, Output=<Self::Unsigned as NonZeroPrimitiveInteger>::Zeroable> 
        + RemAssign<Self::Unsigned>
        + PrimitiveUnsigned
{
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

    /// Computes the absolute value of `Self` without any wrapping or panicking.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn unsigned_abs(self) -> Self::Unsigned;

    /// Returns true if `self` is positive and `false` if it is negative.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn is_positive(self) -> bool;

    /// Returns true if `self` is positive and `false` if it is negative.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn is_negative(self) -> bool;

    /// Checked negation. Compute `-self`, returning `None` if it overflows.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_neg(self) -> Option<Self>;

    /// Checked negation. Compute `-self`, returning a tuple of the result and a bool 
    /// indicating if the operation overflowed.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_neg(self) -> (Self, bool);

    /// Checked negation. Compute `-self`, saturating to `Self::MAX` when it would overflow.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_neg(self) -> Self;

    /// Returns the bit pattern of `self` reinterpreted as an unsigned integer of the same size.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn cast_unsigned(self) -> Self::Unsigned;
}