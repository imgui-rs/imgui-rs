pub extern crate imgui_sys as sys;

use std::cell;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::ptr;
use std::str;
use std::thread;

pub use self::clipboard::*;
pub use self::context::*;
pub use self::fonts::atlas::*;
pub use self::fonts::font::*;
pub use self::fonts::glyph::*;
pub use self::fonts::glyph_ranges::*;
pub use self::input::keyboard::*;
pub use self::input::mouse::*;
pub use self::input_widget::{
    InputFloat, InputFloat2, InputFloat3, InputFloat4, InputInt, InputInt2, InputInt3, InputInt4,
    InputText, InputTextMultiline,
};
pub use self::io::*;
pub use self::layout::*;
pub use self::legacy::*;
pub use self::plothistogram::PlotHistogram;
pub use self::plotlines::PlotLines;
pub use self::popup_modal::PopupModal;
pub use self::render::draw_data::*;
pub use self::render::renderer::*;
pub use self::stacks::*;
pub use self::string::*;
pub use self::style::*;
pub use self::utils::*;
pub use self::widget::color_editors::*;
pub use self::widget::combo_box::*;
pub use self::widget::drag::*;
pub use self::widget::image::*;
pub use self::widget::menu::*;
pub use self::widget::progress_bar::*;
pub use self::widget::selectable::*;
pub use self::widget::slider::*;
pub use self::widget::tab::*;
pub use self::widget::tree::*;
pub use self::window::child_window::*;
pub use self::window::*;
pub use self::window_draw_list::{ChannelsSplit, ImColor, WindowDrawList};
use internal::RawCast;

mod clipboard;
mod columns;
mod context;
mod fonts;
mod input;
mod input_widget;
pub mod internal;
mod io;
mod layout;
mod legacy;
mod plothistogram;
mod plotlines;
mod popup_modal;
mod render;
mod stacks;
mod string;
mod style;
#[cfg(test)]
mod test;
mod utils;
mod widget;
mod window;
mod window_draw_list;

/// Returns the underlying Dear ImGui library version
pub fn dear_imgui_version() -> &'static str {
    unsafe {
        let bytes = CStr::from_ptr(sys::igGetVersion()).to_bytes();
        str::from_utf8_unchecked(bytes)
    }
}

#[test]
fn test_version() {
    assert_eq!(dear_imgui_version(), "1.79");
}

impl Context {
    /// Returns the global imgui-rs time.
    ///
    /// Incremented by Io::delta_time every frame.
    pub fn time(&self) -> f64 {
        unsafe { sys::igGetTime() }
    }
    /// Returns the global imgui-rs frame count.
    ///
    /// Incremented by 1 every frame.
    pub fn frame_count(&self) -> i32 {
        unsafe { sys::igGetFrameCount() }
    }
}

/// A temporary reference for building the user interface for one frame
pub struct Ui<'ui> {
    ctx: &'ui Context,
    font_atlas: Option<cell::RefMut<'ui, SharedFontAtlas>>,
}

impl<'ui> Ui<'ui> {
    /// Returns an immutable reference to the inputs/outputs object
    pub fn io(&self) -> &Io {
        unsafe { &*(sys::igGetIO() as *const Io) }
    }
    /// Returns an immutable reference to the font atlas
    pub fn fonts(&self) -> FontAtlasRef {
        match self.font_atlas {
            Some(ref font_atlas) => FontAtlasRef::Shared(font_atlas),
            None => unsafe {
                let fonts = &*(self.io().fonts as *const FontAtlas);
                FontAtlasRef::Owned(fonts)
            },
        }
    }
    /// Returns a clone of the user interface style
    pub fn clone_style(&self) -> Style {
        *self.ctx.style()
    }
    /// Renders the frame and returns a reference to the resulting draw data
    pub fn render(self) -> &'ui DrawData {
        unsafe {
            sys::igRender();
            &*(sys::igGetDrawData() as *mut DrawData)
        }
    }
}

impl<'a> Drop for Ui<'a> {
    fn drop(&mut self) {
        if !thread::panicking() {
            unsafe {
                sys::igEndFrame();
            }
        }
    }
}

