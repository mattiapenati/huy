use core::{fmt, ops::Div};

use super::{macros::*, RealField};

/// Create a new [`Angle`] from radians.
#[inline]
pub const fn rad<T: RealField>(radians: T) -> Angle<T> {
    Angle::radians(radians)
}

/// Create a new [`Angle`] from degrees.
#[inline]
pub fn deg<T: RealField>(degrees: T) -> Angle<T> {
    Angle::degrees(degrees)
}

impl_vector_space! {
    /// An angle.
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    #[repr(C)]
    pub struct Angle<T: RealField> {
        /// The measure of the angle in radians.
        radians: T,
    }

    impl<T: RealField> Angle<T> {
        /// The right angle (π/2).
        pub const RIGHT: Self = Self::radians(T::FRAC_PI_2);

        /// The straight angle (π).
        pub const STRAIGHT: Self = Self::radians(T::PI);

        /// The full angle (2π).
        pub const FULL: Self = Self::radians(T::TAU);

        /// Creates a new angle from its measure in radians.
        #[inline]
        pub const fn radians(radians: T) -> Self {
            Self { radians }
        }

        /// Returns the measure of the angle in radians.
        #[inline]
        pub fn to_radians(self) -> T {
            self.radians
        }

        /// Creates a new angle from its measure in degrees.
        #[inline]
        pub fn degrees(degrees: T) -> Self {
            Self::radians(degrees.to_radians())
        }

        /// Returns the measure of the angle in degrees.
        #[inline]
        pub fn to_degrees(self) -> T {
            self.radians.to_degrees()
        }

        /// Returns the angle normalized to the range [0, 2π) radians.
        #[inline]
        pub fn normalized(self) -> Self {
            Self::radians(self.radians.rem_euclid(T::TAU))
        }

        /// Computes the sine of the angle.
        #[inline]
        pub fn sin(self) -> T {
            self.radians.sin()
        }

        /// Computes the arc-sine of a number.
        #[inline]
        pub fn asin(value: T) -> Self {
            Self::radians(value.asin())
        }

        /// Computes the cosecant of the angle.
        #[inline]
        pub fn csc(self) -> T {
            self.radians.csc()
        }

        /// Computes the arc-cosecant of a number.
        #[inline]
        pub fn acsc(csc: T) -> Self {
            Self::radians(csc.acsc())
        }

        /// Computes the cosine of the angle.
        #[inline]
        pub fn cos(self) -> T {
            self.radians.cos()
        }

        /// Computes the arc-cosine of a number.
        #[inline]
        pub fn acos(cos: T) -> Self {
            Self::radians(cos.acos())
        }

        /// Computes the secant of the angle.
        #[inline]
        pub fn sec(self) -> T {
            self.radians.sec()
        }

        /// Computes the arc-secant of a number.
        #[inline]
        pub fn asec(sec: T) -> Self {
            Self::radians(sec.asec())
        }

        /// Computes the tangent of the angle.
        #[inline]
        pub fn tan(self) -> T {
            self.radians.tan()
        }

        /// Computes the arc-tangent of a number.
        #[inline]
        pub fn atan(tan: T) -> Self {
            Self::radians(tan.atan())
        }

        /// Computes the cotangent of the angle.
        #[inline]
        pub fn cot(self) -> T {
            self.radians.cot()
        }

        /// Computes the arc-cotangent of a number.
        #[inline]
        pub fn acot(cot: T) -> Self {
            Self::radians(cot.acot())
        }

        /// Computes the four quadrant arc-tangent.
        #[inline]
        pub fn atan2(y: T, x: T) -> Self {
            Self::radians(T::atan2(y, x))
        }
    }
}

impl<T: RealField> Div for Angle<T> {
    type Output = T;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self.radians / rhs.radians
    }
}

impl Angle<f32> {
    /// Cast to [`f64`].
    #[inline]
    pub fn to_f64(self) -> Angle<f64> {
        Angle {
            radians: self.radians as f64,
        }
    }
}

impl Angle<f64> {
    /// Cast to [`f32`].
    #[inline]
    pub fn to_f32(self) -> Angle<f32> {
        Angle {
            radians: self.radians as f32,
        }
    }
}

