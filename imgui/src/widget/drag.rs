use std::os::raw::c_void;
use std::ptr;

use crate::internal::{DataTypeKind, InclusiveRangeBounds};
use crate::string::ImStr;
use crate::sys;
use crate::widget::slider::SliderFlags;
use crate::Ui;

/// Builder for a drag slider widget.
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct Drag<'a, T: DataTypeKind> {
    label: &'a ImStr,
    speed: f32,
    min: Option<T>,
    max: Option<T>,
    display_format: Option<&'a ImStr>,
    flags: SliderFlags,
}

impl<'a, T: DataTypeKind> Drag<'a, T> {
    /// Constructs a new drag slider builder.
    #[doc(alias = "DragScalar", alias = "DragScalarN")]
    pub fn new(label: &ImStr) -> Drag<T> {
        Drag {
            label,
            speed: 1.0,
            min: None,
            max: None,
            display_format: None,
            flags: SliderFlags::empty(),
        }
    }
    /// Sets the range (inclusive)
    #[inline]
    pub fn range<R: InclusiveRangeBounds<T>>(mut self, range: R) -> Self {
        self.min = range.start_bound().copied();
        self.max = range.end_bound().copied();
        self
    }
    /// Sets the value increment for a movement of one pixel.
    ///
    /// Example: speed=0.2 means mouse needs to move 5 pixels to increase the slider value by 1
    #[inline]
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }
    /// Sets the display format using *a C-style printf string*
    #[inline]
    pub fn display_format(mut self, display_format: &'a ImStr) -> Self {
        self.display_format = Some(display_format);
        self
    }
    /// Replaces all current settings with the given flags
    #[inline]
    pub fn flags(mut self, flags: SliderFlags) -> Self {
        self.flags = flags;
        self
    }
    /// Builds a drag slider that is bound to the given value.
    ///
    /// Returns true if the slider value was changed.
    pub fn build(self, _: &Ui, value: &mut T) -> bool {
        unsafe {
            sys::igDragScalar(
                self.label.as_ptr(),
                T::KIND as i32,
                value as *mut T as *mut c_void,
                self.speed,
                self.min
                    .as_ref()
                    .map(|min| min as *const T)
                    .unwrap_or(ptr::null()) as *const c_void,
                self.max
                    .as_ref()
                    .map(|max| max as *const T)
                    .unwrap_or(ptr::null()) as *const c_void,
                self.display_format
                    .map(ImStr::as_ptr)
                    .unwrap_or(ptr::null()),
                self.flags.bits() as i32,
            )
        }
    }
    /// Builds a horizontal array of multiple drag sliders attached to the given slice.
    ///
    /// Returns true if any slider value was changed.
    pub fn build_array(self, _: &Ui, values: &mut [T]) -> bool {
        unsafe {
            sys::igDragScalarN(
                self.label.as_ptr(),
                T::KIND as i32,
                values.as_mut_ptr() as *mut c_void,
                values.len() as i32,
                self.speed,
                self.min
                    .as_ref()
                    .map(|min| min as *const T)
                    .unwrap_or(ptr::null()) as *const c_void,
                self.max
                    .as_ref()
                    .map(|max| max as *const T)
                    .unwrap_or(ptr::null()) as *const c_void,
                self.display_format
                    .map(ImStr::as_ptr)
                    .unwrap_or(ptr::null()),
                self.flags.bits() as i32,
            )
        }
    }
}

/// Builder for a drag slider widget.
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct DragRange<'a, T: DataTypeKind> {
    label: &'a ImStr,
    speed: f32,
    min: Option<T>,
    max: Option<T>,
    display_format: Option<&'a ImStr>,
    max_display_format: Option<&'a ImStr>,
    flags: SliderFlags,
}

impl<'a, T: DataTypeKind> DragRange<'a, T> {
    /// Constructs a new drag slider builder.
    #[doc(alias = "DragIntRange2", alias = "DragFloatRange2")]
    pub fn new(label: &ImStr) -> DragRange<T> {
        DragRange {
            label,
            speed: 1.0,
            min: None,
            max: None,
            display_format: None,
            max_display_format: None,
            flags: SliderFlags::empty(),
        }
    }
    #[inline]
    pub fn range<R: InclusiveRangeBounds<T>>(mut self, range: R) -> Self {
        self.min = range.start_bound().copied();
        self.max = range.end_bound().copied();
        self
    }
    /// Sets the value increment for a movement of one pixel.
    ///
    /// Example: speed=0.2 means mouse needs to move 5 pixels to increase the slider value by 1
    #[inline]
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }
    /// Sets the display format using *a C-style printf string*
    #[inline]
    pub fn display_format(mut self, display_format: &'a ImStr) -> Self {
        self.display_format = Some(display_format);
        self
    }
    /// Sets the display format for the max value using *a C-style printf string*
    #[inline]
    pub fn max_display_format(mut self, max_display_format: &'a ImStr) -> Self {
        self.max_display_format = Some(max_display_format);
        self
    }
    /// Replaces all current settings with the given flags
    #[inline]
    pub fn flags(mut self, flags: SliderFlags) -> Self {
        self.flags = flags;
        self
    }
}

impl<'a> DragRange<'a, f32> {
    /// Builds a drag range slider that is bound to the given min/max values.
    ///
    /// Returns true if the slider value was changed.
    #[doc(alias = "DragFloatRange2")]
    pub fn build(self, _: &Ui, min: &mut f32, max: &mut f32) -> bool {
        unsafe {
            sys::igDragFloatRange2(
                self.label.as_ptr(),
                min as *mut f32,
                max as *mut f32,
                self.speed,
                self.min.unwrap_or(0.0),
                self.max.unwrap_or(0.0),
                self.display_format
                    .map(ImStr::as_ptr)
                    .unwrap_or(ptr::null()),
                self.max_display_format
                    .map(ImStr::as_ptr)
                    .unwrap_or(ptr::null()),
                self.flags.bits() as i32,
            )
        }
    }
}

impl<'a> DragRange<'a, i32> {
    /// Builds a drag range slider that is bound to the given min/max values.
    ///
    /// Returns true if the slider value was changed.
    #[doc(alias = "DragIntRange2")]
    pub fn build(self, _: &Ui, min: &mut i32, max: &mut i32) -> bool {
        unsafe {
            sys::igDragIntRange2(
                self.label.as_ptr(),
                min as *mut i32,
                max as *mut i32,
                self.speed,
                self.min.unwrap_or(0),
                self.max.unwrap_or(0),
                self.display_format
                    .map(ImStr::as_ptr)
                    .unwrap_or(ptr::null()),
                self.max_display_format
                    .map(ImStr::as_ptr)
                    .unwrap_or(ptr::null()),
                self.flags.bits() as i32,
            )
        }
    }
}
