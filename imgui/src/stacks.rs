use crate::fonts::atlas::FontId;
use crate::internal::RawCast;
use crate::math::MintVec4;
use crate::style::{StyleColor, StyleVar};
use crate::sys;
use crate::Ui;
use std::mem;
use std::os::raw::c_char;

/// # Parameter stacks (shared)
impl Ui {
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
    /// font.pop();
    /// ```
    #[doc(alias = "PushFont")]
    pub fn push_font(&self, id: FontId) -> FontStackToken<'_> {
        let fonts = self.fonts();
        let font = fonts
            .get_font(id)
            .expect("Font atlas did not contain the given font");
        unsafe { sys::igPushFont(font.raw() as *const _ as *mut _) };
        FontStackToken::new(self)
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
    /// color.pop();
    /// ```
    #[doc(alias = "PushStyleColorVec4")]
    pub fn push_style_color(
        &self,
        style_color: StyleColor,
        color: impl Into<MintVec4>,
    ) -> ColorStackToken<'_> {
        unsafe { sys::igPushStyleColorVec4(style_color as i32, color.into().into()) };
        ColorStackToken::new(self)
    }

    /// Changes a style variable by pushing a change to the style stack.
    ///
    /// Returns a `StyleStackToken` that can be popped by calling `.end()`
    /// or by allowing to drop.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use imgui::*;
    /// # let mut ctx = Context::create();
    /// # let ui = ctx.frame();
    /// let style = ui.push_style_var(StyleVar::Alpha(0.2));
    /// ui.text("I'm transparent!");
    /// style.pop();
    /// ```
    #[doc(alias = "PushStyleVar")]
    pub fn push_style_var(&self, style_var: StyleVar) -> StyleStackToken<'_> {
        unsafe { push_style_var(style_var) };
        StyleStackToken::new(self)
    }
}

create_token!(
    /// Tracks a font pushed to the font stack that can be popped by calling `.end()`
    /// or by dropping.
    pub struct FontStackToken<'ui>;

    /// Pops a change from the font stack.
    drop { sys::igPopFont() }
);

impl FontStackToken<'_> {
    /// Pops a change from the font stack.
    pub fn pop(self) {
        self.end()
    }
}

create_token!(
    /// Tracks a color pushed to the color stack that can be popped by calling `.end()`
    /// or by dropping.
    pub struct ColorStackToken<'ui>;

    /// Pops a change from the color stack.
    drop { sys::igPopStyleColor(1) }
);

impl ColorStackToken<'_> {
    /// Pops a change from the color stack.
    pub fn pop(self) {
        self.end()
    }
}

create_token!(
    /// Tracks a style pushed to the style stack that can be popped by calling `.end()`
    /// or by dropping.
    pub struct StyleStackToken<'ui>;

    /// Pops a change from the style stack.
    drop { sys::igPopStyleVar(1) }
);

impl StyleStackToken<'_> {
    /// Pops a change from the style stack.
    pub fn pop(self) {
        self.end()
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
        CellPadding(v) => igPushStyleVarVec2(sys::ImGuiStyleVar_CellPadding as i32, v.into()),
    }
}

