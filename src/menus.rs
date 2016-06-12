use imgui_sys;
use std::marker::PhantomData;
use std::ptr;

use super::Ui;

#[must_use]
pub struct Menu<'ui, 'p> {
    label: &'p str,
    enabled: bool,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> Menu<'ui, 'p> {
    pub fn new(label: &'p str) -> Self {
        Menu {
            label: label,
            enabled: true,
            _phantom: PhantomData,
        }
    }
    #[inline]
    pub fn enabled(self, enabled: bool) -> Self { Menu { enabled: enabled, ..self } }
    pub fn build<F: FnOnce()>(self, f: F) {
        let render =
            unsafe { imgui_sys::igBeginMenu(imgui_sys::ImStr::from(self.label), self.enabled) };
        if render {
            f();
            unsafe { imgui_sys::igEndMenu() };
        }
    }
}

#[must_use]
pub struct MenuItem<'ui, 'p> {
    label: &'p str,
    shortcut: Option<&'p str>,
    selected: Option<&'p mut bool>,
    enabled: bool,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> MenuItem<'ui, 'p> {
    pub fn new(label: &'p str) -> Self {
        MenuItem {
            label: label,
            shortcut: None,
            selected: None,
            enabled: true,
            _phantom: PhantomData,
        }
    }
    #[inline]
    pub fn shortcut(self, shortcut: &'p str) -> Self {
        MenuItem { shortcut: Some(shortcut), ..self }
    }
    #[inline]
    pub fn selected(self, selected: &'p mut bool) -> Self {
        MenuItem { selected: Some(selected), ..self }
    }
    #[inline]
    pub fn enabled(self, enabled: bool) -> Self { MenuItem { enabled: enabled, ..self } }
    pub fn build(self) -> bool {
        let label = imgui_sys::ImStr::from(self.label);
        let shortcut =
            self.shortcut.map(|x| imgui_sys::ImStr::from(x)).unwrap_or(imgui_sys::ImStr::null());
        let selected = self.selected.map(|x| x as *mut bool).unwrap_or(ptr::null_mut());
        let enabled = self.enabled;
        unsafe { imgui_sys::igMenuItemPtr(label, shortcut, selected, enabled) }
    }
}
