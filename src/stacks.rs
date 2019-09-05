use std::mem;
use std::os::raw::{c_char, c_void};
use std::ptr;
use std::thread;

use crate::context::Context;
use crate::fonts::atlas::FontId;
use crate::internal::RawCast;
use crate::style::{StyleColor, StyleVar};
use crate::sys;
use crate::{Id, Ui};

/// # Parameter stacks (shared)
impl<'ui> Ui<'ui> {
    /// Switches to the given font by pushing it to the font stack.
    ///
    /// Returns a `FontStackToken` that must be popped by calling `.pop()`
    ///
    /// # Panics
    ///
    /// Panics if the font atlas does not contain the given font
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use imgui::*;
    /// # let mut ctx = Context::create();
    /// # let font_data_sources = [];
    /// // At initialization time
    /// let my_custom_font = ctx.fonts().add_font(&font_data_sources);
    /// # let ui = ctx.frame();
    /// // During UI construction
    /// let font = ui.push_font(my_custom_font);
    /// ui.text("I use the custom font!");
    /// font.pop(&ui);
    /// ```
    #[must_use]
    pub fn push_font(&self, id: FontId) -> FontStackToken {
        let fonts = self.fonts();
        let font = fonts
            .get_font(id)
            .expect("Font atlas did not contain the given font");
        unsafe { sys::igPushFont(font.raw() as *const _ as *mut _) };
        FontStackToken { ctx: self.ctx }
    }
    /// Changes a style color by pushing a change to the color stack.
    ///
    /// Returns a `ColorStackToken` that must be popped by calling `.pop()`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use imgui::*;
    /// # let mut ctx = Context::create();
    /// # let ui = ctx.frame();
    /// const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    /// let color = ui.push_style_color(StyleColor::Text, RED);
    /// ui.text("I'm red!");
    /// color.pop(&ui);
    /// ```
    #[must_use]
    pub fn push_style_color(&self, style_color: StyleColor, color: [f32; 4]) -> ColorStackToken {
        unsafe { sys::igPushStyleColor(style_color as i32, color.into()) };
        ColorStackToken {
            count: 1,
            ctx: self.ctx,
        }
    }
    /// Changes style colors by pushing several changes to the color stack.
    ///
    /// Returns a `ColorStackToken` that must be popped by calling `.pop()`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use imgui::*;
    /// # let mut ctx = Context::create();
    /// # let ui = ctx.frame();
    /// const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    /// const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    /// let colors = ui.push_style_colors(&[
    ///     (StyleColor::Text, RED),
    ///     (StyleColor::TextDisabled, GREEN),
    /// ]);
    /// ui.text("I'm red!");
    /// ui.text_disabled("I'm green!");
    /// colors.pop(&ui);
    /// ```
    #[must_use]
    pub fn push_style_colors<'a, I>(&self, style_colors: I) -> ColorStackToken
    where
        I: IntoIterator<Item = &'a (StyleColor, [f32; 4])>,
    {
        let mut count = 0;
        for &(style_color, color) in style_colors {
            unsafe { sys::igPushStyleColor(style_color as i32, color.into()) };
            count += 1;
        }
        ColorStackToken {
            count,
            ctx: self.ctx,
        }
    }
    /// Changes a style variable by pushing a change to the style stack.
    ///
    /// Returns a `StyleStackToken` that must be popped by calling `.pop()`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use imgui::*;
    /// # let mut ctx = Context::create();
    /// # let ui = ctx.frame();
    /// let style = ui.push_style_var(StyleVar::Alpha(0.2));
    /// ui.text("I'm transparent!");
    /// style.pop(&ui);
    /// ```
    #[must_use]
    pub fn push_style_var(&self, style_var: StyleVar) -> StyleStackToken {
        unsafe { push_style_var(style_var) };
        StyleStackToken {
            count: 1,
            ctx: self.ctx,
        }
    }
    /// Changes style variables by pushing several changes to the style stack.
    ///
    /// Returns a `StyleStackToken` that must be popped by calling `.pop()`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use imgui::*;
    /// # let mut ctx = Context::create();
    /// # let ui = ctx.frame();
    /// let styles = ui.push_style_vars(&[
    ///     StyleVar::Alpha(0.2),
    ///     StyleVar::ItemSpacing([50.0, 50.0])
    /// ]);
    /// ui.text("We're transparent...");
    /// ui.text("...with large spacing as well");
    /// styles.pop(&ui);
    /// ```
    #[must_use]
    pub fn push_style_vars<'a, I>(&self, style_vars: I) -> StyleStackToken
    where
        I: IntoIterator<Item = &'a StyleVar>,
    {
        let mut count = 0;
        for &style_var in style_vars {
            unsafe { push_style_var(style_var) };
            count += 1;
        }
        StyleStackToken {
            count,
            ctx: self.ctx,
        }
    }
}

