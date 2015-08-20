use libc::c_float;
use std::marker::PhantomData;
use std::ptr;

use super::ffi;
use super::{
   Frame,
   ImGuiSetCond,
   ImGuiWindowFlags,
   ImGuiWindowFlags_NoTitleBar, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoMove,
   ImGuiWindowFlags_NoScrollbar, ImGuiWindowFlags_NoScrollWithMouse, ImGuiWindowFlags_NoCollapse,
   ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_ShowBorders,
   ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_NoInputs, ImGuiWindowFlags_MenuBar,
   ImStr, ImVec2
};

pub struct Window<'fr, 'p> {
   pos: (f32, f32),
   pos_cond: ImGuiSetCond,
   size: (f32, f32),
   size_cond: ImGuiSetCond,
   name: ImStr<'p>,
   closable: bool,
   bg_alpha: f32,
   flags: ImGuiWindowFlags,
   _phantom: PhantomData<&'fr Frame<'fr>>
}

impl<'fr, 'p> Window<'fr, 'p> {
   pub fn new() -> Window<'fr, 'p> {
      Window {
         pos: (0.0, 0.0),
         pos_cond: ImGuiSetCond::empty(),
         size: (0.0, 0.0),
         size_cond: ImGuiSetCond::empty(),
         name: unsafe { ImStr::from_bytes(b"Debug\0") },
         closable: false,
         bg_alpha: -1.0,
         flags: ImGuiWindowFlags::empty(),
         _phantom: PhantomData
      }
   }
   #[inline]
   pub fn position(self, pos: (f32, f32), cond: ImGuiSetCond) -> Self {
      Window {
         pos: pos,
         pos_cond: cond,
         .. self
      }
   }
   #[inline]
   pub fn size(self, size: (f32, f32), cond: ImGuiSetCond) -> Self {
      Window {
         size: size,
         size_cond: cond,
         .. self
      }
   }
   #[inline]
   pub fn name(self, name: ImStr<'p>) -> Self {
      Window {
         name: name,
         .. self
      }
   }
   #[inline]
   pub fn closable(self, closable: bool) -> Self {
      Window {
         closable: closable,
         .. self
      }
   }
   #[inline]
   pub fn bg_alpha(self, bg_alpha: f32) -> Self {
      Window {
         bg_alpha: bg_alpha,
         .. self
      }
   }
   pub fn flags(self, flags: ImGuiWindowFlags) -> Self {
      Window {
         flags: flags,
         .. self
      }
   }
   #[inline]
   pub fn title_bar(self, value: bool) -> Self {
      Window {
         flags: self.flags.with(ImGuiWindowFlags_NoTitleBar, !value),
         .. self
      }
   }
   #[inline]
   pub fn resizable(self, value: bool) -> Self {
      Window {
         flags: self.flags.with(ImGuiWindowFlags_NoResize, !value),
         .. self
      }
   }
   #[inline]
   pub fn movable(self, value: bool) -> Self {
      Window {
         flags: self.flags.with(ImGuiWindowFlags_NoMove, !value),
         .. self
      }
   }
   #[inline]
   pub fn scroll_bar(self, value: bool) -> Self {
      Window {
         flags: self.flags.with(ImGuiWindowFlags_NoScrollbar, !value),
         .. self
      }
   }
   #[inline]
   pub fn scrollable(self, value: bool) -> Self {
      Window {
         flags: self.flags.with(ImGuiWindowFlags_NoScrollWithMouse, !value),
         .. self
      }
   }
   #[inline]
   pub fn collapsible(self, value: bool) -> Self {
      Window {
         flags: self.flags.with(ImGuiWindowFlags_NoCollapse, !value),
         .. self
      }
   }
   #[inline]
   pub fn always_auto_resize(self, value: bool) -> Self {
      Window {
         flags: self.flags.with(ImGuiWindowFlags_AlwaysAutoResize, value),
         .. self
      }
   }
   #[inline]
   pub fn show_borders(self, value: bool) -> Self {
      Window {
         flags: self.flags.with(ImGuiWindowFlags_ShowBorders, value),
         .. self
      }
   }
   #[inline]
   pub fn save_settings(self, value: bool) -> Self {
      Window {
         flags: self.flags.with(ImGuiWindowFlags_NoSavedSettings, !value),
         .. self
      }
   }
   #[inline]
   pub fn inputs(self, value: bool) -> Self {
      Window {
         flags: self.flags.with(ImGuiWindowFlags_NoInputs, !value),
         .. self
      }
   }
   #[inline]
   pub fn menu_bar(self, value: bool) -> Self {
      Window {
         flags: self.flags.with(ImGuiWindowFlags_MenuBar, value),
         .. self
      }
   }
   pub fn build<F: FnOnce()>(self, f: F) -> bool {
      let mut opened = true;
      let render = unsafe {
         if !self.pos_cond.is_empty() {
            ffi::igSetNextWindowPos(ImVec2::new(self.pos.0, self.pos.1), self.pos_cond);
         }
         if !self.size_cond.is_empty() {
            ffi::igSetNextWindowSize(ImVec2::new(self.size.0, self.size.1), self.size_cond);
         }
         ffi::igBegin2(self.name.as_ptr(),
            if self.closable { &mut opened } else { ptr::null_mut() },
            ImVec2::new(0.0, 0.0), self.bg_alpha as c_float, self.flags
         )
      };
      if render {
         f();
      }
      unsafe { ffi::igEnd() };
      opened
   }
}
