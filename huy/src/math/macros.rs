macro_rules! impl_vector_space {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident<$field:ident: $trait:ident> {
            $(#[$x0Meta:meta])*
            $x0Vis:vis $x0:ident: $x0Ty:ty,
            $(
                $(#[$xiMeta:meta])*
                $xiVis:vis $xi:ident: $xiTy:ty,
            )*
        }

        $(
            impl<$implField:ident: $implReal:ident> $implName:ident<$implField2:ident> {
                $($implBody:tt)*
            }
        )?
    ) => {
        $(#[$meta])*
        $vis struct $name<$field: $trait> {
            $(#[$x0Meta])*
            $x0Vis $x0: $x0Ty,
            $(
                $(#[$xiMeta])*
                $xiVis $xi: $xiTy,
            )*
        }

        impl<$field: $trait> $name<$field> {
            /// The additive identity element, all zeroes.
            pub const ZERO: Self = Self { $x0: <$x0Ty>::ZERO $(, $xi: <$xiTy>::ZERO)* };

            $($($implBody)*)?

            /// Returns `true` if at least one component is NaN.
            #[inline]
            pub fn is_nan(self) -> bool {
                self.$x0.is_nan() $(|| self.$xi.is_nan())*
            }
        }

        impl<$field: $trait> core::ops::Neg for $name<$field> {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self::Output {
                $name {
                    $x0: -self.$x0,
                    $($xi: -self.$xi,)*
                }
            }
        }

        impl<$field: $trait> core::ops::Add for $name<$field> {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                $name {
                    $x0: self.$x0 + rhs.$x0,
                    $($xi: self.$xi + rhs.$xi,)*
                }
            }
        }

        impl<$field: $trait> core::ops::Sub for $name<$field> {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self::Output {
                $name {
                    $x0: self.$x0 - rhs.$x0,
                    $($xi: self.$xi - rhs.$xi,)*
                }
            }
        }

        impl<$field: $trait> core::ops::Mul<$field> for $name<$field> {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: $field) -> Self::Output {
                $name {
                    $x0: self.$x0 * rhs,
                    $($xi: self.$xi * rhs,)*
                }
            }
        }

        impl<$field: $trait> core::ops::Div<$field> for $name<$field> {
            type Output = Self;

            #[inline]
            fn div(self, rhs: $field) -> Self::Output {
                $name {
                    $x0: self.$x0 / rhs,
                    $($xi: self.$xi / rhs,)*
                }
            }
        }

        impl<$field: $trait> core::ops::AddAssign for $name<$field> {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.$x0 = self.$x0 + rhs.$x0;
                $(self.$xi = self.$xi + rhs.$xi;)*
            }
        }

        impl<$field: $trait> core::ops::SubAssign for $name<$field> {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.$x0 = self.$x0 - rhs.$x0;
                $(self.$xi = self.$xi - rhs.$xi;)*
            }
        }

        impl<$field: $trait> core::ops::MulAssign<$field> for $name<$field> {
            #[inline]
            fn mul_assign(&mut self, rhs: $field) {
                self.$x0 = self.$x0 * rhs;
                $(self.$xi = self.$xi * rhs;)*
            }
        }

        impl<$field: $trait> core::ops::DivAssign<$field> for $name<$field> {
            #[inline]
            fn div_assign(&mut self, rhs: $field) {
                self.$x0 = self.$x0 / rhs;
                $(self.$xi = self.$xi / rhs;)*
            }
        }

        impl<$field: $trait> core::iter::Sum for $name<$field> {
            fn sum<I: Iterator<Item = $name<$field>>>(iter: I) -> Self {
                iter.fold(Self::ZERO, |a, b| a + b)
            }
        }

        impl<'a, $field: $trait> core::iter::Sum<&'a $name<$field>> for $name<$field> {
            fn sum<I: Iterator<Item = &'a $name<$field>>>(iter: I) -> Self {
                iter.fold(Self::ZERO, |a, b| a + *b)
            }
        }

        impl<$field> $crate::approx::ApproxEq for $name<$field>
        where
            $field: $trait + $crate::approx::ApproxEq,
        {
            type Epsilon = $field::Epsilon;

            #[inline]
            fn default_epsilon() -> Self::Epsilon {
                $field::default_epsilon()
            }

            fn almost_eq(&self, other: &Self, max_ulps: usize) -> bool {
                if self.is_nan() || other.is_nan() {
                    return false;
                }
                self.$x0.almost_eq(&other.$x0, max_ulps) $(&& self.$xi.almost_eq(&other.$xi, max_ulps))*
            }

            fn almost_ne(&self, other: &Self, max_ulps: usize) -> bool {
                if self.is_nan() || other.is_nan() {
                    return false;
                }
                self.$x0.almost_ne(&other.$x0, max_ulps) $(|| self.$xi.almost_ne(&other.$xi, max_ulps))*
            }

            fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                if self.is_nan() || other.is_nan() {
                    return false;
                }
                self.$x0.relative_eq(&other.$x0, epsilon) $(&& self.$xi.relative_eq(&other.$xi, epsilon))*
            }

            fn relative_ne(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                if self.is_nan() || other.is_nan() {
                    return false;
                }
                self.$x0.relative_ne(&other.$x0, epsilon) $(|| self.$xi.relative_ne(&other.$xi, epsilon))*
            }

            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                if self.is_nan() || other.is_nan() {
                    return false;
                }
                self.$x0.abs_diff_eq(&other.$x0, epsilon) $(&& self.$xi.abs_diff_eq(&other.$xi, epsilon))*
            }

            fn abs_diff_ne(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                if self.is_nan() || other.is_nan() {
                    return false;
                }
                self.$x0.abs_diff_ne(&other.$x0, epsilon) $(|| self.$xi.abs_diff_ne(&other.$xi, epsilon))*
            }
        }
    };
}

