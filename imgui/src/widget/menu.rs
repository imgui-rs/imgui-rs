// use crate::string::ImStr;
use crate::sys;
use crate::Ui;

/// # Widgets: Menus
impl Ui {
    /// Creates and starts appending to a full-screen menu bar.
    ///
    /// Returns `Some(MainMenuBarToken)` if the menu bar is visible. After content has been
    /// rendered, the token must be ended by calling `.end()`.
    ///
    /// Returns `None` if the menu bar is not visible and no content should be rendered.
    #[must_use]
    #[doc(alias = "BeginMainMenuBar")]
    pub fn begin_main_menu_bar(&self) -> Option<MainMenuBarToken<'_>> {
        if unsafe { sys::ImGui_BeginMainMenuBar() } {
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
        if unsafe { sys::ImGui_BeginMenuBar() } {
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
    pub fn begin_menu(&self, label: impl AsRef<str>) -> Option<MenuToken<'_>> {
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
    pub fn begin_menu_with_enabled(
        &self,
        label: impl AsRef<str>,
        enabled: bool,
    ) -> Option<MenuToken<'_>> {
        if unsafe { sys::ImGui_BeginMenu(self.scratch_txt(label), enabled) } {
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
    pub fn menu<F: FnOnce()>(&self, label: impl AsRef<str>, f: F) {
        self.menu_with_enabled(label, true, f);
    }

    /// Creates a menu and runs a closure to construct the contents.
    ///
    /// Note: the closure is not called if the menu is not visible.
    #[doc(alias = "BeginMenu")]
    pub fn menu_with_enabled<F: FnOnce()>(&self, label: impl AsRef<str>, enabled: bool, f: F) {
        if let Some(_menu) = self.begin_menu_with_enabled(label, enabled) {
            f();
        }
    }

    /// Creates a menu item with the given label, returning `true` if it was pressed.
    ///
    /// If you want to configure this `menu_item` by setting `selection`, or `enablement`,
    /// use [`menu_item_config`].
    ///
    /// Note: a `menu_item` is the actual button/selectable within a Menu.
    ///
    /// [`menu_item_config`]: Self::menu_item_config
    #[doc(alias = "MenuItem")]
    pub fn menu_item(&self, label: impl AsRef<str>) -> bool {
        self.menu_item_config(label).build()
    }

    /// Creates a menu item builder, with further methods on it as needed. Use [`menu_item`]
    /// for simple Menu Items with no features on them.
    ///
    /// Note: a `menu_item` is the actual button/selectable within a Menu.
    ///
    /// [`menu_item`]: Self::menu_item
    #[doc(alias = "MenuItem")]
    pub fn menu_item_config<L: AsRef<str>>(&self, label: L) -> MenuItem<'_, L> {
        MenuItem {
            label,
            shortcut: None,
            selected: false,
            enabled: true,
            ui: self,
        }
    }
}

/// Builder for a menu item.
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct MenuItem<'ui, Label, Shortcut = &'static str> {
    label: Label,
    shortcut: Option<Shortcut>,
    selected: bool,
    enabled: bool,
    ui: &'ui Ui,
}

impl<'ui, Label: AsRef<str>> MenuItem<'ui, Label> {
    /// Construct a new menu item builder.
    #[deprecated(since = "0.9.0", note = "Use `ui.menu_item` or `ui.menu_item_config`")]
    pub fn new(label: Label, ui: &'ui Ui) -> Self {
        MenuItem {
            label,
            shortcut: None,
            selected: false,
            enabled: true,
            ui,
        }
    }
}

impl<'ui, Label: AsRef<str>, Shortcut: AsRef<str>> MenuItem<'ui, Label, Shortcut> {
    /// Sets the menu item shortcut.
    ///
    /// Shortcuts are displayed for convenience only and are not automatically handled.
    #[inline]
    pub fn shortcut<Shortcut2: AsRef<str>>(
        self,
        shortcut: Shortcut2,
    ) -> MenuItem<'ui, Label, Shortcut2> {
        MenuItem {
            label: self.label,
            shortcut: Some(shortcut),
            selected: self.selected,
            enabled: self.enabled,
            ui: self.ui,
        }
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
    pub fn build(self) -> bool {
        unsafe {
            let (label, shortcut) = self.ui.scratch_txt_with_opt(self.label, self.shortcut);
            sys::ImGui_MenuItem_Bool(label, shortcut, self.selected, self.enabled)
        }
    }

    #[doc(alias = "MenuItemBool")]
    /// Builds the menu item using a mutable reference to selected state.
    pub fn build_with_ref(self, selected: &mut bool) -> bool {
        if self.selected(*selected).build() {
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
    drop { sys::ImGui_EndMainMenuBar() }
);

create_token!(
    /// Tracks a menu bar that can be ended by calling `.end()`
    /// or by dropping
    pub struct MenuBarToken<'ui>;

    /// Ends a menu bar
    drop { sys::ImGui_EndMenuBar() }
);

create_token!(
    /// Tracks a menu that can be ended by calling `.end()`
    /// or by dropping
    pub struct MenuToken<'ui>;

    /// Ends a menu
    drop { sys::ImGui_EndMenu() }
);
