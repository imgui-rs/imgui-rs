use imgui_sys;
use std::marker::PhantomData;
use std::ptr;

use super::{ImGuiCond, ImGuiWindowFlags, ImGuiWindowFlags_AlwaysAutoResize,
            ImGuiWindowFlags_AlwaysHorizontalScrollbar, ImGuiWindowFlags_AlwaysUseWindowPadding,
            ImGuiWindowFlags_AlwaysVerticalScrollbar, ImGuiWindowFlags_HorizontalScrollbar,
            ImGuiWindowFlags_MenuBar, ImGuiWindowFlags_NoBringToFrontOnFocus,
            ImGuiWindowFlags_NoCollapse, ImGuiWindowFlags_NoFocusOnAppearing,
            ImGuiWindowFlags_NoInputs, ImGuiWindowFlags_NoMove, ImGuiWindowFlags_NoResize,
            ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_NoScrollWithMouse,
            ImGuiWindowFlags_NoScrollbar, ImGuiWindowFlags_NoTitleBar,
            ImGuiWindowFlags_ShowBorders, ImStr, ImVec2, Ui};

#[must_use]
pub struct Window<'ui, 'p> {
    pos: (f32, f32),
    pos_cond: ImGuiCond,
    size: (f32, f32),
    size_cond: ImGuiCond,
    name: &'p ImStr,
    opened: Option<&'p mut bool>,
    bg_alpha: f32,
    flags: ImGuiWindowFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> Window<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, name: &'p ImStr) -> Window<'ui, 'p> {
        Window {
            pos: (0.0, 0.0),
            pos_cond: ImGuiCond::empty(),
            size: (0.0, 0.0),
            size_cond: ImGuiCond::empty(),
            name: name,
            opened: None,
            bg_alpha: -1.0,
            flags: ImGuiWindowFlags::empty(),
            _phantom: PhantomData,
        }
    }
    #[inline]
    pub fn position(mut self, pos: (f32, f32), cond: ImGuiCond) -> Self {
        self.pos = pos;
        self.pos_cond = cond;
        self
    }
    #[inline]
    pub fn size(mut self, size: (f32, f32), cond: ImGuiCond) -> Self {
        self.size = size;
        self.size_cond = cond;
        self
    }
    #[inline]
    pub fn opened(mut self, opened: &'p mut bool) -> Self {
        self.opened = Some(opened);
        self
    }
    #[inline]
    pub fn bg_alpha(mut self, bg_alpha: f32) -> Self {
        self.bg_alpha = bg_alpha;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: ImGuiWindowFlags) -> Self {
        self.flags = flags;
        self
    }
    #[inline]
    pub fn title_bar(mut self, value: bool) -> Self {
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
    pub fn scroll_bar(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags_NoScrollbar, !value);
        self
    }
    #[inline]
    pub fn scrollable(mut self, value: bool) -> Self {
        self.flags
            .set(ImGuiWindowFlags_NoScrollWithMouse, !value);
        self
    }
    #[inline]
    pub fn collapsible(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags_NoCollapse, !value);
        self
    }
    #[inline]
    pub fn always_auto_resize(mut self, value: bool) -> Self {
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
    pub fn inputs(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags_NoInputs, !value);
        self
    }
    #[inline]
    pub fn menu_bar(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags_MenuBar, value);
        self
    }
    #[inline]
    pub fn horizontal_scrollbar(mut self, value: bool) -> Self {
        self.flags
            .set(ImGuiWindowFlags_HorizontalScrollbar, value);
        self
    }
    #[inline]
    pub fn no_focus_on_appearing(mut self, value: bool) -> Self {
        self.flags
            .set(ImGuiWindowFlags_NoFocusOnAppearing, value);
        self
    }
    #[inline]
    pub fn no_bring_to_front_on_focus(mut self, value: bool) -> Self {
        self.flags
            .set(ImGuiWindowFlags_NoBringToFrontOnFocus, value);
        self
    }
    #[inline]
    pub fn always_vertical_scrollbar(mut self, value: bool) -> Self {
        self.flags
            .set(ImGuiWindowFlags_AlwaysVerticalScrollbar, value);
        self
    }
    #[inline]
    pub fn always_horizontal_scrollbar(mut self, value: bool) -> Self {
        self.flags
            .set(ImGuiWindowFlags_AlwaysHorizontalScrollbar, value);
        self
    }
    #[inline]
    pub fn always_use_window_padding(mut self, value: bool) -> Self {
        self.flags
            .set(ImGuiWindowFlags_AlwaysUseWindowPadding, value);
        self
    }
    pub fn build<F: FnOnce()>(self, f: F) {
        let render = unsafe {
            if !self.pos_cond.is_empty() {
                imgui_sys::igSetNextWindowPos(self.pos.into(), self.pos_cond);
            }
            if !self.size_cond.is_empty() {
                imgui_sys::igSetNextWindowSize(self.size.into(), self.size_cond);
            }
            imgui_sys::igBegin2(self.name.as_ptr(),
                                self.opened
                                    .map(|x| x as *mut bool)
                                    .unwrap_or(ptr::null_mut()),
                                ImVec2::new(0.0, 0.0),
                                self.bg_alpha,
                                self.flags)
        };
        if render {
            f();
        }
        unsafe { imgui_sys::igEnd() };
    }
}
