use super::{macros::*, Complex, Field, RealField};

/// Create a new [`Vector3`] from its components.
#[inline]
pub const fn vec3<T: Field>(x: T, y: T, z: T) -> Vector3<T> {
    Vector3::new(x, y, z)
}

impl_vector_space! {
    /// A 3-dimensional vector.
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[repr(C)]
    pub struct Vector3<T: Field> {
        /// The x component of the vector.
        pub(super) x: T,
        /// The y component of the vector.
        pub(super) y: T,
        /// The z component of the vector.
        pub(super) z: T,
    }

    impl<T: Field> Vector3<T> {
        /// A unit vector parallel to the X axis.
        pub const X: Self = Self::new(T::ONE, T::ZERO, T::ZERO);

        /// A unit vector parallel to the Y axis.
        pub const Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO);

        /// A unit vector parallel to the Z axis.
        pub const Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE);

        /// Construct a new vector from its components.
        #[inline]
        pub const fn new(x: T, y: T, z: T) -> Self {
            Self { x, y, z}
        }

        /// Performs a linear interpolation between `self`` and `rhs`.
        #[inline]
        pub fn lerp(self, other: Self, s: T::Real) -> Self {
            self + (other - self) * T::from(s)
        }
    }
}

impl_vector_norms!(Vector3 { x, y, z });
impl_complex_vector!(Vector3 { x, y, z });
impl_vector_ops_for_float!(Vector3 { x, y, z });

