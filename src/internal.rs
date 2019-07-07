use std::slice;

#[repr(C)]
pub struct ImVector<T> {
    size: i32,
    capacity: i32,
    pub(crate) data: *mut T,
}

impl<T> ImVector<T> {
    pub unsafe fn as_slice(&self) -> &[T] {
        slice::from_raw_parts(self.data, self.size as usize)
    }
}

#[test]
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
    use memoffset::offset_of;
    use sys::ImVector_char;
    type VectorChar = ImVector<u8>;
    macro_rules! assert_field_offset {
        ($l:ident, $r:ident) => {
            assert_eq!(offset_of!(VectorChar, $l), offset_of!(ImVector_char, $r));
        };
    };
    assert_field_offset!(size, Size);
    assert_field_offset!(capacity, Capacity);
    assert_field_offset!(data, Data);
}

/// Marks a type as a transparent wrapper over a raw type
pub trait RawWrapper {
    /// Wrapped raw type
    type Raw;
    /// Returns an immutable reference to the wrapped raw value
    unsafe fn raw(&self) -> &Self::Raw;
    /// Returns a mutable reference to the wrapped raw value
    unsafe fn raw_mut(&mut self) -> &mut Self::Raw;
}

/// Casting from/to a raw type that has the same layout and alignment as the target type
pub unsafe trait RawCast<T>: Sized {
    unsafe fn from_raw(raw: &T) -> &Self {
        &*(raw as *const _ as *const Self)
    }
    unsafe fn from_raw_mut(raw: &mut T) -> &mut Self {
        &mut *(raw as *mut _ as *mut Self)
    }
    unsafe fn raw(&self) -> &T {
        &*(self as *const _ as *const T)
    }
    unsafe fn raw_mut(&mut self) -> &mut T {
        &mut *(self as *mut _ as *mut T)
    }
}