/// # Demo, debug, information
impl<'ui> Ui<'ui> {
    /// Renders a demo window (previously called a test window), which demonstrates most
    /// Dear Imgui features.
    pub fn show_demo_window(&self, opened: &mut bool) {
        unsafe {
            sys::igShowDemoWindow(opened);
        }
    }
    /// Renders an about window.
    ///
    /// Displays the Dear ImGui version/credits, and build/system information.
    pub fn show_about_window(&self, opened: &mut bool) {
        unsafe {
            sys::igShowAboutWindow(opened);
        }
    }
    /// Renders a metrics/debug window.
    ///
    /// Displays Dear ImGui internals: draw commands (with individual draw calls and vertices),
    /// window list, basic internal state, etc.
    pub fn show_metrics_window(&self, opened: &mut bool) {
        unsafe {
            sys::igShowMetricsWindow(opened);
        }
    }
    /// Renders a style editor block (not a window) for the given `Style` structure
    pub fn show_style_editor(&self, style: &mut Style) {
        unsafe {
            sys::igShowStyleEditor(style.raw_mut());
        }
    }
    /// Renders a style editor block (not a window) for the currently active style
    pub fn show_default_style_editor(&self) {
        unsafe { sys::igShowStyleEditor(ptr::null_mut()) };
    }
    /// Renders a basic help/info block (not a window)
    pub fn show_user_guide(&self) {
        unsafe { sys::igShowUserGuide() };
    }
}

/// Unique ID used by widgets
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Id<'a> {
    Int(i32),
    Str(&'a str),
    Ptr(*const c_void),
}

impl From<i32> for Id<'static> {
    fn from(i: i32) -> Self {
        Id::Int(i)
    }
}

impl<'a, T: ?Sized + AsRef<str>> From<&'a T> for Id<'a> {
    fn from(s: &'a T) -> Self {
        Id::Str(s.as_ref())
    }
}

impl<T> From<*const T> for Id<'static> {
    fn from(p: *const T) -> Self {
        Id::Ptr(p as *const c_void)
    }
}

impl<T> From<*mut T> for Id<'static> {
    fn from(p: *mut T) -> Self {
        Id::Ptr(p as *const T as *const c_void)
    }
}

// Widgets: Input
impl<'ui> Ui<'ui> {
    pub fn input_text<'p>(&self, label: &'p ImStr, buf: &'p mut ImString) -> InputText<'ui, 'p> {
        InputText::new(self, label, buf)
    }
    pub fn input_text_multiline<'p>(
        &self,
        label: &'p ImStr,
        buf: &'p mut ImString,
        size: [f32; 2],
    ) -> InputTextMultiline<'ui, 'p> {
        InputTextMultiline::new(self, label, buf, size)
    }
    pub fn input_float<'p>(&self, label: &'p ImStr, value: &'p mut f32) -> InputFloat<'ui, 'p> {
        InputFloat::new(self, label, value)
    }
    pub fn input_float2<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 2],
    ) -> InputFloat2<'ui, 'p> {
        InputFloat2::new(self, label, value)
    }
    pub fn input_float3<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 3],
    ) -> InputFloat3<'ui, 'p> {
        InputFloat3::new(self, label, value)
    }
    pub fn input_float4<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 4],
    ) -> InputFloat4<'ui, 'p> {
        InputFloat4::new(self, label, value)
    }
    pub fn input_int<'p>(&self, label: &'p ImStr, value: &'p mut i32) -> InputInt<'ui, 'p> {
        InputInt::new(self, label, value)
    }
    pub fn input_int2<'p>(&self, label: &'p ImStr, value: &'p mut [i32; 2]) -> InputInt2<'ui, 'p> {
        InputInt2::new(self, label, value)
    }
    pub fn input_int3<'p>(&self, label: &'p ImStr, value: &'p mut [i32; 3]) -> InputInt3<'ui, 'p> {
        InputInt3::new(self, label, value)
    }
    pub fn input_int4<'p>(&self, label: &'p ImStr, value: &'p mut [i32; 4]) -> InputInt4<'ui, 'p> {
        InputInt4::new(self, label, value)
    }
}

