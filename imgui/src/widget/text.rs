use std::os::raw::c_char;

use crate::string::ImStr;
use crate::style::StyleColor;
use crate::Ui;

static FMT: &[u8] = b"%s\0";

#[inline]
fn fmt_ptr() -> *const c_char {
    FMT.as_ptr() as *const c_char
}

/// # Widgets: Text
impl<'ui> Ui<'ui> {
    /// Renders simple text
    #[doc(alias = "TextUnformatted")]
    pub fn text<T: AsRef<str>>(&self, text: T) {
        let s = text.as_ref();
        unsafe {
            let start = s.as_ptr();
            let end = start.add(s.len());
            sys::igTextUnformatted(start as *const c_char, end as *const c_char);
        }
    }
    /// Renders simple text using the given text color
    pub fn text_colored<T: AsRef<str>>(&self, color: [f32; 4], text: T) {
        let style = self.push_style_color(StyleColor::Text, color);
        self.text(text);
        style.end();
    }
    /// Renders simple text using `StyleColor::TextDisabled` color
    pub fn text_disabled<T: AsRef<str>>(&self, text: T) {
        let color = self.style_color(StyleColor::TextDisabled);
        let style = self.push_style_color(StyleColor::Text, color);
        self.text(text);
        style.end();
    }
    /// Renders text wrapped to the end of window (or column)
    #[doc(alias = "TextWrapperd")]
    pub fn text_wrapped(&self, text: impl AsRef<str>) {
        let mut handle = self.buffer.borrow_mut();
        handle.clear();
        handle.extend(text.as_ref().as_bytes());
        handle.push(b'\0');
        unsafe { sys::igTextWrapped(fmt_ptr(), handle.as_ptr()) }
    }
    /// Render a text + label combination aligned the same way as value+label widgets
    #[doc(alias = "LabelText")]
    pub fn label_text(&self, label: impl AsRef<str>, text: impl AsRef<str>) {
        let mut handle = self.buffer.borrow_mut();
        handle.clear();
        handle.extend(label.as_ref().as_bytes());
        handle.push(b'\0');
        handle.extend(text.as_ref().as_bytes());
        handle.push(b'\0');

        let ptr_one = handle.as_ptr();
        let ptr_two = unsafe { ptr_one.add(text.as_ref().len() + 1) };

        unsafe { sys::igLabelText(ptr_one as *const _, fmt_ptr(), ptr_two as *const _) }
    }
    /// Renders text with a little bullet aligned to the typical tree node
    #[doc(alias = "BulletText")]
    pub fn bullet_text(&self, text: &ImStr) {
        unsafe { sys::igBulletText(fmt_ptr(), text.as_ptr()) }
    }
}
