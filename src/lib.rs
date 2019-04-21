#[macro_use]
extern crate lazy_static;
extern crate parking_lot;

pub extern crate imgui_sys as sys;

mod clipboard;
mod context;
mod fonts;
mod input;
pub mod internal;
mod io;
mod layout;
mod render;
mod stacks;
mod string;
mod style;
#[cfg(test)]
mod test;
mod widget;
mod window;

use std::cell;
use std::ffi::CStr;
use std::str;
use std::thread;

pub use self::clipboard::ClipboardBackend;
pub use self::context::*;
pub use self::fonts::atlas::*;
pub use self::fonts::font::*;
pub use self::fonts::glyph::*;
pub use self::fonts::glyph_ranges::*;
pub use self::input::keyboard::*;
pub use self::input::mouse::*;
pub use self::io::*;
pub use self::render::draw_data::*;
pub use self::render::renderer::*;
pub use self::stacks::*;
pub use self::string::*;
pub use self::style::*;
pub use self::widget::color_editors::*;
pub use self::widget::misc::*;
pub use self::widget::progress_bar::*;
pub use self::window::*;

/// Returns the underlying Dear ImGui library version
pub fn dear_imgui_version() -> &'static str {
    unsafe {
        let bytes = CStr::from_ptr(sys::igGetVersion()).to_bytes();
        str::from_utf8_unchecked(bytes)
    }
}

#[test]
fn test_version() {
    assert_eq!(dear_imgui_version(), "1.69");
}

pub struct Ui<'ui> {
    ctx: &'ui Context,
    font_atlas: Option<cell::RefMut<'ui, SharedFontAtlas>>,
}

impl<'ui> Ui<'ui> {
    pub fn io(&self) -> &Io {
        unsafe { &*(sys::igGetIO() as *const Io) }
    }
    pub fn fonts(&self) -> FontAtlasRef {
        match self.font_atlas {
            Some(ref font_atlas) => FontAtlasRef::Shared(font_atlas),
            None => unsafe {
                let fonts = &*(self.io().fonts as *const FontAtlas);
                FontAtlasRef::Owned(fonts)
            },
        }
    }
    pub fn clone_style(&self) -> Style {
        *self.ctx.style()
    }
    pub fn render(self) -> &'ui DrawData {
        unsafe {
            sys::igRender();
            &*(sys::igGetDrawData() as *mut DrawData)
        }
    }
    pub fn show_demo_window(&mut self, opened: &mut bool) {
        unsafe {
            sys::igShowDemoWindow(opened);
        }
    }
    pub fn show_about_window(&self, opened: &mut bool) {
        unsafe {
            sys::igShowAboutWindow(opened);
        }
    }
    pub fn show_metrics_window(&self, opened: &mut bool) {
        unsafe {
            sys::igShowMetricsWindow(opened);
        }
    }
}

impl<'ui> Drop for Ui<'ui> {
    fn drop(&mut self) {
        // If we are panicking, we can't call igEndFrame safely because we might be in an
        // inconsistent state and igEndFrame might abort the process with an assert
        if !thread::panicking() {
            unsafe {
                sys::igEndFrame();
            }
        }
    }
}

/// Condition for applying a setting
#[repr(i8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Condition {
    /// Never apply the setting
    Never = -1,
    /// Always apply the setting
    Always = sys::ImGuiCond_Always as i8,
    /// Apply the setting once per runtime session (only the first call will succeed)
    Once = sys::ImGuiCond_Once as i8,
    /// Apply the setting if the object/window has no persistently saved data (no entry in .ini
    /// file)
    FirstUseEver = sys::ImGuiCond_FirstUseEver as i8,
    /// Apply the setting if the object/window is appearing after being hidden/inactive (or the
    /// first time)
    Appearing = sys::ImGuiCond_Appearing as i8,
}
