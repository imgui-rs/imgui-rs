use imgui_sys;
use std::marker::PhantomData;

use super::{Ui, ImStr};

// TODO: Consider using Range, even though it is half-open

#[must_use]
pub struct SliderInt<'ui, 'p> {
    label: ImStr<'p>,
    value: &'p mut i32,
    min: i32,
    max: i32,
    display_format: ImStr<'p>,
    _phantom: PhantomData<&'ui Ui<'ui>>
}

impl<'ui, 'p> SliderInt<'ui, 'p> {
    pub fn new<S>(label: S, value: &'p mut i32, min: i32, max: i32) -> Self where S: Into<ImStr<'p>> {
        SliderInt {
            label: label.into(),
            value: value,
            min: min,
            max: max,
            display_format: unsafe { ImStr::from_bytes_unchecked(b"%.0f\0") },
            _phantom: PhantomData
        }
    }
    #[inline]
    pub fn display_format<S>(self, display_format: S) -> Self where S: Into<ImStr<'p>> {
        SliderInt {
            display_format: display_format.into(),
            .. self
        }
    }
    pub fn build(self) -> bool {
        unsafe {
            imgui_sys::igSliderInt(self.label.as_ptr(), self.value, self.min, self.max,
            self.display_format.as_ptr()
            )
        }
    }
}

#[must_use]
pub struct SliderFloat<'ui, 'p> {
    label: ImStr<'p>,
    value: &'p mut f32,
    min: f32,
    max: f32,
    display_format: ImStr<'p>,
    power: f32,
    _phantom: PhantomData<&'ui Ui<'ui>>
}

impl<'ui, 'p> SliderFloat<'ui, 'p> {
    pub fn new<S>(label: S, value: &'p mut f32, min: f32, max: f32) -> Self where S: Into<ImStr<'p>> {
        SliderFloat {
            label: label.into(),
            value: value,
            min: min,
            max: max,
            display_format: unsafe { ImStr::from_bytes_unchecked(b"%.3f\0") },
            power: 1.0,
            _phantom: PhantomData
        }
    }
    #[inline]
    pub fn display_format<S>(self, display_format: S    ) -> Self where S: Into<ImStr<'p>> {
        SliderFloat {
            display_format: display_format.into(),
            .. self
        }
    }
    #[inline]
    pub fn power(self, power: f32) -> Self {
        SliderFloat {
            power: power,
            .. self
        }
    }
    pub fn build(self) -> bool {
        unsafe {
            imgui_sys::igSliderFloat(self.label.as_ptr(), self.value, self.min, self.max,
            self.display_format.as_ptr(),
            self.power
            )
        }
    }
}

