//! Random integer generator

use core::ops::RangeBounds;

use super::Rng;

/// A trait for a type that can represent an integer.
pub trait Integer: sealed::Integer {}

macro_rules! impl_integer {
    ($($ty:ty),+) => {
        $(impl Integer for $ty{})+
    };
}

impl_integer![i8, u8, i16, u16, i32, u32, i64, u64, isize, usize];

/// Sample integers from a uniform distribution.
pub struct UniformInt<T: Integer> {
    uniform: Option<T::Uniform>,
}

impl<T: Integer> UniformInt<T> {
    /// Creates a new [`UniformInt`] distribution, sampled values belongs to the given range.
    pub fn new<R: RangeBounds<T>>(range: R) -> Self {
        let uniform = T::build_uniform(range);
        Self { uniform }
    }

    /// Generate a random integer using the given source of randomness.
    #[inline]
    pub fn sample(&self, rng: &mut Rng) -> T {
        match &self.uniform {
            Some(uniform) => T::sample_uniform(rng, uniform),
            None => T::random(rng),
        }
    }
}

impl<T: Integer> Default for UniformInt<T> {
    #[inline]
    fn default() -> Self {
        Self { uniform: None }
    }
}

mod sealed {
    use core::{
        num::NonZero,
        ops::{Bound, RangeBounds},
    };

    use super::{super::Random, Rng};

    pub trait Integer: Random {
        /// The data structure used for uniform distribution.
        type Uniform;

        /// Build a sampler to generate uniform distributed integer in the given range.
        ///
        /// If the range is `..`, or equivalent, then `None` is returned.
        fn build_uniform<R: RangeBounds<Self>>(range: R) -> Option<Self::Uniform>;

        /// Sample a value from the given uniform distribution.
        fn sample_uniform(rng: &mut Rng, uniform: &Self::Uniform) -> Self;
    }

    pub trait UniformHelper {
        /// The type used to represent the range of integers (the unsigned type).
        type Range;
    }

    pub struct Uniform<T: UniformHelper> {
        /// The lower bound of the interval.
        lower: T,
        /// The size of the interval.
        range: T::Range,
    }

    macro_rules! build_uniform {
        ($ty:ty, $unsigned:ty) => {
            fn build_uniform<R: RangeBounds<$ty>>(range: R) -> Option<Uniform<$ty>> {
                let lower = range.start_bound();
                let upper = range.end_bound();
                let empty_range_panic = || panic!("empty range: {lower:?}..{upper:?}");

                let lower = match lower {
                    Bound::Included(lower) => *lower,
                    Bound::Excluded(lower) => {
                        lower.checked_add(1).unwrap_or_else(empty_range_panic)
                    }
                    Bound::Unbounded => <$ty>::MIN,
                };
                let upper = match upper {
                    Bound::Included(upper) => *upper,
                    Bound::Excluded(upper) => {
                        upper.checked_sub(1).unwrap_or_else(empty_range_panic)
                    }
                    Bound::Unbounded => <$ty>::MAX,
                };

                if lower >= upper {
                    empty_range_panic();
                }
                if lower == <$ty>::MIN && upper == <$ty>::MAX {
                    return None;
                };

                let range = lower
                    .abs_diff(upper)
                    .checked_add(1)
                    .and_then(NonZero::new)
                    .unwrap();

                Some(Uniform { lower, range })
            }
        };
    }

    macro_rules! impl_integer {
        ($signed:ty, $unsigned:ty) => {
            impl UniformHelper for $signed {
                type Range = NonZero<$unsigned>;
            }

            impl Integer for $signed {
                type Uniform = Uniform<$signed>;
                build_uniform!($signed, $unsigned);

                #[inline]
                fn sample_uniform(rng: &mut Rng, uniform: &Self::Uniform) -> Self {
                    let rand = sample_u64_in_range(rng, uniform.range.get() as u64) as $unsigned;
                    uniform.lower.checked_add_unsigned(rand).unwrap()
                }
            }

            impl UniformHelper for $unsigned {
                type Range = NonZero<$unsigned>;
            }

            impl Integer for $unsigned {
                type Uniform = Uniform<$unsigned>;
                build_uniform!($unsigned, $unsigned);

                #[inline]
                fn sample_uniform(rng: &mut Rng, uniform: &Self::Uniform) -> Self {
                    let rand = sample_u64_in_range(rng, uniform.range.get() as u64) as $unsigned;
                    uniform.lower + rand
                }
            }
        };
    }

    impl_integer!(i8, u8);
    impl_integer!(i16, u16);
    impl_integer!(i32, u32);
    impl_integer!(i64, u64);
    impl_integer!(isize, usize);

    #[inline]
    fn wide_mul(lhs: u64, rhs: u64) -> (u64, u64) {
        let mul = (lhs as u128) * (rhs as u128);
        let hi = (mul >> 64) as u64;
        let lo = mul as u64;
        (hi, lo)
    }

    #[inline]
    fn sample_u64_in_range(rng: &mut Rng, range: u64) -> u64 {
        let x = rng.next_u64();
        let (mut hi, lo) = wide_mul(x, range);
        if lo < range {
            let threshold = range.wrapping_neg();
            hi = loop {
                let x = rng.next_u64();
                let (hi, lo) = wide_mul(x, range);
                if lo >= threshold {
                    break hi;
                }
            };
        }
        hi
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kolmogorov_smirnov_statistic_u8(data: &[u8], start: u8, range: u8) -> f64 {
        // compute the empirical distribution function
        let mut edf = vec![0_f64; range as usize];

        for x in data {
            edf[(x - start) as usize] += 1.0;
        }

        for index in 1..(range as usize) {
            edf[index] += edf[index - 1];
        }
        edf.iter_mut().for_each(|c| *c /= data.len() as f64);

        // compute the cumulative distribution function
        let cdf = (1..=range)
            .map(|x| (x as f64) / (range as f64))
            .collect::<Vec<_>>();

        // compute the Kolmogorov-Smirnov statistic
        edf.into_iter()
            .zip(cdf)
            .map(|(e, c)| (e - c).abs())
            .max_by(f64::total_cmp)
            .unwrap()
    }

    fn kolmogorov_smirnov_critical_value(sample_size: usize) -> f64 {
        1.63 / (sample_size as f64).sqrt()
    }

    #[test]
    fn kolmogorov_smirnov_test_uniform_u8() {
        let mut rng = Rng::from_random_state();

        {
            let uniform = UniformInt::<u8>::new(0..17);
            let sample_size = 1_000_000;
            let sample = (0..sample_size)
                .map(|_| uniform.sample(&mut rng))
                .collect::<Vec<_>>();

            let statistic = kolmogorov_smirnov_statistic_u8(&sample, 0, 17);
            let critical_value = kolmogorov_smirnov_critical_value(sample_size);
            assert!(statistic < critical_value);
        }

        {
            let uniform = UniformInt::<u8>::new(13..30);
            let sample_size = 1_000_000;
            let sample = (0..sample_size)
                .map(|_| uniform.sample(&mut rng))
                .collect::<Vec<_>>();

            let statistic = kolmogorov_smirnov_statistic_u8(&sample, 13, 17);
            let critical_value = kolmogorov_smirnov_critical_value(sample_size);
            assert!(statistic < critical_value);
        }
    }
}
