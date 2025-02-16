//! Testing approximate equality of floating-point types.

const DEFAULT_MAX_ULPS: usize = 4;

/// Verifies that two values are approximately equal.
pub trait ApproxEq {
    /// The type for the tolerance used when testing values.
    type Epsilon: Copy;

    /// The default ULPs when testing values.
    fn default_max_ulps() -> usize {
        DEFAULT_MAX_ULPS
    }

    /// The default tolerance when testing values.
    fn default_epsilon() -> Self::Epsilon;

    /// Test for equality that uses the given number of ULPs.
    fn almost_eq(&self, other: &Self, max_ulps: usize) -> bool;

    /// Test for inequality that uses the given number of ULPs.
    fn almost_ne(&self, other: &Self, max_ulps: usize) -> bool {
        !self.almost_eq(other, max_ulps)
    }

    /// Test for equality using the relative difference and the given tolerance.
    fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool;

    /// Test for inequality using the relative difference and the given tolerance.
    fn relative_ne(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        !self.relative_eq(other, epsilon)
    }

    /// Test for equality using the absolute difference and the given tolerance.
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool;

    /// Test for inequality using the absolute difference and the given tolerance.
    fn abs_diff_ne(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        !self.abs_diff_eq(other, epsilon)
    }
}

macro_rules! impl_approx_eq_for_float {
    ($ty:ty) => {
        impl ApproxEq for $ty {
            type Epsilon = Self;

            fn default_epsilon() -> Self::Epsilon {
                <$ty>::EPSILON * 2.0
            }

            fn almost_eq(&self, other: &Self, max_ulps: usize) -> bool {
                if self.is_nan() || other.is_nan() {
                    return false;
                }
                if self == other {
                    return true;
                }

                let self_bits = self.to_bits();
                let other_bits = other.to_bits();

                let min = self_bits.min(other_bits);
                let max = self_bits.max(other_bits);

                (max - min) as usize <= max_ulps
            }

            fn almost_ne(&self, other: &Self, max_ulps: usize) -> bool {
                if self.is_nan() || other.is_nan() {
                    return false;
                }

                let self_bits = self.to_bits();
                let other_bits = other.to_bits();

                let min = self_bits.min(other_bits);
                let max = self_bits.max(other_bits);

                (max - min) as usize > max_ulps
            }

            fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                if self.is_nan() || other.is_nan() {
                    return false;
                }
                if self == other {
                    return true;
                }

                let diff = (self - other).abs();
                let largest = self.abs().max(other.abs());

                diff <= epsilon * largest.min(<$ty>::MAX)
            }

            fn relative_ne(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                if self.is_nan() || other.is_nan() {
                    return false;
                }

                let diff = (self - other).abs();
                let largest = self.abs().max(other.abs());

                diff > epsilon * largest.min(<$ty>::MAX)
            }

            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                if self.is_nan() || other.is_nan() {
                    return false;
                }
                if self == other {
                    return true;
                }

                (self - other).abs() <= epsilon
            }

            fn abs_diff_ne(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                if self.is_nan() || other.is_nan() {
                    return false;
                }

                (self - other).abs() > epsilon
            }
        }
    };
}

impl_approx_eq_for_float!(f32);
impl_approx_eq_for_float!(f64);

#[doc(hidden)]
#[inline]
pub fn check_almost_eq<T: ApproxEq>(left: &T, right: &T, max_ulps: usize) -> bool {
    left.almost_eq(right, max_ulps)
}

#[doc(hidden)]
#[inline]
pub fn check_almost_eq_with_default_max_ulps<T: ApproxEq>(left: &T, right: &T) -> bool {
    left.almost_eq(right, T::default_max_ulps())
}

/// Check approximate equality using ULPs.
#[macro_export]
macro_rules! assert_almost_eq {
    ($left:expr, $right:expr, $max_ulps:expr $(,)?) => {
        match (&($left), &($right), ($max_ulps)) {
            (left, right, max_ulps) => assert!(
                $crate::approx::check_almost_eq(left, right, max_ulps),
                "assert_almost_ne!(left, right, max_ulps = {max_ulps}) failed\n  left: {left:?}\n right: {right:?}",
            ),
        }
    };
    ($left:expr, $right:expr $(,)?) => {
        match (&($left), &($right)) {
            (left, right) => assert!(
                $crate::approx::check_almost_eq_with_default_max_ulps(left, right),
                "assert_almost_ne!(left, right) failed\n  left: {left:?}\n right: {right:?}",
            ),
        }
    };
}

#[doc(hidden)]
#[inline]
pub fn check_almost_ne<T: ApproxEq>(left: &T, right: &T, max_ulps: usize) -> bool {
    left.almost_ne(right, max_ulps)
}

