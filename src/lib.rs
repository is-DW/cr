#[allow(unused)]
#[allow(clippy::upper_case_acronyms)]
pub mod alt;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RGB<T> {
    /// Red
    pub r: T,
    /// Green
    pub g: T,
    /// Blue
    pub b: T,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RGBA<T, TA = T> {
    /// Red
    pub r: T,
    /// Green
    pub g: T,
    /// Blue
    pub b: T,
    /// Alpha
    pub a: TA,
}

pub type RGB8 = RGB<u8>;
pub type RGB16 = RGB<u16>;

pub type RGBA8 = RGBA<u8>;
pub type RGBA16 = RGBA<u16>;

#[cfg(test)]
mod tests {}
