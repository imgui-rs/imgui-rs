use super::Ui;
use imgui_sys;
use std::marker::PhantomData;

// TODO: Consider using Range, even though it is half-open

#[must_use]
pub struct SliderInt<'ui, 'p> {
    label: &'p str,
    value: &'p mut i32,
    min: i32,
    max: i32,
    display_format: &'p str,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> SliderInt<'ui, 'p> {
    pub fn new(label: &'p str, value: &'p mut i32, min: i32, max: i32) -> Self {
        SliderInt {
            label: label,
            value: value,
            min: min,
            max: max,
            display_format: "%.0f",
            _phantom: PhantomData,
        }
    }
    #[inline]
    pub fn display_format(mut self, display_format: &'p str) -> Self {
        self.display_format = display_format;
        self
    }
    pub fn build(self) -> bool {
        unsafe {
            imgui_sys::igSliderInt(
                imgui_sys::ImStr::from(self.label),
                self.value,
                self.min,
                self.max,
                imgui_sys::ImStr::from(self.display_format),
            )
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
    _phantom: PhantomData<&'ui Ui<'ui>>,
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
            _phantom: PhantomData,
        }
    }
    #[inline]
    pub fn display_format(mut self, display_format: &'p str) -> Self {
        self.display_format = display_format;
        self
    }
    #[inline]
    pub fn power(mut self, power: f32) -> Self {
        self.power = power;
        self
    }
    pub fn build(self) -> bool {
        unsafe {
            imgui_sys::igSliderFloat(
                imgui_sys::ImStr::from(self.label),
                self.value,
                self.min,
                self.max,
                imgui_sys::ImStr::from(self.display_format),
                self.power,
            )
        }
    }
}
