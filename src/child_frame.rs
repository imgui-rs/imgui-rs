use std::mem::transmute;

use ImVec2;
use ImGuiWindowFlags;

use super::{ImGuiWindowFlags_NoTitleBar, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoMove,
            ImGuiWindowFlags_NoScrollbar, ImGuiWindowFlags_NoScrollWithMouse,
            ImGuiWindowFlags_NoCollapse, ImGuiWindowFlags_AlwaysAutoResize,
            ImGuiWindowFlags_ShowBorders, ImGuiWindowFlags_NoSavedSettings,
            ImGuiWindowFlags_NoInputs, ImGuiWindowFlags_MenuBar,
            ImGuiWindowFlags_HorizontalScrollbar, ImGuiWindowFlags_NoFocusOnAppearing,
            ImGuiWindowFlags_NoBringToFrontOnFocus, ImGuiWindowFlags_AlwaysVerticalScrollbar,
            ImGuiWindowFlags_AlwaysHorizontalScrollbar, ImGuiWindowFlags_AlwaysUseWindowPadding};

#[must_use]
pub struct ChildFrame {
    pub size: ImVec2,
    pub flags: ImGuiWindowFlags,
}

impl ChildFrame {
    pub fn new(size: ImVec2) -> ChildFrame {
        let empty_flag: ImGuiWindowFlags = unsafe { transmute::<i32, ImGuiWindowFlags>(0) };
        ChildFrame {
            size: size,
            flags: empty_flag,
        }
    }
    #[inline]
    pub fn show_title(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags_NoTitleBar, !value);
        self
    }
    #[inline]
    pub fn resizable(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags_NoResize, !value);
        self
    }
    #[inline]
    pub fn movable(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags_NoMove, !value);
        self
    }
    #[inline]
    pub fn show_scrollbar(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags_NoScrollbar, !value);
        self
    }
    #[inline]
    pub fn show_scrollbar_with_mouse(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags_NoScrollWithMouse, !value);
        self
    }
    #[inline]
    pub fn collapsible(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags_NoCollapse, !value);
        self
    }
    #[inline]
    pub fn always_resizable(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags_AlwaysAutoResize, value);
        self
    }
    #[inline]
    pub fn show_borders(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags_ShowBorders, value);
        self
    }
    #[inline]
    pub fn save_settings(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags_NoSavedSettings, !value);
        self
    }
    #[inline]
    pub fn input_allow(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags_NoInputs, !value);
        self
    }
    #[inline]
    pub fn show_menu(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags_MenuBar, value);
        self
    }
    #[inline]
    pub fn scrollbar_horizontal(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags_HorizontalScrollbar, value);
        self
    }
    #[inline]
    pub fn focus_on_appearing(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags_NoFocusOnAppearing, !value);
        self
    }
    #[inline]
    pub fn bring_to_front_on_focus(mut self, value: bool) -> Self {
        self.flags.set(
            ImGuiWindowFlags_NoBringToFrontOnFocus,
            !value,
        );
        self
    }
    #[inline]
    pub fn always_show_vertical_scroll_bar(mut self, value: bool) -> Self {
        self.flags.set(
            ImGuiWindowFlags_AlwaysVerticalScrollbar,
            value,
        );
        self
    }
    #[inline]
    pub fn always_show_horizontal_scroll_bar(mut self, value: bool) -> Self {
        self.flags.set(
            ImGuiWindowFlags_AlwaysHorizontalScrollbar,
            value,
        );
        self
    }
    #[inline]
    pub fn always_use_window_padding(mut self, value: bool) -> Self {
        self.flags.set(
            ImGuiWindowFlags_AlwaysUseWindowPadding,
            value,
        );
        self
    }
}