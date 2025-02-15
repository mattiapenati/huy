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

    /// The value π/2.
    const FRAC_PI_2: Self;

    /// The value π.
    const PI: Self;

    /// The value 2π.
    const TAU: Self;

    /// Computes the distance between the origin and the point with coordinates `x` and `y`.
    fn hypot(x: Self, y: Self) -> Self;

    /// Compute the absolute value of `self`.
    fn abs(self) -> Self;

    /// Returns `true` if the number is Nan.
    fn is_nan(self) -> bool;

    /// Converts from radians to degrees.
    fn to_degrees(self) -> Self;

    /// Converts from degrees to radians.
    fn to_radians(self) -> Self;

    /// Computes the sine of the angle.
    fn sin(self) -> Self;

    /// Computes the arc-sine of a number.
    fn asin(self) -> Self;

    /// Computes the cosecant of the angle.
    #[inline]
    fn csc(self) -> Self {
        self.sin().recip()
    }

    /// Computes the arc-cosecant of a number.
    #[inline]
    fn acsc(self) -> Self {
        self.recip().asin()
    }

    /// Computes the cosine of the angle.
    fn cos(self) -> Self;

    /// Computes the arc-cosine of a number.
    fn acos(self) -> Self;

    /// Computes the secant of the angle.
    #[inline]
    fn sec(self) -> Self {
        self.cos().recip()
    }

    /// Computes the arc-secant of a number.
    #[inline]
    fn asec(self) -> Self {
        self.recip().acos()
    }

    /// Computes the tangent of the angle.
    fn tan(self) -> Self;

    /// Computes the arc-tangent of a number.
    fn atan(self) -> Self;

    /// Computes the cotangent of the angle.
    #[inline]
    fn cot(self) -> Self {
        self.tan().recip()
    }

    /// Computes the arc-cotangent of a number.
    #[inline]
    fn acot(self) -> Self {
        self.recip().atan()
    }

    /// Computes the four quadrant arc-tangent.
    fn atan2(y: Self, x: Self) -> Self;

    /// Computes the least non-negative remainder, the return value `r` satisfies
    /// `0.0 <= r <= rhs.abs()`.
    fn rem_euclid(self, rhs: Self) -> Self;

    /// Takes the reciprocal of a number.
    fn recip(self) -> Self;
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
            const FRAC_PI_2: Self = core::$ty::consts::FRAC_PI_2;
            const PI: Self = core::$ty::consts::PI;
            const TAU: Self = core::$ty::consts::TAU;

            forward! {
                fn hypot(x: Self, y: Self) -> Self;
                fn abs(self) -> Self;
                fn is_nan(self) -> bool;
                fn to_degrees(self) -> Self;
                fn to_radians(self) -> Self;
                fn sin(self) -> Self;
                fn asin(self) -> Self;
                fn cos(self) -> Self;
                fn acos(self) -> Self;
                fn tan(self) -> Self;
                fn atan(self) -> Self;
                fn atan2(y: Self, x: Self) -> Self;
                fn rem_euclid(self, rhs: Self) -> Self;
                fn recip(self) -> Self;
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
