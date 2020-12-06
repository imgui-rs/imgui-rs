use std::ptr;
use std::thread;

use crate::context::Context;
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
    pub fn begin_main_menu_bar(&self) -> Option<MainMenuBarToken> {
        if unsafe { sys::igBeginMainMenuBar() } {
            Some(MainMenuBarToken { ctx: self.ctx })
        } else {
            None
        }
    }
    /// Creates a full-screen main menu bar and runs a closure to construct the contents.
    ///
    /// Note: the closure is not called if the menu bar is not visible.
    pub fn main_menu_bar<F: FnOnce()>(&self, f: F) {
        if let Some(menu_bar) = self.begin_main_menu_bar() {
            f();
            menu_bar.end(self);
        }
    }
    /// Creates and starts appending to the menu bar of the current window.
    ///
    /// Returns `Some(MenuBarToken)` if the menu bar is visible. After content has been
    /// rendered, the token must be ended by calling `.end()`.
    ///
    /// Returns `None` if the menu bar is not visible and no content should be rendered.
    #[must_use]
    pub fn begin_menu_bar(&self) -> Option<MenuBarToken> {
        if unsafe { sys::igBeginMenuBar() } {
            Some(MenuBarToken { ctx: self.ctx })
        } else {
            None
        }
    }
    /// Creates a menu bar in the current window and runs a closure to construct the contents.
    ///
    /// Note: the closure is not called if the menu bar is not visible.
    pub fn menu_bar<F: FnOnce()>(&self, f: F) {
        if let Some(menu_bar) = self.begin_menu_bar() {
            f();
            menu_bar.end(self);
        }
    }
    /// Creates and starts appending to a sub-menu entry.
    ///
    /// Returns `Some(MenuToken)` if the menu is visible. After content has been
    /// rendered, the token must be ended by calling `.end()`.
    ///
    /// Returns `None` if the menu is not visible and no content should be rendered.
    #[must_use]
    pub fn begin_menu(&self, label: &ImStr, enabled: bool) -> Option<MenuToken> {
        if unsafe { sys::igBeginMenu(label.as_ptr(), enabled) } {
            Some(MenuToken { ctx: self.ctx })
        } else {
            None
        }
    }
    /// Creates a menu and runs a closure to construct the contents.
    ///
    /// Note: the closure is not called if the menu is not visible.
    pub fn menu<F: FnOnce()>(&self, label: &ImStr, enabled: bool, f: F) {
        if let Some(menu) = self.begin_menu(label, enabled) {
            f();
            menu.end(self);
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
    pub fn build(self, _: &Ui) -> bool {
        unsafe {
            sys::igMenuItemBool(
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

/// Tracks a main menu bar that must be ended by calling `.end()`
#[must_use]
pub struct MainMenuBarToken {
    ctx: *const Context,
}

impl MainMenuBarToken {
    /// Ends a main menu bar
    pub fn end(mut self, _: &Ui) {
        self.ctx = ptr::null();
        unsafe { sys::igEndMainMenuBar() };
    }
}

impl Drop for MainMenuBarToken {
    fn drop(&mut self) {
        if !self.ctx.is_null() && !thread::panicking() {
            panic!("A MainMenuBarToken was leaked. Did you call .end()?");
        }
    }
}

/// Tracks a menu bar that must be ended by calling `.end()`
#[must_use]
pub struct MenuBarToken {
    ctx: *const Context,
}

impl MenuBarToken {
    /// Ends a menu bar
    pub fn end(mut self, _: &Ui) {
        self.ctx = ptr::null();
        unsafe { sys::igEndMenuBar() };
    }
}

impl Drop for MenuBarToken {
    fn drop(&mut self) {
        if !self.ctx.is_null() && !thread::panicking() {
            panic!("A MenuBarToken was leaked. Did you call .end()?");
        }
    }
}

/// Tracks a menu that must be ended by calling `.end()`
#[must_use]
pub struct MenuToken {
    ctx: *const Context,
}

impl MenuToken {
    /// Ends a menu
    pub fn end(mut self, _: &Ui) {
        self.ctx = ptr::null();
        unsafe { sys::igEndMenu() };
    }
}

impl Drop for MenuToken {
    fn drop(&mut self) {
        if !self.ctx.is_null() && !thread::panicking() {
            panic!("A MenuToken was leaked. Did you call .end()?");
        }
    }
}
