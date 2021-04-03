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
use crate::string::ImStr;
use crate::sys;
use crate::Ui;
use bitflags::bitflags;
use std::ptr;

bitflags! {
    #[repr(transparent)]
    pub struct TabBarFlags: u32 {
        const REORDERABLE = sys::ImGuiTabBarFlags_Reorderable;
        const AUTO_SELECT_NEW_TABS = sys::ImGuiTabBarFlags_AutoSelectNewTabs;
        const TAB_LIST_POPUP_BUTTON = sys::ImGuiTabBarFlags_TabListPopupButton;
        const NO_CLOSE_WITH_MIDDLE_MOUSE_BUTTON = sys::ImGuiTabBarFlags_NoCloseWithMiddleMouseButton;
        const NO_TAB_LIST_SCROLLING_BUTTONS = sys::ImGuiTabBarFlags_NoTabListScrollingButtons;
        const NO_TOOLTIP = sys::ImGuiTabBarFlags_NoTooltip;
        const FITTING_POLICY_RESIZE_DOWN = sys::ImGuiTabBarFlags_FittingPolicyResizeDown;
        const FITTING_POLICY_SCROLL = sys::ImGuiTabBarFlags_FittingPolicyScroll;
        const FITTING_POLICY_MASK = sys::ImGuiTabBarFlags_FittingPolicyMask_;
        const FITTING_POLICY_DEFAULT = sys::ImGuiTabBarFlags_FittingPolicyDefault_;
    }
}

bitflags! {
    #[repr(transparent)]
    pub struct TabItemFlags: u32 {
        const UNSAVED_DOCUMENT = sys::ImGuiTabItemFlags_UnsavedDocument;
        const SET_SELECTED = sys::ImGuiTabItemFlags_SetSelected;
        const NO_CLOSE_WITH_MIDDLE_MOUSE_BUTTON = sys::ImGuiTabItemFlags_NoCloseWithMiddleMouseButton;
        const NO_PUSH_ID = sys::ImGuiTabItemFlags_NoPushId;
        const NO_TOOLTIP = sys::ImGuiTabItemFlags_NoTooltip;
        const NO_REORDER = sys::ImGuiTabItemFlags_NoReorder;
        const LEADING = sys::ImGuiTabItemFlags_Leading;
        const TRAILING = sys::ImGuiTabItemFlags_Trailing;
    }
}

/// Builder for a tab bar.
pub struct TabBar<'a> {
    id: &'a ImStr,
    flags: TabBarFlags,
}

impl<'a> TabBar<'a> {
    #[inline]
    #[doc(alias = "BeginTabBar")]
    pub const fn new(id: &'a ImStr) -> Self {
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
    pub fn begin<'ui>(self, ui: &Ui<'ui>) -> Option<TabBarToken<'ui>> {
        let should_render =
            unsafe { sys::igBeginTabBar(self.id.as_ptr(), self.flags.bits() as i32) };

        if should_render {
            Some(TabBarToken::new(ui))
        } else {
            unsafe { sys::igEndTabBar() };
            None
        }
    }

    /// Creates a tab bar and runs a closure to construct the contents.
    /// Returns the result of the closure, if it is called.
    ///
    /// Note: the closure is not called if no tabbar content is visible
    pub fn build<T, F: FnOnce() -> T>(self, ui: &Ui<'_>, f: F) -> Option<T> {
        self.begin(ui).map(|_tab| f())
    }
}

create_token!(
    /// Tracks a window that can be ended by calling `.end()`
    /// or by dropping
    pub struct TabBarToken<'ui>;

    /// Ends a tab bar.
    drop { sys::igEndTabBar() }
);

pub struct TabItem<'a> {
    name: &'a ImStr,
    opened: Option<&'a mut bool>,
    flags: TabItemFlags,
}

impl<'a> TabItem<'a> {
    #[doc(alias = "BeginTabItem")]
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
    pub fn begin<'ui>(self, ui: &Ui<'ui>) -> Option<TabItemToken<'ui>> {
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
            Some(TabItemToken::new(ui))
        } else {
            None
        }
    }

    /// Creates a tab item and runs a closure to construct the contents.
    /// Returns the result of the closure, if it is called.
    ///
    /// Note: the closure is not called if the tab item is not selected
    pub fn build<T, F: FnOnce() -> T>(self, ui: &Ui<'_>, f: F) -> Option<T> {
        self.begin(ui).map(|_tab| f())
    }
}

create_token!(
    /// Tracks a tab bar item that can be ended by calling `.end()`
    /// or by dropping
    pub struct TabItemToken<'ui>;

    /// Ends a tab bar item.
    drop { sys::igEndTabItem() }
);
