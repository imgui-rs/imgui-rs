use crate::fonts::atlas::FontId;
use crate::internal::RawCast;
use crate::style::{StyleColor, StyleVar};
use crate::sys;
use crate::Ui;

impl<'ui> Ui<'ui> {
    pub fn with_font<T, F>(&self, id: FontId, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let fonts = self.fonts();
        let font = fonts
            .get_font(id)
            .expect("Font atlas did not contain the given font");
        unsafe { sys::igPushFont(font.raw() as *const _ as *mut _) };
        let result = f();
        unsafe { sys::igPopFont() };
        result
    }
    pub fn with_style_color<T, F>(&self, style_color: StyleColor, color: [f32; 4], f: F) -> T
    where
        F: FnOnce() -> T,
    {
        unsafe { sys::igPushStyleColor(style_color as i32, color.into()) };
        let result = f();
        unsafe { sys::igPopStyleColor(1) };
        result
    }
    pub fn with_style_colors<T, F>(&self, style_colors: &[(StyleColor, [f32; 4])], f: F) -> T
    where
        F: FnOnce() -> T,
    {
        for &(style_color, color) in style_colors {
            unsafe { sys::igPushStyleColor(style_color as i32, color.into()) };
        }
        let result = f();
        unsafe { sys::igPopStyleColor(style_colors.len() as i32) };
        result
    }
    pub fn with_style_var<T, F>(&self, style_var: StyleVar, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        self.push_style_var(style_var);
        let result = f();
        unsafe { sys::igPopStyleVar(1) };
        result
    }

    pub fn with_style_vars<T, F>(&self, style_vars: &[StyleVar], f: F)
    where
        F: FnOnce() -> T,
    {
        for &style_var in style_vars {
            self.push_style_var(style_var);
        }
        f();
        unsafe { sys::igPopStyleVar(style_vars.len() as i32) };
    }

    #[inline]
    fn push_style_var(&self, style_var: StyleVar) {
        use crate::style::StyleVar::*;
        use crate::sys::{igPushStyleVarFloat, igPushStyleVarVec2};
        match style_var {
            Alpha(v) => unsafe { igPushStyleVarFloat(sys::ImGuiStyleVar_Alpha as i32, v) },
            WindowPadding(v) => unsafe {
                igPushStyleVarVec2(sys::ImGuiStyleVar_WindowPadding as i32, v.into())
            },
            WindowRounding(v) => unsafe {
                igPushStyleVarFloat(sys::ImGuiStyleVar_WindowRounding as i32, v)
            },
            WindowBorderSize(v) => unsafe {
                igPushStyleVarFloat(sys::ImGuiStyleVar_WindowBorderSize as i32, v)
            },
            WindowMinSize(v) => unsafe {
                igPushStyleVarVec2(sys::ImGuiStyleVar_WindowMinSize as i32, v.into())
            },
            WindowTitleAlign(v) => unsafe {
                igPushStyleVarVec2(sys::ImGuiStyleVar_WindowTitleAlign as i32, v.into())
            },
            ChildRounding(v) => unsafe {
                igPushStyleVarFloat(sys::ImGuiStyleVar_ChildRounding as i32, v)
            },
            ChildBorderSize(v) => unsafe {
                igPushStyleVarFloat(sys::ImGuiStyleVar_ChildBorderSize as i32, v)
            },
            PopupRounding(v) => unsafe {
                igPushStyleVarFloat(sys::ImGuiStyleVar_PopupRounding as i32, v)
            },
            PopupBorderSize(v) => unsafe {
                igPushStyleVarFloat(sys::ImGuiStyleVar_PopupBorderSize as i32, v)
            },
            FramePadding(v) => unsafe {
                igPushStyleVarVec2(sys::ImGuiStyleVar_FramePadding as i32, v.into())
            },
            FrameRounding(v) => unsafe {
                igPushStyleVarFloat(sys::ImGuiStyleVar_FrameRounding as i32, v)
            },
            FrameBorderSize(v) => unsafe {
                igPushStyleVarFloat(sys::ImGuiStyleVar_FrameBorderSize as i32, v)
            },
            ItemSpacing(v) => unsafe {
                igPushStyleVarVec2(sys::ImGuiStyleVar_ItemSpacing as i32, v.into())
            },
            ItemInnerSpacing(v) => unsafe {
                igPushStyleVarVec2(sys::ImGuiStyleVar_ItemInnerSpacing as i32, v.into())
            },
            IndentSpacing(v) => unsafe {
                igPushStyleVarFloat(sys::ImGuiStyleVar_IndentSpacing as i32, v)
            },
            ScrollbarSize(v) => unsafe {
                igPushStyleVarFloat(sys::ImGuiStyleVar_ScrollbarSize as i32, v)
            },
            ScrollbarRounding(v) => unsafe {
                igPushStyleVarFloat(sys::ImGuiStyleVar_ScrollbarRounding as i32, v)
            },
            GrabMinSize(v) => unsafe {
                igPushStyleVarFloat(sys::ImGuiStyleVar_GrabMinSize as i32, v)
            },
            GrabRounding(v) => unsafe {
                igPushStyleVarFloat(sys::ImGuiStyleVar_GrabRounding as i32, v)
            },
            TabRounding(v) => unsafe {
                igPushStyleVarFloat(sys::ImGuiStyleVar_TabRounding as i32, v)
            },
            ButtonTextAlign(v) => unsafe {
                igPushStyleVarVec2(sys::ImGuiStyleVar_ButtonTextAlign as i32, v.into())
            },
            SelectableTextAlign(v) => unsafe {
                igPushStyleVarVec2(sys::ImGuiStyleVar_SelectableTextAlign as i32, v.into())
            },
        }
    }
}

impl<'ui> Ui<'ui> {
    /// Set word-wrapping for `text_*()` commands.
    /// - `< 0.0`: no wrapping;
    /// - `= 0.0`: wrap to end of window (or column);
    /// - `> 0.0`: wrap at `wrap_pos_x` position in window local space
    pub fn with_text_wrap_pos<T, F>(&self, wrap_pos_x: f32, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        unsafe { sys::igPushTextWrapPos(wrap_pos_x) };
        let result = f();
        unsafe { sys::igPopTextWrapPos() };
        result
    }
    pub fn with_allow_keyboard_focus<T, F>(&self, allow_keyboard_focus: bool, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        unsafe { sys::igPushAllowKeyboardFocus(allow_keyboard_focus) };
        let result = f();
        unsafe { sys::igPopAllowKeyboardFocus() };
        result
    }
    pub fn with_button_repeat<T, F>(&self, button_repeat: bool, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        unsafe { sys::igPushButtonRepeat(button_repeat) };
        let result = f();
        unsafe { sys::igPopButtonRepeat() };
        result
    }
}
