use imgui_sys;
use std::marker::PhantomData;
use std::ptr;

use super::{Ui, ImStr};

#[must_use]
pub struct Menu<'ui, 'p> {
    label: ImStr<'p>,
    enabled: bool,
    _phantom: PhantomData<&'ui Ui<'ui>>
}

impl<'ui, 'p> Menu<'ui, 'p> {
    pub fn new<S>(label: S) -> Self where S: Into<ImStr<'p>> {
        Menu {
            label: label.into(),
            enabled: true,
            _phantom: PhantomData
        }
    }
    #[inline]
    pub fn enabled(self, enabled: bool) -> Self {
        Menu {
            enabled: enabled,
            .. self
        }
    }
    pub fn build<F: FnOnce()>(self, f: F) {
        let render = unsafe { imgui_sys::igBeginMenu(self.label.as_ptr(), self.enabled) };
        if render {
            f();
            unsafe { imgui_sys::igEndMenu() };
        }
    }
}

#[must_use]
pub struct MenuItem<'ui, 'p> {
    label: ImStr<'p>,
    shortcut: Option<ImStr<'p>>,
    selected: Option<&'p mut bool>,
    enabled: bool,
    _phantom: PhantomData<&'ui Ui<'ui>>
}

impl<'ui, 'p> MenuItem<'ui, 'p> {
    pub fn new<S>(label: S) -> Self where S: Into<ImStr<'p>> {
        MenuItem {
            label: label.into(),
            shortcut: None,
            selected: None,
            enabled: true,
            _phantom: PhantomData
        }
    }
    #[inline]
    pub fn shortcut<S>(self, shortcut: S) -> Self where S: Into<ImStr<'p>> {
        MenuItem {
            shortcut: Some(shortcut.into()),
            .. self
        }
    }
    #[inline]
    pub fn selected(self, selected: &'p mut bool) -> Self {
        MenuItem {
            selected: Some(selected),
            .. self
        }
    }
    #[inline]
    pub fn enabled(self, enabled: bool) -> Self {
        MenuItem {
            enabled: enabled,
            .. self
        }
    }
    pub fn build(self) -> bool {
        let label = self.label.as_ptr();
        let shortcut = self.shortcut.map(|x| x.as_ptr()).unwrap_or(ptr::null());
        let selected = self.selected.map(|x| x as *mut bool).unwrap_or(ptr::null_mut());
        let enabled = self.enabled;
        unsafe {
            imgui_sys::igMenuItemPtr(label, shortcut, selected, enabled)
        }
    }
}
