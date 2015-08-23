#[macro_use]
extern crate bitflags;

#[cfg(feature = "glium")]
#[macro_use]
extern crate glium;

extern crate libc;

#[cfg(feature = "sdl2")]
extern crate sdl2;

use libc::{c_char, c_float, c_int, c_uchar};
use std::borrow::Cow;
use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::ptr;
use std::slice;
use std::str;

pub use ffi::{
   ImDrawIdx, ImDrawVert,
   ImGuiSetCond,
   ImGuiSetCond_Always, ImGuiSetCond_Once,
   ImGuiSetCond_FirstUseEver, ImGuiSetCond_Appearing,
   ImGuiWindowFlags,
   ImGuiWindowFlags_NoTitleBar, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoMove,
   ImGuiWindowFlags_NoScrollbar, ImGuiWindowFlags_NoScrollWithMouse, ImGuiWindowFlags_NoCollapse,
   ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_ShowBorders,
   ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_NoInputs, ImGuiWindowFlags_MenuBar,
   ImVec2, ImVec4
};
pub use menus::{Menu, MenuItem};
pub use sliders::{SliderFloat, SliderInt};
pub use widgets::{CollapsingHeader};
pub use window::{Window};

pub mod ffi;
mod menus;
mod sliders;
mod widgets;
mod window;

#[cfg(feature = "glium")]
pub mod glium_renderer;

pub struct ImGui;

#[macro_export]
macro_rules! im_str {
   ($e:tt) => ({
      let value = concat!($e, "\0");
      unsafe { ::imgui::ImStr::from_bytes(value.as_bytes()) }
   });
   ($e:tt, $($arg:tt)*) => ({
      ::imgui::ImStr::from_fmt(format_args!($e, $($arg)*))
   })
}

pub struct ImStr<'a> {
   bytes: Cow<'a, [u8]>
}

impl<'a> ImStr<'a> {
   pub unsafe fn from_bytes(bytes: &'a [u8]) -> ImStr<'a> {
      ImStr {
         bytes: Cow::Borrowed(bytes)
      }
   }
   pub fn from_str(value: &str) -> ImStr<'a> {
      let mut bytes: Vec<u8> = value.bytes().collect();
      bytes.push(0);
      ImStr {
         bytes: Cow::Owned(bytes)
      }
   }
   pub fn from_fmt(args: fmt::Arguments) -> ImStr<'a> {
      let mut bytes = fmt::format(args).into_bytes();
      bytes.push(0);
      ImStr {
         bytes: Cow::Owned(bytes)
      }
   }
   fn as_ptr(&self) -> *const c_char { self.bytes.as_ptr() as *const c_char }
}

pub struct TextureHandle<'a> {
   pub width: u32,
   pub height: u32,
   pub pixels: &'a [c_uchar]
}

pub fn get_version() -> &'static str {
   unsafe {
      let bytes = CStr::from_ptr(ffi::igGetVersion()).to_bytes();
      str::from_utf8_unchecked(bytes)
   }
}