impl From<Angle<f32>> for Angle<f64> {
    #[inline]
    fn from(value: Angle<f32>) -> Self {
        value.to_f64()
    }
}

macro_rules! display_angle {
    ($f:ident, $t:expr, $field:ident, $radians:expr) => {
        let radians_neg = $radians < $field::ZERO;
        let radians_abs = $radians.abs();

        if let Some(precision) = $f.precision() {
            write_angle(
                $f,
                radians_neg,
                format_args!(concat!("{:.1$", $t, "}"), radians_abs, precision),
            )
        } else {
            write_angle(
                $f,
                radians_neg,
                format_args!(concat!("{:", $t, "}"), radians_abs),
            )
        }
    };
}

fn write_angle(
    f: &mut fmt::Formatter<'_>,
    radians_neg: bool,
    radians: fmt::Arguments<'_>,
) -> fmt::Result {
    let sign = if radians_neg {
        "-"
    } else if f.sign_plus() {
        "+"
    } else {
        ""
    };

    write!(f, "{sign}{} rad", radians)
}

impl<T: RealField + fmt::Display> fmt::Display for Angle<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        display_angle! {f, "", T, self.radians}
    }
}

impl<T: RealField + fmt::LowerExp> fmt::LowerExp for Angle<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        display_angle! {f, "e", T, self.radians}
    }
}

