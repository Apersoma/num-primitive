use crate::{NonZeroPrimitiveInteger, NonZeroPrimitiveUnsigned, PrimitiveSigned};
use core::num::NonZero;

/// temp
/// 
pub trait NonZeroPrimitiveSigned: 
    NonZeroPrimitiveInteger<Zeroable: PrimitiveSigned> 
    + core::ops::Neg<Output=Self>
    + Into<i128>
    + Into<NonZero<i128>>
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
    fn checked_neg(self) -> Self;

    /// Checked negation. Compute `-self`, returning a tuple of the result and a bool 
    /// indicating if the operation overflowed.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn overflowing_neg(self) -> Self;

    /// Checked negation. Compute `-self`, saturating to `Self::MAX` when it would overflow.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_neg(self) -> Self;

    /// Returns the bit pattern of `self` reinterpreted as an unsigned integer of the same size.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn cast_unsigned(self) -> Self::Unsigned;
}