impl ImGui {
   pub fn init() -> ImGui {
      let io: &mut ffi::ImGuiIO = unsafe { mem::transmute(ffi::igGetIO()) };
      io.render_draw_lists_fn = Some(render_draw_lists);

      ImGui
   }
   pub fn prepare_texture<'a, F, T>(&mut self, f: F) -> T where F: FnOnce(TextureHandle<'a>) -> T {
      let io: &mut ffi::ImGuiIO = unsafe { mem::transmute(ffi::igGetIO()) };
      let mut pixels: *mut c_uchar = ptr::null_mut();
      let mut width: c_int = 0;
      let mut height: c_int = 0;
      let mut bytes_per_pixel: c_int = 0;
      unsafe {
         ffi::ImFontAtlas_GetTexDataAsRGBA32(io.fonts, &mut pixels, &mut width, &mut height, &mut bytes_per_pixel);
         f(TextureHandle {
            width: width as u32,
            height: height as u32,
            pixels: slice::from_raw_parts(pixels, (width * height * bytes_per_pixel) as usize)
         })
      }
   }
   pub fn draw_mouse_cursor(&mut self, value: bool) {
      let io: &mut ffi::ImGuiIO = unsafe { mem::transmute(ffi::igGetIO()) };
      io.mouse_draw_cursor = value;
   }
   pub fn mouse_pos(&self) -> (f32, f32) {
      let io: &mut ffi::ImGuiIO = unsafe { mem::transmute(ffi::igGetIO()) };
      (io.mouse_pos.x, io.mouse_pos.y)
   }
   pub fn set_mouse_pos(&mut self, x: f32, y: f32) {
      let io: &mut ffi::ImGuiIO = unsafe { mem::transmute(ffi::igGetIO()) };
      io.mouse_pos.x = x;
      io.mouse_pos.y = y;
   }
   pub fn set_mouse_down(&mut self, states: &[bool; 5]) {
      let io: &mut ffi::ImGuiIO = unsafe { mem::transmute(ffi::igGetIO()) };
      io.mouse_down = *states;
   }
   pub fn get_time(&self) -> f32 { unsafe { ffi::igGetTime() } }
   pub fn get_frame_count(&self) -> i32 { unsafe { ffi::igGetFrameCount() } }
   pub fn frame<'ui, 'a: 'ui>(&'a mut self, width: u32, height: u32, delta_time: f32) -> Ui<'ui> {
      unsafe {
         let io: &mut ffi::ImGuiIO = mem::transmute(ffi::igGetIO());
         io.display_size.x = width as c_float;
         io.display_size.y = height as c_float;
         io.delta_time = delta_time;

         ffi::igNewFrame();
      }
      Ui {
         imgui: self
      }
   }
}

impl Drop for ImGui {
   fn drop(&mut self) {
      unsafe {
         ffi::igShutdown();
      }
   }
}

#[cfg(feature = "sdl2")]
impl ImGui {
   pub fn update_mouse(&mut self, mouse: &::sdl2::mouse::MouseUtil) {
      let (mouse_state, mouse_x, mouse_y) = mouse.get_mouse_state();
      let io: &mut ffi::ImGuiIO = unsafe { mem::transmute(ffi::igGetIO()) };
      io.mouse_pos.x = mouse_x as f32;
      io.mouse_pos.y = mouse_y as f32;
      io.mouse_down = [
         mouse_state.left(),
         mouse_state.right(),
         mouse_state.middle(),
         mouse_state.x1(),
         mouse_state.x2()
      ];
   }
}

pub struct DrawList<'a> {
   pub cmd_buffer: &'a [ffi::ImDrawCmd],
   pub idx_buffer: &'a [ffi::ImDrawIdx],
   pub vtx_buffer: &'a [ffi::ImDrawVert]
}

pub struct Ui<'ui> {
   imgui: &'ui ImGui
}

static FMT: &'static [u8] = b"%s\0";

fn fmt_ptr() -> *const c_char { FMT.as_ptr() as *const c_char }

impl<'ui> Ui<'ui> {
   pub fn imgui(&self) -> &ImGui { self.imgui }
   pub fn render<F, E>(self, mut f: F) -> Result<(), E>
         where F: FnMut(DrawList<'ui>) -> Result<(), E> {
      unsafe {
         let mut im_draw_data = mem::zeroed();
         RENDER_DRAW_LISTS_STATE.0 = &mut im_draw_data;
         ffi::igRender();
         RENDER_DRAW_LISTS_STATE.0 = 0 as *mut ffi::ImDrawData;

         for &cmd_list in im_draw_data.cmd_lists() {
            let draw_list =
               DrawList {
                  cmd_buffer: (*cmd_list).cmd_buffer.as_slice(),
                  idx_buffer: (*cmd_list).idx_buffer.as_slice(),
                  vtx_buffer: (*cmd_list).vtx_buffer.as_slice()
               };
            try!(f(draw_list));
         }
      }
      Ok(())
   }
   pub fn show_user_guide(&self) { unsafe { ffi::igShowUserGuide() }; }
   pub fn show_test_window(&self, opened: &mut bool) {
      unsafe {
         ffi::igShowTestWindow(opened);
      }
   }
   pub fn show_metrics_window(&self, opened: &mut bool) {
      unsafe {
         ffi::igShowMetricsWindow(opened);
      }
   }
}

// Window
impl<'ui> Ui<'ui> {
   pub fn window<'p>(&self) -> Window<'ui, 'p> { Window::new() }
}