#[doc(hidden)]
#[inline]
pub fn check_almost_ne_with_default_max_ulps<T: ApproxEq>(left: &T, right: &T) -> bool {
    left.almost_ne(right, T::default_max_ulps())
}

/// Check approximate inequality using ULPs.
#[macro_export]
macro_rules! assert_almost_ne {
    ($left:expr, $right:expr, $max_ulps:expr $(,)?) => {
        match (&($left), &($right), ($max_ulps)) {
            (left, right, max_ulps) => assert!(
                $crate::approx::check_almost_ne(left, right, max_ulps),
                "assert_almost_ne!(left, right, max_ulps={max_ulps}) failed\n  left: {left:?}\n right: {right:?}",
            ),
        }
    };
    ($left:expr, $right:expr $(,)?) => {
        match (&($left), &($right)) {
            (left, right) => assert!(
                $crate::approx::check_almost_ne_with_default_max_ulps(left, right),
                "assert_almost_ne!(left, right) failed\n  left: {left:?}\n right: {right:?}",
            ),
        }
    };
}

#[doc(hidden)]
#[inline]
pub fn check_relative_eq<T: ApproxEq>(left: &T, right: &T, epsilon: T::Epsilon) -> bool {
    left.relative_eq(right, epsilon)
}

#[doc(hidden)]
#[inline]
pub fn check_relative_eq_with_default_epsilon<T: ApproxEq>(left: &T, right: &T) -> bool {
    left.relative_eq(right, T::default_epsilon())
}

/// Check approximate equality using relative comparison.
#[macro_export]
macro_rules! assert_relative_eq {
    ($left:expr, $right:expr, $epsilon:expr $(,)?) => {
        match (&($left), &($right), ($epsilon)) {
            (left, right, epsilon) => assert!(
                $crate::approx::check_relative_eq(left, right, epsilon),
                "assert_relative_eq!(left, right, epsilon = {epsilon:?}) failed\n  left: {left:?}\n right: {right:?}",
            ),
        }
    };
    ($left:expr, $right:expr $(,)?) => {
        match (&($left), &($right)) {
            (left, right) => assert!(
                $crate::approx::check_relative_eq_with_default_epsilon(left, right),
                "assert_relative_eq!(left, right) failed\n  left: {left:?}\n right: {right:?}",
            ),
        }
    };
}

#[doc(hidden)]
#[inline]
pub fn check_relative_ne<T: ApproxEq>(left: &T, right: &T, epsilon: T::Epsilon) -> bool {
    left.relative_ne(right, epsilon)
}

#[doc(hidden)]
#[inline]
pub fn check_relative_ne_with_default_epsilon<T: ApproxEq>(left: &T, right: &T) -> bool {
    left.relative_ne(right, T::default_epsilon())
}

/// Check approximate inequality using relative comparison.
#[macro_export]
macro_rules! assert_relative_ne {
    ($left:expr, $right:expr, $epsilon:expr $(,)?) => {
        match (&($left), &($right), ($epsilon)) {
            (left, right, epsilon) => assert!(
                $crate::approx::check_relative_ne(left, right, epsilon),
                "assert_relative_ne!(left, right, epsilon = {epsilon:?}) failed\n  left: {left:?}\n right: {right:?}",
            ),
        }
    };
    ($left:expr, $right:expr $(,)?) => {
        match (&($left), &($right)) {
            (left, right) => assert!(
                $crate::approx::check_relative_ne_with_default_epsilon(left, right),
                "assert_relative_ne!(left, right) failed\n  left: {left:?}\n right: {right:?}",
            ),
        }
    };
}

#[doc(hidden)]
#[inline]
pub fn check_abs_diff_eq<T: ApproxEq>(left: &T, right: &T, epsilon: T::Epsilon) -> bool {
    left.abs_diff_eq(right, epsilon)
}

#[doc(hidden)]
#[inline]
pub fn check_abs_diff_eq_with_default_epsilon<T: ApproxEq>(left: &T, right: &T) -> bool {
    left.abs_diff_eq(right, T::default_epsilon())
}

/// Check approximate equality using absolute difference.
#[macro_export]
macro_rules! assert_abs_diff_eq {
    ($left:expr, $right:expr, $epsilon:expr $(,)?) => {
        match (&($left), &($right), ($epsilon)) {
            (left, right, epsilon) => assert!(
                $crate::approx::check_abs_diff_eq(left, right, epsilon),
                "assert_abs_diff_eq!(left, right, epsilon = {epsilon:?}) failed\n  left: {left:?}\n right: {right:?}",
            ),
        }
    };
    ($left:expr, $right:expr $(,)?) => {
        match (&($left), &($right)) {
            (left, right) => assert!(
                $crate::approx::check_abs_diff_eq_with_default_epsilon(left, right),
                "assert_abs_diff_eq!(left, right) failed\n  left: {left:?}\n right: {right:?}",
            ),
        }
    };
}