/// # Parameter stacks (current window)
impl Ui {
    /// Changes the item width by pushing a change to the item width stack.
    ///
    /// Returns an `ItemWidthStackToken` that may be popped by calling `.pop()`
    ///
    /// - `> 0.0`: width is `item_width` pixels
    /// - `= 0.0`: default to ~2/3 of window width
    /// - `< 0.0`: `item_width` pixels relative to the right of window (-1.0 always aligns width to
    /// the right side)
    #[doc(alias = "PushItemWith")]
    pub fn push_item_width(&self, item_width: f32) -> ItemWidthStackToken<'_> {
        unsafe { sys::igPushItemWidth(item_width) };
        ItemWidthStackToken::new(self)
    }
    /// Sets the width of the next item.
    ///
    /// - `> 0.0`: width is `item_width` pixels
    /// - `= 0.0`: default to ~2/3 of window width
    /// - `< 0.0`: `item_width` pixels relative to the right of window (-1.0 always aligns width to
    /// the right side)
    #[doc(alias = "SetNextItemWidth")]
    pub fn set_next_item_width(&self, item_width: f32) {
        unsafe { sys::igSetNextItemWidth(item_width) };
    }
    /// Returns the width of the item given the pushed settings and the current cursor position.
    ///
    /// This is NOT necessarily the width of last item.
    #[doc(alias = "CalcItemWidth")]
    pub fn calc_item_width(&self) -> f32 {
        unsafe { sys::igCalcItemWidth() }
    }

    /// Changes the text wrapping position to the end of window (or column), which
    /// is generally the default.
    ///
    /// This is the same as calling [push_text_wrap_pos_with_pos](Self::push_text_wrap_pos_with_pos)
    /// with `wrap_pos_x` set to 0.0.
    ///
    /// Returns a `TextWrapPosStackToken` that may be popped by calling `.pop()`
    #[doc(alias = "PushTextWrapPos")]
    pub fn push_text_wrap_pos(&self) -> TextWrapPosStackToken<'_> {
        self.push_text_wrap_pos_with_pos(0.0)
    }

    /// Changes the text wrapping position by pushing a change to the text wrapping position stack.
    ///
    /// Returns a `TextWrapPosStackToken` that may be popped by calling `.pop()`
    ///
    /// - `> 0.0`: wrap at `wrap_pos_x` position in window local space
    /// - `= 0.0`: wrap to end of window (or column)
    /// - `< 0.0`: no wrapping
    #[doc(alias = "PushTextWrapPos")]
    pub fn push_text_wrap_pos_with_pos(&self, wrap_pos_x: f32) -> TextWrapPosStackToken<'_> {
        unsafe { sys::igPushTextWrapPos(wrap_pos_x) };
        TextWrapPosStackToken::new(self)
    }

    /// Tab stop enable.
    /// Allow focusing using TAB/Shift-TAB, enabled by default but you can
    /// disable it for certain widgets
    ///
    /// Returns a [PushAllowKeyboardFocusToken] that should be dropped.
    #[doc(alias = "PushAllowKeyboardFocus")]
    pub fn push_allow_keyboard_focus(&self, allow: bool) -> PushAllowKeyboardFocusToken<'_> {
        unsafe { sys::igPushAllowKeyboardFocus(allow) };
        PushAllowKeyboardFocusToken::new(self)
    }

    /// In 'repeat' mode, button_x functions return repeated true in a typematic
    /// manner (using io.KeyRepeatDelay/io.KeyRepeatRate setting).
    /// Note that you can call IsItemActive() after any Button() to tell if the
    /// button is held in the current frame.
    ///
    /// Returns a [PushButtonRepeatToken] that should be dropped.
    #[doc(alias = "PushAllowKeyboardFocus")]
    pub fn push_button_repeat(&self, allow: bool) -> PushButtonRepeatToken<'_> {
        unsafe { sys::igPushButtonRepeat(allow) };
        PushButtonRepeatToken::new(self)
    }

    /// Changes an item flag by pushing a change to the item flag stack.
    ///
    /// Returns a `ItemFlagsStackToken` that may be popped by calling `.pop()`
    ///
    /// ## Deprecated
    ///
    /// This was deprecated in `0.9.0` because it isn't part of the dear imgui design,
    /// and supporting it required a manual implementation of its drop token.
    ///
    /// Instead, just use [`push_allow_keyboard_focus`] and [`push_button_repeat`].
    ///
    /// [`push_allow_keyboard_focus`]: Self::push_allow_keyboard_focus
    /// [`push_button_repeat`]: Self::push_button_repeat
    #[deprecated(
        since = "0.9.0",
        note = "use `push_allow_keyboard_focus` or `push_button_repeat` instead"
    )]
    pub fn push_item_flag(&self, item_flag: ItemFlag) -> ItemFlagsStackToken<'_> {
        use self::ItemFlag::*;
        match item_flag {
            AllowKeyboardFocus(v) => unsafe { sys::igPushAllowKeyboardFocus(v) },
            ButtonRepeat(v) => unsafe { sys::igPushButtonRepeat(v) },
        }
        ItemFlagsStackToken::new(self, item_flag)
    }
}

