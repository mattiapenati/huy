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

pub(super) use impl_multiplicative_group;
pub(super) use impl_vector_space;
