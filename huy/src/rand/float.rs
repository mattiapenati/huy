//! Random float generator

use super::{Random, Rng};

/// A trait for a type that can represent a float.
pub trait Float: sealed::Float {}

macro_rules! impl_float {
    ($($ty:ty),+) => {
        $(impl Float for $ty{})+
    };
}

impl_float![f32, f64];

/// Sample floating point numbers from a uniform distribution.
pub struct UniformFloat<T: Float> {
    low: T,
    scale: T,
}

impl<T: Float> UniformFloat<T> {
    /// Creates a new [`UniformFloat`] distribution, sampled values belongs to the half-open
    /// interval `[low, high)`.
    ///
    /// Panic if the interval is not finite or low >= high.
    pub fn new(low: T, high: T) -> Self {
        assert!(
            low.is_finite() && high.is_finite() && high > low,
            "invalid interval: {low:?}..{high:?}"
        );

        UniformFloat {
            low,
            scale: high - low,
        }
    }

    /// Generate a random float using the given source of randomness.
    #[inline]
    pub fn sample(&self, rng: &mut Rng) -> T {
        self.low + self.scale * Random::random(rng)
    }
}

mod sealed {
    use core::{
        fmt::Debug,
        ops::{Add, Mul, Sub},
    };

    use super::Random;

    pub trait Float:
        Copy
        + Debug
        + Mul<Output = Self>
        + Add<Output = Self>
        + Sub<Output = Self>
        + PartialOrd
        + Random
    {
        /// Check if the value is finite.
        fn is_finite(self) -> bool;
    }

    macro_rules! impl_float {
        ($ty:ty) => {
            impl Float for $ty {
                #[inline]
                fn is_finite(self) -> bool {
                    self.is_finite()
                }
            }
        };
    }

    impl_float!(f32);
    impl_float!(f64);
}
