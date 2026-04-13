use core::{convert::Infallible, num::NonZero};

use crate::{NonZeroPrimitiveInteger, NonZeroPrimitiveSigned, PrimitiveUnsigned, PrimitiveSigned};

/// A trait for [`NonZero`] of a unsigned integer.
/// 
/// This encapsulates trait implementations, constants, and inherent methods that are common among
/// all of the implementations of `NonZero<T> where T: PrimitiveUnsigned`.
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
/// use num_primitive::NonZeroPrimitiveUnsigned;
///
/// fn gcd<T: NonZeroPrimitiveUnsigned>(mut u: T, mut v: T) -> u32 {
///     loop {
///         if let Some(u0) = T::new(v.get() % u) {
///             v = u;
///             u = u0;
///         } else {
///             return v;
///         };
///     }
/// }
/// 
/// let nz_3 = NonZero::<u64>::new(3u64).unwrap();
/// let nz_22 = NonZero::new(22).unwrap();
/// assert_eq!(gcd(nz_3, nz_2).get(), 1);
/// 
/// let nz_48 = NonZero::new(48u16).unwrap();
/// let nz_18 = NonZero::new(18).unwrap();
/// assert_eq!(gcd::<NonZero<u16>>(nz_48, nz_18).get(), 6);
/// ```
pub trait NonZeroPrimitiveUnsigned: NonZeroPrimitiveInteger
    + TryFrom<NonZero<u8>, Error=Infallible>
where <Self::Signed as NonZeroPrimitiveInteger>::Zeroable: PrimitiveSigned,
    Self::Zeroable: 
        core::ops::Div<Self, Output=Self::Zeroable>
        + core::ops::DivAssign<Self>
        + core::ops::Rem<Self, Output=Self::Zeroable>
        + core::ops::RemAssign<Self>
        + PrimitiveUnsigned
{
    /// The unsigned nonzero type with the same size as this.
    type Signed: NonZeroPrimitiveSigned;

    /// Saturating integer addition. Computes `self + rhs` saturating to `Self::MAX` on overflow.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_add(self, rhs: Self::Zeroable) -> Self;
    /// Checked integer addition. Computes `self + rhs`, returning `None` on overflow.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_add(self, rhs: Self::Zeroable) -> Option<Self>;
    /// Returns the bit pattern of `self` reinterpreted as an signed integer of the same size.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn cast_signed(self) -> Self::Signed;

    /// Integer log base 2. Computes `ilog₂(self)`.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn ilog2(self) -> u32;
    /// Integer log base 2. Computes `ilog₁₀(self)`.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn ilog10(self) -> u32;

    /// Integer square root. Computes the square root of `self`, rounding down to an integer.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn isqrt(self) -> Self;

    /// Computes the average of `self` and `rhs`, rounding down to an integer.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn midpoint(self, rhs: Self) -> Self;

    /// Computes the next power of 2 greater than `self`. Returns `None` if it overflows.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn checked_next_power_of_two(self) -> Option<Self>;

    /// Checks if `self` is a power of 2.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn is_power_of_two(self) -> bool;
}