macro_rules! impl_vector_norms {
    (
        $name:ident {
            $x0:ident $(, $xi:ident)*
        }
    ) => {
        impl<T: Field> $name<T> {
            /// Computes the dot product between `self` and `other`.
            #[inline]
            pub fn dot(self, other: Self) -> T {
                self.$x0.conj() * other.$x0 $(+ self.$xi.conj() * other.$xi)*
            }

            /// Computes the squared norm of `self`.
            #[inline]
            pub fn norm_square(self) -> T::Real {
                self.$x0.abs_square() $(+ self.$xi.abs_square())*
            }

            /// Computes the norm of `self`.
            #[inline]
            pub fn norm(self) -> T::Real {
                let max = self.norm_linf();
                max * ((self.$x0 / max).abs_square() $(+ (self.$xi / max).abs_square())*).sqrt()
            }

            /// Compute the taxicab norm of `self`.
            /// See [norm (mathematics)](https://en.wikipedia.org/wiki/Norm_(mathematics)#p-norm).
            #[inline]
            pub fn norm_l1(self) -> T::Real {
                self.$x0.abs() $(+ self.$xi.abs())*
            }

            /// Compute the maximum norm of `self`.
            /// See [norm (mathematics)](https://en.wikipedia.org/wiki/Norm_(mathematics)#p-norm).
            #[inline]
            pub fn norm_linf(self) -> T::Real {
                let max = self.$x0.abs();
                $(let max = T::Real::max(max, self.$xi.abs());)*
                max
            }

            /// Returns `self` with norm equal to 1.
            #[inline]
            pub fn unit(self) -> Self {
                let norm = self.norm();
                Self {
                    $x0: self.$x0 / norm,
                    $($xi: self.$xi / norm,)*
                }
            }

            /// Returns `self` with norm equal to 1 if possible, else `None`.
            #[inline]
            pub fn try_unit(self) -> Option<Self> {
                let norm = self.norm();
                (norm > <T::Real as Field>::ZERO).then(|| Self {
                    $x0: self.$x0 / norm,
                    $($xi: self.$xi / norm,)*
                })
            }

            /// Returns `self` with norm equal to 1 if possible, else the fallback value.
            #[inline]
            pub fn unit_or(self, fallback: Self) -> Self {
                self.try_unit().unwrap_or(fallback)
            }

            /// Returns `self` with norm equal to 1 if possible, else zero.
            #[inline]
            pub fn unit_or_zero(self) -> Self {
                self.try_unit().unwrap_or(Self::ZERO)
            }

        }
    };
}

macro_rules! impl_complex_vector {
    (
        $name:ident { $($xi:ident),* $(,)? }
    ) => {
        impl<T: RealField> core::ops::Mul<T> for $name<Complex<T>> {
            type Output = $name<Complex<T>>;

            #[inline]
            fn mul(self, rhs: T) -> Self::Output {
                $name { $($xi: self.$xi * rhs),* }
            }
        }

        impl<T: RealField> core::ops::Div<T> for $name<Complex<T>> {
            type Output = $name<Complex<T>>;

            #[inline]
            fn div(self, rhs: T) -> Self::Output {
                $name { $($xi: self.$xi / rhs),* }
            }
        }

        impl<T: RealField> core::ops::MulAssign<T> for $name<Complex<T>> {
            #[inline]
            fn mul_assign(&mut self, rhs: T) {
                $(self.$xi = self.$xi * rhs;)*
            }
        }

        impl<T: RealField> core::ops::DivAssign<T> for $name<Complex<T>> {
            #[inline]
            fn div_assign(&mut self, rhs: T) {
                $(self.$xi = self.$xi / rhs;)*
            }
        }

        impl<T: RealField> $name<Complex<T>> {
            /// Returns a real vector with the real part of each component.
            #[inline]
            pub fn real(self) -> $name<T> {
                $name { $($xi: self.$xi.real),* }
            }

            /// Returns a real vector with the imaginary part of each component.
            #[inline]
            pub fn imag(self) -> $name<T> {
                $name { $($xi: self.$xi.imag),* }
            }
        }

        impl<T: RealField> $name<T> {
            /// Construct a new vector with complex components from a real one.
            pub fn to_complex(self) -> $name<Complex<T>> {
                $name { $($xi: self.$xi.into()),* }
            }
        }
    };
}