/// A temporary change in item flags
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ItemFlag {
    AllowKeyboardFocus(bool),
    ButtonRepeat(bool),
}

create_token!(
    /// Tracks a window that can be ended by calling `.end()`
    /// or by dropping.
    pub struct ItemWidthStackToken<'ui>;

    /// Ends a window
    #[doc(alias = "PopItemWidth")]
    drop { sys::igPopItemWidth() }
);

create_token!(
    /// Tracks a window that can be ended by calling `.end()`
    /// or by dropping.
    pub struct TextWrapPosStackToken<'ui>;

    /// Ends a window
    #[doc(alias = "PopTextWrapPos")]
    drop { sys::igPopTextWrapPos() }
);

create_token!(
    /// Tracks a window that can be ended by calling `.end()`
    /// or by dropping.
    pub struct PushAllowKeyboardFocusToken<'ui>;

    /// Ends a window
    #[doc(alias = "PopAllowKeyboardFocus")]
    drop { sys::igPopAllowKeyboardFocus() }
);

create_token!(
    /// Tracks a window that can be ended by calling `.end()`
    /// or by dropping.
    pub struct PushButtonRepeatToken<'ui>;

    /// Ends a window
    #[doc(alias = "PopButtonRepeat")]
    drop { sys::igPopButtonRepeat() }
);

/// Tracks a change pushed to the item flags stack.
///
/// The "item flags" stack was a concept invented in imgui-rs that doesn't have an
/// ImGui equivalent. We're phasing these out to make imgui-rs feel simpler to use.
#[must_use]
pub struct ItemFlagsStackToken<'a>(
    std::marker::PhantomData<&'a Ui>,
    mem::Discriminant<ItemFlag>,
);

impl<'a> ItemFlagsStackToken<'a> {
    /// Creates a new token type.
    pub(crate) fn new(_: &'a crate::Ui, kind: ItemFlag) -> Self {
        Self(std::marker::PhantomData, mem::discriminant(&kind))
    }

    #[inline]
    pub fn end(self) {
        // left empty for drop
    }
}

impl Drop for ItemFlagsStackToken<'_> {
    fn drop(&mut self) {
        unsafe {
            if self.1 == mem::discriminant(&ItemFlag::AllowKeyboardFocus(true)) {
                sys::igPopAllowKeyboardFocus();
            } else if self.1 == mem::discriminant(&ItemFlag::ButtonRepeat(true)) {
                sys::igPopButtonRepeat();
            } else {
                unreachable!();
            }
        }
    }
}

create_token!(
    /// Tracks an ID pushed to the ID stack that can be popped by calling `.pop()`
    /// or by dropping.
    pub struct IdStackToken<'ui>;

    /// Pops a change from the ID stack
    drop { sys::igPopID() }
);

impl IdStackToken<'_> {
    /// Pops a change from the ID stack
    pub fn pop(self) {
        self.end()
    }
}

