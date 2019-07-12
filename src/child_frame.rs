use std::marker::PhantomData;

use crate::sys;
use crate::window::WindowFlags;
use crate::{ImStr, Ui};

#[must_use]
pub struct ChildFrame<'ui, 'p> {
    name: &'p ImStr,
    size: [f32; 2],
    border: bool,
    flags: WindowFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> ChildFrame<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, name: &'p ImStr, size: [f32; 2]) -> ChildFrame<'ui, 'p> {
        ChildFrame {
            name,
            size: size.into(),
            border: false,
            flags: WindowFlags::empty(),
            _phantom: PhantomData,
        }
    }
    #[inline]
    pub fn movable(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::NO_MOVE, !value);
        self
    }
    #[inline]
    pub fn show_scrollbar(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::NO_SCROLLBAR, !value);
        self
    }
    #[inline]
    pub fn show_scrollbar_with_mouse(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::NO_SCROLL_WITH_MOUSE, !value);
        self
    }
    #[inline]
    pub fn collapsible(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::NO_COLLAPSE, !value);
        self
    }
    #[inline]
    pub fn always_resizable(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::ALWAYS_AUTO_RESIZE, value);
        self
    }
    #[inline]
    pub fn show_borders(mut self, value: bool) -> Self {
        self.border = value;
        self
    }
    #[inline]
    pub fn input_allow(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::NO_INPUTS, !value);
        self
    }
    #[inline]
    pub fn show_menu(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::MENU_BAR, value);
        self
    }
    #[inline]
    pub fn scrollbar_horizontal(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::HORIZONTAL_SCROLLBAR, value);
        self
    }
    #[inline]
    pub fn focus_on_appearing(mut self, value: bool) -> Self {
        self.flags.set(WindowFlags::NO_FOCUS_ON_APPEARING, !value);
        self
    }
    #[inline]
    pub fn bring_to_front_on_focus(mut self, value: bool) -> Self {
        self.flags
            .set(WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS, !value);
        self
    }
    #[inline]
    pub fn always_show_vertical_scroll_bar(mut self, value: bool) -> Self {
        self.flags
            .set(WindowFlags::ALWAYS_VERTICAL_SCROLLBAR, value);
        self
    }
    #[inline]
    pub fn always_show_horizontal_scroll_bar(mut self, value: bool) -> Self {
        self.flags
            .set(WindowFlags::ALWAYS_HORIZONTAL_SCROLLBAR, value);
        self
    }
    #[inline]
    pub fn always_use_window_padding(mut self, value: bool) -> Self {
        self.flags
            .set(WindowFlags::ALWAYS_USE_WINDOW_PADDING, value);
        self
    }
    pub fn build<F: FnOnce()>(self, f: F) {
        let render_child_frame = unsafe {
            sys::igBeginChild(
                self.name.as_ptr(),
                self.size.into(),
                self.border,
                self.flags.bits() as i32,
            )
        };
        if render_child_frame {
            f();
        }
        unsafe { sys::igEndChild() };
    }
}
