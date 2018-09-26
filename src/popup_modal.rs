use std::marker::PhantomData;
use std::ptr;

use super::{ImGuiWindowFlags, ImStr, Ui};

use sys;

#[must_use]
pub struct PopupModal<'ui, 'p> {
    label: &'p ImStr,
    opened: Option<&'p mut bool>,
    flags: ImGuiWindowFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> PopupModal<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p ImStr) -> Self {
        PopupModal {
            label,
            opened: None,
            flags: ImGuiWindowFlags::empty(),
            _phantom: PhantomData,
        }
    }
    pub fn opened(mut self, opened: &'p mut bool) -> Self {
        self.opened = Some(opened);
        self
    }
    pub fn flags(mut self, flags: ImGuiWindowFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn title_bar(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoTitleBar, !value);
        self
    }
    pub fn resizable(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoResize, !value);
        self
    }
    pub fn movable(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoMove, !value);
        self
    }
    pub fn scroll_bar(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoScrollbar, !value);
        self
    }
    pub fn scrollable(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoScrollWithMouse, !value);
        self
    }
    pub fn collapsible(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoCollapse, !value);
        self
    }
    pub fn always_auto_resize(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::AlwaysAutoResize, value);
        self
    }
    pub fn save_settings(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoSavedSettings, !value);
        self
    }
    pub fn inputs(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoInputs, !value);
        self
    }
    pub fn menu_bar(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::MenuBar, value);
        self
    }
    pub fn horizontal_scrollbar(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::HorizontalScrollbar, value);
        self
    }
    pub fn no_focus_on_appearing(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoFocusOnAppearing, value);
        self
    }
    pub fn no_bring_to_front_on_focus(mut self, value: bool) -> Self {
        self.flags
            .set(ImGuiWindowFlags::NoBringToFrontOnFocus, value);
        self
    }
    pub fn always_vertical_scrollbar(mut self, value: bool) -> Self {
        self.flags
            .set(ImGuiWindowFlags::AlwaysVerticalScrollbar, value);
        self
    }
    pub fn always_horizontal_scrollbar(mut self, value: bool) -> Self {
        self.flags
            .set(ImGuiWindowFlags::AlwaysHorizontalScrollbar, value);
        self
    }
    pub fn always_use_window_padding(mut self, value: bool) -> Self {
        self.flags
            .set(ImGuiWindowFlags::AlwaysUseWindowPadding, value);
        self
    }
    pub fn build<F: FnOnce()>(self, f: F) {
        let render = unsafe {
            sys::igBeginPopupModal(
                self.label.as_ptr(),
                self.opened
                    .map(|x| x as *mut bool)
                    .unwrap_or(ptr::null_mut()),
                self.flags,
            )
        };
        if render {
            f();
            unsafe { sys::igEndMenu() };
        }
    }
}
