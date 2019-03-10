use bitflags::bitflags;
use std::borrow::Cow;
use std::f32;
use std::marker::PhantomData;
use std::ptr;

use crate::string::ImStr;
use crate::sys;
use crate::{Condition, Ui};

bitflags! {
    /// Window hover check option flags
    #[repr(transparent)]
    pub struct WindowHoveredFlags: u32 {
        /// Return true if any child of the window is hovered
        const CHILD_WINDOWS = sys::ImGuiHoveredFlags_ChildWindows;
        /// Test from root window (top-most parent of the current hierarchy)
        const ROOT_WINDOW = sys::ImGuiHoveredFlags_RootWindow;
        /// Return true if any window is hovered
        const ANY_WINDOW = sys::ImGuiHoveredFlags_AnyWindow;
        /// Return true even if a popup window is blocking access to this window
        const ALLOW_WHEN_BLOCKED_BY_POPUP = sys::ImGuiHoveredFlags_AllowWhenBlockedByPopup;
        /// Return true even if an active item is blocking access to this window
        const ALLOW_WHEN_BLOCKED_BY_ACTIVE_ITEM = sys::ImGuiHoveredFlags_AllowWhenBlockedByActiveItem;
        /// Test from root window, and return true if any child is hovered
        const ROOT_AND_CHILD_WINDOWS = Self::ROOT_WINDOW.bits | Self::CHILD_WINDOWS.bits;
    }
}

bitflags! {
    /// Window focus check option flags
    #[repr(transparent)]
    pub struct WindowFocusedFlags: u32 {
        /// Return true if any child of the window is focused
        const CHILD_WINDOWS = sys::ImGuiFocusedFlags_ChildWindows;
        /// Test from root window (top-most parent of the current hierarchy)
        const ROOT_WINDOW = sys::ImGuiFocusedFlags_RootWindow;
        /// Return true if any window is focused
        const ANY_WINDOW = sys::ImGuiFocusedFlags_AnyWindow;
        /// Test from root window, and return true if any child is focused
        const ROOT_AND_CHILD_WINDOWS = Self::ROOT_WINDOW.bits | Self::CHILD_WINDOWS.bits;
    }
}

impl<'ui> Ui<'ui> {
    /// Returns true if the current window appeared during this frame
    pub fn is_window_appearing(&self) -> bool {
        unsafe { sys::igIsWindowAppearing() }
    }
    /// Returns true if the current window is in collapsed state (= only the title bar is visible)
    pub fn is_window_collapsed(&self) -> bool {
        unsafe { sys::igIsWindowCollapsed() }
    }
    /// Returns true if the current window is focused
    pub fn is_window_focused(&self) -> bool {
        unsafe { sys::igIsWindowFocused(0) }
    }
    /// Returns true if the current window is focused based on the given flags
    pub fn is_window_focused_with_flags(&self, flags: WindowFocusedFlags) -> bool {
        unsafe { sys::igIsWindowFocused(flags.bits() as i32) }
    }
    /// Returns true if the current window is hovered
    pub fn is_window_hovered(&self) -> bool {
        unsafe { sys::igIsWindowHovered(0) }
    }
    /// Returns true if the current window is hovered based on the given flags
    pub fn is_window_hovered_with_flags(&self, flags: WindowHoveredFlags) -> bool {
        unsafe { sys::igIsWindowHovered(flags.bits() as i32) }
    }
    /// Returns the size of the current window
    pub fn get_window_size(&self) -> [f32; 2] {
        unsafe { sys::igGetWindowSize_nonUDT2().into() }
    }
    /// Returns the current content boundaries in *window coordinates*
    pub fn get_content_region_max(&self) -> [f32; 2] {
        unsafe { sys::igGetContentRegionMax_nonUDT2().into() }
    }
    /// Equal to `ui.get_content_region_max()` - `ui.get_cursor_pos()`
    pub fn get_content_region_avail(&self) -> [f32; 2] {
        unsafe { sys::igGetContentRegionAvail_nonUDT2().into() }
    }
    /// Content boundaries min in *window coordinates*.
    ///
    /// Roughly equal to [0.0, 0.0] - scroll.
    pub fn get_window_content_region_min(&self) -> [f32; 2] {
        unsafe { sys::igGetWindowContentRegionMin_nonUDT2().into() }
    }
    /// Content boundaries max in *window coordinates*.
    ///
    /// Roughly equal to [0.0, 0.0] + size - scroll.
    pub fn get_window_content_region_max(&self) -> [f32; 2] {
        unsafe { sys::igGetWindowContentRegionMax_nonUDT2().into() }
    }
}

