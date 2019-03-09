#[macro_use]
extern crate lazy_static;
extern crate parking_lot;

pub extern crate imgui_sys as sys;

mod clipboard;
mod context;
mod font_atlas;
pub mod internal;
mod io;
mod mouse;
mod render;
mod string;
mod style;
#[cfg(test)]
mod test;

use std::cell;
use std::ffi::CStr;
use std::str;

pub use self::clipboard::Clipboard;
pub use self::context::*;
pub use self::font_atlas::*;
pub use self::io::*;
pub use self::mouse::*;
pub use self::render::draw_data::*;
pub use self::render::renderer::*;
pub use self::string::*;
pub use self::style::*;

/// Returns the underlying Dear ImGui library version
pub fn get_dear_imgui_version() -> &'static str {
    unsafe {
        let bytes = CStr::from_ptr(sys::igGetVersion()).to_bytes();
        str::from_utf8_unchecked(bytes)
    }
}

#[test]
fn test_get_version() {
    assert_eq!(get_dear_imgui_version(), "1.68");
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
    pub fn show_demo_window(&self, opened: &mut bool) {
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
        unsafe {
            sys::igEndFrame();
        }
    }
}
