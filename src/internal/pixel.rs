/// 将 struct 转换为其 `components` 的 `slices`
/// ```rust,ignore
/// struct A {
///     a: i32 // component of struct A
/// }
/// ```
pub trait ComponentSlice<T> {
    /// components将被转化为array, 比如: `RGB` 表示为有三个元素的array
    fn as_slice(&self) -> &[T];

    /// components将被转化为mut array, 比如: `RGB` 表示为有三个元素的array
    /// 当calling array 发生error, 添加`[..]`解决
    /// ``` rust,ignore
    /// arr[..].as_mut_slice()
    /// ```
    fn as_mut_slice(&mut self) -> &mut [T];
}

/// `RGB/A` values 转换为 `u8` slice
///
/// 如果 你使用的不是 `RGB8` 而是 `RGB<YourCustomType>`, 然后你同时想转换为 bytes, 你需要实现
/// `Plain` trait:
///
/// ```rust,ignore
/// #[derive(Copy, Clone)]
/// struct YourCustomType;
///
/// unsafe impl rgb::Pod for YourCustomType {}
/// unsafe impl rgb::Zeroable for YourCustomType {}
/// ```
///
/// `Plain` 类型不允许有: struct padding, booleans, chars, enums, references or pointers.
#[cfg(feature = "as-bytes")]
pub trait ComponentBytes<T: crate::Pod>
where
    Self: ComponentSlice<T>,
{
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        assert_ne!(0, core::mem::size_of::<T>());
        let slice = self.as_slice();

        unsafe {
            // `slice.as_ptr() as *const _` 获取切片的底层指针(即: 指向切片内存起始位置的原始指针, 而不是Rust引用;
            // 原始指针比Rust引用更低级，直接操作内存, 不受借用和所有权规则限制. 可能导致内存不安全情况, 需格外小心.)
            // 底层指针类型: `*const T` 不可变常量指针, `*mut T` 可变指针, 可用于数据的读取和修改.
            //
            // 需注意生命周期管理
            core::slice::from_raw_parts(slice.as_ptr() as *const _, std::mem::size_of_val(slice))
        }
    }

    fn as_mut_bytes(&mut self) -> &mut [u8] {
        assert_ne!(0, core::mem::size_of::<T>());
        let slice = self.as_mut_slice();

        unsafe {
            core::slice::from_raw_parts_mut(slice.as_ptr() as *mut _, std::mem::size_of_val(slice))
        }
    }
}

/// 为每个component执行此操作
/// ```rust,ignore
/// use rgb::ComponentMap;
///
/// let pixel = rgb::RGB::new(0u8, 0, 0);
/// let inverted = pixel.map(|c| 255 - c);
///
/// let halved = pixel.map(|c| c / 2);
/// let doubled = pixel * 2;
/// ```
pub trait ComponentMap<DestPixel, SrcComponent, DestComponent> {
    /// Convenience function (equivalent of `self.iter().map().collect()`) for applying the same formula to every component.
    ///
    /// Note that it returns the pixel directly, not an Interator.
    fn map<Callback>(&self, f: Callback) -> DestPixel
    where
        Callback: FnMut(SrcComponent) -> DestComponent;
}

/// 与 `ComponentMap`一致, 但不改变alpha channel(如果存在alpha)
pub trait ColorComponentMap<DestPixel, SrcComponent, DestComponent> {
    fn map_c<Callback>(&self, f: Callback) -> DestPixel
    where
        Callback: FnMut(SrcComponent) -> DestComponent;
}