// Layout
impl<'ui> Ui<'ui> {
   pub fn separator(&self) { unsafe { ffi::igSeparator() }; }
   pub fn same_line(&self, pos_x: f32) {
      unsafe {
         ffi::igSameLine(pos_x, -1.0f32)
      }
   }
   pub fn same_line_spacing(&self, pos_x: f32, spacing_w: f32) {
      unsafe {
         ffi::igSameLine(pos_x, spacing_w)
      }
   }
   pub fn spacing(&self) { unsafe { ffi::igSpacing() }; }
}

// Widgets
impl<'ui> Ui<'ui> {
   pub fn text<'b>(&self, text: ImStr<'b>) {
      // TODO: use igTextUnformatted
      unsafe {
         ffi::igText(fmt_ptr(), text.as_ptr());
      }
   }
   pub fn text_colored<'b, A>(&self, col: A, text: ImStr<'b>) where A: Into<ImVec4> {
      unsafe {
         ffi::igTextColored(col.into(), fmt_ptr(), text.as_ptr());
      }
   }
   pub fn text_disabled<'b>(&self, text: ImStr<'b>) {
      unsafe {
         ffi::igTextDisabled(fmt_ptr(), text.as_ptr());
      }
   }
   pub fn text_wrapped<'b>(&self, text: ImStr<'b>) {
      unsafe {
         ffi::igTextWrapped(fmt_ptr(), text.as_ptr());
      }
   }
   pub fn label_text<'b>(&self, label: ImStr<'b>, text: ImStr<'b>) {
      unsafe {
         ffi::igLabelText(label.as_ptr(), fmt_ptr(), text.as_ptr());
      }
   }
   pub fn bullet(&self) {
      unsafe {
         ffi::igBullet();
      }
   }
   pub fn bullet_text<'b>(&self, text: ImStr<'b>) {
      unsafe {
         ffi::igBulletText(fmt_ptr(), text.as_ptr());
      }
   }
   pub fn collapsing_header<'p>(&self, label: ImStr<'p>) -> CollapsingHeader<'ui, 'p> {
      CollapsingHeader::new(label)
   }
   pub fn checkbox<'p>(&self, label: ImStr<'p>, value: &'p mut bool) -> bool {
      unsafe { ffi::igCheckbox(label.as_ptr(), value) }
   }
}

// Widgets: Sliders
impl<'ui> Ui<'ui> {
   pub fn slider_f32<'p>(&self, label: ImStr<'p>,
                         value: &'p mut f32, min: f32, max: f32) -> SliderFloat<'ui, 'p> {
      SliderFloat::new(label, value, min, max)
   }
   pub fn slider_i32<'p>(&self, label: ImStr<'p>,
                         value: &'p mut i32, min: i32, max: i32) -> SliderInt<'ui, 'p> {
      SliderInt::new(label, value, min, max)
   }
}

// Widgets: Menus
impl<'ui> Ui<'ui> {
   pub fn main_menu_bar<F>(&self, f: F) where F: FnOnce() {
      let render = unsafe { ffi::igBeginMainMenuBar() };
      if render {
         f();
         unsafe { ffi::igEndMainMenuBar() };
      }
   }
   pub fn menu_bar<F>(&self, f: F) where F: FnOnce() {
      let render = unsafe { ffi::igBeginMenuBar() };
      if render {
         f();
         unsafe { ffi::igEndMenuBar() };
      }
   }
   pub fn menu<'p>(&self, label: ImStr<'p>) -> Menu<'ui, 'p> { Menu::new(label) }
   pub fn menu_item<'p>(&self, label: ImStr<'p>) -> MenuItem<'ui, 'p> { MenuItem::new(label) }
}

struct RenderDrawListsState(*mut ffi::ImDrawData);
unsafe impl Sync for RenderDrawListsState {}

static mut RENDER_DRAW_LISTS_STATE: RenderDrawListsState =
   RenderDrawListsState(0 as *mut ffi::ImDrawData);

extern "C" fn render_draw_lists(data: *mut ffi::ImDrawData) {
   unsafe {
      ptr::copy_nonoverlapping(data, RENDER_DRAW_LISTS_STATE.0, 1);
   }
}
