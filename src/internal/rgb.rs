use super::pixel::*;
use crate::alt::BGR;
use crate::alt::BGRA;

#[cfg(feature = "grb")]
use crate::alt::GRB;

use crate::RGB;
use crate::RGBA;
use core::fmt;

impl<T> RGB<T> {
    /// Convenience function for creating a new pixel
    /// The order of arguments is R,G,B
    #[inline(always)]
    pub const fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
}

impl<T> BGR<T> {
    #[inline(always)]
    pub const fn new(b: T, g: T, r: T) -> Self {
        Self { b, g, r }
    }
}

/// `Pod` trait: bytemuck 库定义的类型("Plain Old Data", 是一种数据结构，其内部没有包含引用、
/// 指针或其他复杂的数据类型，只包含简单的基本数据类型，如整数、浮点数和其他POD类型), 只有标记为此
/// 类型的才能使用bytemuck库进行操作
#[cfg(feature = "as-bytes")]
unsafe impl<T> crate::Pod for RGB<T> where T: crate::Pod {}

#[cfg(feature = "as-bytes")]
unsafe impl<T> crate::Pod for BGR<T> where T: crate::Pod {}

/// `Zeroable` trait:
#[cfg(feature = "as-bytes")]
unsafe impl<T> crate::Zeroable for RGB<T> where T: crate::Zeroable {}

#[cfg(feature = "as-bytes")]
unsafe impl<T> crate::Zeroable for BGR<T> where T: crate::Zeroable {}

macro_rules! impl_rgb {
    ($RGB:ident) => {
        impl<T: Clone> $RGB<T> {
            pub fn iter(&self) -> core::iter::Cloned<core::slice::Iter<'_, T>> {
                self.as_slice().iter().cloned()
            }
        }

        impl<T: Copy, B> ComponentMap<$RGB<B>, T, B> for $RGB<T> {
            #[inline(always)]
            fn map<F>(&self, mut f: F) -> $RGB<B>
            where
                F: FnMut(T) -> B,
            {
                $RGB {
                    r: f(self.r),
                    g: f(self.g),
                    b: f(self.b),
                }
            }
        }

        impl<T: Copy, B> ColorComponentMap<$RGB<B>, T, B> for $RGB<T> {
            #[inline(always)]
            fn map_c<F>(&self, mut f: F) -> $RGB<B>
            where
                F: FnMut(T) -> B,
            {
                $RGB {
                    r: f(self.r),
                    g: f(self.g),
                    b: f(self.b),
                }
            }
        }

        impl<T> ComponentSlice<T> for $RGB<T> {
            #[inline(always)]
            fn as_slice(&self) -> &[T] {
                unsafe { core::slice::from_raw_parts(self as *const Self as *const T, 3) }
            }

            #[inline(always)]
            fn as_mut_slice(&mut self) -> &mut [T] {
                unsafe { core::slice::from_raw_parts_mut(self as *mut Self as *mut T, 3) }
            }
        }

        impl<T> ComponentSlice<T> for [$RGB<T>] {
            #[inline]
            fn as_slice(&self) -> &[T] {
                unsafe { core::slice::from_raw_parts(self.as_ptr() as *const _, self.len() * 3) }
            }

            #[inline]
            fn as_mut_slice(&mut self) -> &mut [T] {
                unsafe {
                    core::slice::from_raw_parts_mut(self.as_mut_ptr() as *mut _, self.len() * 3)
                }
            }
        }

        #[cfg(feature = "as-bytes")]
        impl<T: crate::Pod> ComponentBytes<T> for [$RGB<T>] {}
    };
}

macro_rules! impl_rgb_to_alpha {
    ($RGB:ident, $RGBA:ident) => {
        impl<T: Clone> $RGB<T> {
            /// Convenience function for converting to RGBA
            /// 由于 $RGBA<T> 是#[derive(Clone, Copy)]的, 因此 T 类型的r, g, b也必须是#[derive(Clone, Copy)]的，
            /// 所以 `self.r.clone()` 没问题
            #[inline(always)]
            pub fn alpha(&self, a: T) -> $RGBA<T> {
                $RGBA {
                    r: self.r.clone(),
                    g: self.g.clone(),
                    b: self.b.clone(),
                    a,
                }
            }

            /// Convenience function for converting to RGBA with alpha channel of a different type than type of the pixels
            #[inline(always)]
            pub fn new_alpha<A>(&self, a: A) -> $RGBA<T, A> {
                $RGBA {
                    r: self.r.clone(),
                    g: self.g.clone(),
                    b: self.b.clone(),
                    a,
                }
            }
        }
    };
}

impl_rgb! {RGB}
impl_rgb_to_alpha! {RGB, RGBA}

impl_rgb! {BGR}
impl_rgb_to_alpha! {BGR, BGRA}

#[cfg(feature = "grb")]
impl_rgb! {GRB}

impl<T> core::iter::FromIterator<T> for RGB<T> {
    /// Takes exactly 3 elements from the iterator and creates a new instance.
    /// Panics if there are fewer elements in the iterator.
    #[inline(always)]
    fn from_iter<I: IntoIterator<Item = T>>(into_iter: I) -> Self {
        let mut iter = into_iter.into_iter();

        Self {
            r: iter.next().unwrap(),
            g: iter.next().unwrap(),
            b: iter.next().unwrap(),
        }
    }
}

impl<T: fmt::Display> fmt::Display for RGB<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({},{},{})", self.r, self.g, self.b)
    }
}

impl<T: fmt::UpperHex> fmt::UpperHex for RGB<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RGB {{ #{:02X}{:02X}{:02X} }}", self.r, self.g, self.b)
    }
}

impl<T: fmt::LowerHex> fmt::LowerHex for RGB<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RGB {{ #{:02x}{:02x}{:02x} }}", self.r, self.g, self.b)
    }
}

impl<T: fmt::Display> fmt::Display for BGR<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bgr({},{},{})", self.b, self.g, self.r)
    }
}

impl<T: fmt::UpperHex> fmt::UpperHex for BGR<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BGR {{ #{:02X}{:02X}{:02X} }}", self.b, self.g, self.r)
    }
}

impl<T: fmt::LowerHex> fmt::LowerHex for BGR<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BGR {{ #{:02x}{:02x}{:02x} }}", self.b, self.g, self.r)
    }
}

#[cfg(test)]
mod rgb_tests {
    use super::*;
    use std;

    #[test]
    #[cfg(feature = "grb")]
    fn grb_test() {
        let grb = GRB { g: 1, r: 2, b: 3 }.map(|c| c * 2) + 1;

        let rgb: crate::RGB8 = grb.into();

        assert_eq!(rgb, RGB::new(5, 3, 7));
    }
}
