#[macro_use]
extern crate bitflags;

#[cfg(feature = "glium")]
#[macro_use]
extern crate glium;

extern crate libc;

#[cfg(feature = "sdl2")]
extern crate sdl2;

use libc::{c_char, c_float, c_int, c_uchar};
use std::marker::PhantomData;
use std::mem;
use std::ptr;
use std::slice;

pub use ffi::{ImDrawIdx, ImDrawVert, ImGuiWindowFlags, ImVec2, ImVec4};
pub use menus::{Menu, MenuItem};

pub mod ffi;
mod menus;

#[cfg(feature = "glium")]
pub mod glium_renderer;

pub struct ImGui;

#[macro_export]
macro_rules! im_str {
   ($e:expr) => ({
      let value = concat!($e, "\0");
      unsafe { ::imgui::ImStr::from_bytes(value.as_bytes()) }
   });
}

pub struct ImStr<'a> {
   bytes: &'a [u8]
}

impl<'a> ImStr<'a> {
   pub unsafe fn from_bytes(bytes: &'a [u8]) -> ImStr<'a> {
      ImStr {
         bytes: bytes
      }
   }
   fn as_ptr(&self) -> *const c_char { self.bytes.as_ptr() as *const c_char }
}

pub struct TextureHandle<'a> {
   pub width: u32,
   pub height: u32,
   pub pixels: &'a [c_uchar]
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
   pub fn set_mouse_pos(&mut self, x: f32, y: f32) {
      let io: &mut ffi::ImGuiIO = unsafe { mem::transmute(ffi::igGetIO()) };
      io.mouse_pos.x = x;
      io.mouse_pos.y = y;
   }
   pub fn set_mouse_down(&mut self, states: &[bool; 5]) {
      let io: &mut ffi::ImGuiIO = unsafe { mem::transmute(ffi::igGetIO()) };
      io.mouse_down = *states;
   }
   pub fn frame<'a>(&mut self, width: u32, height: u32, delta_time: f32) -> Frame<'a> {
      unsafe {
         let io: &mut ffi::ImGuiIO = mem::transmute(ffi::igGetIO());
         io.display_size.x = width as c_float;
         io.display_size.y = height as c_float;
         io.delta_time = delta_time;

         ffi::igNewFrame();
      }
      Frame {
         _phantom: PhantomData
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

pub struct Frame<'fr> {
   _phantom: PhantomData<&'fr ImGui>
}

static FMT: &'static [u8] = b"%s\0";

fn fmt_ptr() -> *const c_char { FMT.as_ptr() as *const c_char }

impl<'fr> Frame<'fr> {
   pub fn render<F, E>(self, mut f: F) -> Result<(), E>
         where F: FnMut(DrawList<'fr>) -> Result<(), E> {
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
   pub fn show_test_window(&self) -> bool {
      let mut opened = true;
      unsafe {
         ffi::igShowTestWindow(&mut opened);
      }
      opened
   }
}

// Widgets
impl<'fr> Frame<'fr> {
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
}

// Widgets: Menus
impl<'fr> Frame<'fr> {
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
   pub fn menu<'p>(&self, label: ImStr<'p>) -> Menu<'fr, 'p> { Menu::new(label) }
   pub fn menu_item<'p>(&self, label: ImStr<'p>) -> MenuItem<'fr, 'p> { MenuItem::new(label) }
}

impl<'fr> Frame<'fr> {
   pub fn separator(&self) {
      unsafe { ffi:: igSeparator() };
   }
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
