use imgui_sys::{igButton, ImVec2};
use std::ffi::CString;

/// Builder for a button.
///
/// # Examples
///
/// ```no_run
/// # use imgui::*;
/// # let mut imgui = Context::create();
/// # let ui = imgui.frame();
/// Button::new("Button Label", &50.0f32, &32.0f32);
/// ```
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct Button
{
    
}

impl Button
{
    pub fn new(label: &str, size_x: &f32, size_y: &f32) -> bool
    {
        let mut result: bool = false;
        let button_size: ImVec2 = ImVec2::new(*size_x,*size_y);
        let button_label: CString = CString::new(label).expect("CString::new failed");
        
        unsafe
        {
            let label_pointer: *const i8 = button_label.as_ptr();
            result = igButton(label_pointer, button_size);
        }

        result
    }
}