#[cfg(feature = "argb")]
use crate::alt::ARGB;

use crate::alt::{BGR, BGRA};
use crate::{RGB, RGBA};

impl<T: Copy> From<[T; 3]> for RGB<T> {
    #[inline(always)]
    fn from(other: [T; 3]) -> Self {
        Self {
            r: other[0],
            g: other[1],
            b: other[2],
        }
    }
}

impl<T> From<RGB<T>> for [T; 3] {
    #[inline(always)]
    fn from(val: RGB<T>) -> Self {
        [val.r, val.g, val.b]
    }
}

impl<T: Copy> From<[T; 4]> for RGBA<T> {
    #[inline(always)]
    fn from(other: [T; 4]) -> Self {
        Self {
            r: other[0],
            g: other[1],
            b: other[2],
            a: other[3],
        }
    }
}

impl<T> From<RGBA<T>> for [T; 4] {
    #[inline(always)]
    fn from(val: RGBA<T>) -> Self {
        [val.r, val.g, val.b, val.a]
    }
}

#[cfg(feature = "argb")]
impl<T: Copy> From<[T; 4]> for ARGB<T> {
    #[inline(always)]
    fn from(other: [T; 4]) -> Self {
        Self {
            a: other[0],
            r: other[1],
            g: other[2],
            b: other[3],
        }
    }
}

#[cfg(feature = "argb")]
impl<T> Into<[T; 4]> for ARGB<T> {
    #[inline(always)]
    fn into(self) -> [T; 4] {
        [self.a, self.r, self.g, self.b]
    }
}

impl<T: Copy> From<[T; 3]> for BGR<T> {
    #[inline(always)]
    fn from(other: [T; 3]) -> Self {
        Self {
            b: other[0],
            g: other[1],
            r: other[2],
        }
    }
}

impl<T> From<BGR<T>> for [T; 3] {
    #[inline(always)]
    fn from(val: BGR<T>) -> Self {
        [val.b, val.g, val.r]
    }
}

impl<T: Copy> From<[T; 4]> for BGRA<T> {
    #[inline(always)]
    fn from(other: [T; 4]) -> Self {
        Self {
            b: other[0],
            g: other[1],
            r: other[2],
            a: other[3],
        }
    }
}

impl<T> From<BGRA<T>> for [T; 4] {
    #[inline(always)]
    fn from(val: BGRA<T>) -> Self {
        [val.b, val.g, val.r, val.a]
    }
}

#[test]
#[allow(deprecated)]
fn convert_array() {
    use crate::alt::{BGR8, BGRA8};
    use crate::{RGB8, RGBA8};

    assert_eq!(RGB8::from([1, 2, 3]), RGB8::new(1, 2, 3));
    assert_eq!(Into::<[u8; 3]>::into(RGB8::new(1, 2, 3)), [1, 2, 3]);
    assert_eq!(RGBA8::from([1, 2, 3, 4]), RGBA8::new(1, 2, 3, 4));
    assert_eq!(Into::<[u8; 4]>::into(RGBA8::new(1, 2, 3, 4)), [1, 2, 3, 4]);
    assert_eq!(BGR8::from([3, 2, 1]), BGR8::new(1, 2, 3));
    assert_eq!(Into::<[u8; 3]>::into(BGR8::new(1, 2, 3)), [3, 2, 1]);
    assert_eq!(BGRA8::from([3, 2, 1, 4]), BGRA8::new(1, 2, 3, 4));
    assert_eq!(Into::<[u8; 4]>::into(BGRA8::new(1, 2, 3, 4)), [3, 2, 1, 4]);
}
