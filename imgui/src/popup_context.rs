use bitflags::bitflags;
use std::ptr;
use std::thread;

use crate::context::Context;
use crate::sys;
use crate::{ImStr, Ui};

bitflags! {
    #[repr(transparent)]
    pub struct PopupFlags: u32 {
        /// For PopupContext.begin*(): open on Left Mouse release.
        const MOUSE_BUTTON_LEFT = sys::ImGuiPopupFlags_MouseButtonLeft;
        /// For PopupContext.begin*(): open on Right Mouse release.
        const MOUSE_BUTTON_RIGHT = sys::ImGuiPopupFlags_MouseButtonRight;
        /// For PopupContext.begin*(): open on Middle Mouse release.
        const MOUSE_BUTTON_MIDDLE = sys::ImGuiPopupFlags_MouseButtonMiddle;
        /// For PopupContext.begin*(): by default open on Right Mouse release.
        const MOUSE_BUTTON_DEFAULT = sys::ImGuiPopupFlags_MouseButtonDefault_;
    }
}

pub struct PopupContext<'p> {
    label: Option<&'p ImStr>,
    flags: PopupFlags,
}

impl<'p> PopupContext<'p> {
    pub fn new() -> Self {
        PopupContext {
            label: None,
            flags: PopupFlags::empty(),
        }
    }

    pub fn label(mut self, label: &'p ImStr) -> Self {
        self.label = Some(label);
        self
    }

    pub fn flags(mut self, flags: PopupFlags) -> Self {
        self.flags = flags;
        self
    }

    pub fn on_left_button(mut self, value: bool) -> Self {
        self.flags.set(PopupFlags::MOUSE_BUTTON_LEFT, value);
        self
    }

    pub fn on_middle_button(mut self, value: bool) -> Self {
        self.flags.set(PopupFlags::MOUSE_BUTTON_MIDDLE, value);
        self
    }

    pub fn on_right_button(mut self, value: bool) -> Self {
        self.flags.set(PopupFlags::MOUSE_BUTTON_RIGHT, value);
        self
    }

    /// Open a popup when clicked on the current window.
    ///
    /// Returns `Some(PopupContextToken)` if the popup is visible. After content
    /// has been rendered, the token must be ended by calling `.end()`.
    ///
    /// Returns `None` if the popup is not visible and no content should be
    /// rendered.
    pub fn begin_window(self, ui: &Ui) -> Option<PopupContextToken> {
        if unsafe {
            sys::igBeginPopupContextWindow(
                self.label.unwrap_or_default().as_ptr(),
                self.flags.bits() as i32,
            )
        } {
            Some(PopupContextToken { ctx: ui.ctx })
        } else {
            None
        }
    }

    /// Open a popup when clicked in void (where there are no windows).
    ///
    /// Returns `Some(PopupContextToken)` if the popup is visible. After content
    /// has been rendered, the token must be ended by calling `.end()`.
    ///
    /// Returns `None` if the popup is not visible and no content should be
    /// rendered.
    pub fn begin_void(self, ui: &Ui) -> Option<PopupContextToken> {
        if unsafe {
            sys::igBeginPopupContextVoid(
                self.label.unwrap_or_default().as_ptr(),
                self.flags.bits() as i32,
            )
        } {
            Some(PopupContextToken { ctx: ui.ctx })
        } else {
            None
        }
    }

    /// Open a popup when clicked on the current window.
    ///
    /// Contents of the popup can be contructed using the closure.
    pub fn build_window<F: FnOnce()>(self, ui: &Ui, f: F) {
        if let Some(popup) = self.begin_window(ui) {
            f();
            popup.end(ui);
        }
    }

    /// Open a popup when clicked in void (where there are no windows).
    ///
    /// Contents of the popup can be contructed using the closure.
    pub fn build_void<F: FnOnce()>(self, ui: &Ui, f: F) {
        if let Some(popup) = self.begin_void(ui) {
            f();
            popup.end(ui);
        }
    }
}

impl<'p> Default for PopupContext<'p> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PopupContextToken {
    ctx: *const Context,
}

impl PopupContextToken {
    pub fn end(mut self, _: &Ui) {
        self.ctx = ptr::null();
        unsafe { sys::igEndPopup() };
    }
}

impl Drop for PopupContextToken {
    fn drop(&mut self) {
        if !self.ctx.is_null() && !thread::panicking() {
            panic!("A PopupContextToken was leaked. Did you call .end()?");
        }
    }
}
