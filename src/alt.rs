use core::ops;

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

/// 实现智能指针与类型的转换，提高灵活性
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
/// 实现Copy意味着"="即可进行值的复制，无需使用clone进行显示复制；
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

#[cfg(test)]
mod tests {}
