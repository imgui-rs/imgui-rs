#[macro_use]
extern crate lazy_static;
extern crate parking_lot;

pub extern crate imgui_sys as sys;

mod context;
mod font_atlas;
mod io;
mod render;
mod string;
mod style;
#[cfg(test)]
mod test;

use std::ffi::CStr;
use std::str;

pub use self::context::*;
pub use self::font_atlas::*;
pub use self::io::*;
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
}

impl<'ui> Ui<'ui> {
    pub fn io(&self) -> &Io {
        unsafe { &*(sys::igGetIO() as *const Io) }
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
    pub fn render_with<T, R: Renderer<T>>(
        self,
        renderer: &mut R,
        output: &mut T,
    ) -> Result<(), R::Error> {
        let draw_data = self.render();
        renderer.render_draw_data(draw_data, output)
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
    pub fn mouse_cursor(&self) -> Option<MouseCursor> {
        match unsafe { sys::igGetMouseCursor() } {
            sys::ImGuiMouseCursor_Arrow => Some(MouseCursor::Arrow),
            sys::ImGuiMouseCursor_TextInput => Some(MouseCursor::TextInput),
            sys::ImGuiMouseCursor_ResizeAll => Some(MouseCursor::ResizeAll),
            sys::ImGuiMouseCursor_ResizeNS => Some(MouseCursor::ResizeNS),
            sys::ImGuiMouseCursor_ResizeEW => Some(MouseCursor::ResizeEW),
            sys::ImGuiMouseCursor_ResizeNESW => Some(MouseCursor::ResizeNESW),
            sys::ImGuiMouseCursor_ResizeNWSE => Some(MouseCursor::ResizeNWSE),
            sys::ImGuiMouseCursor_Hand => Some(MouseCursor::Hand),
            _ => None,
        }
    }
    pub fn set_mouse_cursor(&self, cursor_type: Option<MouseCursor>) {
        unsafe {
            sys::igSetMouseCursor(
                cursor_type
                    .map(|x| x as i32)
                    .unwrap_or(sys::ImGuiMouseCursor_None),
            );
        }
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum MouseCursor {
    Arrow = sys::ImGuiMouseCursor_Arrow,
    TextInput = sys::ImGuiMouseCursor_TextInput,
    ResizeAll = sys::ImGuiMouseCursor_ResizeAll,
    ResizeNS = sys::ImGuiMouseCursor_ResizeNS,
    ResizeEW = sys::ImGuiMouseCursor_ResizeEW,
    ResizeNESW = sys::ImGuiMouseCursor_ResizeNESW,
    ResizeNWSE = sys::ImGuiMouseCursor_ResizeNWSE,
    Hand = sys::ImGuiMouseCursor_Hand,
}
impl MouseCursor {
    pub const VARIANTS: [MouseCursor; 8] = [
        MouseCursor::Arrow,
        MouseCursor::TextInput,
        MouseCursor::ResizeAll,
        MouseCursor::ResizeNS,
        MouseCursor::ResizeEW,
        MouseCursor::ResizeNESW,
        MouseCursor::ResizeNWSE,
        MouseCursor::Hand,
    ];
    const SKIPPED_COUNT: usize = 1;
    pub const COUNT: usize = sys::ImGuiMouseCursor_COUNT as usize - MouseCursor::SKIPPED_COUNT;
}

impl<'ui> Drop for Ui<'ui> {
    fn drop(&mut self) {
        unsafe {
            sys::igEndFrame();
        }
    }
}
