use core::ops::{Add, Div, Mul, Neg, Sub};

/// A trait for a type that can represent a real number.
pub trait RealField:
    Copy
    + Add<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Neg<Output = Self>
    + Sub<Output = Self>
    + PartialEq
    + PartialOrd
    + sealed::RealField
{
    /// The additive identity element.
    const ZERO: Self;

    /// The multiplicative identity element.
    const ONE: Self;

    /// Computes the distance between the origin and the point with coordinates `x` and `y`.
    fn hypot(x: Self, y: Self) -> Self;

    /// Compute the absolute value of `self`.
    fn abs(self) -> Self;

    /// Returns `true` if the number is Nan.
    fn is_nan(self) -> bool;
}

macro_rules! forward {
    (
        $(
            fn $name:ident(
                $self:ident $(: $selfTy:ident)?
                $(, $args:ident: $argsTy:ident)*
            ) -> $resTy:ident;
        )+
    ) => {
        $(
            #[inline]
            fn $name($self $(: $selfTy)? $(, $args: $argsTy)*) -> $resTy {
                $self.$name($($args,)*)
            }
        )*
    };
}

macro_rules! impl_real_field_for_float {
    ($ty:ident) => {
        impl RealField for $ty {
            const ZERO: Self = 0.0;
            const ONE: Self = 1.0;

            forward! {
                fn hypot(x: Self, y: Self) -> Self;
                fn abs(self) -> Self;
                fn is_nan(self) -> bool;
            }
        }
    };
}

impl_real_field_for_float!(f32);
impl_real_field_for_float!(f64);

mod sealed {
    pub trait RealField {}

    impl RealField for f32 {}
    impl RealField for f64 {}
}
