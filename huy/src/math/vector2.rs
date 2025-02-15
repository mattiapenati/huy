use super::{macros::*, Complex, Field, RealField};

/// Create a new [`Vector2`] from its components.
#[inline]
pub const fn vec2<T: Field>(x: T, y: T) -> Vector2<T> {
    Vector2::new(x, y)
}

impl_vector_space! {
    /// A 2-dimensional vector.
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[repr(C)]
    pub struct Vector2<T: Field> {
        /// The x component of the vector.
        pub(super) x: T,
        /// The y component of the vector.
        pub(super) y: T,
    }

    impl<T: Field> Vector2<T> {
        /// A unit vector parallel to the X axis.
        pub const X: Self = Self::new(T::ONE, T::ZERO);

        /// A unit vector parallel to the Y axis.
        pub const Y: Self = Self::new(T::ZERO, T::ONE);

        /// Construct a new vector from its components.
        #[inline]
        pub const fn new(x: T, y: T) -> Self {
            Self { x, y }
        }

        /// Performs a linear interpolation between `self`` and `rhs`.
        #[inline]
        pub fn lerp(self, other: Self, s: T::Real) -> Self {
            self + (other - self) * T::from(s)
        }
    }
}

impl_vector_norms!(Vector2 { x, y });
impl_complex_vector!(Vector2 { x, y });
impl_vector_ops_for_float!(Vector2 { x, y });