macro_rules! impl_vector_ops_for_float {
    (
        $name:ident { $($xi:ident),* $(,)? }
    ) => {
        impl $name<f32> {
            /// Cast to [`f64`].
            #[inline]
            pub fn to_f64(self) -> $name<f64> {
                $name { $($xi: self.$xi as f64,)* }
            }
        }

        impl $name<f64> {
            /// Cast to [`f32`].
            #[inline]
            pub fn to_f32(self) -> $name<f32> {
                $name { $($xi: self.$xi as f32,)* }
            }
        }

        impl From<$name<f32>> for $name<f64> {
            #[inline]
            fn from(value: $name<f32>) -> Self {
                value.to_f64()
            }
        }

        impl $name<Complex<f32>> {
            /// Cast to [`f64`].
            #[inline]
            pub fn to_f64(self) -> $name<Complex<f64>> {
                $name { $($xi: self.$xi.to_f64(),)* }
            }
        }

        impl $name<Complex<f64>> {
            /// Cast to [`f32`].
            #[inline]
            pub fn to_f32(self) -> $name<Complex<f32>> {
                $name { $($xi: self.$xi.to_f32(),)* }
            }
        }

        impl From<$name<Complex<f32>>> for $name<Complex<f64>> {
            #[inline]
            fn from(value: $name<Complex<f32>>) -> Self {
                value.to_f64()
            }
        }
    };
}

macro_rules! impl_multiplicative_group {
    (
        impl $name:ident<$field:ident: $trait:ident> {
            $(#[$oneMeta:meta])*
            pub const $one:ident: Self = $oneValue:expr;

            fn mul($mulSelf:ident, $mulRhs:ident: Self) -> Self $mulBlock:block

            $(fn div($divSelf:ident, $divRhs:ident: Self) -> Self $divBlock:block)?
         }
    ) => {
        impl<$field: $trait> $name<$field> {
            $(#[$oneMeta])*
            pub const $one: Self = $oneValue;
         }

        impl<$field: $trait> core::ops::Mul for $name<$field> {
            type Output = Self;

            #[inline]
            fn mul($mulSelf, $mulRhs: Self) -> Self $mulBlock
         }

        impl<$field: $trait> core::ops::MulAssign for $name<$field> {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        $(
            impl<$field: $trait> core::ops::Div for $name<$field> {
                type Output = Self;

                #[inline]
                fn div($divSelf, $divRhs: Self) -> Self $divBlock
            }

            impl<$field: $trait> core::ops::DivAssign for $name<$field> {
                #[inline]
                fn div_assign(&mut self, rhs: Self) {
                    *self = *self / rhs;
                }
            }
        )?

        impl<$field: $trait> core::iter::Product for $name<$field> {
            #[inline]
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::$one, |a, b| a * b)
            }
        }

        impl<'a, $field: $trait> core::iter::Product<&'a $name<$field>> for $name<$field> {
            #[inline]
            fn product<I: Iterator<Item = &'a $name<$field>>>(iter: I) -> Self {
                iter.fold(Self::$one, |a, b| a * *b)
            }
        }
    };
}

macro_rules! impl_aggregate_conversion {
    (From<[$fromTy:ident; $n:expr]> for $name:ident<$field:ident: $trait:ident> { $($xi:ident),+ }) => {
        impl<$field: $trait> From<[$fromTy; $n]> for $name<$field> {
            #[inline]
            fn from(value: [$fromTy; $n]) -> Self {
                let [$($xi),+] = value;
                $name{ $($xi),+ }
            }
        }

        impl<$field: $trait> From<$name<$field>> for [$fromTy; $n] {
            #[inline]
            fn from(value: $name<$field>) -> Self {
                [$(value.$xi),+]
            }
        }
    };
    (From<($($xiTy:ident),+)> for $name:ident<$field:ident: $trait:ident> { $($xi:ident),+ }) => {
        impl<$field: $trait> From<($($xiTy),+)> for $name<$field> {
            #[inline]
            fn from(value: ($($xiTy),+)) -> Self {
                let ($($xi),+) = value;
                $name{ $($xi),+ }
            }
        }

        impl<$field: $trait> From<$name<$field>> for ($($xiTy),+) {
            #[inline]
            fn from(value: $name<$field>) -> Self {
                ($(value.$xi),+)
            }
        }
    }
}

pub(super) use impl_aggregate_conversion;
pub(super) use impl_complex_vector;
pub(super) use impl_multiplicative_group;
pub(super) use impl_vector_norms;
pub(super) use impl_vector_ops_for_float;
pub(super) use impl_vector_space;