impl<T: RealField + fmt::UpperExp> fmt::UpperExp for Angle<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        display_angle! {f, "E", T, self.radians}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_suite {
        ($ty:ident) => {
            mod c {
                #![allow(clippy::excessive_precision)]
                #![allow(non_upper_case_globals)]

                use super::*;

                pub const _rad1_5: Angle<$ty> = Angle::radians(1.5);

                pub const _deg00: Angle<$ty> = Angle::radians(0.0);
                pub const _deg30: Angle<$ty> = Angle::radians(core::$ty::consts::PI / 6.0);
                pub const _deg45: Angle<$ty> = Angle::radians(core::$ty::consts::PI / 4.0);
                pub const _deg60: Angle<$ty> = Angle::radians(core::$ty::consts::PI / 3.0);
                pub const _deg90: Angle<$ty> = Angle::radians(core::$ty::consts::PI / 2.0);

                pub const _deg180: Angle<$ty> = Angle::radians(core::$ty::consts::PI);
                pub const _deg270: Angle<$ty> = Angle::radians(core::$ty::consts::PI * 3.0 / 2.0);

                pub const _sin00: $ty = 0.0;
                pub const _sin30: $ty = 0.5;
                pub const _sin45: $ty = 0.70710678118654752440084436210484903928483;
                pub const _sin60: $ty = 0.86602540378443864676372317075293618347140;
                pub const _sin90: $ty = 1.0;
            }

            #[test]
            fn constructor_facility() {
                assert_eq!(rad(1.0 as $ty), Angle::<$ty>::radians(1.0));
                assert_eq!(deg(1.0 as $ty), Angle::<$ty>::degrees(1.0));
            }

            #[test]
            fn consts() {
                use core::$ty;

                assert_eq!(Angle::<$ty>::ZERO, rad(0.0 as $ty));
                assert_eq!(Angle::<$ty>::RIGHT, rad($ty::consts::FRAC_PI_2));
                assert_eq!(Angle::<$ty>::STRAIGHT, rad($ty::consts::PI));
                assert_eq!(Angle::<$ty>::FULL, rad($ty::consts::TAU));
            }

            #[test]
            fn to_radians() {
                assert_eq!(rad(1.0 as $ty).to_radians(), 1.0 as $ty);
            }

            #[test]
            #[allow(clippy::excessive_precision)]
            fn to_degrees() {
                assert_eq!(rad(1.0 as $ty).to_degrees(), 57.29577951308232 as $ty);
            }

            #[test]
            fn normalized() {
                assert_eq!(Angle::<$ty>::ZERO.normalized(), Angle::<$ty>::ZERO);
                assert_eq!(Angle::<$ty>::FULL.normalized(), Angle::<$ty>::ZERO);
                assert_eq!((-Angle::<$ty>::FULL).normalized(), Angle::<$ty>::ZERO);
                assert_eq!(Angle::<$ty>::STRAIGHT.normalized(), Angle::<$ty>::STRAIGHT);
                assert_eq!(
                    (-Angle::<$ty>::STRAIGHT).normalized(),
                    Angle::<$ty>::STRAIGHT
                );
            }

            #[test]
            fn sin() {
                use crate::assert_abs_diff_eq;

                assert_abs_diff_eq!(c::_deg00.sin(), c::_sin00);
                assert_abs_diff_eq!(c::_deg30.sin(), c::_sin30);
                assert_abs_diff_eq!(c::_deg45.sin(), c::_sin45);
                assert_abs_diff_eq!(c::_deg60.sin(), c::_sin60);
                assert_abs_diff_eq!(c::_deg90.sin(), c::_sin90);
            }

            #[test]
            fn asin() {
                use crate::assert_abs_diff_eq;

                assert_abs_diff_eq!(Angle::asin(c::_sin00), c::_deg00);
                assert_abs_diff_eq!(Angle::asin(c::_sin30), c::_deg30);
                assert_abs_diff_eq!(Angle::asin(c::_sin45), c::_deg45);
                assert_abs_diff_eq!(Angle::asin(c::_sin60), c::_deg60);
                assert_abs_diff_eq!(Angle::asin(c::_sin90), c::_deg90);
            }

            #[test]
            fn csc() {
                use crate::assert_abs_diff_eq;

                assert_abs_diff_eq!(c::_deg30.csc(), 1.0 / c::_sin30);
                assert_abs_diff_eq!(c::_deg45.csc(), 1.0 / c::_sin45);
                assert_abs_diff_eq!(c::_deg60.csc(), 1.0 / c::_sin60);
                assert_abs_diff_eq!(c::_deg90.csc(), 1.0 / c::_sin90);
            }

            #[test]
            fn acsc() {
                use crate::assert_abs_diff_eq;

                assert_abs_diff_eq!(Angle::acsc(1.0 / c::_sin30), c::_deg30);
                assert_abs_diff_eq!(Angle::acsc(1.0 / c::_sin45), c::_deg45);
                assert_abs_diff_eq!(Angle::acsc(1.0 / c::_sin60), c::_deg60);
                assert_abs_diff_eq!(Angle::acsc(1.0 / c::_sin90), c::_deg90);
            }

            #[test]
            fn cos() {
                use crate::assert_abs_diff_eq;

                assert_abs_diff_eq!(c::_deg00.cos(), c::_sin90);
                assert_abs_diff_eq!(c::_deg30.cos(), c::_sin60);
                assert_abs_diff_eq!(c::_deg45.cos(), c::_sin45);
                assert_abs_diff_eq!(c::_deg60.cos(), c::_sin30);
                assert_abs_diff_eq!(c::_deg90.cos(), c::_sin00);
            }

            #[test]
            fn acos() {
                use crate::assert_abs_diff_eq;

                assert_abs_diff_eq!(Angle::acos(c::_sin90), c::_deg00);
                assert_abs_diff_eq!(Angle::acos(c::_sin60), c::_deg30);
                assert_abs_diff_eq!(Angle::acos(c::_sin45), c::_deg45);
                assert_abs_diff_eq!(Angle::acos(c::_sin30), c::_deg60);
                assert_abs_diff_eq!(Angle::acos(c::_sin00), c::_deg90);
            }

            #[test]
            fn sec() {
                use crate::assert_abs_diff_eq;

                assert_abs_diff_eq!(c::_deg00.sec(), 1.0 / c::_sin90);
                assert_abs_diff_eq!(c::_deg30.sec(), 1.0 / c::_sin60);
                assert_abs_diff_eq!(c::_deg45.sec(), 1.0 / c::_sin45);
                assert_abs_diff_eq!(c::_deg60.sec(), 1.0 / c::_sin30);
            }

            #[test]
            fn asec() {
                use crate::assert_abs_diff_eq;

                assert_abs_diff_eq!(Angle::asec(1.0 / c::_sin90), c::_deg00);
                assert_abs_diff_eq!(Angle::asec(1.0 / c::_sin60), c::_deg30);
                assert_abs_diff_eq!(Angle::asec(1.0 / c::_sin45), c::_deg45);
                assert_abs_diff_eq!(Angle::asec(1.0 / c::_sin30), c::_deg60);
            }

            #[test]
            fn tan() {
                use crate::assert_abs_diff_eq;

                assert_abs_diff_eq!(c::_deg00.tan(), c::_sin00 / c::_sin90);
                assert_abs_diff_eq!(c::_deg30.tan(), c::_sin30 / c::_sin60);
                assert_abs_diff_eq!(c::_deg45.tan(), c::_sin45 / c::_sin45);
                assert_abs_diff_eq!(c::_deg60.tan(), c::_sin60 / c::_sin30);
            }

            #[test]
            fn atan() {
                use crate::assert_abs_diff_eq;

                assert_abs_diff_eq!(Angle::atan(c::_sin00 / c::_sin90), c::_deg00);
                assert_abs_diff_eq!(Angle::atan(c::_sin30 / c::_sin60), c::_deg30);
                assert_abs_diff_eq!(Angle::atan(c::_sin45 / c::_sin45), c::_deg45);
                assert_abs_diff_eq!(Angle::atan(c::_sin60 / c::_sin30), c::_deg60);
            }

            #[test]
            fn cot() {
                use crate::assert_abs_diff_eq;

                assert_abs_diff_eq!(c::_deg30.cot(), c::_sin60 / c::_sin30);
                assert_abs_diff_eq!(c::_deg45.cot(), c::_sin45 / c::_sin45);
                assert_abs_diff_eq!(c::_deg60.cot(), c::_sin30 / c::_sin60);
                assert_abs_diff_eq!(c::_deg90.cot(), c::_sin00 / c::_sin90);
            }

            #[test]
            fn acot() {
                use crate::assert_abs_diff_eq;

                assert_abs_diff_eq!(Angle::acot(c::_sin60 / c::_sin30), c::_deg30);
                assert_abs_diff_eq!(Angle::acot(c::_sin45 / c::_sin45), c::_deg45);
                assert_abs_diff_eq!(Angle::acot(c::_sin30 / c::_sin60), c::_deg60);
                assert_abs_diff_eq!(Angle::acot(c::_sin00 / c::_sin90), c::_deg90);
            }

            #[test]
            fn atan2() {
                use crate::assert_abs_diff_eq;

                assert_abs_diff_eq!(Angle::atan2(c::_sin00, c::_sin90), c::_deg00);
                assert_abs_diff_eq!(Angle::atan2(c::_sin30, c::_sin60), c::_deg30);
                assert_abs_diff_eq!(Angle::atan2(c::_sin45, c::_sin45), c::_deg45);
                assert_abs_diff_eq!(Angle::atan2(c::_sin60, c::_sin30), c::_deg60);
                assert_abs_diff_eq!(Angle::atan2(c::_sin90, c::_sin00), c::_deg90);

                assert_abs_diff_eq!(Angle::atan2(c::_sin90, -c::_sin00), c::_deg90 + c::_deg00);
                assert_abs_diff_eq!(Angle::atan2(c::_sin60, -c::_sin30), c::_deg90 + c::_deg30);
                assert_abs_diff_eq!(Angle::atan2(c::_sin45, -c::_sin45), c::_deg90 + c::_deg45);
                assert_abs_diff_eq!(Angle::atan2(c::_sin30, -c::_sin60), c::_deg90 + c::_deg60);
                assert_abs_diff_eq!(Angle::atan2(c::_sin00, -c::_sin90), c::_deg90 + c::_deg90);

                assert_abs_diff_eq!(Angle::atan2(-c::_sin00, -c::_sin90), c::_deg00 - c::_deg180);
                assert_abs_diff_eq!(Angle::atan2(-c::_sin30, -c::_sin60), c::_deg30 - c::_deg180);
                assert_abs_diff_eq!(Angle::atan2(-c::_sin45, -c::_sin45), c::_deg45 - c::_deg180);
                assert_abs_diff_eq!(Angle::atan2(-c::_sin60, -c::_sin30), c::_deg60 - c::_deg180);
                assert_abs_diff_eq!(Angle::atan2(-c::_sin90, -c::_sin00), c::_deg90 - c::_deg180);

                assert_abs_diff_eq!(Angle::atan2(-c::_sin90, c::_sin00), c::_deg00 - c::_deg90);
                assert_abs_diff_eq!(Angle::atan2(-c::_sin60, c::_sin30), c::_deg30 - c::_deg90);
                assert_abs_diff_eq!(Angle::atan2(-c::_sin45, c::_sin45), c::_deg45 - c::_deg90);
                assert_abs_diff_eq!(Angle::atan2(-c::_sin30, c::_sin60), c::_deg60 - c::_deg90);
                assert_abs_diff_eq!(Angle::atan2(-c::_sin00, c::_sin90), c::_deg90 - c::_deg90);
            }

            #[test]
            fn add_and_sub_angles() {
                use crate::assert_abs_diff_eq;

                assert_abs_diff_eq!(c::_deg30 + c::_deg30, c::_deg60);
                assert_abs_diff_eq!(c::_deg45 + c::_deg45, c::_deg90);

                assert_abs_diff_eq!(c::_deg30 - c::_deg30, c::_deg00);
                assert_abs_diff_eq!(c::_deg30 - c::_deg90, -c::_deg60);
            }

            #[test]
            fn div_angles() {
                use crate::assert_abs_diff_eq;

                assert_abs_diff_eq!(c::_deg45 / c::_deg90, 0.5);
                assert_abs_diff_eq!(c::_deg30 / c::_deg30, 1.0);
                assert_abs_diff_eq!(c::_deg90 / c::_deg45, 2.0);
                assert_abs_diff_eq!(c::_deg90 / c::_deg30, 3.0);
            }

            #[test]
            fn mul_and_div_by_scalar() {
                use crate::assert_abs_diff_eq;

                assert_abs_diff_eq!(c::_deg30 * 2.0, c::_deg60);
                assert_abs_diff_eq!(c::_deg45 * 2.0, c::_deg90);
                assert_abs_diff_eq!(c::_deg30 * 3.0, c::_deg90);

                assert_abs_diff_eq!(c::_deg60 / 2.0, c::_deg30);
                assert_abs_diff_eq!(c::_deg90 / 3.0, c::_deg30);
            }

            #[test]
            fn display() {
                assert_eq!(format!("{:}", c::_rad1_5), "1.5 rad");
                assert_eq!(format!("{:e}", c::_rad1_5), "1.5e0 rad");
                assert_eq!(format!("{:E}", c::_rad1_5), "1.5E0 rad");

                assert_eq!(format!("{:+}", c::_rad1_5), "+1.5 rad");
                assert_eq!(format!("{:+e}", c::_rad1_5), "+1.5e0 rad");
                assert_eq!(format!("{:+E}", c::_rad1_5), "+1.5E0 rad");

                assert_eq!(format!("{:}", -c::_rad1_5), "-1.5 rad");
                assert_eq!(format!("{:e}", -c::_rad1_5), "-1.5e0 rad");
                assert_eq!(format!("{:E}", -c::_rad1_5), "-1.5E0 rad");

                assert_eq!(format!("{:.2}", c::_rad1_5), "1.50 rad");
                assert_eq!(format!("{:.2e}", c::_rad1_5), "1.50e0 rad");
                assert_eq!(format!("{:.2E}", c::_rad1_5), "1.50E0 rad");
            }
        };
    }

    mod f32 {
        use super::*;
        test_suite!(f32);

        #[test]
        fn cast_to_f64() {
            assert_eq!(rad(2.5f32).to_f64(), rad(2.5f64));
            assert_eq!(Angle::<f64>::from(rad(2.5f32)), rad(2.5f64));
        }
    }

    mod f64 {
        use super::*;
        test_suite!(f64);

        #[test]
        fn cast_to_f32() {
            assert_eq!(rad(2.5f64).to_f32(), rad(2.5f32));
        }
    }
}
