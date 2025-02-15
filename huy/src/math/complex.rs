use core::{
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use super::{macros::*, RealField};

/// Create a new [`Complex`] from real and imaginary parts.
#[inline]
pub const fn complex<T: RealField>(real: T, imag: T) -> Complex<T> {
    Complex::new(real, imag)
}

impl_vector_space! {
    /// A complex number.
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    #[repr(C)]
    pub struct Complex<T: RealField> {
        /// The real part of the complex number.
        pub real: T,
        /// The imaginary part of the complex number.
        pub imag: T,
    }

    impl<T: RealField> Complex<T> {
        /// The imaginary unit.
        pub const I: Self = Self::new(T::ZERO, T::ONE);

        /// Construct a new complex number from the real and imaginary parts.
        #[inline]
        pub const fn new(real: T, imag: T) -> Self {
            Self { real, imag }
        }

        /// Performs a linear interpolation between `self`` and `rhs`.
        #[inline]
        pub fn lerp(self, other: Self, s: T) -> Self {
            self + (other - self) * s
        }

        /// Returns the complex conjugate.
        #[inline]
        pub fn conj(self) -> Self {
            Self {
                real: self.real,
                imag: -self.imag,
            }
        }

        /// Computes the absolute value of the complex number.
        #[inline]
        pub fn abs(self) -> T {
            T::hypot(self.real, self.imag)
        }

        /// Computes the square of absolute value of the complex number.
        #[inline]
        pub fn abs_square(self) -> T {
            self.real * self.real +  self.imag * self.imag
        }
    }
}

impl_multiplicative_group! {
    impl Complex<T: RealField> {
        /// The multiplicative identity element.
        pub const ONE: Self = Self::new(T::ONE, T::ZERO);

        fn mul(self, rhs: Self) -> Self {
            Complex {
                real: self.real * rhs.real - self.imag * rhs.imag,
                imag: self.real * rhs.imag + self.imag * rhs.real,
            }
        }

        fn div(self, rhs: Self) -> Self {
            let num = self * rhs.conj();
            let den = rhs.real * rhs.real + rhs.imag * rhs.imag;
            num / den
        }
    }
}

impl<T: RealField> Add<T> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        Complex {
            real: self.real + rhs,
            imag: self.imag,
        }
    }
}

impl<T: RealField> Sub<T> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        Complex {
            real: self.real - rhs,
            imag: self.imag,
        }
    }
}

impl<T: RealField> AddAssign<T> for Complex<T> {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        self.real = self.real + rhs;
    }
}

impl<T: RealField> SubAssign<T> for Complex<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        self.real = self.real - rhs;
    }
}

impl<T: RealField> PartialEq<T> for Complex<T> {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        self.real == *other && self.imag == T::ZERO
    }
}

impl<T: RealField> From<T> for Complex<T> {
    #[inline]
    fn from(value: T) -> Self {
        Complex {
            real: value,
            imag: T::ZERO,
        }
    }
}

macro_rules! impl_complex_for_float {
    ($float:ty) => {
        impl core::ops::Add<Complex<$float>> for $float {
            type Output = Complex<$float>;

            #[inline]
            fn add(self, rhs: Complex<$float>) -> Self::Output {
                Complex {
                    real: self + rhs.real,
                    imag: rhs.imag,
                }
            }
        }

        impl core::ops::Sub<Complex<$float>> for $float {
            type Output = Complex<$float>;

            #[inline]
            fn sub(self, rhs: Complex<$float>) -> Self::Output {
                Complex {
                    real: self - rhs.real,
                    imag: -rhs.imag,
                }
            }
        }

        impl core::ops::Mul<Complex<$float>> for $float {
            type Output = Complex<$float>;

            #[inline]
            fn mul(self, rhs: Complex<$float>) -> Self::Output {
                Complex {
                    real: self * rhs.real,
                    imag: self * rhs.imag,
                }
            }
        }

        impl core::ops::Div<Complex<$float>> for $float {
            type Output = Complex<$float>;

            #[inline]
            fn div(self, rhs: Complex<$float>) -> Self::Output {
                let num = self * rhs.conj();
                let den = rhs.real * rhs.real + rhs.imag * rhs.imag;
                num / den
            }
        }

        impl PartialEq<Complex<$float>> for $float {
            #[inline]
            fn eq(&self, other: &Complex<$float>) -> bool {
                *self == other.real && other.imag == 0.0
            }
        }
    };
}

