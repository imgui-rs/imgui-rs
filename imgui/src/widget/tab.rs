//! Safe wrapper around imgui-sys for tab menu.
//!
//!   # Examples
//
//! ```no_run
//! # use imgui::*;
//! # let mut ctx = Context::create();
//! # let ui = ctx.frame();
//!
//! // During UI construction
//! TabBar::new(im_str!("tabbar")).build(&ui, || {
//!                   TabItem::new(im_str!("a tab")).build(&ui, || {
//!                       ui.text(im_str!("tab content 1"));
//!                   });
//!                   TabItem::new(im_str!("2tab")).build(&ui, || {
//!                       ui.text(im_str!("tab content 2"));
//!                   });
//!               });
//! ```
//!
//! See `test_window_impl.rs` for a more complicated example.
use crate::context::Context;
use crate::string::ImStr;
use crate::sys;
use crate::Ui;
use bitflags::bitflags;
use std::{ptr, thread};

bitflags! {
    #[repr(transparent)]
    pub struct TabBarFlags: u32 {
        const REORDERABLE = sys::ImGuiTabBarFlags_Reorderable as u32;
        const AUTO_SELECT_NEW_TABS = sys::ImGuiTabBarFlags_AutoSelectNewTabs as u32;
        const TAB_LIST_POPUP_BUTTON = sys::ImGuiTabBarFlags_TabListPopupButton as u32;
        const NO_CLOSE_WITH_MIDDLE_MOUSE_BUTTON = sys::ImGuiTabBarFlags_NoCloseWithMiddleMouseButton as u32;
        const NO_TAB_LIST_SCROLLING_BUTTONS = sys::ImGuiTabBarFlags_NoTabListScrollingButtons as u32;
        const NO_TOOLTIP = sys::ImGuiTabBarFlags_NoTooltip as u32;
        const FITTING_POLICY_RESIZE_DOWN = sys::ImGuiTabBarFlags_FittingPolicyResizeDown as u32;
        const FITTING_POLICY_SCROLL = sys::ImGuiTabBarFlags_FittingPolicyScroll as u32;
        const FITTING_POLICY_MASK = sys::ImGuiTabBarFlags_FittingPolicyMask_ as u32;
        const FITTING_POLICY_DEFAULT = sys::ImGuiTabBarFlags_FittingPolicyDefault_ as u32;
    }
}

bitflags! {
    #[repr(transparent)]
    pub struct TabItemFlags: u32 {
        const UNSAVED_DOCUMENT = sys::ImGuiTabItemFlags_UnsavedDocument as u32;
        const SET_SELECTED = sys::ImGuiTabItemFlags_SetSelected as u32;
        const NO_CLOSE_WITH_MIDDLE_MOUSE_BUTTON = sys::ImGuiTabItemFlags_NoCloseWithMiddleMouseButton as u32;
        const NO_PUSH_ID = sys::ImGuiTabItemFlags_NoPushId as u32;
        const NO_TOOLTIP = sys::ImGuiTabItemFlags_NoTooltip as u32;
        const NO_REORDER = sys::ImGuiTabItemFlags_NoReorder as u32;
        const LEADING = sys::ImGuiTabItemFlags_Leading as u32;
        const TRAILING = sys::ImGuiTabItemFlags_Trailing as u32;
    }
}

/// Builder for a tab bar.
pub struct TabBar<'a> {
    id: &'a ImStr,
    flags: TabBarFlags,
}

impl<'a> TabBar<'a> {
    pub fn new(id: &'a ImStr) -> Self {
        Self {
            id,
            flags: TabBarFlags::empty(),
        }
    }

    /// Enable/Disable the reorderable property
    ///
    /// Disabled by default
    #[inline]
    pub fn reorderable(mut self, value: bool) -> Self {
        self.flags.set(TabBarFlags::REORDERABLE, value);
        self
    }

    /// Set the flags of the tab bar.
    ///
    /// Flags are empty by default
    #[inline]
    pub fn flags(mut self, flags: TabBarFlags) -> Self {
        self.flags = flags;
        self
    }

    #[must_use]
    pub fn begin(self, ui: &Ui) -> Option<TabBarToken> {
        let should_render =
            unsafe { sys::igBeginTabBar(self.id.as_ptr(), self.flags.bits() as i32) };

        if should_render {
            Some(TabBarToken { ctx: ui.ctx })
        } else {
            unsafe { sys::igEndTabBar() };
            None
        }
    }

    /// Creates a tab bar and runs a closure to construct the contents.
    ///
    /// Note: the closure is not called if no tabbar content is visible
    pub fn build<F: FnOnce()>(self, ui: &Ui, f: F) {
        if let Some(tab) = self.begin(ui) {
            f();
            tab.end(ui);
        }
    }
}

/// Tracks a window that must be ended by calling `.end()`
pub struct TabBarToken {
    ctx: *const Context,
}

impl TabBarToken {
    /// Ends a tab bar
    pub fn end(mut self, _: &Ui) {
        self.ctx = ptr::null();
        unsafe { sys::igEndTabBar() };
    }
}

impl Drop for TabBarToken {
    fn drop(&mut self) {
        if !self.ctx.is_null() && !thread::panicking() {
            panic!("A TabBarToken was leaked. Did you call .end()?");
        }
    }
}

pub struct TabItem<'a> {
    name: &'a ImStr,
    opened: Option<&'a mut bool>,
    flags: TabItemFlags,
}

impl<'a> TabItem<'a> {
    pub fn new(name: &'a ImStr) -> Self {
        Self {
            name,
            opened: None,
            flags: TabItemFlags::empty(),
        }
    }

    /// Will open or close the tab.\
    ///
    /// True to display the tab. Tab item is visible by default.
    #[inline]
    pub fn opened(mut self, opened: &'a mut bool) -> Self {
        self.opened = Some(opened);
        self
    }

    /// Set the flags of the tab item.
    ///
    /// Flags are empty by default
    #[inline]
    pub fn flags(mut self, flags: TabItemFlags) -> Self {
        self.flags = flags;
        self
    }

    #[must_use]
    pub fn begin(self, ui: &Ui) -> Option<TabItemToken> {
        let should_render = unsafe {
            sys::igBeginTabItem(
                self.name.as_ptr(),
                self.opened
                    .map(|x| x as *mut bool)
                    .unwrap_or(ptr::null_mut()),
                self.flags.bits() as i32,
            )
        };

        if should_render {
            Some(TabItemToken { ctx: ui.ctx })
        } else {
            None
        }
    }

    /// Creates a tab item and runs a closure to construct the contents.
    ///
    /// Note: the closure is not called if the tab item is not selected
    pub fn build<F: FnOnce()>(self, ui: &Ui, f: F) {
        if let Some(tab) = self.begin(ui) {
            f();
            tab.end(ui);
        }
    }
}

pub struct TabItemToken {
    ctx: *const Context,
}

impl TabItemToken {
    pub fn end(mut self, _: &Ui) {
        self.ctx = ptr::null();
        unsafe { sys::igEndTabItem() };
    }
}

impl Drop for TabItemToken {
    fn drop(&mut self) {
        if !self.ctx.is_null() && !thread::panicking() {
            panic!("A TabItemToken was leaked. Did you call .end()?");
        }
    }
}