impl_aggregate_conversion!(From<[T; 3]> for Vector3<T: Field> { x, y, z });
impl_aggregate_conversion!(From<(T, T, T)> for Vector3<T: Field> { x, y, z });

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

                pub const _zero: Vector3<$ty> = Vector3::ZERO;

                pub const _v1: Vector3<$ty> = Vector3::new(1.0, 2.0, 3.0);
                pub const _v2: Vector3<$ty> = Vector3::new(4.0, 5.0, 6.0);
                pub const _v3: Vector3<$ty> = Vector3::new(4.0, -5.0, 6.0);

                pub const _v1_norm: $ty = 3.7416573867739413;
                pub const _v2_norm: $ty = 8.774964387392123;
                pub const _v3_norm_l1: $ty = 15.0;
                pub const _v3_norm_linf: $ty = 6.0;
            }

            #[test]
            fn consts() {
                assert_eq!(Vector3::<$ty>::ZERO, vec3::<$ty>(0.0, 0.0, 0.0));
                assert_eq!(Vector3::<$ty>::X, vec3::<$ty>(1.0, 0.0, 0.0));
                assert_eq!(Vector3::<$ty>::Y, vec3::<$ty>(0.0, 1.0, 0.0));
                assert_eq!(Vector3::<$ty>::Z, vec3::<$ty>(0.0, 0.0, 1.0));
            }

            #[test]
            fn constructor() {
                assert_eq!(c::_zero, vec3::<$ty>(0.0, 0.0, 0.0));
                assert_eq!(c::_v1, vec3::<$ty>(1.0, 2.0, 3.0));
            }

            #[test]
            fn dot() {
                assert_almost_eq!(c::_v1.dot(c::_v2), 32.0);
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
                assert_almost_eq!(c::_v1.lerp(c::_v2, 0.5), vec3::<$ty>(2.5, 3.5, 4.5));
            }

            #[test]
            fn array_conversion() {
                let v: Vector3<$ty> = vec3(1.0, 2.0, 3.0);
                let a: [$ty; 3] = [1.0, 2.0, 3.0];

                assert_eq!(a, <[$ty; 3]>::from(v));
                assert_eq!(v, Vector3::from(a));
            }

            #[test]
            fn tuple_conversion() {
                let v: Vector3<$ty> = vec3(1.0, 2.0, 3.0);
                let t: ($ty, $ty, $ty) = (1.0, 2.0, 3.0);

                assert_eq!(t, <($ty, $ty, $ty)>::from(v));
                assert_eq!(v, Vector3::from(t));
            }
        };
    }

    mod f32 {
        use super::*;
        real_test_suite!(f32);

        #[test]
        fn to_f64() {
            let v_f32: Vector3<f32> = vec3(1.0, 2.0, 3.0);
            let v_f64: Vector3<f64> = vec3(1.0, 2.0, 3.0);

            assert_eq!(v_f32.to_f64(), v_f64);
            assert_eq!(Vector3::<f64>::from(v_f32), v_f64);
        }
    }

    mod f64 {
        use super::*;
        real_test_suite!(f64);

        #[test]
        fn to_f64() {
            let v_f32: Vector3<f32> = vec3(1.0, 2.0, 3.0);
            let v_f64: Vector3<f64> = vec3(1.0, 2.0, 3.0);
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

                pub const _v1: Vector3<Complex<$ty>> =
                    vec3(complex(1.0, 2.0), complex(3.0, 4.0), complex(5.0, 6.0));
                pub const _v2: Vector3<Complex<$ty>> =
                    vec3(complex(7.0, 8.0), complex(9.0, 10.0), complex(11.0, 12.0));

                pub const _v1_norm: $ty = 9.539392014169456;
                pub const _v2_norm: $ty = 23.64318083507377;
            }

            #[test]
            fn dot() {
                assert_almost_eq!(c::_v1.dot(c::_v2), complex(217.0, -18.0));
            }

            #[test]
            fn norm() {
                assert_almost_eq!(c::_v1.norm(), c::_v1_norm);
                assert_almost_eq!(c::_v2.norm(), c::_v2_norm);
            }

            #[test]
            fn mul_and_div_by_real() {
                let expected: Vector3<Complex<$ty>> =
                    vec3(complex(2.0, 4.0), complex(6.0, 8.0), complex(10.0, 12.0));
                assert_eq!(c::_v1 * 2.0, expected);

                let expected: Vector3<Complex<$ty>> =
                    vec3(complex(0.5, 1.0), complex(1.5, 2.0), complex(2.5, 3.0));
                assert_eq!(c::_v1 / 2.0, expected);
            }

            #[test]
            fn mul_and_div_by_real_in_place() {
                let mut v = c::_v1;
                v *= 2.0;
                let expected: Vector3<Complex<$ty>> =
                    vec3(complex(2.0, 4.0), complex(6.0, 8.0), complex(10.0, 12.0));
                assert_eq!(v, expected);

                let mut v = c::_v1;
                v /= 2.0;
                let expected: Vector3<Complex<$ty>> =
                    vec3(complex(0.5, 1.0), complex(1.5, 2.0), complex(2.5, 3.0));
                assert_eq!(v, expected);
            }

            #[test]
            fn to_complex() {
                let v: Vector3<$ty> = vec3(1.0, 2.0, 3.0);
                let c: Vector3<Complex<$ty>> =
                    vec3(complex(1.0, 0.0), complex(2.0, 0.0), complex(3.0, 0.0));
                assert_eq!(v.to_complex(), c);
            }

            #[test]
            fn real() {
                let v: Vector3<Complex<$ty>> =
                    vec3(complex(1.0, 2.0), complex(3.0, 4.0), complex(5.0, 6.0));
                let r: Vector3<$ty> = vec3(1.0, 3.0, 5.0);

                assert_eq!(v.real(), r);
            }

            #[test]
            fn imag() {
                let v: Vector3<Complex<$ty>> =
                    vec3(complex(1.0, 2.0), complex(3.0, 4.0), complex(5.0, 6.0));
                let i: Vector3<$ty> = vec3(2.0, 4.0, 6.0);

                assert_eq!(v.imag(), i);
            }
        };
    }

    mod c32 {
        use crate::math::complex;

        use super::*;
        complex_test_suite!(f32);

        #[test]
        fn to_f64() {
            let v32: Vector3<Complex<f32>> =
                vec3(complex(1.0, 2.0), complex(3.0, 4.0), complex(5.0, 6.0));
            let v64: Vector3<Complex<f64>> =
                vec3(complex(1.0, 2.0), complex(3.0, 4.0), complex(5.0, 6.0));

            assert_eq!(v32.to_f64(), v64);
            assert_eq!(Vector3::<Complex<f64>>::from(v32), v64);
        }
    }

    mod c64 {
        use crate::math::complex;

        use super::*;
        complex_test_suite!(f64);

        #[test]
        fn to_f32() {
            let v32: Vector3<Complex<f32>> =
                vec3(complex(1.0, 2.0), complex(3.0, 4.0), complex(5.0, 6.0));
            let v64: Vector3<Complex<f64>> =
                vec3(complex(1.0, 2.0), complex(3.0, 4.0), complex(5.0, 6.0));

            assert_eq!(v64.to_f32(), v32);
        }
    }
}