/// Tracks a layout tooltip that must be ended by calling `.end()`
#[must_use]
pub struct TooltipToken {
    ctx: *const Context,
}

impl TooltipToken {
    /// Ends a layout tooltip
    pub fn end(mut self, _: &Ui) {
        self.ctx = ptr::null();
        unsafe { sys::igEndTooltip() };
    }
}

impl Drop for TooltipToken {
    fn drop(&mut self) {
        if !self.ctx.is_null() && !thread::panicking() {
            panic!("A TooltipToken was leaked. Did you call .end()?");
        }
    }
}

/// # Tooltips
impl<'ui> Ui<'ui> {
    /// Construct a tooltip window that can have any kind of content.
    ///
    /// Typically used with `Ui::is_item_hovered()` or some other conditional check.
    ///
    /// # Examples
    ///
    /// ```
    /// # use imgui::*;
    /// fn user_interface(ui: &Ui) {
    ///     ui.text("Hover over me");
    ///     if ui.is_item_hovered() {
    ///         ui.tooltip(|| {
    ///             ui.text_colored([1.0, 0.0, 0.0, 1.0], im_str!("I'm red!"));
    ///         });
    ///     }
    /// }
    /// ```
    pub fn tooltip<F: FnOnce()>(&self, f: F) {
        unsafe { sys::igBeginTooltip() };
        f();
        unsafe { sys::igEndTooltip() };
    }
    /// Construct a tooltip window that can have any kind of content.
    ///
    /// Returns a `TooltipToken` that must be ended by calling `.end()`
    pub fn begin_tooltip(&self) -> TooltipToken {
        unsafe { sys::igBeginTooltip() };
        TooltipToken { ctx: self.ctx }
    }
    /// Construct a tooltip window with simple text content.
    ///
    /// Typically used with `Ui::is_item_hovered()` or some other conditional check.
    ///
    /// # Examples
    ///
    /// ```
    /// # use imgui::*;
    /// fn user_interface(ui: &Ui) {
    ///     ui.text("Hover over me");
    ///     if ui.is_item_hovered() {
    ///         ui.tooltip_text("I'm a tooltip!");
    ///     }
    /// }
    /// ```
    pub fn tooltip_text<T: AsRef<str>>(&self, text: T) {
        self.tooltip(|| self.text(text));
    }
}

// Widgets: Popups
impl<'ui> Ui<'ui> {
    pub fn open_popup<'p>(&self, str_id: &'p ImStr) {
        unsafe { sys::igOpenPopup(str_id.as_ptr(), 0) };
    }
    pub fn popup<'p, F>(&self, str_id: &'p ImStr, f: F)
    where
        F: FnOnce(),
    {
        let render =
            unsafe { sys::igBeginPopup(str_id.as_ptr(), WindowFlags::empty().bits() as i32) };
        if render {
            f();
            unsafe { sys::igEndPopup() };
        }
    }
    /// Create a modal pop-up.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = Context::create();
    /// # let ui = imgui.frame();
    /// if ui.button(im_str!("Show modal"), [0.0, 0.0]) {
    ///     ui.open_popup(im_str!("modal"));
    /// }
    /// ui.popup_modal(im_str!("modal")).build(|| {
    ///     ui.text("Content of my modal");
    ///     if ui.button(im_str!("OK"), [0.0, 0.0]) {
    ///         ui.close_current_popup();
    ///     }
    /// });
    /// ```
    pub fn popup_modal<'p>(&self, str_id: &'p ImStr) -> PopupModal<'ui, 'p> {
        PopupModal::new(self, str_id)
    }
    /// Close a popup. Should be called within the closure given as argument to
    /// [`Ui::popup`] or [`Ui::popup_modal`].
    pub fn close_current_popup(&self) {
        unsafe { sys::igCloseCurrentPopup() };
    }
}

