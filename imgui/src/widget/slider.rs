use bitflags::bitflags;
use std::os::raw::c_void;
use std::ptr;

use crate::internal::{DataTypeKind, InclusiveRangeBounds};
use crate::string::ImStr;
use crate::sys;
use crate::Ui;

bitflags!(
    /// Flags for sliders
    #[repr(transparent)]
    pub struct SliderFlags: u32 {
        /// Clamp value to min/max bounds when input manually with CTRL+Click.
        ///
        /// By default CTRL+click allows going out of bounds.
        const ALWAYS_CLAMP = sys::ImGuiSliderFlags_AlwaysClamp;
        /// Make the widget logarithmic instead of linear
        const LOGARITHMIC = sys::ImGuiSliderFlags_Logarithmic;
        /// Disable rounding underlying value to match precision of the display format string
        const NO_ROUND_TO_FORMAT = sys::ImGuiSliderFlags_NoRoundToFormat;
        /// Disable CTRL+Click or Enter key allowing to input text directly into the widget
        const NO_INPUT = sys::ImGuiSliderFlags_NoInput;
    }
);

/// Builder for a slider widget.
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct Slider<'a, T: DataTypeKind> {
    label: &'a ImStr,
    min: T,
    max: T,
    display_format: Option<&'a ImStr>,
    flags: SliderFlags,
}

impl<'a, T: DataTypeKind> Slider<'a, T> {
    /// Constructs a new slider builder with the given range.
    #[doc(alias = "SliderScalar", alias = "SliderScalarN")]
    pub fn new(label: &ImStr) -> Slider<T> {
        Slider {
            label,
            min: T::SLIDER_MIN,
            max: T::SLIDER_MAX,
            display_format: None,
            flags: SliderFlags::empty(),
        }
    }
    /// Sets the range (inclusive)
    #[inline]
    pub fn range<R: InclusiveRangeBounds<T>>(mut self, range: R) -> Self {
        self.min = range.start_bound().copied().unwrap_or(T::SLIDER_MIN);
        self.max = range.end_bound().copied().unwrap_or(T::SLIDER_MAX);
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
    /// Builds a slider that is bound to the given value.
    ///
    /// Returns true if the slider value was changed.
    pub fn build(self, _: &Ui, value: &mut T) -> bool {
        unsafe {
            sys::igSliderScalar(
                self.label.as_ptr(),
                T::KIND as i32,
                value as *mut T as *mut c_void,
                &self.min as *const T as *const c_void,
                &self.max as *const T as *const c_void,
                self.display_format
                    .map(ImStr::as_ptr)
                    .unwrap_or(ptr::null()),
                self.flags.bits() as i32,
            )
        }
    }
    /// Builds a horizontal array of multiple sliders attached to the given slice.
    ///
    /// Returns true if any slider value was changed.
    pub fn build_array(self, _: &Ui, values: &mut [T]) -> bool {
        unsafe {
            sys::igSliderScalarN(
                self.label.as_ptr(),
                T::KIND as i32,
                values.as_mut_ptr() as *mut c_void,
                values.len() as i32,
                &self.min as *const T as *const c_void,
                &self.max as *const T as *const c_void,
                self.display_format
                    .map(ImStr::as_ptr)
                    .unwrap_or(ptr::null()),
                self.flags.bits() as i32,
            )
        }
    }
}

/// Builder for a vertical slider widget.
#[derive(Clone, Debug)]
#[must_use]
pub struct VerticalSlider<'a, T: DataTypeKind + Copy> {
    label: &'a ImStr,
    size: [f32; 2],
    min: T,
    max: T,
    display_format: Option<&'a ImStr>,
    flags: SliderFlags,
}

impl<'a, T: DataTypeKind> VerticalSlider<'a, T> {
    /// Constructs a new vertical slider builder with the given size and range.
    #[doc(alias = "VSliderScalar")]
    pub fn new(label: &ImStr, size: [f32; 2]) -> VerticalSlider<T> {
        VerticalSlider {
            label,
            size,
            min: T::SLIDER_MIN,
            max: T::SLIDER_MAX,
            display_format: None,
            flags: SliderFlags::empty(),
        }
    }
    /// Sets the range (inclusive)
    #[inline]
    pub fn range<R: InclusiveRangeBounds<T>>(mut self, range: R) -> Self {
        self.min = range.start_bound().copied().unwrap_or(T::SLIDER_MIN);
        self.max = range.end_bound().copied().unwrap_or(T::SLIDER_MAX);
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
    /// Builds a vertical slider that is bound to the given value.
    ///
    /// Returns true if the slider value was changed.
    pub fn build(self, _: &Ui, value: &mut T) -> bool {
        unsafe {
            sys::igVSliderScalar(
                self.label.as_ptr(),
                self.size.into(),
                T::KIND as i32,
                value as *mut T as *mut c_void,
                &self.min as *const T as *const c_void,
                &self.max as *const T as *const c_void,
                self.display_format
                    .map(ImStr::as_ptr)
                    .unwrap_or(ptr::null()),
                self.flags.bits() as i32,
            )
        }
    }
}

/// Builder for an angle slider widget.
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct AngleSlider<'a> {
    label: &'a ImStr,
    min_degrees: f32,
    max_degrees: f32,
    display_format: &'a ImStr,
    flags: SliderFlags,
}

impl<'a> AngleSlider<'a> {
    /// Constructs a new angle slider builder.
    #[doc(alias = "SliderAngle")]
    pub fn new(label: &ImStr) -> AngleSlider {
        AngleSlider {
            label,
            min_degrees: -360.0,
            max_degrees: 360.0,
            display_format: im_str!("%.0f deg"),
            flags: SliderFlags::empty(),
        }
    }
    /// Sets the range (in degrees, inclusive)
    #[inline]
    pub fn range_degrees<R: InclusiveRangeBounds<f32>>(mut self, range: R) -> Self {
        self.min_degrees = range.start_bound().copied().unwrap_or(-360.0);
        self.max_degrees = range.end_bound().copied().unwrap_or(360.0);
        self
    }
    /// Sets the minimum value (in degrees)
    #[inline]
    pub fn min_degrees(mut self, min_degrees: f32) -> Self {
        self.min_degrees = min_degrees;
        self
    }
    /// Sets the maximum value (in degrees)
    #[inline]
    pub fn max_degrees(mut self, max_degrees: f32) -> Self {
        self.max_degrees = max_degrees;
        self
    }
    /// Sets the display format using *a C-style printf string*
    #[inline]
    pub fn display_format(mut self, display_format: &'a ImStr) -> Self {
        self.display_format = display_format;
        self
    }
    /// Replaces all current settings with the given flags
    #[inline]
    pub fn flags(mut self, flags: SliderFlags) -> Self {
        self.flags = flags;
        self
    }
    /// Builds an angle slider that is bound to the given value (in radians).
    ///
    /// Returns true if the slider value was changed.
    pub fn build(self, _: &Ui, value_rad: &mut f32) -> bool {
        unsafe {
            sys::igSliderAngle(
                self.label.as_ptr(),
                value_rad as *mut _,
                self.min_degrees,
                self.max_degrees,
                self.display_format.as_ptr(),
                self.flags.bits() as i32,
            )
        }
    }
}