#[must_use]
pub struct Window<'a> {
    name: Cow<'a, ImStr>,
    opened: Option<&'a mut bool>,
    pos: [f32; 2],
    pos_cond: Condition,
    pos_pivot: [f32; 2],
    size: [f32; 2],
    size_cond: Condition,
    content_size: [f32; 2],
    collapsed: bool,
    collapsed_cond: Condition,
    focused: bool,
    bg_alpha: f32,
}

impl<'a> Window<'a> {
    pub fn new<T: Into<Cow<'a, ImStr>>>(name: T) -> Window<'a> {
        Window {
            name: name.into(),
            opened: None,
            pos: [0.0, 0.0],
            pos_cond: Condition::Never,
            pos_pivot: [0.0, 0.0],
            size: [0.0, 0.0],
            size_cond: Condition::Never,
            content_size: [0.0, 0.0],
            collapsed: false,
            collapsed_cond: Condition::Never,
            focused: false,
            bg_alpha: f32::NAN,
        }
    }
    pub fn opened(mut self, opened: &'a mut bool) -> Self {
        self.opened = Some(opened);
        self
    }
    pub fn position(mut self, position: [f32; 2], condition: Condition) -> Self {
        self.pos = position;
        self.pos_cond = condition;
        self
    }
    pub fn position_pivot(mut self, pivot: [f32; 2]) -> Self {
        self.pos_pivot = pivot;
        self
    }
    pub fn size(mut self, size: [f32; 2], condition: Condition) -> Self {
        self.size = size;
        self.size_cond = condition;
        self
    }
    pub fn content_size(mut self, size: [f32; 2]) -> Self {
        self.content_size = size;
        self
    }
    pub fn collapsed(mut self, collapsed: bool, condition: Condition) -> Self {
        self.collapsed = collapsed;
        self.collapsed_cond = condition;
        self
    }
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }
    pub fn bg_alpha(mut self, bg_alpha: f32) -> Self {
        self.bg_alpha = bg_alpha;
        self
    }
    pub fn begin<'ui>(self, _: &'ui Ui<'ui>) -> WindowStackToken<'ui> {
        if self.pos_cond != Condition::Never {
            unsafe {
                sys::igSetNextWindowPos(
                    self.pos.into(),
                    self.pos_cond as i32,
                    self.pos_pivot.into(),
                )
            };
        }
        if self.size_cond != Condition::Never {
            unsafe { sys::igSetNextWindowSize(self.size.into(), self.size_cond as i32) };
        }
        if self.content_size[0] != 0.0 || self.content_size[1] != 0.0 {
            unsafe { sys::igSetNextWindowContentSize(self.content_size.into()) };
        }
        if self.collapsed_cond != Condition::Never {
            unsafe { sys::igSetNextWindowCollapsed(self.collapsed, self.collapsed_cond as i32) };
        }
        if self.focused {
            unsafe { sys::igSetNextWindowFocus() };
        }
        if self.bg_alpha.is_finite() {
            unsafe { sys::igSetNextWindowBgAlpha(self.bg_alpha) };
        }
        let should_render = unsafe {
            sys::igBegin(
                self.name.as_ptr(),
                self.opened
                    .map(|x| x as *mut bool)
                    .unwrap_or(ptr::null_mut()),
                0,
            )
        };
        WindowStackToken {
            should_render,
            should_end: true,
            _ui: PhantomData,
        }
    }
    pub fn build<F: FnOnce()>(self, ui: &Ui, f: F) {
        let window = self.begin(ui);
        if window.should_render {
            f();
        }
        window.end();
    }
}

impl<'ui> Ui<'ui> {
    pub fn window<'a, T: Into<Cow<'a, ImStr>>>(name: T) -> Window<'a> {
        Window::new(name)
    }
}

pub struct WindowStackToken<'ui> {
    pub should_render: bool,
    should_end: bool,
    _ui: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui> WindowStackToken<'ui> {
    pub fn end(mut self) {
        self.should_end = false;
        unsafe { sys::igEnd() };
    }
}

impl<'ui> Drop for WindowStackToken<'ui> {
    fn drop(&mut self) {
        if self.should_end {
            unsafe { sys::igEnd() };
        }
    }
}
