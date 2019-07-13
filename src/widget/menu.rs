use std::marker::PhantomData;
use std::ptr;

use crate::string::ImStr;
use crate::sys;
use crate::Ui;

/// # Widgets: Menus
impl<'ui> Ui<'ui> {
    /// Creates and starts appending to a full-screen menu bar.
    ///
    /// Returns `None` if the menu bar is not visible and no content should be rendered.
    pub fn main_menu_bar<'a>(&'a self) -> Option<MainMenuBarToken<'a>> {
        match unsafe { sys::igBeginMainMenuBar() } {
            true => Some(MainMenuBarToken { _ui: PhantomData }),
            false => None,
        }
    }
    /// Creates and starts appending to the menu bar of the current window.
    ///
    /// Returns `None` if the menu bar is not visible and no content should be rendered.
    pub fn menu_bar<'a>(&'a self) -> Option<MenuBarToken<'a>> {
        match unsafe { sys::igBeginMenuBar() } {
            true => Some(MenuBarToken { _ui: PhantomData }),
            false => None,
        }
    }
    /// Creates and starts appending to a sub-menu entry.
    ///
    /// Returns `None` if the menu is not visible and no content should be rendered.
    pub fn menu<'a>(&'a self, label: &ImStr, enabled: bool) -> Option<MenuToken<'a>> {
        match unsafe { sys::igBeginMenu(label.as_ptr(), enabled) } {
            true => Some(MenuToken { _ui: PhantomData }),
            false => None,
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
            *selected = true;
            true
        } else {
            false
        }
    }
}

/// Represents a main menu bar
pub struct MainMenuBarToken<'ui> {
    _ui: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui> Drop for MainMenuBarToken<'ui> {
    fn drop(&mut self) {
        unsafe { sys::igEndMainMenuBar() };
    }
}

/// Represents a menu bar
pub struct MenuBarToken<'ui> {
    _ui: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui> Drop for MenuBarToken<'ui> {
    fn drop(&mut self) {
        unsafe { sys::igEndMenuBar() };
    }
}

/// Represents a menu
pub struct MenuToken<'ui> {
    _ui: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui> Drop for MenuToken<'ui> {
    fn drop(&mut self) {
        unsafe { sys::igEndMenu() };
    }
}
