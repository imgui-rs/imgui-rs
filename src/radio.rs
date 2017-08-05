use imgui_sys;
use std::marker::PhantomData;

use super::{ImStr, Ui};

#[must_use]
pub struct RadioButton<'ui, 'p> {
    label: &'p ImStr,
    value: &'p mut i32,
    wanted: i32,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> RadioButton<'ui, 'p> {
    pub fn new(label: &'p ImStr, value: &'p mut i32, wanted: i32) -> Self {
        RadioButton {
            label: label,
            value: value,
            wanted: wanted,
            _phantom: PhantomData,
        }
    }
    pub fn build(self) -> bool {
        unsafe { imgui_sys::igRadioButton(self.label.as_ptr(), self.value, self.wanted) }
    }
}

#[must_use]
pub struct RadioButtonBool<'ui, 'p> {
    label: &'p ImStr,
    value: bool,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> RadioButtonBool<'ui, 'p> {
    pub fn new(label: &'p ImStr, value: bool) -> Self {
        RadioButtonBool {
            label: label,
            value: value,
            _phantom: PhantomData,
        }
    }
    pub fn build(self) -> bool {
        unsafe { imgui_sys::igRadioButtonBool(self.label.as_ptr(), self.value) }
    }
}