#[doc(hidden)]
#[inline]
pub fn check_abs_diff_ne<T: ApproxEq>(left: &T, right: &T, epsilon: T::Epsilon) -> bool {
    left.abs_diff_ne(right, epsilon)
}

#[doc(hidden)]
#[inline]
pub fn check_abs_diff_ne_with_default_epsilon<T: ApproxEq>(left: &T, right: &T) -> bool {
    left.abs_diff_ne(right, T::default_epsilon())
}

/// Check approximate inequality using absolute difference.
#[macro_export]
macro_rules! assert_abs_diff_ne {
    ($left:expr, $right:expr, $epsilon:expr $(,)?) => {
        match (&($left), &($right), ($epsilon)) {
            (left, right, epsilon) => assert!(
                $crate::approx::check_abs_diff_ne(left, right, epsilon),
                "assert_abs_diff_ne!(left, right, epsilon = {epsilon:?}) failed\n  left: {left:?}\n right: {right:?}",
            ),
        }
    };
    ($left:expr, $right:expr $(,)?) => {
        match (&($left), &($right)) {
            (left, right) => assert!(
                $crate::approx::check_abs_diff_ne_with_default_epsilon(left, right),
                "assert_abs_diff_ne!(left, right) failed\n  left: {left:?}\n right: {right:?}",
            ),
        }
    };
}

#[cfg(test)]
mod tests {

