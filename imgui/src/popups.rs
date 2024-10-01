use std::ptr;

use crate::sys;
use crate::window::WindowFlags;
use crate::Ui;

/// Create a modal pop-up.
///
/// # Example
/// ```rust,no_run
/// # use imgui::*;
/// # let mut imgui = Context::create();
/// # let ui = imgui.frame();
/// if ui.button("Show modal") {
///     ui.open_popup("modal");
/// }
/// if let Some(_token) = ui.begin_modal_popup("modal") {
///     ui.text("Content of my modal");
///     if ui.button("OK") {
///         ui.close_current_popup();
///     }
/// };
/// ```
#[must_use]
pub struct PopupModal<'ui, 'p, Label> {
    ui: &'ui Ui,
    label: Label,
    opened: Option<&'p mut bool>,
    flags: WindowFlags,
}

impl<'ui, 'p, Label: AsRef<str>> PopupModal<'ui, 'p, Label> {
    #[deprecated(since = "0.9.0", note = "Use `ui.modal_popup_config(...)` instead")]
    pub fn new(ui: &'ui Ui, label: Label) -> Self {
        PopupModal {
            ui,
            label,
            opened: None,
            flags: WindowFlags::empty(),
        }
    }
    /// Pass a mutable boolean which will be updated to refer to the current
    /// "open" state of the modal.
    pub fn opened(mut self, opened: &'p mut bool) -> Self {
        self.opened = Some(opened);
        self
    }
    pub fn flags(mut self, flags: WindowFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn title_bar(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::NO_TITLE_BAR, !value);
        self
    }
    pub fn resizable(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::NO_RESIZE, !value);
        self
    }
    pub fn movable(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::NO_MOVE, !value);
        self
    }
    pub fn scroll_bar(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::NO_SCROLLBAR, !value);
        self
    }
    pub fn scrollable(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::NO_SCROLL_WITH_MOUSE, !value);
        self
    }
    pub fn collapsible(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::NO_COLLAPSE, !value);
        self
    }
    pub fn always_auto_resize(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::ALWAYS_AUTO_RESIZE, value);
        self
    }
    pub fn save_settings(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::NO_SAVED_SETTINGS, !value);
        self
    }
    pub fn inputs(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::NO_INPUTS, !value);
        self
    }
    pub fn menu_bar(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::MENU_BAR, value);
        self
    }
    pub fn horizontal_scrollbar(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::HORIZONTAL_SCROLLBAR, value);
        self
    }
    pub fn no_focus_on_appearing(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::NO_FOCUS_ON_APPEARING, value);
        self
    }
    pub fn no_bring_to_front_on_focus(mut self, value: bool) -> Self {
        self.flags
            .set(WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS, value);
        self
    }
    pub fn always_vertical_scrollbar(mut self, value: bool) -> Self {
        self.flags
            .set(WindowFlags::ALWAYS_VERTICAL_SCROLLBAR, value);
        self
    }
    pub fn always_horizontal_scrollbar(mut self, value: bool) -> Self {
        self.flags
            .set(WindowFlags::ALWAYS_HORIZONTAL_SCROLLBAR, value);
        self
    }

    /// Consume and draw the PopupModal.
    /// Returns the result of the closure, if it is called.
    #[doc(alias = "BeginPopupModal")]
    pub fn build<T, F: FnOnce() -> T>(self, f: F) -> Option<T> {
        self.begin_popup().map(|_popup| f())
    }

    /// Consume and draw the PopupModal.
    /// Construct a popup that can have any kind of content.
    ///
    /// This should be called *per frame*, whereas [`Ui::open_popup`]
    /// should be called *once* when you want to actual create the popup.
    #[doc(alias = "BeginPopupModal")]
    pub fn begin_popup(self) -> Option<PopupToken<'ui>> {
        let render = unsafe {
            sys::igBeginPopupModal(
                self.ui.scratch_txt(self.label),
                self.opened
                    .map(|x| x as *mut bool)
                    .unwrap_or(ptr::null_mut()),
                self.flags.bits() as i32,
            )
        };

        if render {
            Some(PopupToken::new(self.ui))
        } else {
            None
        }
    }
}

