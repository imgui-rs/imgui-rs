#![allow(clippy::float_cmp)]
pub extern crate imgui_sys as sys;

use std::cell;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::ptr;
use std::str;
use std::thread;

pub use self::clipboard::*;
pub use self::color::ImColor32;
pub use self::context::*;
pub use self::drag_drop::{DragDropFlags, DragDropSource, DragDropTarget};
pub use self::draw_list::{ChannelsSplit, DrawListMut};
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
pub use self::list_clipper::ListClipper;
pub use self::plothistogram::PlotHistogram;
pub use self::plotlines::PlotLines;
pub use self::popups::*;
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
pub use self::widget::list_box::*;
pub use self::widget::menu::*;
pub use self::widget::progress_bar::*;
pub use self::widget::selectable::*;
pub use self::widget::slider::*;
pub use self::widget::tab::*;
pub use self::widget::tree::*;
pub use self::window::child_window::*;
pub use self::window::*;
use internal::RawCast;

#[macro_use]
mod string;

#[macro_use]
mod tokens;

mod clipboard;
pub mod color;
mod columns;
mod context;
pub mod drag_drop;
pub mod draw_list;
mod fonts;
mod input;
mod input_widget;
pub mod internal;
mod io;
mod layout;
mod list_clipper;
mod plothistogram;
mod plotlines;
mod popups;
mod render;
mod stacks;
mod style;
#[cfg(test)]
mod test;
mod utils;
mod widget;
mod window;

// Used by macros. Underscores are just to make it clear it's not part of the
// public API.
#[doc(hidden)]
pub use core as __core;

/// Returns the underlying Dear ImGui library version
#[doc(alias = "GetVersion")]
pub fn dear_imgui_version() -> &'static str {
    unsafe {
        let bytes = CStr::from_ptr(sys::igGetVersion()).to_bytes();
        str::from_utf8_unchecked(bytes)
    }
}

#[test]
fn test_version() {
    // TODO: what's the point of this test?
    // cfrantz: To fail.
    //assert_eq!(dear_imgui_version(), "1.82");
}

impl Context {
    /// Returns the global imgui-rs time.
    ///
    /// Incremented by Io::delta_time every frame.
    #[doc(alias = "GetTime")]
    pub fn time(&self) -> f64 {
        unsafe { sys::igGetTime() }
    }
    /// Returns the global imgui-rs frame count.
    ///
    /// Incremented by 1 every frame.
    #[doc(alias = "GetFrameCount")]
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
    #[doc(alias = "GetIO")]
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
    #[doc(alias = "Render", alias = "GetDrawData")]
    pub fn render(self) -> &'ui DrawData {
        unsafe {
            sys::igRender();
            &*(sys::igGetDrawData() as *mut DrawData)
        }
    }
}

impl<'a> Drop for Ui<'a> {
    #[doc(alias = "EndFrame")]
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
    #[doc(alias = "SnowDemoWindow")]
    pub fn show_demo_window(&self, opened: &mut bool) {
        unsafe {
            sys::igShowDemoWindow(opened);
        }
    }
    /// Renders an about window.
    ///
    /// Displays the Dear ImGui version/credits, and build/system information.
    #[doc(alias = "ShowAboutWindow")]
    pub fn show_about_window(&self, opened: &mut bool) {
        unsafe {
            sys::igShowAboutWindow(opened);
        }
    }
    /// Renders a metrics/debug window.
    ///
    /// Displays Dear ImGui internals: draw commands (with individual draw calls and vertices),
    /// window list, basic internal state, etc.
    #[doc(alias = "ShowMetricsWindow")]
    pub fn show_metrics_window(&self, opened: &mut bool) {
        unsafe {
            sys::igShowMetricsWindow(opened);
        }
    }
    /// Renders a style editor block (not a window) for the given `Style` structure
    #[doc(alias = "ShowStyleEditor")]
    pub fn show_style_editor(&self, style: &mut Style) {
        unsafe {
            sys::igShowStyleEditor(style.raw_mut());
        }
    }
    /// Renders a style editor block (not a window) for the currently active style
    #[doc(alias = "ShowStyleEditor")]
    pub fn show_default_style_editor(&self) {
        unsafe { sys::igShowStyleEditor(ptr::null_mut()) };
    }
    /// Renders a basic help/info block (not a window)
    #[doc(alias = "ShowUserGuide")]
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
    ImGuiID(sys::ImGuiID),
}

