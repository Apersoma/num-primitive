use super::{NonZeroPrimitiveInteger, NonZeroPrimitiveUnsigned, NonZeroPrimitiveSigned};
use crate::PrimitiveUnsigned;
use core::num::NonZero;

macro_rules! impl_nonzero {
    (@uint $($t:ty),+) => {
        impl_nonzero!($($t, <$t as PrimitiveUnsigned>::Signed),+);
        $(
            impl NonZeroPrimitiveUnsigned for NonZero<$t> {
                type Signed = NonZero<<$t as PrimitiveUnsigned>::Signed>;
                forward!(
                    fn saturating_add(self, rhs: $t) -> Self;
                    fn checked_add(self, rhs: $t) -> Option<Self>;
                    fn cast_signed(self) -> Self::Signed;
                    fn ilog2(self) -> u32;
                    fn ilog10(self) -> u32;
                    fn isqrt(self) -> Self;
                    fn midpoint(self, rhs: Self) -> Self;
                    fn checked_next_power_of_two(self) -> Option<Self>;
                    fn is_power_of_two(self) -> bool;
                );
            }
            impl NonZeroPrimitiveSigned for NonZero<<$t as PrimitiveUnsigned>::Signed> {
                type Unsigned = NonZero<$t>;
                forward!(
                    fn abs(self) -> Self;
                    fn checked_abs(self) -> Option<Self>;
                    fn saturating_abs(self) -> Self;
                    fn overflowing_abs(self) -> (Self, bool);
                    fn wrapping_abs(self) -> Self;
                    fn unsigned_abs(self) -> Self::Unsigned;
                    fn is_positive(self) -> bool;
                    fn is_negative(self) -> bool;
                    fn checked_neg(self) -> Option<Self>;
                    fn overflowing_neg(self) -> (Self, bool);
                    fn saturating_neg(self) -> Self;
                    fn cast_unsigned(self) -> Self::Unsigned;
                );
            }
        )+
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