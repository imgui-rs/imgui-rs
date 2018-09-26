use std::marker::PhantomData;
use std::ptr;

use super::{ImGuiWindowFlags, ImStr, Ui};

use sys;

#[must_use]
pub struct PopupModal<'ui, 'p> {
    label: &'p ImStr,
    opened: Option<&'p mut bool>,
    flags: ImGuiWindowFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> PopupModal<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p ImStr) -> Self {
        PopupModal {
            label,
            opened: None,
            flags: ImGuiWindowFlags::empty(),
            _phantom: PhantomData,
        }
    }
    pub fn opened(mut self, opened: &'p mut bool) -> Self {
        self.opened = Some(opened);
        self
    }
    pub fn build<F: FnOnce()>(self, f: F) {
        let render = unsafe {
            sys::igBeginPopupModal(
                self.label.as_ptr(),
                self.opened
                    .map(|x| x as *mut bool)
                    .unwrap_or(ptr::null_mut()),
                self.flags,
            )
        };
        if render {
            f();
            unsafe { sys::igEndMenu() };
        }
    }
}