/// Tracks a font pushed to the font stack that must be popped by calling `.pop()`
#[must_use]
pub struct FontStackToken {
    ctx: *const Context,
}

impl FontStackToken {
    /// Pops a change from the font stack
    pub fn pop(mut self, _: &Ui) {
        self.ctx = ptr::null();
        unsafe { sys::igPopFont() };
    }
}

impl Drop for FontStackToken {
    fn drop(&mut self) {
        if !self.ctx.is_null() && !thread::panicking() {
            panic!("A FontStackToken was leaked. Did you call .pop()?");
        }
    }
}

/// Tracks one or more changes pushed to the color stack that must be popped by calling `.pop()`
#[must_use]
pub struct ColorStackToken {
    count: usize,
    ctx: *const Context,
}

impl ColorStackToken {
    /// Pops changes from the color stack
    pub fn pop(mut self, _: &Ui) {
        self.ctx = ptr::null();
        unsafe { sys::igPopStyleColor(self.count as i32) };
    }
}

impl Drop for ColorStackToken {
    fn drop(&mut self) {
        if !self.ctx.is_null() && !thread::panicking() {
            panic!("A ColorStackToken was leaked. Did you call .pop()?");
        }
    }
}

/// Tracks one or more changes pushed to the style stack that must be popped by calling `.pop()`
#[must_use]
pub struct StyleStackToken {
    count: usize,
    ctx: *const Context,
}

impl StyleStackToken {
    /// Pops changes from the style stack
    pub fn pop(mut self, _: &Ui) {
        self.ctx = ptr::null();
        unsafe { sys::igPopStyleVar(self.count as i32) };
    }
}

impl Drop for StyleStackToken {
    fn drop(&mut self) {
        if !self.ctx.is_null() && !thread::panicking() {
            panic!("A StyleStackToken was leaked. Did you call .pop()?");
        }
    }
}

