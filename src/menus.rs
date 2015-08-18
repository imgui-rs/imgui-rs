use std::marker::PhantomData;
use std::ptr;

use super::ffi;
use super::{Frame, ImStr};

pub struct Menu<'fr, 'p> {
   label: ImStr<'p>,
   enabled: bool,
   _phantom: PhantomData<&'fr Frame<'fr>>
}

impl<'fr, 'p> Menu<'fr, 'p> {
   pub fn new(label: ImStr<'p>) -> Self {
      Menu {
         label: label,
         enabled: true,
         _phantom: PhantomData
      }
   }
   pub fn enabled(self, enabled: bool) -> Self {
      Menu {
         enabled: enabled,
         .. self
      }
   }
   pub fn build<F: FnOnce()>(self, f: F) {
      let render = unsafe { ffi::igBeginMenu(self.label.as_ptr(), self.enabled) };
      if render {
         f();
         unsafe { ffi::igEndMenu() };
      }
   }
}

pub struct MenuItem<'fr, 'p> {
   label: ImStr<'p>,
   shortcut: Option<ImStr<'p>>,
   selected: bool,
   enabled: bool,
   _phantom: PhantomData<&'fr Frame<'fr>>
}

impl<'fr, 'p> MenuItem<'fr, 'p> {
   pub fn new(label: ImStr<'p>) -> Self {
      MenuItem {
         label: label,
         shortcut: None,
         selected: false,
         enabled: true,
         _phantom: PhantomData
      }
   }
   pub fn shortcut(self, shortcut: ImStr<'p>) -> Self {
      MenuItem {
         shortcut: Some(shortcut),
         .. self
      }
   }
   pub fn selected(self, selected: bool) -> Self {
      MenuItem {
         selected: selected,
         .. self
      }
   }
   pub fn enabled(self, enabled: bool) -> Self {
      MenuItem {
         enabled: enabled,
         .. self
      }
   }
   pub fn build(self) -> bool {
      let label = self.label.as_ptr();
      let shortcut = self.shortcut.map(|x| x.as_ptr()).unwrap_or(ptr::null());
      let selected = self.selected;
      let enabled = self.enabled;
      unsafe {
         ffi::igMenuItem(label, shortcut, selected, enabled)
      }
   }
}
