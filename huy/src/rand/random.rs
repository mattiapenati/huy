//! Define the default random distribution for native types.

use core::{mem::MaybeUninit, slice};

use super::Rng;

/// Generate a random value using the given source of randomness.
///
/// See [`Random`] trait for more information about the distribution of the generated values.
#[inline]
pub fn random<T: Random>(rng: &mut Rng) -> T {
    Random::random(rng)
}

/// Fill the slice with random values using the given source of randomness.
///
/// See [`Random`] trait for more information about the distribution of the generated values.
#[inline]
pub fn fill_random<T: Random>(rng: &mut Rng, data: &mut [T]) {
    Random::fill_random(rng, data);
}

/// Generate a random value using the given source of randomness.
///
/// This trait generate values with the following distributions:
/// * `bool` samples `true` and `false` with equal probability.
/// * Integers are uniformly distributed over the entire range of the type.
/// * Floating-point numbers are uniformly distributed over the half-open interval `[0, 1)`.
/// * Tuple types are generated by recursively generating each field.
/// * Arrays are generated by recursively generating each element.
pub trait Random: Sized {
    /// Generate a random value.
    fn random(rng: &mut Rng) -> Self;

    /// Fill the slice with random values.
    #[inline]
    fn fill_random(rng: &mut Rng, data: &mut [Self]) {
        for x in data.iter_mut() {
            *x = Self::random(rng);
        }
    }
}

impl Random for bool {
    #[inline]
    fn random(rng: &mut Rng) -> Self {
        i64::random(rng) > 0
    }
}

macro_rules! impl_random_integers {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl Random for $ty {
                #[inline]
                fn random(rng: &mut Rng) -> Self {
                    const OFFSET: usize = 8 * (size_of::<u64>() - size_of::<$ty>());
                    (rng.next_u64() >> OFFSET) as $ty
                }

                #[inline]
                fn fill_random(rng: &mut Rng, data: &mut [Self]) {
                    rng.fill_bytes(as_byte_slice(data));
                }
            }
        )+
    };
}

macro_rules! imlp_random_large_integers {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl Random for $ty {
                #[inline]
                fn random(rng: &mut Rng) -> Self {
                    let lo = u64::random(rng) as $ty;
                    let hi = u64::random(rng) as $ty;
                    (hi << 64) | lo
                }

                #[inline]
                fn fill_random(rng: &mut Rng, data: &mut [Self]) {
                    rng.fill_bytes(as_byte_slice(data));
                }
            }
        )+
    };
}

fn as_byte_slice<T: Sized>(data: &mut [T]) -> &mut [u8] {
    unsafe { slice::from_raw_parts_mut(data.as_ptr() as *mut u8, size_of_val(data)) }
}

impl_random_integers![u8, u16, u32, u64, usize, i8, i16, i32, i64, isize];
imlp_random_large_integers![u128, i128];

macro_rules! impl_random_float {
    ($ty:ty, $offset:expr) => {
        impl Random for $ty {
            #[inline]
            fn random(rng: &mut Rng) -> Self {
                const SCALE: $ty = 1.0 / ((1u64 << (64 - $offset)) as $ty);
                let unsigned = rng.next_u64() >> $offset;
                (unsigned as $ty) * SCALE
            }
        }
    };
    () => {};
}

impl_random_float!(f32, 32 - 23);
impl_random_float!(f64, 64 - 52);

macro_rules! impl_random_tuple {
    ($x0:ident $($xi:ident)*) => {
        impl_random_tuple!([] [$x0 $($xi)*]);
    };
    ([$($xi:ident)*] [$y0:ident $($yi:ident)*]) => {
        impl<$($xi: Random,)* $y0: Random> Random for ($($xi,)* $y0,) {
            #[inline]
            fn random(rng: &mut Rng) -> Self {
                ($($xi::random(rng),)* $y0::random(rng), )
            }
        }
        impl_random_tuple!([$($xi)* $y0] [$($yi)*]);
    };
    ([$($xi:ident)+] []) => {

    };
}

impl_random_tuple!(A B C D E F G H I J K L);

impl<T: Random, const N: usize> Random for [T; N] {
    #[inline]
    fn random(rng: &mut Rng) -> Self {
        let mut data: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for x in data.iter_mut() {
            x.write(T::random(rng));
        }

        unsafe { core::mem::transmute_copy(&data) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chi_squred_test_for_bool() {
        let sample_size = 1_000_000;

        let mut rng = Rng::from_random_state();
        let mut data = vec![true; sample_size];
        fill_random(&mut rng, &mut data);

        let true_values = data.into_iter().filter(|&x| x).count() as f64;
        let expected_true = sample_size as f64 / 2.0;

        let false_values = sample_size as f64 - true_values;
        let expected_false = sample_size as f64 / 2.0;

        let chi_squared = (true_values - expected_true).powi(2) / expected_true
            + (false_values - expected_false).powi(2) / expected_false;

        let critical_value = 3.841; // 0.05 significance level
        assert!(chi_squared < critical_value);
    }
}