    macro_rules! test_suite {
        ($ty:ident) => {
            // ----------------------------------------------------------------
            // almost_eq
            #[test]
            fn almost_eq_pass_with_identical_values() {
                let lhs: $ty = 1.0;
                let rhs: $ty = 1.0;

                assert_almost_eq!(lhs, rhs);
                assert_almost_eq!(lhs, rhs, 10);
            }

            #[test]
            fn almost_eq_pass_with_different_values_but_close_enough() {
                let lhs: $ty = 1.0;
                let rhs: $ty = 1.0 + $ty::EPSILON;

                assert_ne!(lhs, rhs);
                assert_almost_eq!(lhs, rhs);
                assert_almost_eq!(lhs, rhs, 10);
            }

            #[test]
            #[should_panic]
            fn almost_eq_fail_with_nan_lhs() {
                let lhs: $ty = $ty::NAN;
                let rhs: $ty = 1.0;

                assert_almost_eq!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn almost_eq_fail_with_nan_rhs() {
                let lhs: $ty = 1.0;
                let rhs: $ty = $ty::NAN;

                assert_almost_eq!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn almost_eq_fail_with_different_values() {
                let lhs: $ty = 1.0;
                let rhs: $ty = 1.0 + 20.0 * $ty::EPSILON;

                assert_almost_eq!(lhs, rhs);
            }

            // ----------------------------------------------------------------
            // relative_eq
            #[test]
            fn relative_eq_pass_with_identical_values() {
                let lhs: $ty = 1.0;
                let rhs: $ty = 1.0;

                assert_relative_eq!(lhs, rhs);
                assert_relative_eq!(lhs, rhs, $ty::EPSILON);
            }

            #[test]
            fn relative_eq_pass_with_different_values_but_close_enough() {
                let lhs: $ty = 1.0;
                let rhs: $ty = 1.0 + $ty::EPSILON;

                assert_ne!(lhs, rhs);
                assert_relative_eq!(lhs, rhs);
                assert_relative_eq!(lhs, rhs, 4.0 * $ty::EPSILON);
            }

            #[test]
            #[should_panic]
            fn relative_eq_fail_with_nan_lhs() {
                let lhs: $ty = $ty::NAN;
                let rhs: $ty = 1.0;

                assert_relative_eq!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn relative_eq_fail_with_nan_rhs() {
                let lhs: $ty = 1.0;
                let rhs: $ty = $ty::NAN;

                assert_relative_eq!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn relative_eq_fail_with_different_values() {
                let lhs: $ty = 1.0;
                let rhs: $ty = 1.0 + 20.0 * $ty::EPSILON;

                assert_relative_eq!(lhs, rhs);
            }

            // ----------------------------------------------------------------
            // abs_diff_eq
            #[test]
            fn abs_diff_eq_pass_with_identical_values() {
                let lhs: $ty = 1.0;
                let rhs: $ty = 1.0;

                assert_abs_diff_eq!(lhs, rhs);
                assert_abs_diff_eq!(lhs, rhs, $ty::EPSILON);
            }

            #[test]
            fn abs_diff_eq_pass_with_different_values_but_close_enough() {
                let lhs: $ty = 1.0;
                let rhs: $ty = 1.0 + $ty::EPSILON;

                assert_ne!(lhs, rhs);
                assert_abs_diff_eq!(lhs, rhs);
                assert_abs_diff_eq!(lhs, rhs, 4.0 * $ty::EPSILON);
            }

            #[test]
            #[should_panic]
            fn abs_diff_eq_fail_with_nan_lhs() {
                let lhs: $ty = $ty::NAN;
                let rhs: $ty = 1.0;

                assert_abs_diff_eq!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn abs_diff_eq_fail_with_nan_rhs() {
                let lhs: $ty = 1.0;
                let rhs: $ty = $ty::NAN;

                assert_abs_diff_eq!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn abs_diff_eq_fail_with_different_values() {
                let lhs: $ty = 1.0;
                let rhs: $ty = 1.0 + 20.0 * $ty::EPSILON;

                assert_abs_diff_eq!(lhs, rhs);
            }

            // ----------------------------------------------------------------
            // almost_ne
            #[test]
            #[should_panic]
            fn almost_ne_fail_with_identical_values() {
                let lhs: $ty = 1.0;
                let rhs: $ty = 1.0;

                assert_almost_ne!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn almost_ne_pass_with_different_values_but_close_enough() {
                let lhs: $ty = 1.0;
                let rhs: $ty = 1.0 + $ty::EPSILON;

                assert_ne!(lhs, rhs);
                assert_almost_ne!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn almost_ne_fail_with_nan_lhs() {
                let lhs: $ty = $ty::NAN;
                let rhs: $ty = 1.0;

                assert_almost_ne!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn almost_ne_fail_with_nan_rhs() {
                let lhs: $ty = 1.0;
                let rhs: $ty = $ty::NAN;

                assert_almost_ne!(lhs, rhs);
            }

            #[test]
            fn almost_ne_pass_with_different_values() {
                let lhs: $ty = 1.0;
                let rhs: $ty = 1.0 + 20.0 * $ty::EPSILON;

                assert_almost_ne!(lhs, rhs);
                assert_almost_ne!(lhs, rhs, 10);
            }

            // ----------------------------------------------------------------
            // relative_ne
            #[test]
            #[should_panic]
            fn relative_ne_fail_with_identical_values() {
                let lhs: $ty = 1.0;
                let rhs: $ty = 1.0;

                assert_relative_ne!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn relative_ne_pass_with_different_values_but_close_enough() {
                let lhs: $ty = 1.0;
                let rhs: $ty = 1.0 + $ty::EPSILON;

                assert_ne!(lhs, rhs);
                assert_relative_ne!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn relative_ne_fail_with_nan_lhs() {
                let lhs: $ty = $ty::NAN;
                let rhs: $ty = 1.0;

                assert_relative_ne!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn relative_ne_fail_with_nan_rhs() {
                let lhs: $ty = 1.0;
                let rhs: $ty = $ty::NAN;

                assert_relative_ne!(lhs, rhs);
            }

            #[test]
            fn relative_ne_pass_with_different_values() {
                let lhs: $ty = 1.0;
                let rhs: $ty = 1.0 + 20.0 * $ty::EPSILON;

                assert_relative_ne!(lhs, rhs);
                assert_relative_ne!(lhs, rhs, 2.0 * $ty::EPSILON);
            }

            // ----------------------------------------------------------------
            // abs_diff_ne
            #[test]
            #[should_panic]
            fn abs_diff_ne_fail_with_identical_values() {
                let lhs: $ty = 1.0;
                let rhs: $ty = 1.0;

                assert_abs_diff_ne!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn abs_diff_ne_pass_with_different_values_but_close_enough() {
                let lhs: $ty = 1.0;
                let rhs: $ty = 1.0 + $ty::EPSILON;

                assert_ne!(lhs, rhs);
                assert_abs_diff_ne!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn abs_diff_ne_fail_with_nan_lhs() {
                let lhs: $ty = $ty::NAN;
                let rhs: $ty = 1.0;

                assert_abs_diff_ne!(lhs, rhs);
            }

            #[test]
            #[should_panic]
            fn abs_diff_ne_fail_with_nan_rhs() {
                let lhs: $ty = 1.0;
                let rhs: $ty = $ty::NAN;

                assert_abs_diff_ne!(lhs, rhs);
            }

            #[test]
            fn abs_diff_ne_pass_with_different_values() {
                let lhs: $ty = 1.0;
                let rhs: $ty = 1.0 + 20.0 * $ty::EPSILON;

                assert_abs_diff_ne!(lhs, rhs);
                assert_abs_diff_ne!(lhs, rhs, 2.0 * $ty::EPSILON);
            }
        };
    }

    mod f32 {
        use super::*;
        test_suite!(f32);
    }

    mod f64 {
        use super::*;
        test_suite!(f64);
    }
}
