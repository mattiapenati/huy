use super::{macros::*, RealField, Vector2};

/// Create a new [`Point2`] from its components.
#[inline]
pub const fn point2<T: RealField>(x: T, y: T) -> Point2<T> {
    Point2::new(x, y)
}

impl_affine_space! {
    /// A 2-dimensional point.
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[repr(C)]
    pub struct Point2<T: RealField>
    where
        Vector: Vector2
    {
        /// The x coordinate of the point.
        pub(super) x: T,
        /// The y coordinate of the point.
        pub(super) y: T,
    }
}

impl_affine_space_ops_for_float!(Point2 { x, y });

#[cfg(test)]
mod tests {
    use super::{super::vec2, *};

    macro_rules! test_suite {
        ($ty:ty) => {
            mod c {
                #![allow(clippy::excessive_precision)]
                #![allow(non_upper_case_globals)]

                use super::*;
                pub const _zero: Point2<$ty> = point2(0.0, 0.0);
            }

            #[test]
            fn consts() {
                assert_eq!(Point2::<$ty>::ORIGIN, point2::<$ty>(0.0, 0.0));
            }

            #[test]
            fn constructors() {
                assert_eq!(Point2::<$ty>::new(1.0, 2.0), point2::<$ty>(1.0, 2.0));
            }

            #[test]
            fn lerp() {
                let a = point2::<$ty>(1.0, 2.0);
                let b = point2::<$ty>(3.0, 4.0);

                assert_eq!(a.lerp(b, 0.0), a);
                assert_eq!(a.lerp(b, 0.5), point2(2.0, 3.0));
                assert_eq!(a.lerp(b, 0.5), a.midpoint(b));
                assert_eq!(a.lerp(b, 1.0), b);
            }

            #[test]
            fn distance() {
                let a = point2::<$ty>(1.0, 2.0);
                let b = point2::<$ty>(4.0, 6.0);

                assert_eq!(a.dist(b), 5.0);
                assert_eq!(a.dist_square(b), 25.0);
            }

            #[test]
            fn sub_point() {
                let a = point2::<$ty>(1.0, 2.0);
                let b = point2::<$ty>(3.0, 4.0);

                assert_eq!(b - a, vec2::<$ty>(2.0, 2.0));
            }

            #[test]
            fn add_and_sub_vector() {
                let a = point2::<$ty>(1.0, 2.0);
                let b = vec2::<$ty>(3.0, 4.0);

                assert_eq!(a + b, point2::<$ty>(4.0, 6.0));
                assert_eq!(a - b, point2::<$ty>(-2.0, -2.0));
            }

            #[test]
            fn add_and_sub_vector_in_place() {
                let mut a = point2::<$ty>(1.0, 2.0);
                let b = vec2::<$ty>(3.0, 4.0);

                a += b;
                assert_eq!(a, point2::<$ty>(4.0, 6.0));

                a -= b;
                assert_eq!(a, point2::<$ty>(1.0, 2.0));
            }

            #[test]
            fn is_nan() {
                let a = point2::<$ty>(1.0, 2.0);
                assert!(!a.is_nan());

                let b = point2::<$ty>(<$ty>::NAN, 2.0);
                assert!(b.is_nan());

                let b = point2::<$ty>(1.0, <$ty>::NAN);
                assert!(b.is_nan());
            }
        };
    }

    mod f32 {
        use super::*;
        test_suite!(f32);

        #[test]
        fn to_f64() {
            let a = point2::<f32>(1.0, 2.0);

            assert_eq!(a.to_f64(), point2::<f64>(1.0, 2.0));
            assert_eq!(Point2::<f64>::from(a), point2::<f64>(1.0, 2.0));
        }
    }

    mod f64 {
        use super::*;
        test_suite!(f64);

        #[test]
        fn to_f32() {
            let a = point2::<f64>(1.0, 2.0);

            assert_eq!(a.to_f32(), point2::<f32>(1.0, 2.0));
        }
    }
}
