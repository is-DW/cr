use super::pixel::*;
use crate::alt::Gray;
use crate::alt::GrayAlpha;

#[cfg(feature = "argb")]
use crate::alt::ARGB;

#[cfg(feature = "grb")]
use crate::alt::GRB;

use crate::RGB;
use crate::RGBA;

use core::iter::Sum;
use core::ops::*;

macro_rules! impl_struct_ops_opaque {
    ($ty:ident => $($field:tt)+) => {
        /// `px + px`
        impl<T: Add> Add for $ty<T> {
            type Output = $ty<<T as Add>::Output>;

            #[inline(always)]
            fn add(self, other: $ty<T>) -> Self::Output {
                $ty {
                    $(
                        $field: self.$field + other.$field,
                    )+
                }
            }
        }

        /// `px + px`
        impl<T> AddAssign for $ty<T> where
            T: Add<Output = T> + Copy
        {
            #[inline(always)]
            fn add_assign(&mut self, other: $ty<T>) {
                *self = Self {
                    $(
                        $field: self.$field + other.$field,
                    )+
                };
            }
        }

        /// `px * px`
        impl<T: Mul> Mul for $ty<T> {
            type Output = $ty<<T as Mul>::Output>;

            #[inline(always)]
            fn mul(self, other: $ty<T>) -> Self::Output {
                $ty {
                    $(
                        $field: self.$field * other.$field,
                    )+
                }
            }
        }

        /// `px * px`
        impl<T> MulAssign for $ty<T> where
            T: Mul<Output = T> + Copy
        {
            #[inline(always)]
            fn mul_assign(&mut self, other: $ty<T>) {
                *self = Self {
                    $(
                        $field: self.$field * other.$field,
                    )+
                };
            }
        }

        /// `px - px`
        impl<T: Sub> Sub for $ty<T> {
            type Output = $ty<<T as Sub>::Output>;

            #[inline(always)]
            fn sub(self, other: $ty<T>) -> Self::Output {
                $ty {
                    $(
                        $field: self.$field - other.$field,
                    )+
                }
            }
        }

        /// `px - px`
        impl<T> SubAssign for $ty<T> where
            T: Sub<Output = T> + Copy
        {
            #[inline(always)]
            fn sub_assign(&mut self, other: $ty<T>) {
                *self = Self {
                    $(
                        $field: self.$field - other.$field,
                    )+
                };
            }
        }

        impl<T> Sum<$ty<T>> for $ty<T> where T: Default + Add<Output=T> {
            #[inline(always)]
            fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
                iter.fold($ty::default(), Add::add)
            }
        }
    };
}

macro_rules! impl_struct_ops_alpha {
    ($ty:ident => $($field:tt)+) => {
        /// `px + px`
        impl<T: Add, A: Add> Add for $ty<T, A> {
            type Output = $ty<<T as Add>::Output, <A as Add>::Output>;

            #[inline(always)]
            fn add(self, other: $ty<T, A>) -> Self::Output {
                $ty {
                    $(
                        $field: self.$field + other.$field,
                    )+
                }
            }
        }

        /// `px + px`
        impl<T, A> AddAssign for $ty<T, A> where
            T: Add<Output = T> + Copy,
            A: Add<Output = A> + Copy
        {
            #[inline(always)]
            fn add_assign(&mut self, other: $ty<T, A>) {
                *self = Self {
                    $(
                        $field: self.$field + other.$field,
                    )+
                };
            }
        }

        /// `px - px`
        impl<T: Sub, A: Sub> Sub for $ty<T, A> {
            type Output = $ty<<T as Sub>::Output, <A as Sub>::Output>;

            #[inline(always)]
            fn sub(self, other: $ty<T, A>) -> Self::Output {
                $ty {
                    $(
                        $field: self.$field - other.$field,
                    )+
                }
            }
        }

        /// `px - px`
        impl<T, A> SubAssign for $ty<T, A> where
            T: Sub<Output = T> + Copy,
            A: Sub<Output = A> + Copy
        {
            #[inline(always)]
            fn sub_assign(&mut self, other: $ty<T, A>) {
                *self = Self {
                    $(
                        $field: self.$field - other.$field,
                    )+
                };
            }
        }

        impl<T, A> Sum<$ty<T, A>> for $ty<T, A> where T: Default + Add<Output=T>, A: Default + Add<Output=A> {
            #[inline(always)]
            fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
                iter.fold($ty::default(), Add::add)
            }
        }
    };
}

macro_rules! impl_scalar {
    ($ty:ident) => {
        /// `px - 1`
        impl<T> Sub<T> for $ty<T>
        where
            T: Copy + Sub<Output = T>,
        {
            type Output = $ty<<T as Sub>::Output>;

            #[inline(always)]
            fn sub(self, r: T) -> Self::Output {
                self.map(|l| l - r)
            }
        }

        /// `px - 1`
        impl<T> SubAssign<T> for $ty<T>
        where
            T: Copy + Sub<Output = T>,
        {
            #[inline(always)]
            fn sub_assign(&mut self, r: T) {
                *self = self.map(|l| l - r);
            }
        }

        /// `px + 1`
        impl<T> Add<T> for $ty<T>
        where
            T: Copy + Add<Output = T>,
        {
            type Output = $ty<T>;

            #[inline(always)]
            fn add(self, r: T) -> Self::Output {
                self.map(|l| l + r)
            }
        }

        /// `px + 1`
        impl<T> AddAssign<T> for $ty<T>
        where
            T: Copy + Add<Output = T>,
        {
            #[inline(always)]
            fn add_assign(&mut self, r: T) {
                *self = self.map(|l| l + r);
            }
        }

        /// `px * 1`
        impl<T> Mul<T> for $ty<T>
        where
            T: Copy + Mul<Output = T>,
        {
            type Output = $ty<T>;

            #[inline(always)]
            fn mul(self, r: T) -> Self::Output {
                self.map(|l| l * r)
            }
        }

        /// `px * 1`
        impl<T> MulAssign<T> for $ty<T>
        where
            T: Copy + Mul<Output = T>,
        {
            #[inline(always)]
            fn mul_assign(&mut self, r: T) {
                *self = self.map(|l| l * r);
            }
        }

        /// `px / 1`
        impl<T> Div<T> for $ty<T>
        where
            T: Copy + Div<Output = T>,
        {
            type Output = $ty<T>;

            #[inline(always)]
            fn div(self, r: T) -> Self::Output {
                self.map(|l| l / r)
            }
        }

        /// `px * 1`
        impl<T> DivAssign<T> for $ty<T>
        where
            T: Copy + Div<Output = T>,
        {
            #[inline(always)]
            fn div_assign(&mut self, r: T) {
                *self = self.map(|l| l / r);
            }
        }
    };
}

impl_scalar! {RGB}
impl_scalar! {RGBA}

#[cfg(feature = "argb")]
impl_scalar! {ARGB}

#[cfg(feature = "grb")]
impl_scalar! {GRB}

impl_scalar! {Gray}
impl_scalar! {GrayAlpha}

impl_struct_ops_opaque! {RGB => r g b}

#[cfg(feature = "grb")]
impl_struct_ops_opaque! {GRB => g r b}

impl_struct_ops_opaque! {Gray => 0}

impl_struct_ops_alpha! {RGBA => r g b a}

#[cfg(feature = "argb")]
impl_struct_ops_alpha! {ARGB => a r g b}

impl_struct_ops_alpha! {GrayAlpha => 0 1}
