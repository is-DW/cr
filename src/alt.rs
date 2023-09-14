use crate::internal::pixel::*;
use core::ops;
use core::slice;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BGR<T> {
    /// Blue
    pub b: T,
    /// Green
    pub g: T,
    /// Red
    pub r: T,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BGRA<T, TA = T> {
    /// Blue
    pub b: T,
    /// Green
    pub g: T,
    /// Red
    pub r: T,
    /// Alpha
    pub a: TA,
}

#[cfg(feature = "argb")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ABGR<T, TA = T> {
    /// Alpha
    pub a: TA,
    /// Blue
    pub b: T,
    /// Green
    pub g: T,
    /// Red
    pub r: T,
}

#[cfg(feature = "argb")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ARGB<T, TA = T> {
    /// Alpha
    pub a: TA,
    /// Red
    pub r: T,
    /// Green
    pub g: T,
    /// Blue
    pub b: T,
}

/// 8-bit BGR
pub type BGR8 = BGR<u8>;

/// 16-bit BGR in machine's native endian
pub type BGR16 = BGR<u16>;

/// 8-bit BGRA
pub type BGRA8 = BGRA<u8>;

/// 8-bit ABGR, alpha is first. 0 = transparent, 255 = opaque.
#[cfg(feature = "argb")]
pub type ABGR8 = ABGR<u8>;

/// 8-bit ARGB, alpha is first. 0 = transparent, 255 = opaque.
#[cfg(feature = "argb")]
pub type ARGB8 = ARGB<u8>;

/// 16-bit BGR in machine's native endian
pub type BGRA16 = BGRA<u16>;

/// 16-bit ABGR in machine's native endian. 0 = transparent, 65535 = opaque.
#[cfg(feature = "argb")]
pub type ABGR16 = ABGR<u16>;

/// 16-bit ARGB in machine's native endian. 0 = transparent, 65535 = opaque.
#[cfg(feature = "argb")]
pub type ARGB16 = ARGB<u16>;

#[cfg(feature = "grb")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GRB<T> {
    /// Green
    pub g: T,
    /// Red
    pub r: T,
    /// Blue
    pub b: T,
}

/// 8-bit GRB
#[cfg(feature = "grb")]
pub type GRB8 = GRB<u8>;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Grayscale. Use `.0` or `*` (deref) to access the value.
/// brightness level
pub struct Gray<T>(pub T);

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Grayscale with alpha. Use `.0`/`.1` to access.
pub struct GrayAlpha<T, TA = T>(pub T, pub TA);

#[cfg(feature = "as-bytes")]
unsafe impl<T> crate::Pod for Gray<T> where T: crate::Pod {}

#[cfg(feature = "as-bytes")]
unsafe impl<T, A> crate::Pod for GrayAlpha<T, A>
where
    T: crate::Pod,
    A: crate::Pod,
{
}

#[cfg(feature = "as-bytes")]
unsafe impl<T> crate::Zeroable for Gray<T> where T: crate::Zeroable {}

#[cfg(feature = "as-bytes")]
unsafe impl<T, A> crate::Zeroable for GrayAlpha<T, A>
where
    T: crate::Zeroable,
    A: crate::Zeroable,
{
}

/// 8-bit gray
pub type GRAY8 = Gray<u8>;

/// 16-bit gray in machine's native endian
pub type GRAY16 = Gray<u16>;

/// 8-bit gray with alpha in machine's native endian
pub type GRAYA8 = GrayAlpha<u8>;

/// 16-bit gray with alpha in machine's native endian
pub type GRAYA16 = GrayAlpha<u16>;

impl<T> Gray<T> {
    /// New grayscale pixel
    #[inline(always)]
    pub const fn new(brightness: T) -> Self {
        Self(brightness)
    }
}

/// 实现智能指针与类型的转换, 提高灵活性;
/// 允许你通过 * 运算符对自定义类型进行解引用
/// ```
/// use cr::alt::Gray;
///
/// let gray = Gray(1);
/// let v = *gray;
///
/// assert_eq!(gray.0, v);
/// ```
impl<T> ops::Deref for Gray<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// T 需要实现Copy trait, 通过Copy进行值在传递;
/// 实现Copy意味着"="即可进行值的复制, 无需使用clone进行显示复制；
/// 一般用于复制操作廉价高效的场景
impl<T: Copy> From<T> for Gray<T> {
    fn from(value: T) -> Self {
        Gray(value)
    }
}

impl<T: Clone, TA> GrayAlpha<T, TA> {
    /// Copy `GrayAlpha` as `Gray` component
    #[inline(always)]
    pub fn gray(&self) -> Gray<T> {
        Gray(self.0.clone())
    }
}

