use super::{macros::*, RealField, Vector3};

/// Create a new [`Point3`] from its components.
#[inline]
pub const fn point3<T: RealField>(x: T, y: T, z: T) -> Point3<T> {
    Point3::new(x, y, z)
}

impl_affine_space! {
    /// A 3-dimensional point.
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[repr(C)]
    pub struct Point3<T: RealField>
    where
        Vector: Vector3
    {
        /// The x coordinate of the point.
        pub(super) x: T,
        /// The y coordinate of the point.
        pub(super) y: T,
        /// The z coordinate of the point.
        pub(super) z: T,
    }
}

impl_affine_space_ops_for_float!(Point3 { x, y, z });

#[cfg(test)]
mod tests {
    use super::{super::vec3, *};

    macro_rules! test_suite {
        ($ty:ty) => {
            mod c {
                #![allow(clippy::excessive_precision)]
                #![allow(non_upper_case_globals)]

                use super::*;
                pub const _zero: Point3<$ty> = point3(0.0, 0.0, 0.0);
            }

            #[test]
            fn consts() {
                assert_eq!(Point3::<$ty>::ORIGIN, point3::<$ty>(0.0, 0.0, 0.0));
            }

            #[test]
            fn constructors() {
                assert_eq!(
                    Point3::<$ty>::new(1.0, 2.0, 3.0),
                    point3::<$ty>(1.0, 2.0, 3.0)
                );
            }

            #[test]
            fn lerp() {
                let a = point3::<$ty>(1.0, 2.0, 3.0);
                let b = point3::<$ty>(4.0, 6.0, 8.0);

                assert_eq!(a.lerp(b, 0.0), a);
                assert_eq!(a.lerp(b, 0.5), point3(2.5, 4.0, 5.5));
                assert_eq!(a.lerp(b, 0.5), a.midpoint(b));
                assert_eq!(a.lerp(b, 1.0), b);
            }

            #[test]
            fn distance() {
                #![allow(clippy::excessive_precision)]

                let a = point3::<$ty>(1.0, 2.0, 3.0);
                let b = point3::<$ty>(4.0, 6.0, 8.0);

                assert_eq!(a.dist(b), 7.071067811865475244008);
                assert_eq!(a.dist_square(b), 50.0);
            }

            #[test]
            fn sub_point() {
                let a = point3::<$ty>(1.0, 2.0, 3.0);
                let b = point3::<$ty>(4.0, 6.0, 8.0);

                assert_eq!(b - a, vec3::<$ty>(3.0, 4.0, 5.0));
            }

            #[test]
            fn add_and_sub_vector() {
                let a = point3::<$ty>(1.0, 2.0, 3.0);
                let b = vec3::<$ty>(3.0, 4.0, 5.0);

                assert_eq!(a + b, point3::<$ty>(4.0, 6.0, 8.0));
                assert_eq!(a - b, point3::<$ty>(-2.0, -2.0, -2.0));
            }

            #[test]
            fn add_and_sub_vector_in_place() {
                let mut a = point3::<$ty>(1.0, 2.0, 3.0);
                let b = vec3::<$ty>(3.0, 4.0, 5.0);

                a += b;
                assert_eq!(a, point3::<$ty>(4.0, 6.0, 8.0));

                a -= b;
                assert_eq!(a, point3::<$ty>(1.0, 2.0, 3.0));
            }

            #[test]
            fn is_nan() {
                let a = point3::<$ty>(1.0, 2.0, 3.0);
                assert!(!a.is_nan());

                let b = point3::<$ty>(<$ty>::NAN, 2.0, 3.0);
                assert!(b.is_nan());

                let b = point3::<$ty>(1.0, <$ty>::NAN, 3.0);
                assert!(b.is_nan());

                let b = point3::<$ty>(1.0, 2.0, <$ty>::NAN);
                assert!(b.is_nan());
            }
        };
    }

    mod f32 {
        use super::*;
        test_suite!(f32);

        #[test]
        fn to_f64() {
            let a = point3::<f32>(1.0, 2.0, 3.0);

            assert_eq!(a.to_f64(), point3::<f64>(1.0, 2.0, 3.0));
            assert_eq!(Point3::<f64>::from(a), point3::<f64>(1.0, 2.0, 3.0));
        }
    }

    mod f64 {
        use super::*;
        test_suite!(f64);

        #[test]
        fn to_f32() {
            let a = point3::<f64>(1.0, 2.0, 3.0);

            assert_eq!(a.to_f32(), point3::<f32>(1.0, 2.0, 3.0));
        }
    }
}
