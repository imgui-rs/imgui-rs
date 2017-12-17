use sys;
use std::marker::PhantomData;

use super::Ui;

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
    pub fn new(_: &Ui<'ui>, label: &'p str, value: &'p mut i32, min: i32, max: i32) -> Self {
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
            sys::igSliderInt(
                sys::ImStr::from(self.label),
                self.value,
                self.min,
                self.max,
                sys::ImStr::from(self.display_format),
            )
        }
    }
}

macro_rules! impl_slider_intn {
    ($SliderIntN:ident, $N:expr, $igSliderIntN:ident) => {
        #[must_use]
        pub struct $SliderIntN<'ui, 'p> {
            label: &'p str,
            value: &'p mut [i32; $N],
            min: i32,
            max: i32,
            display_format: &'p str,
            _phantom: PhantomData<&'ui Ui<'ui>>,
        }

        impl<'ui, 'p> $SliderIntN<'ui, 'p> {
            pub fn new(_: &Ui<'ui>, label: &'p str, value: &'p mut [i32; $N], min: i32, max: i32) -> Self {
                $SliderIntN {
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
                    sys::$igSliderIntN(
                        sys::ImStr::from(self.label),
                        self.value.as_mut_ptr(),
                        self.min,
                        self.max,
                        sys::ImStr::from(self.display_format))
                }
            }
        }
    }
}

impl_slider_intn!(SliderInt2, 2, igSliderInt2);
impl_slider_intn!(SliderInt3, 3, igSliderInt3);
impl_slider_intn!(SliderInt4, 4, igSliderInt4);

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
    pub fn new(_: &Ui<'ui>, label: &'p str, value: &'p mut f32, min: f32, max: f32) -> Self {
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
            sys::igSliderFloat(
                sys::ImStr::from(self.label),
                self.value,
                self.min,
                self.max,
                sys::ImStr::from(self.display_format),
                self.power,
            )
        }
    }
}

macro_rules! impl_slider_floatn {
    ($SliderFloatN:ident, $N:expr, $igSliderFloatN:ident) => {
        #[must_use]
        pub struct $SliderFloatN<'ui, 'p> {
            label: &'p str,
            value: &'p mut [f32; $N],
            min: f32,
            max: f32,
            display_format: &'p str,
            power: f32,
            _phantom: PhantomData<&'ui Ui<'ui>>,
        }

        impl<'ui, 'p> $SliderFloatN<'ui, 'p> {
            pub fn new(_: &Ui<'ui>, label: &'p str, value: &'p mut [f32; $N], min: f32, max: f32) -> Self {
                $SliderFloatN {
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
                    sys::$igSliderFloatN(
                        sys::ImStr::from(self.label),
                        self.value.as_mut_ptr(),
                        self.min,
                        self.max,
                        sys::ImStr::from(self.display_format),
                        self.power)
                }
            }
        }
    }
}

impl_slider_floatn!(SliderFloat2, 2, igSliderFloat2);
impl_slider_floatn!(SliderFloat3, 3, igSliderFloat3);
impl_slider_floatn!(SliderFloat4, 4, igSliderFloat4);
