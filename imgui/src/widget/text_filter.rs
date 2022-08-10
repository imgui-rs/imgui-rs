use crate::sys;
use crate::Ui;
use std::ptr;

pub struct TextFilter {
    id: String,
    size: f32,
    raw: *mut sys::ImGuiTextFilter,
}

impl TextFilter {
    pub fn new(id: String) -> Self {
        Self::new_with_filter(id, String::new())
    }

    pub fn new_with_filter(id: String, mut filter: String) -> Self {
        filter.push('\0');
        let ptr = filter.as_mut_ptr();
        Self {
            id,
            size: 0.0,
            raw: unsafe { sys::ImGuiTextFilter_ImGuiTextFilter(ptr as *mut sys::cty::c_char) },
        }
    }

    pub fn set_size(&mut self, size: f32) {
        self.size = size;
    }

    pub fn build(&mut self) {
        unsafe {
            sys::ImGuiTextFilter_Build(self.raw);
        }
    }

    pub fn draw(&self) {
        self.draw_size(0.0);
    }

    pub fn draw_size(&self, size: f32) {
        unsafe {
            let mut id = self.id.clone();
            id.push('\0');
            let ptr = id.as_mut_ptr();
            sys::ImGuiTextFilter_Draw(self.raw, ptr as *mut sys::cty::c_char, size);
        }
    }

    pub fn is_active(&self) -> bool {
        unsafe { sys::ImGuiTextFilter_IsActive(self.raw) }
    }

    pub fn pass_filter(&self, mut buf: String) -> bool {
        buf.push('\0');
        let ptr = buf.as_mut_ptr();
        unsafe {
            sys::ImGuiTextFilter_PassFilter(self.raw, ptr as *mut sys::cty::c_char, ptr::null())
        }
    }

    pub fn pass_filter_end(&self, mut start: String, mut end: String) -> bool {
        start.push('\0');
        end.push('\0');
        let b_ptr = start.as_mut_ptr();
        let e_ptr = end.as_mut_ptr();
        unsafe {
            sys::ImGuiTextFilter_PassFilter(
                self.raw,
                b_ptr as *mut sys::cty::c_char,
                e_ptr as *mut sys::cty::c_char,
            )
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            sys::ImGuiTextFilter_Clear(self.raw);
        }
    }
}

impl Ui {
    pub fn text_filter(label: String) -> TextFilter {
        TextFilter::new(label)
    }
}