impl_complex_for_float!(f32);
impl_complex_for_float!(f64);

impl Complex<f32> {
    /// Cast to [`f64`].
    #[inline]
    pub fn to_f64(self) -> Complex<f64> {
        Complex {
            real: self.real as f64,
            imag: self.imag as f64,
        }
    }
}

impl Complex<f64> {
    /// Cast to [`f32`].
    #[inline]
    pub fn to_f32(self) -> Complex<f32> {
        Complex {
            real: self.real as f32,
            imag: self.imag as f32,
        }
    }
}

impl From<Complex<f32>> for Complex<f64> {
    #[inline]
    fn from(value: Complex<f32>) -> Self {
        value.to_f64()
    }
}

macro_rules! display_complex {
    ($f:ident, $t:expr, $field:ident, $real:expr, $imag:expr) => {
        let real_neg = $real < $field::ZERO;
        let real_abs = $real.abs();

        let imag_neg = $imag < $field::ZERO;
        let imag_abs = $imag.abs();

        if let Some(precision) = $f.precision() {
            write_complex(
                $f,
                real_neg,
                format_args!(concat!("{:.1$", $t, "}"), real_abs, precision),
                imag_neg,
                format_args!(concat!("{:.1$", $t, "}"), imag_abs, precision),
            )
        } else {
            write_complex(
                $f,
                real_neg,
                format_args!(concat!("{:", $t, "}"), real_abs),
                imag_neg,
                format_args!(concat!("{:", $t, "}"), imag_abs),
            )
        }
    };
}

fn write_complex(
    f: &mut fmt::Formatter<'_>,
    real_neg: bool,
    real: fmt::Arguments<'_>,
    imag_neg: bool,
    imag: fmt::Arguments<'_>,
) -> fmt::Result {
    let real_sign = if real_neg {
        "-"
    } else if f.sign_plus() {
        "+"
    } else {
        ""
    };
    let imag_sign = if imag_neg { "-" } else { "+" };

    write!(f, "{real_sign}{real}{imag_sign}{imag}i")
}

impl<T: RealField + fmt::Display> fmt::Display for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        display_complex! {f, "", T, self.real, self.imag}
    }
}

impl<T: RealField + fmt::LowerExp> fmt::LowerExp for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        display_complex! {f, "e", T, self.real, self.imag}
    }
}

