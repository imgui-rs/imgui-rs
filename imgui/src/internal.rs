//! Internal raw utilities (don't use unless you know what you're doing!)

use std::ops::{RangeFrom, RangeInclusive, RangeToInclusive};
use std::slice;

/// A generic version of the raw imgui-sys ImVector struct types
#[repr(C)]
pub struct ImVector<T> {
    size: i32,
    capacity: i32,
    pub(crate) data: *mut T,
}

impl<T> ImVector<T> {
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.data, self.size as usize) }
    }
}

#[test]
#[cfg(test)]
fn test_imvector_memory_layout() {
    use std::mem;
    assert_eq!(
        mem::size_of::<ImVector<u8>>(),
        mem::size_of::<sys::ImVector_char>()
    );
    assert_eq!(
        mem::align_of::<ImVector<u8>>(),
        mem::align_of::<sys::ImVector_char>()
    );
    use sys::ImVector_char;
    type VectorChar = ImVector<u8>;
    macro_rules! assert_field_offset {
        ($l:ident, $r:ident) => {
            assert_eq!(
                memoffset::offset_of!(VectorChar, $l),
                memoffset::offset_of!(ImVector_char, $r)
            );
        };
    }
    assert_field_offset!(size, Size);
    assert_field_offset!(capacity, Capacity);
    assert_field_offset!(data, Data);
}

/// Marks a type as a transparent wrapper over a raw type
pub trait RawWrapper {
    /// Wrapped raw type
    type Raw;
    /// Returns an immutable reference to the wrapped raw value
    ///
    /// # Safety
    ///
    /// It is up to the caller to use the returned raw reference without causing undefined
    /// behaviour or breaking safety rules.
    unsafe fn raw(&self) -> &Self::Raw;
    /// Returns a mutable reference to the wrapped raw value
    ///
    /// # Safety
    ///
    /// It is up to the caller to use the returned mutable raw reference without causing undefined
    /// behaviour or breaking safety rules.
    unsafe fn raw_mut(&mut self) -> &mut Self::Raw;
}

/// Casting from/to a raw type that has the same layout and alignment as the target type
pub unsafe trait RawCast<T>: Sized {
    /// Casts an immutable reference from the raw type
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee the cast is valid.
    #[inline]
    unsafe fn from_raw(raw: &T) -> &Self {
        &*(raw as *const _ as *const Self)
    }
    /// Casts a mutable reference from the raw type
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee the cast is valid.
    #[inline]
    unsafe fn from_raw_mut(raw: &mut T) -> &mut Self {
        &mut *(raw as *mut _ as *mut Self)
    }
    /// Casts an immutable reference to the raw type
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee the cast is valid.
    #[inline]
    unsafe fn raw(&self) -> &T {
        &*(self as *const _ as *const T)
    }
    /// Casts a mutable reference to the raw type
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee the cast is valid.
    #[inline]
    unsafe fn raw_mut(&mut self) -> &mut T {
        &mut *(self as *mut _ as *mut T)
    }
}

/// A primary data type
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DataType {
    I8 = sys::ImGuiDataType_S8,
    U8 = sys::ImGuiDataType_U8,
    I16 = sys::ImGuiDataType_S16,
    U16 = sys::ImGuiDataType_U16,
    I32 = sys::ImGuiDataType_S32,
    U32 = sys::ImGuiDataType_U32,
    I64 = sys::ImGuiDataType_S64,
    U64 = sys::ImGuiDataType_U64,
    F32 = sys::ImGuiDataType_Float,
    F64 = sys::ImGuiDataType_Double,
}

/// Primitive type marker.
///
/// If this trait is implemented for a type, it is assumed to have *exactly* the same
/// representation in memory as the primitive value described by the associated `KIND` constant.
pub unsafe trait DataTypeKind: Copy {
    const KIND: DataType;
    const SLIDER_MIN: Self;
    const SLIDER_MAX: Self;
}
unsafe impl DataTypeKind for i8 {
    const KIND: DataType = DataType::I8;
    const SLIDER_MIN: Self = std::i8::MIN;
    const SLIDER_MAX: Self = std::i8::MAX;
}
unsafe impl DataTypeKind for u8 {
    const KIND: DataType = DataType::U8;
    const SLIDER_MIN: Self = std::u8::MIN;
    const SLIDER_MAX: Self = std::u8::MAX;
}
unsafe impl DataTypeKind for i16 {
    const KIND: DataType = DataType::I16;
    const SLIDER_MIN: Self = std::i16::MIN;
    const SLIDER_MAX: Self = std::i16::MAX;
}
unsafe impl DataTypeKind for u16 {
    const KIND: DataType = DataType::U16;
    const SLIDER_MIN: Self = std::u16::MIN;
    const SLIDER_MAX: Self = std::u16::MAX;
}
unsafe impl DataTypeKind for i32 {
    const KIND: DataType = DataType::I32;
    const SLIDER_MIN: Self = std::i32::MIN / 2;
    const SLIDER_MAX: Self = std::i32::MAX / 2;
}
unsafe impl DataTypeKind for u32 {
    const KIND: DataType = DataType::U32;
    const SLIDER_MIN: Self = std::u32::MIN / 2;
    const SLIDER_MAX: Self = std::u32::MAX / 2;
}
unsafe impl DataTypeKind for i64 {
    const KIND: DataType = DataType::I64;
    const SLIDER_MIN: Self = std::i64::MIN / 2;
    const SLIDER_MAX: Self = std::i64::MAX / 2;
}
unsafe impl DataTypeKind for u64 {
    const KIND: DataType = DataType::U64;
    const SLIDER_MIN: Self = std::u64::MIN / 2;
    const SLIDER_MAX: Self = std::u64::MAX / 2;
}
unsafe impl DataTypeKind for f32 {
    const KIND: DataType = DataType::F32;
    const SLIDER_MIN: Self = std::f32::MIN / 2.0;
    const SLIDER_MAX: Self = std::f32::MAX / 2.0;
}
unsafe impl DataTypeKind for f64 {
    const KIND: DataType = DataType::F64;
    const SLIDER_MIN: Self = std::f64::MIN / 2.0;
    const SLIDER_MAX: Self = std::f64::MAX / 2.0;
}

pub trait InclusiveRangeBounds<T: Copy> {
    fn start_bound(&self) -> Option<&T>;
    fn end_bound(&self) -> Option<&T>;
}

impl<T: Copy> InclusiveRangeBounds<T> for RangeFrom<T> {
    #[inline]
    fn start_bound(&self) -> Option<&T> {
        Some(&self.start)
    }
    #[inline]
    fn end_bound(&self) -> Option<&T> {
        None
    }
}

impl<T: Copy> InclusiveRangeBounds<T> for RangeInclusive<T> {
    #[inline]
    fn start_bound(&self) -> Option<&T> {
        Some(self.start())
    }
    #[inline]
    fn end_bound(&self) -> Option<&T> {
        Some(self.end())
    }
}

impl<T: Copy> InclusiveRangeBounds<T> for RangeToInclusive<T> {
    #[inline]
    fn start_bound(&self) -> Option<&T> {
        None
    }
    #[inline]
    fn end_bound(&self) -> Option<&T> {
        Some(&self.end)
    }
}
