use crate::sys;
use crate::Ui;
use std::ptr;

/// Helper to parse and apply text filters
pub struct TextFilter {
    id: String,
    raw: *mut sys::ImGuiTextFilter,
}

impl TextFilter {
    /// Creates a new TextFilter with an empty filter.
    ///
    /// This is equivalent of [new_with_filter](Self::new_with_filter) with `filter` set to `""`.
    pub fn new(label: String) -> Self {
        Self::new_with_filter(label, String::new())
    }

    /// Creates a new TextFilter with a custom filter.
    pub fn new_with_filter(label: String, mut filter: String) -> Self {
        filter.push('\0');
        let ptr = filter.as_mut_ptr();
        Self {
            id: label,
            raw: unsafe { sys::ImGuiTextFilter_ImGuiTextFilter(ptr as *mut sys::cty::c_char) },
        }
    }

    /// Builds the TextFilter with its filter attribute. You can use
    /// [`pass_filter()`](Self::pass_filter) after it.
    ///
    /// If you want control the filter with an InputText, check [`draw()`](Self::draw).
    pub fn build(&self) {
        unsafe {
            sys::ImGuiTextFilter_Build(self.raw);
        }
    }

    /// Draws an [InputText](crate::input_widget::InputText) to control the filter of the TextFilter.
    ///
    /// This is equivalent of [draw_with_size](Self::draw_with_size) with `size` set to `0.0`.
    pub fn draw(&self) {
        self.draw_with_size(0.0);
    }

    /// Draws an [InputText](crate::input_widget::InputText) to control the filter of the TextFilter.
    ///
    /// The InputText has the size passed in parameters.
    pub fn draw_with_size(&self, size: f32) {
        unsafe {
            let mut id = self.id.clone();
            id.push('\0');
            let ptr = id.as_mut_ptr();
            sys::ImGuiTextFilter_Draw(self.raw, ptr as *mut sys::cty::c_char, size);
        }
    }

    /// Returns true if the filter is not empty (`""`).
    pub fn is_active(&self) -> bool {
        unsafe { sys::ImGuiTextFilter_IsActive(self.raw) }
    }

    /// Returns true if the buffer matches the filter.
    ///
    /// [`draw()`](Self::draw) or [`build()`](Self::build) mut be called **before** this function.
    pub fn pass_filter(&self, buf: &str) -> bool {
        let mut buf = String::from(buf);
        buf.push('\0');
        let ptr = buf.as_mut_ptr();
        unsafe {
            sys::ImGuiTextFilter_PassFilter(self.raw, ptr as *mut sys::cty::c_char, ptr::null())
        }
    }

    pub fn pass_filter_with_end(&self, start: &str, end: &str) -> bool {
        let (mut start, mut end) = (String::from(start), String::from(end));
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

    /// Clears the filter.
    pub fn clear(&self) {
        unsafe {
            sys::ImGuiTextFilter_Clear(self.raw);
        }
    }
}

impl Ui {
    pub fn text_filter(label: String) -> TextFilter {
        TextFilter::new(label)
    }

    pub fn text_filter_with_filter(label: String, filter: String) -> TextFilter {
        TextFilter::new_with_filter(label, filter)
    }
}
