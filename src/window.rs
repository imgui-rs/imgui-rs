use imgui_sys;
use std::marker::PhantomData;
use std::ptr;

use super::{ImGuiSetCond, ImGuiWindowFlags, ImGuiWindowFlags_AlwaysAutoResize,
            ImGuiWindowFlags_HorizontalScrollbar, ImGuiWindowFlags_MenuBar,
            ImGuiWindowFlags_NoBringToFrontOnFocus, ImGuiWindowFlags_NoCollapse,
            ImGuiWindowFlags_NoFocusOnAppearing, ImGuiWindowFlags_NoInputs,
            ImGuiWindowFlags_NoMove, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoSavedSettings,
            ImGuiWindowFlags_NoScrollWithMouse, ImGuiWindowFlags_NoScrollbar,
            ImGuiWindowFlags_NoTitleBar, ImGuiWindowFlags_ShowBorders, ImVec2, Ui};

#[must_use]
pub struct Window<'ui, 'p> {
    pos: (f32, f32),
    pos_cond: ImGuiSetCond,
    size: (f32, f32),
    size_cond: ImGuiSetCond,
    name: &'p str,
    opened: Option<&'p mut bool>,
    bg_alpha: f32,
    flags: ImGuiWindowFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> Window<'ui, 'p> {
    pub fn new(name: &'p str) -> Window<'ui, 'p> {
        Window {
            pos: (0.0, 0.0),
            pos_cond: ImGuiSetCond::empty(),
            size: (0.0, 0.0),
            size_cond: ImGuiSetCond::empty(),
            name: name,
            opened: None,
            bg_alpha: -1.0,
            flags: ImGuiWindowFlags::empty(),
            _phantom: PhantomData,
        }
    }
    #[inline]
    pub fn position(self, pos: (f32, f32), cond: ImGuiSetCond) -> Self {
        Window {
            pos: pos,
            pos_cond: cond,
            ..self
        }
    }
    #[inline]
    pub fn size(self, size: (f32, f32), cond: ImGuiSetCond) -> Self {
        Window {
            size: size,
            size_cond: cond,
            ..self
        }
    }
    #[inline]
    pub fn opened(self, opened: &'p mut bool) -> Self { Window { opened: Some(opened), ..self } }
    #[inline]
    pub fn bg_alpha(self, bg_alpha: f32) -> Self { Window { bg_alpha: bg_alpha, ..self } }
    #[inline]
    pub fn flags(self, flags: ImGuiWindowFlags) -> Self { Window { flags: flags, ..self } }
    #[inline]
    pub fn title_bar(self, value: bool) -> Self {
        Window { flags: self.flags.with(ImGuiWindowFlags_NoTitleBar, !value), ..self }
    }
    #[inline]
    pub fn resizable(self, value: bool) -> Self {
        Window { flags: self.flags.with(ImGuiWindowFlags_NoResize, !value), ..self }
    }
    #[inline]
    pub fn movable(self, value: bool) -> Self {
        Window { flags: self.flags.with(ImGuiWindowFlags_NoMove, !value), ..self }
    }
    #[inline]
    pub fn scroll_bar(self, value: bool) -> Self {
        Window { flags: self.flags.with(ImGuiWindowFlags_NoScrollbar, !value), ..self }
    }
    #[inline]
    pub fn scrollable(self, value: bool) -> Self {
        Window { flags: self.flags.with(ImGuiWindowFlags_NoScrollWithMouse, !value), ..self }
    }
    #[inline]
    pub fn collapsible(self, value: bool) -> Self {
        Window { flags: self.flags.with(ImGuiWindowFlags_NoCollapse, !value), ..self }
    }
    #[inline]
    pub fn always_auto_resize(self, value: bool) -> Self {
        Window { flags: self.flags.with(ImGuiWindowFlags_AlwaysAutoResize, value), ..self }
    }
    #[inline]
    pub fn show_borders(self, value: bool) -> Self {
        Window { flags: self.flags.with(ImGuiWindowFlags_ShowBorders, value), ..self }
    }
    #[inline]
    pub fn save_settings(self, value: bool) -> Self {
        Window { flags: self.flags.with(ImGuiWindowFlags_NoSavedSettings, !value), ..self }
    }
    #[inline]
    pub fn inputs(self, value: bool) -> Self {
        Window { flags: self.flags.with(ImGuiWindowFlags_NoInputs, !value), ..self }
    }
    #[inline]
    pub fn menu_bar(self, value: bool) -> Self {
        Window { flags: self.flags.with(ImGuiWindowFlags_MenuBar, value), ..self }
    }
    #[inline]
    pub fn horizontal_scrollbar(self, value: bool) -> Self {
        Window { flags: self.flags.with(ImGuiWindowFlags_HorizontalScrollbar, value), ..self }
    }
    #[inline]
    pub fn no_focus_on_appearing(self, value: bool) -> Self {
        Window { flags: self.flags.with(ImGuiWindowFlags_NoFocusOnAppearing, value), ..self }
    }
    #[inline]
    pub fn no_bring_to_front_on_focus(self, value: bool) -> Self {
        Window { flags: self.flags.with(ImGuiWindowFlags_NoBringToFrontOnFocus, value), ..self }
    }
    pub fn build<F: FnOnce()>(self, f: F) {
        let render = unsafe {
            if !self.pos_cond.is_empty() {
                imgui_sys::igSetNextWindowPos(self.pos.into(), self.pos_cond);
            }
            if !self.size_cond.is_empty() {
                imgui_sys::igSetNextWindowSize(self.size.into(), self.size_cond);
            }
            imgui_sys::igBegin2(imgui_sys::ImStr::from(self.name),
                                self.opened.map(|x| x as *mut bool).unwrap_or(ptr::null_mut()),
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