impl<'a> Id<'a> {
    fn as_imgui_id(&self) -> sys::ImGuiID {
        unsafe {
            match self {
                Id::Ptr(p) => sys::igGetID_Ptr(*p),
                Id::Str(s) => {
                    let s1 = s.as_ptr() as *const std::os::raw::c_char;
                    let s2 = s1.add(s.len());
                    sys::igGetID_StrStr(s1, s2)
                }
                Id::Int(i) => {
                    let p = *i as *const std::os::raw::c_void;
                    sys::igGetID_Ptr(p)
                }
                Id::ImGuiID(n) => *n,
            }
        }
    }
}

impl From<i32> for Id<'static> {
    #[inline]
    fn from(i: i32) -> Self {
        Id::Int(i)
    }
}

impl<'a, T: ?Sized + AsRef<str>> From<&'a T> for Id<'a> {
    #[inline]
    fn from(s: &'a T) -> Self {
        Id::Str(s.as_ref())
    }
}

impl<T> From<*const T> for Id<'static> {
    #[inline]
    fn from(p: *const T) -> Self {
        Id::Ptr(p as *const c_void)
    }
}

impl<T> From<*mut T> for Id<'static> {
    #[inline]
    fn from(p: *mut T) -> Self {
        Id::Ptr(p as *const T as *const c_void)
    }
}

// Widgets: Input
impl<'ui> Ui<'ui> {
    #[doc(alias = "InputText", alias = "InputTextWithHint")]
    pub fn input_text<'p>(&self, label: &'p ImStr, buf: &'p mut ImString) -> InputText<'ui, 'p> {
        InputText::new(self, label, buf)
    }
    #[doc(alias = "InputText", alias = "InputTextMultiline")]
    pub fn input_text_multiline<'p>(
        &self,
        label: &'p ImStr,
        buf: &'p mut ImString,
        size: [f32; 2],
    ) -> InputTextMultiline<'ui, 'p> {
        InputTextMultiline::new(self, label, buf, size)
    }
    #[doc(alias = "InputFloat2")]
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
    #[doc(alias = "InputFloat3")]
    pub fn input_float3<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 3],
    ) -> InputFloat3<'ui, 'p> {
        InputFloat3::new(self, label, value)
    }
    #[doc(alias = "InputFloat4")]
    pub fn input_float4<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 4],
    ) -> InputFloat4<'ui, 'p> {
        InputFloat4::new(self, label, value)
    }
    #[doc(alias = "InputInt")]
    pub fn input_int<'p>(&self, label: &'p ImStr, value: &'p mut i32) -> InputInt<'ui, 'p> {
        InputInt::new(self, label, value)
    }
    #[doc(alias = "InputInt2")]
    pub fn input_int2<'p>(&self, label: &'p ImStr, value: &'p mut [i32; 2]) -> InputInt2<'ui, 'p> {
        InputInt2::new(self, label, value)
    }
    #[doc(alias = "InputInt3")]
    pub fn input_int3<'p>(&self, label: &'p ImStr, value: &'p mut [i32; 3]) -> InputInt3<'ui, 'p> {
        InputInt3::new(self, label, value)
    }
    #[doc(alias = "InputInt4")]
    pub fn input_int4<'p>(&self, label: &'p ImStr, value: &'p mut [i32; 4]) -> InputInt4<'ui, 'p> {
        InputInt4::new(self, label, value)
    }
}

