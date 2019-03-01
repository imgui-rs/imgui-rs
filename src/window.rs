use std::marker::PhantomData;
use std::ptr;
use sys;

use super::{ImGuiCond, ImGuiWindowFlags, ImStr, Ui};

#[must_use]
pub struct Window<'ui, 'p> {
    pos: (f32, f32),
    pos_cond: ImGuiCond,
    pos_pivot: (f32, f32),
    size: (f32, f32),
    size_cond: ImGuiCond,
    name: &'p ImStr,
    opened: Option<&'p mut bool>,
    flags: ImGuiWindowFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> Window<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, name: &'p ImStr) -> Window<'ui, 'p> {
        Window {
            pos: (0.0, 0.0),
            pos_cond: ImGuiCond::empty(),
            pos_pivot: (0.0, 0.0),
            size: (0.0, 0.0),
            size_cond: ImGuiCond::empty(),
            name,
            opened: None,
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
    pub fn position_pivot(mut self, pivot: (f32, f32)) -> Self {
        self.pos_pivot = pivot;
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
    pub fn flags(mut self, flags: ImGuiWindowFlags) -> Self {
        self.flags = flags;
        self
    }
    #[inline]
    pub fn title_bar(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoTitleBar, !value);
        self
    }
    #[inline]
    pub fn resizable(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoResize, !value);
        self
    }
    #[inline]
    pub fn movable(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoMove, !value);
        self
    }
    #[inline]
    pub fn scroll_bar(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoScrollbar, !value);
        self
    }
    #[inline]
    pub fn scrollable(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoScrollWithMouse, !value);
        self
    }
    #[inline]
    pub fn collapsible(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoCollapse, !value);
        self
    }
    #[inline]
    pub fn always_auto_resize(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::AlwaysAutoResize, value);
        self
    }
    #[inline]
    pub fn save_settings(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoSavedSettings, !value);
        self
    }
    #[inline]
    pub fn inputs(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoInputs, !value);
        self
    }
    #[inline]
    pub fn menu_bar(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::MenuBar, value);
        self
    }
    #[inline]
    pub fn horizontal_scrollbar(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::HorizontalScrollbar, value);
        self
    }
    #[inline]
    pub fn no_focus_on_appearing(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoFocusOnAppearing, value);
        self
    }
    #[inline]
    pub fn no_bring_to_front_on_focus(mut self, value: bool) -> Self {
        self.flags
            .set(ImGuiWindowFlags::NoBringToFrontOnFocus, value);
        self
    }
    #[inline]
    pub fn always_vertical_scrollbar(mut self, value: bool) -> Self {
        self.flags
            .set(ImGuiWindowFlags::AlwaysVerticalScrollbar, value);
        self
    }
    #[inline]
    pub fn always_horizontal_scrollbar(mut self, value: bool) -> Self {
        self.flags
            .set(ImGuiWindowFlags::AlwaysHorizontalScrollbar, value);
        self
    }
    #[inline]
    pub fn always_use_window_padding(mut self, value: bool) -> Self {
        self.flags
            .set(ImGuiWindowFlags::AlwaysUseWindowPadding, value);
        self
    }
    pub fn build<F: FnOnce()>(self, f: F) {
        let render = unsafe {
            if !self.pos_cond.is_empty() {
                sys::igSetNextWindowPos(self.pos.into(), self.pos_cond, self.pos_pivot.into());
            }
            if !self.size_cond.is_empty() {
                sys::igSetNextWindowSize(self.size.into(), self.size_cond);
            }
            sys::igBegin(
                self.name.as_ptr(),
                self.opened
                    .map(|x| x as *mut bool)
                    .unwrap_or(ptr::null_mut()),
                self.flags,
            )
        };
        if render {
            f();
        }
        unsafe {
            sys::igEnd();
        };
    }
}
