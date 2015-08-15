#[macro_use]
extern crate bitflags;

#[cfg(feature = "glium")]
#[macro_use]
extern crate glium;

extern crate libc;

#[cfg(feature = "sdl2")]
extern crate sdl2;

use libc::{c_float, c_int, c_uchar};
use std::marker::PhantomData;
use std::mem;
use std::ptr;
use std::slice;

pub use ffi::{ImDrawIdx, ImDrawVert, ImVec2, ImVec4};

pub mod ffi;

#[cfg(feature = "glium")]
pub mod glium_renderer;

pub struct ImGui;

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
   #[cfg(feature = "sdl2")]
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
   pub fn frame<'a>(&'a mut self, width: u32, height: u32, delta_time: f32) -> Frame<'a> {
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

pub struct DrawList<'a> {
   pub cmd_buffer: &'a [ffi::ImDrawCmd],
   pub idx_buffer: &'a [ffi::ImDrawIdx],
   pub vtx_buffer: &'a [ffi::ImDrawVert]
}

pub struct Frame<'a> {
   _phantom: PhantomData<&'a ImGui>
}

impl<'a> Frame<'a> {
   pub fn show_test_window(&mut self) -> bool {
      let mut opened = true;
      unsafe {
         ffi::igShowTestWindow(&mut opened);
      }
      opened
   }
   pub fn render<F, E>(self, mut f: F) -> Result<(), E>
         where F: FnMut(DrawList<'a>) -> Result<(), E> {
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
