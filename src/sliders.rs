use imgui_sys;
use std::marker::PhantomData;

use super::{Ui};

// TODO: Consider using Range, even though it is half-open

#[must_use]
pub struct SliderInt<'ui, 'p> {
    label: &'p str,
    value: &'p mut i32,
    min: i32,
    max: i32,
    display_format: &'p str,
    _phantom: PhantomData<&'ui Ui<'ui>>
}

impl<'ui, 'p> SliderInt<'ui, 'p> {
    pub fn new(label: &'p str, value: &'p mut i32, min: i32, max: i32) -> Self {
        SliderInt {
            label: label,
            value: value,
            min: min,
            max: max,
            display_format: "%.0f",
            _phantom: PhantomData
        }
    }
    #[inline]
    pub fn display_format(self, display_format: &'p str) -> Self {
        SliderInt {
            display_format: display_format,
            .. self
        }
    }
    pub fn build(self) -> bool {
        let label = imgui_sys::ImStr::from(self.label);
        let display_format = imgui_sys::ImStr::from(self.display_format);
        unsafe {
            imgui_sys::igSliderInt(label, self.value, self.min, self.max, display_format)
        }
    }
}

#[must_use]
pub struct SliderFloat<'ui, 'p> {
    label: &'p str,
    value: &'p mut f32,
    min: f32,
    max: f32,
    display_format: &'p str,
    power: f32,
    _phantom: PhantomData<&'ui Ui<'ui>>
}

impl<'ui, 'p> SliderFloat<'ui, 'p> {
    pub fn new(label: &'p str, value: &'p mut f32, min: f32, max: f32) -> Self {
        SliderFloat {
            label: label,
            value: value,
            min: min,
            max: max,
            display_format: "%.3f",
            power: 1.0,
            _phantom: PhantomData
        }
    }
    #[inline]
    pub fn display_format(self, display_format: &'p str) -> Self {
        SliderFloat {
            display_format: display_format,
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
        let label = imgui_sys::ImStr::from(self.label);
        let display_format = imgui_sys::ImStr::from(self.display_format);
        unsafe {
            imgui_sys::igSliderFloat(label, self.value, self.min, self.max, display_format,
                                     self.power)
        }
    }
}