// Widgets: Popups
impl Ui {
    /// Instructs ImGui that a popup is open.
    ///
    /// You should **call this function once** while calling any of the following per-frame:
    ///
    /// - [`begin_popup`](Self::begin_popup)
    /// - [`popup`](Self::popup)
    /// - [`modal_popup`](Self::modal_popup)
    /// - [`modal_popup_config`](Self::modal_popup_config)
    ///
    /// The confusing aspect to popups is that ImGui holds control over the popup itself.
    #[doc(alias = "OpenPopup")]
    pub fn open_popup(&self, str_id: impl AsRef<str>) {
        unsafe { sys::igOpenPopup_Str(self.scratch_txt(str_id), 0) };
    }

    /// Construct a popup that can have any kind of content.
    ///
    /// This should be called *per frame*, whereas [`open_popup`](Self::open_popup) should be called *once*
    /// to signal that this popup is active.
    #[doc(alias = "BeginPopup")]
    pub fn begin_popup(&self, str_id: impl AsRef<str>) -> Option<PopupToken<'_>> {
        let render = unsafe {
            sys::igBeginPopup(self.scratch_txt(str_id), WindowFlags::empty().bits() as i32)
        };

        if render {
            Some(PopupToken::new(self))
        } else {
            None
        }
    }

    /// Construct a popup that can have any kind of content.
    ///
    /// This should be called *per frame*, whereas [`open_popup`](Self::open_popup) should be called *once*
    /// to signal that this popup is active.
    #[doc(alias = "BeginPopup")]
    pub fn popup<F>(&self, str_id: impl AsRef<str>, f: F)
    where
        F: FnOnce(),
    {
        if let Some(_t) = self.begin_popup(str_id) {
            f();
        }
    }

    /// Creates a [PopupModal], and runs a closure on it.
    ///
    /// To customize the behavior of this [PopupModal], use [`modal_popup_config`](Self::modal_popup_config).
    pub fn modal_popup<Label, Func, R>(&self, str_id: Label, f: Func) -> Option<R>
    where
        Label: AsRef<str>,
        Func: FnOnce() -> R,
    {
        PopupModal {
            ui: self,
            label: str_id,
            opened: None,
            flags: WindowFlags::empty(),
        }
        .build(f)
    }

    /// Creates a [PopupModal], returning a drop token.
    ///
    /// To customize the behavior of this [PopupModal], use [`modal_popup_config`](Self::modal_popup_config).
    pub fn begin_modal_popup<Label: AsRef<str>>(&self, str_id: Label) -> Option<PopupToken<'_>> {
        PopupModal {
            ui: self,
            label: str_id,
            opened: None,
            flags: WindowFlags::empty(),
        }
        .begin_popup()
    }

    /// Creates a [PopupModal] builder.
    pub fn modal_popup_config<Label: AsRef<str>>(
        &self,
        str_id: Label,
    ) -> PopupModal<'_, '_, Label> {
        PopupModal {
            ui: self,
            label: str_id,
            opened: None,
            flags: WindowFlags::empty(),
        }
    }

    /// Close a popup. Should be called within the closure given as argument to
    /// [`Ui::popup`] or [`Ui::modal_popup`].
    #[doc(alias = "CloseCurrentPopup")]
    pub fn close_current_popup(&self) {
        unsafe { sys::igCloseCurrentPopup() };
    }

    /// Open and begin popup when clicked with the right mouse button on last item.
    ///
    /// This does not take a label, which means that multiple calls **in a row** will use the same label, which
    /// is based on the last node which had a label. Text and other non-interactive elements generally don't have
    /// ids, so you'll need to use [begin_popup_context_with_label](Self::begin_popup_context_with_label) for them.
    #[doc(alias = "BeginPopupContextItem")]
    pub fn begin_popup_context_item(&self) -> Option<PopupToken<'_>> {
        let render = unsafe {
            sys::igBeginPopupContextItem(
                std::ptr::null(),
                imgui_sys::ImGuiPopupFlags_MouseButtonRight as i32,
            )
        };

        if render {
            Some(PopupToken::new(self))
        } else {
            None
        }
    }

    /// Open and begin popup when clicked with the right mouse button on the given item with a dedicated label.
    ///
    /// If you want to use the label of the previous popup (outside of `Text` and other non-interactive cases, that
    /// is the more normal case), use [begin_popup_context_item](Self::begin_popup_context_item).
    #[doc(alias = "BeginPopupContextItem")]
    pub fn begin_popup_context_with_label<Label: AsRef<str>>(
        &self,
        str_id: Label,
    ) -> Option<PopupToken<'_>> {
        let render = unsafe {
            sys::igBeginPopupContextItem(
                self.scratch_txt(str_id),
                imgui_sys::ImGuiPopupFlags_MouseButtonRight as i32,
            )
        };

        if render {
            Some(PopupToken::new(self))
        } else {
            None
        }
    }

    /// Open and begin popup when clicked on current window.
    ///
    /// This does not take a label, which means that multiple calls will use the same provided label.
    /// If you want an explicit label, such as having two different kinds of windows popups in your program,
    /// use [begin_popup_context_window_with_label](Self::begin_popup_context_window_with_label).
    #[doc(alias = "BeginPopupContextWindow")]
    pub fn begin_popup_context_window(&self) -> Option<PopupToken<'_>> {
        let render = unsafe {
            sys::igBeginPopupContextWindow(
                std::ptr::null(),
                imgui_sys::ImGuiPopupFlags_MouseButtonRight as i32,
            )
        };

        if render {
            Some(PopupToken::new(self))
        } else {
            None
        }
    }

    /// Open and begin popup when clicked on current window.
    ///
    /// This takes a label explicitly. This is useful when multiple code
    /// locations may want to manipulate/open the same popup, given an explicit id.
    #[doc(alias = "BeginPopupContextWindow")]
    pub fn begin_popup_context_window_with_label<Label: AsRef<str>>(
        &self,
        str_id: Label,
    ) -> Option<PopupToken<'_>> {
        let render = unsafe {
            sys::igBeginPopupContextWindow(
                self.scratch_txt(str_id),
                imgui_sys::ImGuiPopupFlags_MouseButtonRight as i32,
            )
        };

        if render {
            Some(PopupToken::new(self))
        } else {
            None
        }
    }

    /// Open and begin popup when right clicked in void (where there are no windows).
    ///
    /// This does not take a label, which means that multiple calls will use the same provided label.
    /// If you want an explicit label, such as having two different kinds of void popups in your program,
    /// use [begin_popup_context_void_with_label](Self::begin_popup_context_void_with_label).
    #[doc(alias = "BeginPopupContextWindow")]
    pub fn begin_popup_context_void(&self) -> Option<PopupToken<'_>> {
        let render = unsafe {
            sys::igBeginPopupContextVoid(
                std::ptr::null(),
                imgui_sys::ImGuiPopupFlags_MouseButtonRight as i32,
            )
        };

        if render {
            Some(PopupToken::new(self))
        } else {
            None
        }
    }

    /// Open and begin popup when right clicked in void (where there are no windows).
    ///
    /// This takes a label explicitly. This is useful when multiple code
    /// locations may want to manipulate/open the same popup, given an explicit id.
    #[doc(alias = "BeginPopupContextVoid")]
    pub fn begin_popup_context_void_with_label<Label: AsRef<str>>(
        &self,
        str_id: Label,
    ) -> Option<PopupToken<'_>> {
        let render = unsafe {
            sys::igBeginPopupContextVoid(
                self.scratch_txt(str_id),
                imgui_sys::ImGuiPopupFlags_MouseButtonRight as i32,
            )
        };

        if render {
            Some(PopupToken::new(self))
        } else {
            None
        }
    }
}

create_token!(
    /// Tracks a popup token that can be ended with `end` or by dropping.
    pub struct PopupToken<'ui>;

    /// Drops the popup token manually. You can also just allow this token
    /// to drop on its own.
    drop { sys::igEndPopup() }
);