create_token!(
    /// Tracks a layout tooltip that can be ended by calling `.end()` or by dropping.
    pub struct TooltipToken<'ui>;

    /// Drops the layout tooltip manually. You can also just allow this token
    /// to drop on its own.
    drop { sys::igEndTooltip() }
);

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
    #[doc(alias = "BeginTooltip", alias = "EndTootip")]
    pub fn tooltip<F: FnOnce()>(&self, f: F) {
        unsafe { sys::igBeginTooltip() };
        f();
        unsafe { sys::igEndTooltip() };
    }
    /// Construct a tooltip window that can have any kind of content.
    ///
    /// Returns a `TooltipToken` that must be ended by calling `.end()`
    #[doc(alias = "BeginTooltip")]
    pub fn begin_tooltip(&self) -> TooltipToken<'_> {
        unsafe { sys::igBeginTooltip() };
        TooltipToken::new(self)
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
    #[doc(alias = "BeginTooltip", alias = "EndTootip")]
    pub fn tooltip_text<T: AsRef<str>>(&self, text: T) {
        self.tooltip(|| self.text(text));
    }
}

// Widgets: ListBox
impl<'ui> Ui<'ui> {
    #[doc(alias = "ListBox")]
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
            sys::igListBox_Str_arr(
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
    #[doc(alias = "PlotLines")]
    pub fn plot_lines<'p>(&self, label: &'p ImStr, values: &'p [f32]) -> PlotLines<'ui, 'p> {
        PlotLines::new(self, label, values)
    }
}

impl<'ui> Ui<'ui> {
    #[doc(alias = "PlotHistogram")]
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
    /// This is the same as [calc_text_size_with_opts](Self::calc_text_size_with_opts)
    /// with `hide_text_after_double_hash` set to false and `wrap_width` set to `-1.0`.
    #[doc(alias = "CalcTextSize")]
    pub fn calc_text_size<T: AsRef<str>>(&self, text: T) -> [f32; 2] {
        self.calc_text_size_with_opts(text, false, -1.0)
    }