/// # ID stack
impl<'ui> Ui {
    /// Pushes an identifier to the ID stack.
    ///
    /// Returns an `IdStackToken` that can be popped by calling `.end()`
    /// or by dropping manually.
    ///
    /// # Examples
    /// Dear ImGui uses labels to uniquely identify widgets. For a good explaination, see this part of the [Dear ImGui FAQ][faq]
    ///
    /// [faq]: https://github.com/ocornut/imgui/blob/v1.84.2/docs/FAQ.md#q-why-is-my-widget-not-reacting-when-i-click-on-it
    ///
    /// In `imgui-rs` the same applies, we can manually specify labels with the `##` syntax:
    ///
    /// ```no_run
    /// # let mut imgui = imgui::Context::create();
    /// # let ui = imgui.frame();
    ///
    /// ui.button("Click##button1");
    /// ui.button("Click##button2");
    /// ```
    ///
    /// Here `Click` is the label used for both buttons, and everything after `##` is used as the identifier.
    ///
    /// However when you either have many items (say, created in a loop), we can use our loop number as an item in the "ID stack":
    ///
    /// ```no_run
    /// # let mut imgui = imgui::Context::create();
    /// # let ui = imgui.frame();
    ///
    /// ui.window("Example").build(|| {
    ///     // The window adds "Example" to the token stack.
    ///     for num in 0..10 {
    ///         // And now we add the loop number to the stack too,
    ///         // to make our buttons unique within this window.
    ///         let _id = ui.push_id_usize(num);
    ///         if ui.button("Click!") {
    ///             println!("Button {} clicked", num);
    ///         }
    ///     }
    /// });
    /// ```
    ///
    /// We don't have to use numbers - strings also work:
    ///
    /// ```no_run
    /// # let mut imgui = imgui::Context::create();
    /// # let ui = imgui.frame();
    ///
    /// fn callback1(ui: &imgui::Ui) {
    ///     if ui.button("Click") {
    ///         println!("First button clicked")
    ///     }
    /// }
    ///
    /// fn callback2(ui: &imgui::Ui) {
    ///     if ui.button("Click") {
    ///         println!("Second button clicked")
    ///     }
    /// }
    ///
    /// ui.window("Example")
    /// .build(||{
    ///     {
    ///         // Since we don't know what callback1 might do, we create
    ///         // a unique ID stack by pushing (a hash of) "first" to the ID stack:
    ///         let _id1 = ui.push_id("first");
    ///         callback1(&ui);
    ///     }
    ///     {
    ///         // Same for second callback. If we didn't do this, clicking the "Click" button
    ///         // would trigger both println statements!
    ///         let _id2 = ui.push_id("second");
    ///         callback2(&ui);
    ///     }
    /// });
    /// ```
    #[doc(alias = "PushId")]
    pub fn push_id(&self, s: impl AsRef<str>) -> IdStackToken<'_> {
        unsafe {
            let s = s.as_ref();
            let start = s.as_ptr() as *const c_char;
            let end = start.add(s.len());
            sys::igPushIDStrStr(start, end)
        }
        IdStackToken::new(self)
    }

    /// Pushes a `usize` to the ID stack.
    ///
    /// Returns an `IdStackToken` that can be popped by calling `.end()`
    /// or by dropping manually.
    ///
    /// See [push_id] for more information.
    ///
    /// [push_id]: Self::push_id
    #[doc(alias = "PushId")]
    pub fn push_id_usize(&self, id: usize) -> IdStackToken<'_> {
        unsafe { sys::igPushIDPtr(id as *const _) }
        IdStackToken::new(self)
    }

    /// Pushes an `i32` to the ID stack.
    ///
    /// Returns an `IdStackToken` that can be popped by calling `.end()`
    /// or by dropping manually.
    ///
    /// See [push_id] for more information.
    ///
    /// [push_id]: Self::push_id
    #[doc(alias = "PushId")]
    pub fn push_id_int(&self, id: i32) -> IdStackToken<'_> {
        unsafe { sys::igPushIDInt(id) }
        IdStackToken::new(self)
    }

    /// Pushes a `ptr` to the ID stack.
    ///
    /// Returns an `IdStackToken` that can be popped by calling `.end()`
    /// or by dropping manually.
    ///
    /// See [push_id] for more information.
    ///
    /// [push_id]: Self::push_id
    #[doc(alias = "PushId")]
    pub fn push_id_ptr<T>(&self, value: &T) -> IdStackToken<'_> {
        unsafe { sys::igPushIDPtr(value as *const T as *const _) }
        IdStackToken::new(self)
    }
}