impl<T: RealField + fmt::UpperExp> fmt::UpperExp for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        display_complex! {f, "E", T, self.real, self.imag}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_suite {
        ($ty:ident) => {
            #[test]
            fn constructor_facility() {
                assert_eq!(
                    complex(1.0 as $ty, 2.0 as $ty),
                    Complex::<$ty>::new(1.0, 2.0)
                );
            }

            #[test]
            fn consts() {
                assert_eq!(Complex::<$ty>::ZERO, Complex::<$ty>::new(0.0, 0.0));
                assert_eq!(Complex::<$ty>::ONE, Complex::<$ty>::new(1.0, 0.0));
                assert_eq!(Complex::<$ty>::I, Complex::<$ty>::new(0.0, 1.0));
            }

            #[test]
            fn eq_comparison() {
                assert_eq!(Complex::<$ty>::new(1.0, 2.0), Complex::<$ty>::new(1.0, 2.0));
                assert_ne!(Complex::<$ty>::new(1.0, 2.0), Complex::<$ty>::new(2.0, 1.0));
                assert_eq!(Complex::<$ty>::new(1.0, 0.0), 1.0 as $ty);
                assert_ne!(Complex::<$ty>::new(1.0, 1.0), 1.0 as $ty);
                assert_eq!(1.0 as $ty, Complex::<$ty>::new(1.0, 0.0));
                assert_ne!(1.0 as $ty, Complex::<$ty>::new(1.0, 1.0));
            }

            #[test]
            fn conj() {
                assert_eq!(
                    Complex::<$ty>::new(1.0, 2.0).conj(),
                    Complex::<$ty>::new(1.0, -2.0)
                );
                assert_eq!(
                    Complex::<$ty>::new(1.0, 2.0).conj().conj(),
                    Complex::<$ty>::new(1.0, 2.0)
                );
            }

            #[test]
            fn abs() {
                assert_eq!(Complex::<$ty>::new(0.3, 0.4).abs(), 0.5 as $ty);
                assert_eq!(Complex::<$ty>::new(0.3, 0.4).abs_square(), 0.25 as $ty);
            }

            #[test]
            fn lerp() {
                let c1 = Complex::<$ty>::new(1.0, 2.0);
                let c2 = Complex::<$ty>::new(3.0, 4.0);
                assert_eq!(c1.lerp(c2, 0.0), c1);
                assert_eq!(c1.lerp(c2, 1.0), c2);
                assert_eq!(c1.lerp(c2, 0.5), Complex::<$ty>::new(2.0, 3.0));
                assert_eq!(c1.lerp(c2, -1.0), Complex::<$ty>::new(-1.0, 0.0));
                assert_eq!(c1.lerp(c2, 2.0), Complex::<$ty>::new(5.0, 6.0));
            }

            #[test]
            fn neg() {
                assert_eq!(
                    -Complex::<$ty>::new(0.3, 0.4),
                    Complex::<$ty>::new(-0.3, -0.4)
                );
                assert_eq!(
                    -(-Complex::<$ty>::new(0.3, 0.4)),
                    Complex::<$ty>::new(0.3, 0.4)
                );
            }

            #[test]
            fn add() {
                assert_eq!(
                    Complex::<$ty>::new(1.0, 2.0) + Complex::<$ty>::new(3.0, 4.0),
                    Complex::<$ty>::new(4.0, 6.0)
                );
                assert_eq!(
                    Complex::<$ty>::new(1.0, 2.0) + 2.0 as $ty,
                    Complex::<$ty>::new(3.0, 2.0)
                );
                assert_eq!(
                    2.0 as $ty + Complex::<$ty>::new(1.0, 2.0),
                    Complex::<$ty>::new(3.0, 2.0)
                );
            }

            #[test]
            fn sub() {
                assert_eq!(
                    Complex::<$ty>::new(1.0, 2.0) - Complex::<$ty>::new(3.0, 4.0),
                    Complex::<$ty>::new(-2.0, -2.0)
                );
                assert_eq!(
                    Complex::<$ty>::new(1.0, 2.0) - 2.0 as $ty,
                    Complex::<$ty>::new(-1.0, 2.0)
                );
                assert_eq!(
                    2.0 as $ty - Complex::<$ty>::new(1.0, 2.0),
                    Complex::<$ty>::new(1.0, -2.0)
                );
            }

            #[test]
            fn mul() {
                assert_eq!(
                    Complex::<$ty>::new(1.0, 2.0) * Complex::<$ty>::new(3.0, 4.0),
                    Complex::<$ty>::new(-5.0, 10.0)
                );
                assert_eq!(
                    Complex::<$ty>::new(1.0, 2.0) * 2.0 as $ty,
                    Complex::<$ty>::new(2.0, 4.0)
                );
                assert_eq!(
                    2.0 as $ty * Complex::<$ty>::new(1.0, 2.0),
                    Complex::<$ty>::new(2.0, 4.0)
                );
            }

            #[test]
            fn div() {
                assert_eq!(
                    Complex::<$ty>::new(1.0, 2.0) / Complex::<$ty>::new(3.0, 4.0),
                    Complex::<$ty>::new(0.44, 0.08)
                );
                assert_eq!(
                    Complex::<$ty>::new(1.0, 2.0) / 2.0 as $ty,
                    Complex::<$ty>::new(0.5, 1.0)
                );
                assert_eq!(
                    2.0 as $ty / Complex::<$ty>::new(3.0, 4.0),
                    Complex::<$ty>::new(0.24, -0.32)
                );
            }

            #[test]
            fn add_assign() {
                let mut c = Complex::<$ty>::new(1.0, 2.0);
                c += Complex::<$ty>::new(3.0, 4.0);
                assert_eq!(c, Complex::<$ty>::new(4.0, 6.0));

                let mut c = Complex::<$ty>::new(1.0, 2.0);
                c += 2.0 as $ty;
                assert_eq!(c, Complex::<$ty>::new(3.0, 2.0));
            }

            #[test]
            fn sub_assign() {
                let mut c = Complex::<$ty>::new(1.0, 2.0);
                c -= Complex::<$ty>::new(3.0, 4.0);
                assert_eq!(c, Complex::<$ty>::new(-2.0, -2.0));

                let mut c = Complex::<$ty>::new(1.0, 2.0);
                c -= 2.0 as $ty;
                assert_eq!(c, Complex::<$ty>::new(-1.0, 2.0));
            }

            #[test]
            fn mul_assign() {
                let mut c = Complex::<$ty>::new(1.0, 2.0);
                c *= Complex::<$ty>::new(3.0, 4.0);
                assert_eq!(c, Complex::<$ty>::new(-5.0, 10.0));

                let mut c = Complex::<$ty>::new(1.0, 2.0);
                c *= 2.0 as $ty;
                assert_eq!(c, Complex::<$ty>::new(2.0, 4.0));
            }

            #[test]
            fn div_assign() {
                let mut c = Complex::<$ty>::new(1.0, 2.0);
                c /= Complex::<$ty>::new(3.0, 4.0);
                assert_eq!(c, Complex::<$ty>::new(0.44, 0.08));

                let mut c = Complex::<$ty>::new(1.0, 2.0);
                c /= 2.0 as $ty;
                assert_eq!(c, Complex::<$ty>::new(0.5, 1.0));
            }

            #[test]
            fn conversion_from_scalar() {
                assert_eq!(Complex::<$ty>::from(1.0), Complex::<$ty>::new(1.0, 0.0));
            }

            #[test]
            fn slice_sum() {
                let v = [
                    Complex::<$ty>::new(1.0, 2.0),
                    Complex::<$ty>::new(3.0, 4.0),
                    Complex::<$ty>::new(5.0, 6.0),
                ];
                assert_eq!(
                    v.iter().sum::<Complex<$ty>>(),
                    Complex::<$ty>::new(9.0, 12.0)
                );
                assert_eq!(
                    v.into_iter().sum::<Complex<$ty>>(),
                    Complex::<$ty>::new(9.0, 12.0)
                );
            }

            #[test]
            fn slice_prod() {
                let v = [
                    Complex::<$ty>::new(1.0, 2.0),
                    Complex::<$ty>::new(3.0, 4.0),
                    Complex::<$ty>::new(5.0, 6.0),
                ];
                assert_eq!(
                    v.iter().product::<Complex<$ty>>(),
                    Complex::<$ty>::new(-85.0, 20.0)
                );
                assert_eq!(
                    v.into_iter().product::<Complex<$ty>>(),
                    Complex::<$ty>::new(-85.0, 20.0)
                );
            }

            #[test]
            fn display() {
                assert_eq!(format!("{}", Complex::<$ty>::new(1.0, 2.0)), "1+2i");
                assert_eq!(format!("{}", Complex::<$ty>::new(1.0, -2.0)), "1-2i");
                assert_eq!(format!("{}", Complex::<$ty>::new(-1.0, 2.0)), "-1+2i");
                assert_eq!(format!("{}", Complex::<$ty>::new(-1.0, -2.0)), "-1-2i");
                assert_eq!(format!("{:+}", Complex::<$ty>::new(1.0, 2.0)), "+1+2i");
                assert_eq!(format!("{:+}", Complex::<$ty>::new(1.0, -2.0)), "+1-2i");
                assert_eq!(format!("{:e}", Complex::<$ty>::new(1.0, 2.0)), "1e0+2e0i");
                assert_eq!(format!("{:e}", Complex::<$ty>::new(1.0, -2.0)), "1e0-2e0i");
                assert_eq!(format!("{:E}", Complex::<$ty>::new(1.0, 2.0)), "1E0+2E0i");
                assert_eq!(format!("{:E}", Complex::<$ty>::new(1.0, -2.0)), "1E0-2E0i");
                assert_eq!(
                    format!("{:.1}", Complex::<$ty>::new(1.23, 2.67)),
                    "1.2+2.7i"
                );
                assert_eq!(
                    format!("{:.1}", Complex::<$ty>::new(1.23, -2.67)),
                    "1.2-2.7i"
                );
            }

            // ----------------------------------------------------------------
            // almost_eq
            #[test]
            fn almost_eq_pass_with_identical_values() {
                use crate::assert_almost_eq;

                let lhs = Complex::<$ty>::new(1.0, 2.0);
                let rhs = Complex::<$ty>::new(1.0, 2.0);

                assert_almost_eq!(lhs, rhs);
            }

            #[test]
            fn almost_eq_pass_with_small_difference() {
                use crate::assert_almost_eq;

                let lhs = Complex::<$ty>::new(1.0, 2.0);
                let rhs = Complex::<$ty>::new(1.0 + $ty::EPSILON, 2.0 + $ty::EPSILON);

                assert_ne!(lhs, rhs);
                assert_almost_eq!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn almost_eq_fail_with_nan_lhs() {
                use crate::assert_almost_eq;

                let lhs = Complex::<$ty>::new(1.0, $ty::NAN);
                let rhs = Complex::<$ty>::new(1.0, 2.0);

                assert_almost_eq!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn almost_eq_fail_with_nan_rhs() {
                use crate::assert_almost_eq;

                let lhs = Complex::<$ty>::new(1.0, 2.0);
                let rhs = Complex::<$ty>::new(1.0, $ty::NAN);

                assert_almost_eq!(lhs, rhs);
            }

            // ----------------------------------------------------------------
            // relative_eq
            #[test]
            fn relative_eq_pass_with_identical_values() {
                use crate::assert_relative_eq;

                let lhs = Complex::<$ty>::new(1.0, 2.0);
                let rhs = Complex::<$ty>::new(1.0, 2.0);

                assert_relative_eq!(lhs, rhs);
            }

            #[test]
            fn relative_eq_pass_with_small_difference() {
                use crate::assert_relative_eq;

                let lhs = Complex::<$ty>::new(1.0, 2.0);
                let rhs = Complex::<$ty>::new(1.0 + $ty::EPSILON, 2.0 + $ty::EPSILON);

                assert_ne!(lhs, rhs);
                assert_relative_eq!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn relative_eq_fail_with_nan_lhs() {
                use crate::assert_relative_eq;

                let lhs = Complex::<$ty>::new(1.0, $ty::NAN);
                let rhs = Complex::<$ty>::new(1.0, 2.0);

                assert_relative_eq!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn relative_eq_fail_with_nan_rhs() {
                use crate::assert_relative_eq;

                let lhs = Complex::<$ty>::new(1.0, 2.0);
                let rhs = Complex::<$ty>::new(1.0, $ty::NAN);

                assert_relative_eq!(lhs, rhs);
            }

            // ----------------------------------------------------------------
            // abs_diff_eq
            #[test]
            fn abs_diff_eq_pass_with_identical_values() {
                use crate::assert_abs_diff_eq;

                let lhs = Complex::<$ty>::new(1.0, 2.0);
                let rhs = Complex::<$ty>::new(1.0, 2.0);

                assert_abs_diff_eq!(lhs, rhs);
            }

            #[test]
            fn abs_diff_eq_pass_with_small_difference() {
                use crate::assert_abs_diff_eq;

                let lhs = Complex::<$ty>::new(1.0, 2.0);
                let rhs = Complex::<$ty>::new(1.0 + $ty::EPSILON, 2.0 + $ty::EPSILON);

                assert_ne!(lhs, rhs);
                assert_abs_diff_eq!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn abs_diff_eq_fail_with_nan_lhs() {
                use crate::assert_abs_diff_eq;

                let lhs = Complex::<$ty>::new(1.0, $ty::NAN);
                let rhs = Complex::<$ty>::new(1.0, 2.0);

                assert_abs_diff_eq!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn abs_diff_eq_fail_with_nan_rhs() {
                use crate::assert_abs_diff_eq;

                let lhs = Complex::<$ty>::new(1.0, 2.0);
                let rhs = Complex::<$ty>::new(1.0, $ty::NAN);

                assert_abs_diff_eq!(lhs, rhs);
            }

            // ----------------------------------------------------------------
            // almost_ne
            #[test]
            #[should_panic]
            fn almost_ne_fail_with_identical_values() {
                use crate::assert_almost_ne;

                let lhs = Complex::<$ty>::new(1.0, 2.0);
                let rhs = Complex::<$ty>::new(1.0, 2.0);

                assert_almost_ne!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn almost_ne_fail_with_nan_lhs() {
                use crate::assert_almost_ne;

                let lhs = Complex::<$ty>::new(1.0, $ty::NAN);
                let rhs = Complex::<$ty>::new(1.0, 2.0);

                assert_almost_ne!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn almost_ne_fail_with_nan_rhs() {
                use crate::assert_almost_ne;

                let lhs = Complex::<$ty>::new(1.0, 2.0);
                let rhs = Complex::<$ty>::new(1.0, $ty::NAN);

                assert_almost_ne!(lhs, rhs);
            }

            // ----------------------------------------------------------------
            // relative_ne
            #[test]
            #[should_panic]
            fn relative_ne_fail_with_identical_values() {
                use crate::assert_relative_ne;

                let lhs = Complex::<$ty>::new(1.0, 2.0);
                let rhs = Complex::<$ty>::new(1.0, 2.0);

                assert_relative_ne!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn relative_ne_fail_with_nan_lhs() {
                use crate::assert_relative_ne;

                let lhs = Complex::<$ty>::new(1.0, $ty::NAN);
                let rhs = Complex::<$ty>::new(1.0, 2.0);

                assert_relative_ne!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn relative_ne_fail_with_nan_rhs() {
                use crate::assert_relative_ne;

                let lhs = Complex::<$ty>::new(1.0, 2.0);
                let rhs = Complex::<$ty>::new(1.0, $ty::NAN);

                assert_relative_ne!(lhs, rhs);
            }

            // ----------------------------------------------------------------
            // abs_diff_ne
            #[test]
            #[should_panic]
            fn abs_diff_ne_fail_with_identical_values() {
                use crate::assert_abs_diff_ne;

                let lhs = Complex::<$ty>::new(1.0, 2.0);
                let rhs = Complex::<$ty>::new(1.0, 2.0);

                assert_abs_diff_ne!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn abs_diff_ne_fail_with_nan_lhs() {
                use crate::assert_abs_diff_ne;

                let lhs = Complex::<$ty>::new(1.0, $ty::NAN);
                let rhs = Complex::<$ty>::new(1.0, 2.0);

                assert_abs_diff_ne!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn abs_diff_ne_fail_with_nan_rhs() {
                use crate::assert_abs_diff_ne;

                let lhs = Complex::<$ty>::new(1.0, 2.0);
                let rhs = Complex::<$ty>::new(1.0, $ty::NAN);

                assert_abs_diff_ne!(lhs, rhs);
            }
        };
    }

    mod f32 {
        use super::*;
        test_suite!(f32);

        #[test]
        fn cast_to_f64() {
            assert_eq!(complex(1.0f32, 2.0f32).to_f64(), complex(1.0f64, 2.0f64));
            assert_eq!(
                Complex::<f64>::from(complex(1.0f32, 2.0f32)),
                complex(1.0f64, 2.0f64)
            );
        }
    }

    mod f64 {
        use super::*;
        test_suite!(f64);

        #[test]
        fn cast_to_f32() {
            assert_eq!(complex(1.0f64, 2.0f64).to_f32(), complex(1.0f32, 2.0f32));
        }
    }
}