#[inline]
unsafe fn push_style_var(style_var: StyleVar) {
    use crate::style::StyleVar::*;
    use crate::sys::{igPushStyleVarFloat, igPushStyleVarVec2};
    match style_var {
        Alpha(v) => igPushStyleVarFloat(sys::ImGuiStyleVar_Alpha as i32, v),
        WindowPadding(v) => igPushStyleVarVec2(sys::ImGuiStyleVar_WindowPadding as i32, v.into()),
        WindowRounding(v) => igPushStyleVarFloat(sys::ImGuiStyleVar_WindowRounding as i32, v),
        WindowBorderSize(v) => igPushStyleVarFloat(sys::ImGuiStyleVar_WindowBorderSize as i32, v),
        WindowMinSize(v) => igPushStyleVarVec2(sys::ImGuiStyleVar_WindowMinSize as i32, v.into()),
        WindowTitleAlign(v) => {
            igPushStyleVarVec2(sys::ImGuiStyleVar_WindowTitleAlign as i32, v.into())
        }
        ChildRounding(v) => igPushStyleVarFloat(sys::ImGuiStyleVar_ChildRounding as i32, v),
        ChildBorderSize(v) => igPushStyleVarFloat(sys::ImGuiStyleVar_ChildBorderSize as i32, v),
        PopupRounding(v) => igPushStyleVarFloat(sys::ImGuiStyleVar_PopupRounding as i32, v),
        PopupBorderSize(v) => igPushStyleVarFloat(sys::ImGuiStyleVar_PopupBorderSize as i32, v),
        FramePadding(v) => igPushStyleVarVec2(sys::ImGuiStyleVar_FramePadding as i32, v.into()),
        FrameRounding(v) => igPushStyleVarFloat(sys::ImGuiStyleVar_FrameRounding as i32, v),
        FrameBorderSize(v) => igPushStyleVarFloat(sys::ImGuiStyleVar_FrameBorderSize as i32, v),
        ItemSpacing(v) => igPushStyleVarVec2(sys::ImGuiStyleVar_ItemSpacing as i32, v.into()),
        ItemInnerSpacing(v) => {
            igPushStyleVarVec2(sys::ImGuiStyleVar_ItemInnerSpacing as i32, v.into())
        }
        IndentSpacing(v) => igPushStyleVarFloat(sys::ImGuiStyleVar_IndentSpacing as i32, v),
        ScrollbarSize(v) => igPushStyleVarFloat(sys::ImGuiStyleVar_ScrollbarSize as i32, v),
        ScrollbarRounding(v) => igPushStyleVarFloat(sys::ImGuiStyleVar_ScrollbarRounding as i32, v),
        GrabMinSize(v) => igPushStyleVarFloat(sys::ImGuiStyleVar_GrabMinSize as i32, v),
        GrabRounding(v) => igPushStyleVarFloat(sys::ImGuiStyleVar_GrabRounding as i32, v),
        TabRounding(v) => igPushStyleVarFloat(sys::ImGuiStyleVar_TabRounding as i32, v),
        ButtonTextAlign(v) => {
            igPushStyleVarVec2(sys::ImGuiStyleVar_ButtonTextAlign as i32, v.into())
        }
        SelectableTextAlign(v) => {
            igPushStyleVarVec2(sys::ImGuiStyleVar_SelectableTextAlign as i32, v.into())
        }
    }
}

/// # Parameter stacks (current window)
impl<'ui> Ui<'ui> {
    /// Changes the item width by pushing a change to the item width stack.
    ///
    /// Returns an `ItemWidthStackToken` that may be popped by calling `.pop()`
    ///
    /// - `> 0.0`: width is `item_width` pixels
    /// - `= 0.0`: default to ~2/3 of window width
    /// - `< 0.0`: `item_width` pixels relative to the right of window (-1.0 always aligns width to
    /// the right side)
    pub fn push_item_width(&self, item_width: f32) -> ItemWidthStackToken {
        unsafe { sys::igPushItemWidth(item_width) };
        ItemWidthStackToken { ctx: self.ctx }
    }
    /// Sets the width of the next item.
    ///
    /// - `> 0.0`: width is `item_width` pixels
    /// - `= 0.0`: default to ~2/3 of window width
    /// - `< 0.0`: `item_width` pixels relative to the right of window (-1.0 always aligns width to
    /// the right side)
    pub fn set_next_item_width(&self, item_width: f32) {
        unsafe { sys::igSetNextItemWidth(item_width) };
    }
    /// Returns the width of the item given the pushed settings and the current cursor position.
    ///
    /// This is NOT necessarily the width of last item.
    pub fn calc_item_width(&self) -> f32 {
        unsafe { sys::igCalcItemWidth() }
    }
    /// Changes the text wrapping position by pushing a change to the text wrapping position stack.
    ///
    /// Returns a `TextWrapPosStackToken` that may be popped by calling `.pop()`
    ///
    /// - `> 0.0`: wrap at `wrap_pos_x` position in window local space
    /// - `= 0.0`: wrap to end of window (or column)
    /// - `< 0.0`: no wrapping
    pub fn push_text_wrap_pos(&self, wrap_pos_x: f32) -> TextWrapPosStackToken {
        unsafe { sys::igPushTextWrapPos(wrap_pos_x) };
        TextWrapPosStackToken { ctx: self.ctx }
    }
    /// Changes an item flag by pushing a change to the item flag stack.
    ///
    /// Returns a `ItemFlagsStackToken` that may be popped by calling `.pop()`
    pub fn push_item_flag(&self, item_flag: ItemFlag) -> ItemFlagsStackToken {
        use self::ItemFlag::*;
        match item_flag {
            AllowKeyboardFocus(v) => unsafe { sys::igPushAllowKeyboardFocus(v) },
            ButtonRepeat(v) => unsafe { sys::igPushButtonRepeat(v) },
        }
        ItemFlagsStackToken {
            discriminant: mem::discriminant(&item_flag),
            ctx: self.ctx,
        }
    }
}