impl<T, TA> GrayAlpha<T, TA> {
    #[inline(always)]
    pub const fn new(brightness: T, alpha: TA) -> Self {
        Self(brightness, alpha)
    }

    /// Provide a mutable view of only `Gray` component (leaving out alpha).
    #[inline(always)]
    pub fn gray_mut(&mut self) -> &mut Gray<T> {
        unsafe { &mut *(self as *mut _ as *mut _) }
    }
}

impl<T: Copy, TA: Clone> GrayAlpha<T, TA> {
    /// 只改变alpha值返回新的`GrayAlpha`
    #[inline(always)]
    pub fn alpha(&self, alpha: TA) -> Self {
        Self(self.0, alpha)
    }

    /// 通过FnOnce闭包产生alpha值, 创建新的`GrayAlpha`
    #[inline(always)]
    pub fn map_alpha<F, B>(&self, f: F) -> GrayAlpha<T, B>
    where
        F: FnOnce(TA) -> B,
    {
        GrayAlpha(self.0, f(self.1.clone()))
    }

    /// 通过FnOnce闭包产生Gray值, 创建新的`GrayAlpha`
    #[inline(always)]
    pub fn map_gray<F, U, B>(&self, f: F) -> GrayAlpha<U, B>
    where
        F: FnOnce(T) -> U,
        U: Clone,
        B: From<TA> + Clone,
    {
        /// 原来self.1 是TA的, 返回是B, 因此需要.into()进行转换, 转换的前提是实现了 `From` trait 和 `Into` trait
        GrayAlpha(f(self.0), self.1.clone().into())
    }
}

impl<T: Copy, B> ComponentMap<Gray<B>, T, B> for Gray<T> {
    #[inline(always)]
    fn map<F>(&self, mut f: F) -> Gray<B>
    where
        F: FnMut(T) -> B,
    {
        Gray(f(self.0))
    }
}

impl<T: Copy, B> ColorComponentMap<Gray<B>, T, B> for Gray<T> {
    #[inline(always)]
    fn map_c<F>(&self, mut f: F) -> Gray<B>
    where
        F: FnMut(T) -> B,
    {
        Gray(f(self.0))
    }
}

impl<T: Copy, B> ComponentMap<GrayAlpha<B>, T, B> for GrayAlpha<T> {
    #[inline(always)]
    fn map<F>(&self, mut f: F) -> GrayAlpha<B>
    where
        F: FnMut(T) -> B,
    {
        GrayAlpha(f(self.0), f(self.1))
    }
}

impl<T: Copy, A: Copy, B> ColorComponentMap<GrayAlpha<B, A>, T, B> for GrayAlpha<T, A> {
    #[inline(always)]
    fn map_c<F>(&self, mut f: F) -> GrayAlpha<B, A>
    where
        F: FnMut(T) -> B,
    {
        GrayAlpha(f(self.0), self.1)
    }
}

impl<T> ComponentSlice<T> for GrayAlpha<T> {
    #[inline(always)]
    fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self as *const Self as *const T, 2) }
    }

    #[inline(always)]
    fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self as *mut Self as *mut T, 2) }
    }
}

impl<T> ComponentSlice<T> for [GrayAlpha<T>] {
    #[inline]
    fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len() * 2) }
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_ptr() as *mut _, self.len() * 2) }
    }
}

#[cfg(feature = "as-bytes")]
impl<T: crate::Pod> ComponentBytes<T> for [GrayAlpha<T>] {}

impl<T> ComponentSlice<T> for Gray<T> {
    #[inline(always)]
    fn as_slice(&self) -> &[T] {
        slice::from_ref(&self.0)
    }

    #[inline(always)]
    fn as_mut_slice(&mut self) -> &mut [T] {
        slice::from_mut(&mut self.0)
    }
}

impl<T> ComponentSlice<T> for [Gray<T>] {
    #[inline]
    fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len()) }
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_ptr() as *mut _, self.len()) }
    }
}

#[cfg(feature = "as-bytes")]
impl<T: crate::Pod> ComponentBytes<T> for [Gray<T>] {}

/// Assumes 255 is opaque
impl<T: Copy> From<Gray<T>> for GrayAlpha<T, u8> {
    #[inline(always)]
    fn from(other: Gray<T>) -> Self {
        GrayAlpha(other.0, 0xFF)
    }
}

/// Assumes 65535 is opaque
impl<T: Copy> From<Gray<T>> for GrayAlpha<T, u16> {
    #[inline(always)]
    fn from(other: Gray<T>) -> Self {
        GrayAlpha(other.0, 0xFFFF)
    }
}

#[cfg(test)]
mod tests {}