impl_aggregate_conversion!(From<[T; 2]> for Vector2<T: Field> { x, y });
impl_aggregate_conversion!(From<(T, T)> for Vector2<T: Field> { x, y });

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! real_test_suite {
        ($ty:ty) => {
            use crate::*;

            mod c {
                #![allow(clippy::excessive_precision)]
                #![allow(non_upper_case_globals)]

                use super::*;

                pub const _zero: Vector2<$ty> = Vector2::ZERO;

                pub const _v1: Vector2<$ty> = Vector2::new(1.0, 2.0);
                pub const _v2: Vector2<$ty> = Vector2::new(3.0, 4.0);
                pub const _v3: Vector2<$ty> = Vector2::new(3.0, -4.0);

                pub const _v1_norm: $ty = 2.23606797749979;
                pub const _v2_norm: $ty = 5.0;
                pub const _v3_norm_l1: $ty = 7.0;
                pub const _v3_norm_linf: $ty = 4.0;
            }

            #[test]
            fn consts() {
                assert_eq!(Vector2::<$ty>::ZERO, vec2::<$ty>(0.0, 0.0));
                assert_eq!(Vector2::<$ty>::X, vec2::<$ty>(1.0, 0.0));
                assert_eq!(Vector2::<$ty>::Y, vec2::<$ty>(0.0, 1.0));
            }

            #[test]
            fn constructor() {
                assert_eq!(c::_zero, vec2::<$ty>(0.0, 0.0));
                assert_eq!(c::_v1, vec2::<$ty>(1.0, 2.0));
            }

            #[test]
            fn dot() {
                assert_almost_eq!(c::_v1.dot(c::_v2), 11.0);
            }

            #[test]
            fn norm() {
                assert_almost_eq!(c::_v1.norm(), c::_v1_norm);
                assert_almost_eq!(c::_v2.norm(), c::_v2_norm);

                assert_almost_eq!(c::_v1.norm_square(), c::_v1_norm * c::_v1_norm);
                assert_almost_eq!(c::_v2.norm_square(), c::_v2_norm * c::_v2_norm);

                assert_almost_eq!(c::_v3.norm_l1(), c::_v3_norm_l1);
                assert_almost_eq!(c::_v3.norm_linf(), c::_v3_norm_linf);
            }

            #[test]
            fn unit() {
                assert_almost_eq!(c::_v1.unit().norm(), 1.0);
                assert_almost_eq!(c::_v2.unit().norm(), 1.0);
                assert_almost_eq!(c::_v3.unit().norm(), 1.0);

                assert!(c::_v1.try_unit().is_some());
                assert!(c::_zero.try_unit().is_none());
                assert_eq!(c::_zero.unit_or_zero(), c::_zero);
                assert_eq!(c::_zero.unit_or(c::_v1), c::_v1);
            }

            #[test]
            fn lerp() {
                assert_eq!(c::_v1.lerp(c::_v2, 0.0), c::_v1);
                assert_eq!(c::_v1.lerp(c::_v2, 1.0), c::_v2);
                assert_almost_eq!(c::_v1.lerp(c::_v2, 0.5), vec2::<$ty>(2.0, 3.0));
            }

            #[test]
            fn array_conversion() {
                let v: Vector2<$ty> = vec2(1.0, 2.0);
                let a: [$ty; 2] = [1.0, 2.0];

                assert_eq!(a, <[$ty; 2]>::from(v));
                assert_eq!(v, Vector2::from(a));
            }

            #[test]
            fn tuple_conversion() {
                let v: Vector2<$ty> = vec2(1.0, 2.0);
                let t: ($ty, $ty) = (1.0, 2.0);

                assert_eq!(t, <($ty, $ty)>::from(v));
                assert_eq!(v, Vector2::from(t));
            }
        };
    }

    mod f32 {
        use super::*;
        real_test_suite!(f32);

        #[test]
        fn to_f64() {
            let v_f32: Vector2<f32> = vec2(1.0, 2.0);
            let v_f64: Vector2<f64> = vec2(1.0, 2.0);

            assert_eq!(v_f32.to_f64(), v_f64);
            assert_eq!(Vector2::<f64>::from(v_f32), v_f64);
        }
    }

    mod f64 {
        use super::*;
        real_test_suite!(f64);

        #[test]
        fn to_f64() {
            let v_f32: Vector2<f32> = vec2(1.0, 2.0);
            let v_f64: Vector2<f64> = vec2(1.0, 2.0);
            assert_eq!(v_f64.to_f32(), v_f32);
        }
    }

    macro_rules! complex_test_suite {
        ($ty:ty) => {
            use crate::*;

            mod c {
                #![allow(clippy::excessive_precision)]
                #![allow(non_upper_case_globals)]

                use super::*;

                pub const _v1: Vector2<Complex<$ty>> = vec2(complex(1.0, 2.0), complex(3.0, 4.0));
                pub const _v2: Vector2<Complex<$ty>> = vec2(complex(5.0, 6.0), complex(7.0, 8.0));

                pub const _v1_norm: $ty = 5.47722557505166113;
                pub const _v2_norm: $ty = 13.1909059582729191;
            }

            #[test]
            fn dot() {
                assert_almost_eq!(c::_v1.dot(c::_v2), complex(70.0, -8.0));
            }

            #[test]
            fn norm() {
                assert_almost_eq!(c::_v1.norm(), c::_v1_norm);
                assert_almost_eq!(c::_v2.norm(), c::_v2_norm);
            }

            #[test]
            fn mul_and_div_by_real() {
                let expected: Vector2<Complex<$ty>> = vec2(complex(2.0, 4.0), complex(6.0, 8.0));
                assert_eq!(c::_v1 * 2.0, expected);

                let expected: Vector2<Complex<$ty>> = vec2(complex(0.5, 1.0), complex(1.5, 2.0));
                assert_eq!(c::_v1 / 2.0, expected);
            }

            #[test]
            fn mul_and_div_by_real_in_place() {
                let mut v = c::_v1;
                v *= 2.0;
                let expected: Vector2<Complex<$ty>> = vec2(complex(2.0, 4.0), complex(6.0, 8.0));
                assert_eq!(v, expected);

                let mut v = c::_v1;
                v /= 2.0;
                let expected: Vector2<Complex<$ty>> = vec2(complex(0.5, 1.0), complex(1.5, 2.0));
                assert_eq!(v, expected);
            }

            #[test]
            fn to_complex() {
                let v: Vector2<$ty> = vec2(1.0, 2.0);
                let c: Vector2<Complex<$ty>> = vec2(complex(1.0, 0.0), complex(2.0, 0.0));
                assert_eq!(v.to_complex(), c);
            }
        };
    }

    mod c32 {
        use crate::math::complex;

        use super::*;
        complex_test_suite!(f32);

        #[test]
        fn to_f64() {
            let v32: Vector2<Complex<f32>> = vec2(complex(1.0, 2.0), complex(3.0, 4.0));
            let v64: Vector2<Complex<f64>> = vec2(complex(1.0, 2.0), complex(3.0, 4.0));

            assert_eq!(v32.to_f64(), v64);
            assert_eq!(Vector2::<Complex<f64>>::from(v32), v64);
        }
    }

    mod c64 {
        use crate::math::complex;

        use super::*;
        complex_test_suite!(f64);

        #[test]
        fn to_f32() {
            let v32: Vector2<Complex<f32>> = vec2(complex(1.0, 2.0), complex(3.0, 4.0));
            let v64: Vector2<Complex<f64>> = vec2(complex(1.0, 2.0), complex(3.0, 4.0));

            assert_eq!(v64.to_f32(), v32);
        }
    }
}