// Widgets: ListBox
impl<'ui> Ui<'ui> {
    pub fn list_box<'p, StringType: AsRef<ImStr> + ?Sized>(
        &self,
        label: &'p ImStr,
        current_item: &mut i32,
        items: &'p [&'p StringType],
        height_in_items: i32,
    ) -> bool {
        let items_inner: Vec<*const c_char> =
            items.iter().map(|item| item.as_ref().as_ptr()).collect();
        unsafe {
            sys::igListBoxStr_arr(
                label.as_ptr(),
                current_item,
                items_inner.as_ptr() as *mut *const c_char,
                items_inner.len() as i32,
                height_in_items,
            )
        }
    }
}

impl<'ui> Ui<'ui> {
    pub fn plot_lines<'p>(&self, label: &'p ImStr, values: &'p [f32]) -> PlotLines<'ui, 'p> {
        PlotLines::new(self, label, values)
    }
}

impl<'ui> Ui<'ui> {
    pub fn plot_histogram<'p>(
        &self,
        label: &'p ImStr,
        values: &'p [f32],
    ) -> PlotHistogram<'ui, 'p> {
        PlotHistogram::new(self, label, values)
    }
}

impl<'ui> Ui<'ui> {
    /// Calculate the size required for a given text string.
    ///
    /// hide_text_after_double_hash allows the user to insert comments into their text, using a double hash-tag prefix.
    /// This is a feature of imgui.
    ///
    /// wrap_width allows you to request a width at which to wrap the text to a newline for the calculation.
    pub fn calc_text_size(
        &self,
        text: &ImStr,
        hide_text_after_double_hash: bool,
        wrap_width: f32,
    ) -> [f32; 2] {
        let mut out = sys::ImVec2::zero();
        unsafe {
            sys::igCalcTextSize(
                &mut out,
                text.as_ptr(),
                std::ptr::null(),
                hide_text_after_double_hash,
                wrap_width,
            )
        };
        out.into()
    }
}

/// # Draw list for custom drawing
impl<'ui> Ui<'ui> {
    /// Get access to drawing API
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use imgui::*;
    /// fn custom_draw(ui: &Ui) {
    ///     let draw_list = ui.get_window_draw_list();
    ///     // Draw a line
    ///     const WHITE: [f32; 3] = [1.0, 1.0, 1.0];
    ///     draw_list.add_line([100.0, 100.0], [200.0, 200.0], WHITE).build();
    ///     // Continue drawing ...
    /// }
    /// ```
    ///
    /// This function will panic if several instances of [`WindowDrawList`]
    /// coexist. Before a new instance is got, a previous instance should be
    /// dropped.
    ///
    /// ```rust
    /// # use imgui::*;
    /// fn custom_draw(ui: &Ui) {
    ///     let draw_list = ui.get_window_draw_list();
    ///     // Draw something...
    ///
    ///     // This second call will panic!
    ///     let draw_list = ui.get_window_draw_list();
    /// }
    /// ```
    #[must_use]
    pub fn get_window_draw_list(&'ui self) -> WindowDrawList<'ui> {
        WindowDrawList::new(self)
    }

    #[must_use]
    pub fn get_background_draw_list(&'ui self) -> WindowDrawList<'ui> {
        WindowDrawList::new(self).background()
    }
}

/// Condition for applying a setting
#[repr(i8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Condition {
    /// Never apply the setting
    Never = -1,
    /// Always apply the setting
    Always = sys::ImGuiCond_Always as i8,
    /// Apply the setting once per runtime session (only the first call will succeed)
    Once = sys::ImGuiCond_Once as i8,
    /// Apply the setting if the object/window has no persistently saved data (no entry in .ini
    /// file)
    FirstUseEver = sys::ImGuiCond_FirstUseEver as i8,
    /// Apply the setting if the object/window is appearing after being hidden/inactive (or the
    /// first time)
    Appearing = sys::ImGuiCond_Appearing as i8,
}

/// A cardinal direction
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    None = sys::ImGuiDir_None,
    Left = sys::ImGuiDir_Left,
    Right = sys::ImGuiDir_Right,
    Up = sys::ImGuiDir_Up,
    Down = sys::ImGuiDir_Down,
}