    /// Calculate the size required for a given text string.
    ///
    /// hide_text_after_double_hash allows the user to insert comments into their text, using a double hash-tag prefix.
    /// This is a feature of imgui.
    ///
    /// wrap_width allows you to request a width at which to wrap the text to a newline for the calculation.
    #[doc(alias = "CalcTextSize")]
    pub fn calc_text_size_with_opts<T: AsRef<str>>(
        &self,
        text: T,
        hide_text_after_double_hash: bool,
        wrap_width: f32,
    ) -> [f32; 2] {
        let mut out = sys::ImVec2::zero();
        let text = text.as_ref();

        unsafe {
            let start = text.as_ptr();
            let end = start.add(text.len());

            sys::igCalcTextSize(
                &mut out,
                start as *const c_char,
                end as *const c_char,
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
    /// This function will panic if several instances of [`DrawListMut`]
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
    #[doc(alias = "GetWindowDrawList")]
    pub fn get_window_draw_list(&'ui self) -> DrawListMut<'ui> {
        DrawListMut::window(self)
    }

    #[must_use]
    #[doc(alias = "GetBackgroundDrawList")]
    pub fn get_background_draw_list(&'ui self) -> DrawListMut<'ui> {
        DrawListMut::background(self)
    }

    #[must_use]
    #[doc(alias = "GetForegroundDrawList")]
    pub fn get_foreground_draw_list(&'ui self) -> DrawListMut<'ui> {
        DrawListMut::foreground(self)
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


pub struct DockNodeFlags;
impl DockNodeFlags {
    pub const NONE: i32 = sys::ImGuiDockNodeFlags_None as i32;
    pub const KEEP_ALIVE_ONLY: i32 = sys::ImGuiDockNodeFlags_KeepAliveOnly as i32;
    pub const NO_DOCKING_IN_CENTRAL_NODE: i32 = sys::ImGuiDockNodeFlags_NoDockingInCentralNode as i32;
    pub const PASS_THRU_CENTRAL_NODE: i32 = sys::ImGuiDockNodeFlags_PassthruCentralNode as i32;
    pub const NO_SPLIT: i32 = sys::ImGuiDockNodeFlags_NoSplit as i32;
    pub const NO_RESIZE: i32 = sys::ImGuiDockNodeFlags_NoResize as i32;
    pub const AUTO_HIDE_TAB_BAR: i32 = sys::ImGuiDockNodeFlags_AutoHideTabBar as i32;

    pub const DOCK_SPACE: i32 = sys::ImGuiDockNodeFlags_DockSpace;
    pub const CENTRAL_NODE: i32 = sys::ImGuiDockNodeFlags_CentralNode;
    pub const NO_TAB_BAR: i32 = sys::ImGuiDockNodeFlags_NoTabBar;
    pub const HIDDEN_TAB_BAR: i32 = sys::ImGuiDockNodeFlags_HiddenTabBar;
    pub const NO_WINDOW_MENU_BUTTON: i32 = sys::ImGuiDockNodeFlags_NoWindowMenuButton;
    pub const NO_CLOSE_BUTTON: i32 = sys::ImGuiDockNodeFlags_NoCloseButton;
    pub const NO_DOCKING: i32 = sys::ImGuiDockNodeFlags_NoDocking;
    pub const NO_DOCKING_SPLIT_ME: i32 = sys::ImGuiDockNodeFlags_NoDockingSplitMe;
    pub const NO_DOCKING_SPLIT_OTHER: i32 = sys::ImGuiDockNodeFlags_NoDockingSplitOther;
    pub const NO_DOCKING_OVER_ME: i32 = sys::ImGuiDockNodeFlags_NoDockingOverMe;
    pub const NO_DOCKING_OVER_OTHER: i32 = sys::ImGuiDockNodeFlags_NoDockingOverOther;
    pub const NO_RESIZE_X: i32 = sys::ImGuiDockNodeFlags_NoResizeX;
    pub const NO_RESIZE_Y: i32 = sys::ImGuiDockNodeFlags_NoResizeY;
}


impl<'ui> Ui<'ui> {
    pub fn dock_space(&self, id: Id, size: [f32; 2]) {
        unsafe {
            sys::igDockSpace(id.as_imgui_id(), size.into(), 0, std::ptr::null());
        }
    }

//    pub fn dock_space_over_viewport() -> ImGuiID {
//    }
//    pub fn set_next_window_class() -> ImGuiID {
//    }

    pub fn set_next_window_dock_id(&self, id: Id, cond: Condition) {
        unsafe {
            sys::igSetNextWindowDockID(id.as_imgui_id(), cond as i32);
        }
    }

    pub fn get_window_dock_id(&self) -> Id {
        unsafe {
            Id::ImGuiID(sys::igGetWindowDockID())
        }
    }

    pub fn is_window_docked(&self) -> bool {
        unsafe {
            sys::igIsWindowDocked()
        }
    }

    pub fn dock_builder_has_node(&self, id: Id) -> bool {
        unsafe {
            sys::igDockBuilderGetNode(id.as_imgui_id()) != std::ptr::null_mut()
        }
    }

    pub fn dock_builder_remove_node(&self, id: Id) {
        unsafe {
            sys::igDockBuilderRemoveNode(id.as_imgui_id());
        }
    }

    pub fn dock_builder_add_node(&self, id: Id, flags: i32) {
        unsafe {
            sys::igDockBuilderAddNode(id.as_imgui_id(), flags);
        }
    }

    pub fn dock_builder_set_node_size(&self, id: Id, size: [f32; 2]) {
        unsafe {
            sys::igDockBuilderSetNodeSize(id.as_imgui_id(), size.into());
        }
    }

    pub fn dock_builder_split_node(&self, id: Id, split_dir: Direction, split_ratio: f32) -> (Id<'static>, Id<'static>) {
        unsafe {
            let mut opposite: sys::ImGuiID = 0;
            let id1 = sys::igDockBuilderSplitNode(id.as_imgui_id(), split_dir as i32, split_ratio,
                std::ptr::null_mut(),
                &mut opposite as *mut u32);
            (Id::ImGuiID(id1), Id::ImGuiID(opposite))
        }
    }

    pub fn dock_builder_dock_window<'p>(&self, window_name: &'p ImStr, id: Id) {
        unsafe {
            sys::igDockBuilderDockWindow(window_name.as_ptr(), id.as_imgui_id());
        }
    }

    pub fn dock_builder_finish(&self, id: Id) {
        unsafe {
            sys::igDockBuilderFinish(id.as_imgui_id());
        }
    }
}
