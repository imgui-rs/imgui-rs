use std::ptr;

use crate::string::ImStr;
use crate::sys;
use crate::Ui;

/// # Widgets: Menus
impl<'ui> Ui<'ui> {
    /// Creates and starts appending to a full-screen menu bar.
    ///
    /// Returns `Some(MainMenuBarToken)` if the menu bar is visible. After content has been
    /// rendered, the token must be ended by calling `.end()`.
    ///
    /// Returns `None` if the menu bar is not visible and no content should be rendered.
    #[must_use]
    #[doc(alias = "BeginMainMenuBar")]
    pub fn begin_main_menu_bar(&self) -> Option<MainMenuBarToken<'ui>> {
        if unsafe { sys::igBeginMainMenuBar() } {
            Some(MainMenuBarToken::new(self))
        } else {
            None
        }
    }
    /// Creates a full-screen main menu bar and runs a closure to construct the contents.
    ///
    /// Note: the closure is not called if the menu bar is not visible.
    #[doc(alias = "BeginMenuBar")]
    pub fn main_menu_bar<F: FnOnce()>(&self, f: F) {
        if let Some(_menu_bar) = self.begin_main_menu_bar() {
            f();
        }
    }
    /// Creates and starts appending to the menu bar of the current window.
    ///
    /// Returns `Some(MenuBarToken)` if the menu bar is visible. After content has been
    /// rendered, the token must be ended by calling `.end()`.
    ///
    /// Returns `None` if the menu bar is not visible and no content should be rendered.
    #[must_use]
    #[doc(alias = "BeginMenuBar")]
    pub fn begin_menu_bar(&self) -> Option<MenuBarToken<'_>> {
        if unsafe { sys::igBeginMenuBar() } {
            Some(MenuBarToken::new(self))
        } else {
            None
        }
    }
    /// Creates a menu bar in the current window and runs a closure to construct the contents.
    ///
    /// Note: the closure is not called if the menu bar is not visible.
    #[doc(alias = "BeginMenuBar")]
    pub fn menu_bar<F: FnOnce()>(&self, f: F) {
        if let Some(_menu_bar) = self.begin_menu_bar() {
            f();
        }
    }

    /// Creates and starts appending to a sub-menu entry.
    ///
    /// Returns `Some(MenuToken)` if the menu is visible. After content has been
    /// rendered, the token must be ended by calling `.end()`.
    ///
    /// Returns `None` if the menu is not visible and no content should be rendered.
    ///
    /// This is the equivalent of [begin_menu_with_enabled](Self::begin_menu_with_enabled)
    /// with `enabled` set to `true`.
    #[must_use]
    #[doc(alias = "BeginMenu")]
    pub fn begin_menu(&self, label: &ImStr) -> Option<MenuToken<'_>> {
        self.begin_menu_with_enabled(label, true)
    }

    /// Creates and starts appending to a sub-menu entry.
    ///
    /// Returns `Some(MenuToken)` if the menu is visible. After content has been
    /// rendered, the token must be ended by calling `.end()`.
    ///
    /// Returns `None` if the menu is not visible and no content should be rendered.
    #[must_use]
    #[doc(alias = "BeginMenu")]
    pub fn begin_menu_with_enabled(&self, label: &ImStr, enabled: bool) -> Option<MenuToken<'_>> {
        if unsafe { sys::igBeginMenu(label.as_ptr(), enabled) } {
            Some(MenuToken::new(self))
        } else {
            None
        }
    }
    /// Creates a menu and runs a closure to construct the contents.
    ///
    /// Note: the closure is not called if the menu is not visible.
    ///
    /// This is the equivalent of [menu_with_enabled](Self::menu_with_enabled)
    /// with `enabled` set to `true`.
    #[doc(alias = "BeginMenu")]
    pub fn menu<F: FnOnce()>(&self, label: &ImStr, f: F) {
        self.menu_with_enabled(label, true, f);
    }

    /// Creates a menu and runs a closure to construct the contents.
    ///
    /// Note: the closure is not called if the menu is not visible.
    #[doc(alias = "BeginMenu")]
    pub fn menu_with_enabled<F: FnOnce()>(&self, label: &ImStr, enabled: bool, f: F) {
        if let Some(_menu) = self.begin_menu_with_enabled(label, enabled) {
            f();
        }
    }
}

/// Builder for a menu item.
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct MenuItem<'a> {
    label: &'a ImStr,
    shortcut: Option<&'a ImStr>,
    selected: bool,
    enabled: bool,
}

impl<'a> MenuItem<'a> {
    /// Construct a new menu item builder.
    pub fn new(label: &ImStr) -> MenuItem {
        MenuItem {
            label,
            shortcut: None,
            selected: false,
            enabled: true,
        }
    }
    /// Sets the menu item shortcut.
    ///
    /// Shortcuts are displayed for convenience only and are not automatically handled.
    #[inline]
    pub fn shortcut(mut self, shortcut: &'a ImStr) -> Self {
        self.shortcut = Some(shortcut);
        self
    }
    /// Sets the selected state of the menu item.
    ///
    /// Default: false
    #[inline]
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
    /// Enables/disables the menu item.
    ///
    /// Default: enabled
    #[inline]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    /// Builds the menu item.
    ///
    /// Returns true if the menu item is activated.
    #[doc(alias = "MenuItemBool")]
    pub fn build(self, _: &Ui) -> bool {
        unsafe {
            sys::igMenuItem_Bool(
                self.label.as_ptr(),
                self.shortcut.map(ImStr::as_ptr).unwrap_or(ptr::null()),
                self.selected,
                self.enabled,
            )
        }
    }
}

/// # Convenience functions
impl<'a> MenuItem<'a> {
    /// Builds the menu item using a mutable reference to selected state.
    pub fn build_with_ref(self, ui: &Ui, selected: &mut bool) -> bool {
        if self.selected(*selected).build(ui) {
            *selected = !*selected;
            true
        } else {
            false
        }
    }
}

create_token!(
    /// Tracks a main menu bar that can be ended by calling `.end()`
    /// or by dropping
    pub struct MainMenuBarToken<'ui>;

    /// Ends a main menu bar
    drop { sys::igEndMainMenuBar() }
);

create_token!(
    /// Tracks a menu bar that can be ended by calling `.end()`
    /// or by dropping
    pub struct MenuBarToken<'ui>;

    /// Ends a menu bar
    drop { sys::igEndMenuBar() }
);

create_token!(
    /// Tracks a menu that can be ended by calling `.end()`
    /// or by dropping
    pub struct MenuToken<'ui>;

    /// Ends a menu
    drop { sys::igEndMenu() }
);
