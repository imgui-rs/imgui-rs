use imgui_sys::{igButton, ImVec2};
use std::ffi::CString;

/// Builder for a button.
///
/// # Examples
///
/// ```no_run
/// # use imgui::*;
/// 
/// Button::new("Button Label", &50.0f32, &32.0f32).build();
/// ```
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct Button<'a>
{
    label: &'a str,
    size_x: &'a f32,
    size_y: &'a f32,
}

impl<'a> Button<'a>
{
    pub fn new(label: &'a str, size_x: &'a f32, size_y: &'a f32) -> Button<'a>
    {
        Button{label: label, size_x: size_x, size_y: size_y}
    }

    pub fn build(&self) -> bool
    {
        let mut result: bool = false;
        let button_size: ImVec2 = ImVec2::new(*self.size_x, *self.size_y);
        let button_label: CString = CString::new(self.label).unwrap();
        unsafe
        {
            let label_pointer: *const i8 = button_label.as_ptr();
            result = igButton(label_pointer, button_size);
        }

        result
    }
}