/// A temporary change in item flags
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ItemFlag {
    AllowKeyboardFocus(bool),
    ButtonRepeat(bool),
}

/// Tracks a change pushed to the item width stack
pub struct ItemWidthStackToken {
    ctx: *const Context,
}

impl ItemWidthStackToken {
    /// Pops a change from the item width stack
    pub fn pop(mut self, _: &Ui) {
        self.ctx = ptr::null();
        unsafe { sys::igPopItemWidth() };
    }
}

/// Tracks a change pushed to the text wrap position stack
pub struct TextWrapPosStackToken {
    ctx: *const Context,
}

impl TextWrapPosStackToken {
    /// Pops a change from the text wrap position stack
    pub fn pop(mut self, _: &Ui) {
        self.ctx = ptr::null();
        unsafe { sys::igPopTextWrapPos() };
    }
}

/// Tracks a change pushed to the item flags stack
pub struct ItemFlagsStackToken {
    discriminant: mem::Discriminant<ItemFlag>,
    ctx: *const Context,
}

impl ItemFlagsStackToken {
    /// Pops a change from the item flags stack
    pub fn pop(mut self, _: &Ui) {
        self.ctx = ptr::null();
        const ALLOW_KEYBOARD_FOCUS: ItemFlag = ItemFlag::AllowKeyboardFocus(true);
        const BUTTON_REPEAT: ItemFlag = ItemFlag::ButtonRepeat(true);

        if self.discriminant == mem::discriminant(&ALLOW_KEYBOARD_FOCUS) {
            unsafe { sys::igPopAllowKeyboardFocus() };
        } else if self.discriminant == mem::discriminant(&BUTTON_REPEAT) {
            unsafe { sys::igPopButtonRepeat() };
        } else {
            unreachable!();
        }
    }
}

/// # ID stack
impl<'ui> Ui<'ui> {
    /// Pushes an identifier to the ID stack.
    ///
    /// Returns an `IdStackToken` that must be popped by calling `.pop()`
    ///
    #[must_use]
    pub fn push_id<'a, I: Into<Id<'a>>>(&self, id: I) -> IdStackToken {
        let id = id.into();

        unsafe {
            match id {
                Id::Int(i) => sys::igPushIDInt(i),
                Id::Str(s) => {
                    let start = s.as_ptr() as *const c_char;
                    let end = start.add(s.len());
                    sys::igPushIDRange(start, end)
                }
                Id::Ptr(p) => sys::igPushIDPtr(p as *const c_void),
            }
        }
        IdStackToken { ctx: self.ctx }
    }
}

/// Tracks an ID pushed to the ID stack that must be popped by calling `.pop()`
#[must_use]
pub struct IdStackToken {
    ctx: *const Context,
}

impl IdStackToken {
    /// Pops a change from the ID stack
    pub fn pop(mut self, _: &Ui) {
        self.ctx = ptr::null();
        unsafe { sys::igPopID() };
    }
}

impl Drop for IdStackToken {
    fn drop(&mut self) {
        if !self.ctx.is_null() && !thread::panicking() {
            panic!("A IdStackToken was leaked. Did you call .pop()?");
        }
    }
}
