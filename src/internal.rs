use crate::sys;

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

/// Raw function used as a marker for `DrawCmd::FnCallback`
pub unsafe extern "C" fn fn_callback_marker(_: *const sys::ImDrawList, _: *const sys::ImDrawCmd) {}

/// Raw function used as a marker for `DrawCmd::ClosureCallback`
pub unsafe extern "C" fn closure_callback_marker(
    _: *const sys::ImDrawList,
    _: *const sys::ImDrawCmd,
) {
}
