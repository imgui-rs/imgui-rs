pub extern crate imgui_sys as sys;
#[macro_use]
extern crate lazy_static;

use std::cell;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::ptr;
use std::str;
use std::thread;

pub use self::child_frame::ChildFrame;
pub use self::clipboard::*;
pub use self::color_editors::{
    ColorButton, ColorEdit, ColorEditMode, ColorFormat, ColorPicker, ColorPickerMode, ColorPreview,
    EditableColor,
};
pub use self::context::*;
pub use self::drag::{
    DragFloat, DragFloat2, DragFloat3, DragFloat4, DragFloatRange2, DragInt, DragInt2, DragInt3,
    DragInt4, DragIntRange2,
};
pub use self::fonts::atlas::*;
pub use self::fonts::font::*;
pub use self::fonts::glyph::*;
pub use self::fonts::glyph_ranges::*;
pub use self::image::{Image, ImageButton};
pub use self::input::keyboard::*;
pub use self::input::mouse::*;
pub use self::input_widget::{
    InputFloat, InputFloat2, InputFloat3, InputFloat4, InputInt, InputInt2, InputInt3, InputInt4,
    InputText, InputTextMultiline,
};
pub use self::io::*;
pub use self::legacy::*;
pub use self::menus::{Menu, MenuItem};
pub use self::plothistogram::PlotHistogram;
pub use self::plotlines::PlotLines;
pub use self::popup_modal::PopupModal;
pub use self::progressbar::ProgressBar;
pub use self::render::draw_data::*;
pub use self::render::renderer::*;
pub use self::sliders::{
    SliderFloat, SliderFloat2, SliderFloat3, SliderFloat4, SliderInt, SliderInt2, SliderInt3,
    SliderInt4,
};
pub use self::stacks::*;
pub use self::string::*;
pub use self::style::*;
pub use self::trees::{CollapsingHeader, TreeNode};
pub use self::window::Window;
pub use self::window_draw_list::{ChannelsSplit, ImColor, WindowDrawList};
use internal::RawCast;

mod child_frame;
mod clipboard;
mod color_editors;
mod context;
mod drag;
mod fonts;
mod image;
mod input;
mod input_widget;
pub mod internal;
mod io;
mod layout;
mod legacy;
mod menus;
mod plothistogram;
mod plotlines;
mod popup_modal;
mod progressbar;
mod render;
mod sliders;
mod stacks;
mod string;
mod style;
#[cfg(test)]
mod test;
mod trees;
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
    assert_eq!(dear_imgui_version(), "1.71");
}

impl Context {
    pub fn time(&self) -> f64 {
        unsafe { sys::igGetTime() }
    }
    pub fn frame_count(&self) -> i32 {
        unsafe { sys::igGetFrameCount() }
    }
}

pub struct Ui<'ui> {
    ctx: &'ui Context,
    font_atlas: Option<cell::RefMut<'ui, SharedFontAtlas>>,
}

static FMT: &'static [u8] = b"%s\0";

fn fmt_ptr() -> *const c_char {
    FMT.as_ptr() as *const c_char
}

impl<'ui> Ui<'ui> {
    pub fn io(&self) -> &Io {
        unsafe { &*(sys::igGetIO() as *const Io) }
    }
    pub fn fonts(&self) -> FontAtlasRef {
        match self.font_atlas {
            Some(ref font_atlas) => FontAtlasRef::Shared(font_atlas),
            None => unsafe {
                let fonts = &*(self.io().fonts as *const FontAtlas);
                FontAtlasRef::Owned(fonts)
            },
        }
    }
    pub fn clone_style(&self) -> Style {
        *self.ctx.style()
    }
    pub fn style_color(&self, style_color: StyleColor) -> [f32; 4] {
        self.ctx.style()[style_color]
    }
    pub fn time(&self) -> f64 {
        unsafe { sys::igGetTime() }
    }
    pub fn frame_count(&self) -> i32 {
        unsafe { sys::igGetFrameCount() }
    }
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

// Window
impl<'ui> Ui<'ui> {
    pub fn window<'p>(&self, name: &'p ImStr) -> Window<'ui, 'p> {
        Window::new(self, name)
    }
    /// Get current window's size in pixels
    pub fn get_window_size(&self) -> [f32; 2] {
        let size = unsafe { sys::igGetWindowSize_nonUDT2() };
        size.into()
    }
    /// Get current window's position in pixels
    pub fn get_window_pos(&self) -> [f32; 2] {
        let size = unsafe { sys::igGetWindowPos_nonUDT2() };
        size.into()
    }

    pub fn get_window_content_region_min(&self) -> [f32; 2] {
        let size = unsafe { sys::igGetWindowContentRegionMin_nonUDT2() };
        size.into()
    }

    pub fn get_window_content_region_max(&self) -> [f32; 2] {
        let size = unsafe { sys::igGetWindowContentRegionMax_nonUDT2() };
        size.into()
    }
}

// Layout
impl<'ui> Ui<'ui> {
    pub fn columns<'p>(&self, count: i32, id: &'p ImStr, border: bool) {
        unsafe { sys::igColumns(count, id.as_ptr(), border) }
    }

    pub fn next_column(&self) {
        unsafe { sys::igNextColumn() }
    }

    pub fn get_column_index(&self) -> i32 {
        unsafe { sys::igGetColumnIndex() }
    }

    pub fn get_column_offset(&self, column_index: i32) -> f32 {
        unsafe { sys::igGetColumnOffset(column_index) }
    }

    pub fn set_column_offset(&self, column_index: i32, offset_x: f32) {
        unsafe { sys::igSetColumnOffset(column_index, offset_x) }
    }

    pub fn get_column_width(&self, column_index: i32) -> f32 {
        unsafe { sys::igGetColumnWidth(column_index) }
    }

    pub fn get_columns_count(&self) -> i32 {
        unsafe { sys::igGetColumnsCount() }
    }

    /// Get cursor position on the screen, in screen coordinates.
    /// This sets the point on which the next widget will be drawn.
    ///
    /// This is especially useful for drawing, as the drawing API uses
    /// screen coordiantes.
    pub fn get_cursor_screen_pos(&self) -> [f32; 2] {
        let size = unsafe { sys::igGetCursorScreenPos_nonUDT2() };
        size.into()
    }

    /// Set cursor position on the screen, in screen coordinates.
    /// This sets the point on which the next widget will be drawn.
    pub fn set_cursor_screen_pos(&self, pos: [f32; 2]) {
        unsafe { sys::igSetCursorScreenPos(pos.into()) }
    }

    /// Get cursor position on the screen, in window coordinates.
    pub fn get_cursor_pos(&self) -> [f32; 2] {
        let size = unsafe { sys::igGetCursorPos_nonUDT2() };
        size.into()
    }

    /// Set cursor position on the screen, in window coordinates.
    /// This sets the point on which the next widget will be drawn.
    pub fn set_cursor_pos(&self, pos: [f32; 2]) {
        unsafe { sys::igSetCursorPos(pos.into()) }
    }

    pub fn get_content_region_max(&self) -> [f32; 2] {
        let size = unsafe { sys::igGetContentRegionMax_nonUDT2() };
        size.into()
    }

    /// Get available space left between the cursor and the edges of the current
    /// window.
    pub fn get_content_region_avail(&self) -> [f32; 2] {
        let size = unsafe { sys::igGetContentRegionAvail_nonUDT2() };
        size.into()
    }
}

pub enum ImId<'a> {
    Int(i32),
    Str(&'a str),
    Ptr(*const c_void),
}

impl From<i32> for ImId<'static> {
    fn from(i: i32) -> Self {
        ImId::Int(i)
    }
}

impl<'a, T: ?Sized + AsRef<str>> From<&'a T> for ImId<'a> {
    fn from(s: &'a T) -> Self {
        ImId::Str(s.as_ref())
    }
}

impl<T> From<*const T> for ImId<'static> {
    fn from(p: *const T) -> Self {
        ImId::Ptr(p as *const c_void)
    }
}

impl<T> From<*mut T> for ImId<'static> {
    fn from(p: *mut T) -> Self {
        ImId::Ptr(p as *const T as *const c_void)
    }
}

// ID scopes
impl<'ui> Ui<'ui> {
    /// Pushes an identifier to the ID stack.
    pub fn push_id<'a, I: Into<ImId<'a>>>(&self, id: I) {
        let id = id.into();

        unsafe {
            match id {
                ImId::Int(i) => {
                    sys::igPushIDInt(i);
                }
                ImId::Str(s) => {
                    let start = s.as_ptr() as *const c_char;
                    let end = start.add(s.len());
                    sys::igPushIDRange(start, end);
                }
                ImId::Ptr(p) => {
                    sys::igPushIDPtr(p as *const c_void);
                }
            }
        }
    }

    /// Pops an identifier from the ID stack.
    ///
    /// # Aborts
    /// The current process is aborted if the ID stack is empty.
    pub fn pop_id(&self) {
        unsafe { sys::igPopID() };
    }

    /// Runs a function after temporarily pushing a value to the ID stack.
    pub fn with_id<'a, F, I>(&self, id: I, f: F)
    where
        F: FnOnce(),
        I: Into<ImId<'a>>,
    {
        self.push_id(id);
        f();
        self.pop_id();
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
        InputTextMultiline::new(self, label, buf, size.into())
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

// Widgets: Drag
impl<'ui> Ui<'ui> {
    pub fn drag_float<'p>(&self, label: &'p ImStr, value: &'p mut f32) -> DragFloat<'ui, 'p> {
        DragFloat::new(self, label, value)
    }
    pub fn drag_float2<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 2],
    ) -> DragFloat2<'ui, 'p> {
        DragFloat2::new(self, label, value)
    }
    pub fn drag_float3<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 3],
    ) -> DragFloat3<'ui, 'p> {
        DragFloat3::new(self, label, value)
    }
    pub fn drag_float4<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 4],
    ) -> DragFloat4<'ui, 'p> {
        DragFloat4::new(self, label, value)
    }
    pub fn drag_float_range2<'p>(
        &self,
        label: &'p ImStr,
        current_min: &'p mut f32,
        current_max: &'p mut f32,
    ) -> DragFloatRange2<'ui, 'p> {
        DragFloatRange2::new(self, label, current_min, current_max)
    }
    pub fn drag_int<'p>(&self, label: &'p ImStr, value: &'p mut i32) -> DragInt<'ui, 'p> {
        DragInt::new(self, label, value)
    }
    pub fn drag_int2<'p>(&self, label: &'p ImStr, value: &'p mut [i32; 2]) -> DragInt2<'ui, 'p> {
        DragInt2::new(self, label, value)
    }
    pub fn drag_int3<'p>(&self, label: &'p ImStr, value: &'p mut [i32; 3]) -> DragInt3<'ui, 'p> {
        DragInt3::new(self, label, value)
    }
    pub fn drag_int4<'p>(&self, label: &'p ImStr, value: &'p mut [i32; 4]) -> DragInt4<'ui, 'p> {
        DragInt4::new(self, label, value)
    }
    pub fn drag_int_range2<'p>(
        &self,
        label: &'p ImStr,
        current_min: &'p mut i32,
        current_max: &'p mut i32,
    ) -> DragIntRange2<'ui, 'p> {
        DragIntRange2::new(self, label, current_min, current_max)
    }
}

// Widgets: Sliders
impl<'ui> Ui<'ui> {
    pub fn slider_float<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut f32,
        min: f32,
        max: f32,
    ) -> SliderFloat<'ui, 'p> {
        SliderFloat::new(self, label, value, min, max)
    }
    pub fn slider_float2<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 2],
        min: f32,
        max: f32,
    ) -> SliderFloat2<'ui, 'p> {
        SliderFloat2::new(self, label, value, min, max)
    }
    pub fn slider_float3<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 3],
        min: f32,
        max: f32,
    ) -> SliderFloat3<'ui, 'p> {
        SliderFloat3::new(self, label, value, min, max)
    }
    pub fn slider_float4<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [f32; 4],
        min: f32,
        max: f32,
    ) -> SliderFloat4<'ui, 'p> {
        SliderFloat4::new(self, label, value, min, max)
    }
    pub fn slider_int<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut i32,
        min: i32,
        max: i32,
    ) -> SliderInt<'ui, 'p> {
        SliderInt::new(self, label, value, min, max)
    }
    pub fn slider_int2<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [i32; 2],
        min: i32,
        max: i32,
    ) -> SliderInt2<'ui, 'p> {
        SliderInt2::new(self, label, value, min, max)
    }
    pub fn slider_int3<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [i32; 3],
        min: i32,
        max: i32,
    ) -> SliderInt3<'ui, 'p> {
        SliderInt3::new(self, label, value, min, max)
    }
    pub fn slider_int4<'p>(
        &self,
        label: &'p ImStr,
        value: &'p mut [i32; 4],
        min: i32,
        max: i32,
    ) -> SliderInt4<'ui, 'p> {
        SliderInt4::new(self, label, value, min, max)
    }
}

// Widgets: Color Editor/Picker
impl<'ui> Ui<'ui> {
    /// Constructs a new color editor builder.
    pub fn color_edit<'p, V: Into<EditableColor<'p>>>(
        &self,
        label: &'p ImStr,
        value: V,
    ) -> ColorEdit<'ui, 'p> {
        ColorEdit::new(self, label, value.into())
    }
    /// Constructs a new color picker builder.
    pub fn color_picker<'p, V: Into<EditableColor<'p>>>(
        &self,
        label: &'p ImStr,
        value: V,
    ) -> ColorPicker<'ui, 'p> {
        ColorPicker::new(self, label, value.into())
    }
    /// Constructs a new color button builder.
    pub fn color_button<'p>(&self, desc_id: &'p ImStr, color: [f32; 4]) -> ColorButton<'ui, 'p> {
        ColorButton::new(self, desc_id, color.into())
    }
    /// Initialize current options (generally on application startup) if you want to select a
    /// default format, picker type, etc. Users will be able to change many settings, unless you
    /// use .options(false) in your widget builders.
    pub fn set_color_edit_options(&self, flags: ImGuiColorEditFlags) {
        unsafe {
            sys::igSetColorEditOptions(flags.bits());
        }
    }
}

// Widgets: Trees
impl<'ui> Ui<'ui> {
    pub fn tree_node<'p>(&self, id: &'p ImStr) -> TreeNode<'ui, 'p> {
        TreeNode::new(self, id)
    }
    pub fn collapsing_header<'p>(&self, label: &'p ImStr) -> CollapsingHeader<'ui, 'p> {
        CollapsingHeader::new(self, label)
    }
}

// Widgets: Selectable / Lists
impl<'ui> Ui<'ui> {
    pub fn selectable(
        &self,
        label: &ImStr,
        selected: bool,
        flags: ImGuiSelectableFlags,
        size: [f32; 2],
    ) -> bool {
        unsafe { sys::igSelectable(label.as_ptr(), selected, flags.bits(), size.into()) }
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
    /// # fn main() {
    /// # }
    /// ```
    pub fn tooltip<F: FnOnce()>(&self, f: F) {
        unsafe { sys::igBeginTooltip() };
        f();
        unsafe { sys::igEndTooltip() };
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
    /// # fn main() {
    /// # }
    /// ```
    pub fn tooltip_text<T: AsRef<str>>(&self, text: T) {
        self.tooltip(|| self.text(text));
    }
}

// Widgets: Menus
impl<'ui> Ui<'ui> {
    pub fn main_menu_bar<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        let render = unsafe { sys::igBeginMainMenuBar() };
        if render {
            f();
            unsafe { sys::igEndMainMenuBar() };
        }
    }
    pub fn menu_bar<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        let render = unsafe { sys::igBeginMenuBar() };
        if render {
            f();
            unsafe { sys::igEndMenuBar() };
        }
    }
    pub fn menu<'p>(&self, label: &'p ImStr) -> Menu<'ui, 'p> {
        Menu::new(self, label)
    }
    pub fn menu_item<'p>(&self, label: &'p ImStr) -> MenuItem<'ui, 'p> {
        MenuItem::new(self, label)
    }
}

// Widgets: Popups
impl<'ui> Ui<'ui> {
    pub fn open_popup<'p>(&self, str_id: &'p ImStr) {
        unsafe { sys::igOpenPopup(str_id.as_ptr()) };
    }
    pub fn popup<'p, F>(&self, str_id: &'p ImStr, f: F)
    where
        F: FnOnce(),
    {
        let render =
            unsafe { sys::igBeginPopup(str_id.as_ptr(), ImGuiWindowFlags::empty().bits()) };
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

// Widgets: Combos
impl<'ui> Ui<'ui> {
    pub fn combo<'p, StringType: AsRef<ImStr> + ?Sized>(
        &self,
        label: &'p ImStr,
        current_item: &mut i32,
        items: &'p [&'p StringType],
        height_in_items: i32,
    ) -> bool {
        let items_inner: Vec<*const c_char> = items
            .into_iter()
            .map(|item| item.as_ref().as_ptr())
            .collect();
        unsafe {
            sys::igCombo(
                label.as_ptr(),
                current_item,
                items_inner.as_ptr() as *mut *const c_char,
                items_inner.len() as i32,
                height_in_items,
            )
        }
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
        let items_inner: Vec<*const c_char> = items
            .into_iter()
            .map(|item| item.as_ref().as_ptr())
            .collect();
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

// Image
impl<'ui> Ui<'ui> {
    pub fn image(&self, texture: TextureId, size: [f32; 2]) -> Image {
        Image::new(self, texture, size)
    }
}

// ImageButton
impl<'ui> Ui<'ui> {
    pub fn image_button(&self, texture: TextureId, size: [f32; 2]) -> ImageButton {
        ImageButton::new(self, texture, size)
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
        unsafe {
            sys::igCalcTextSize_nonUDT2(
                text.as_ptr(),
                std::ptr::null(),
                hide_text_after_double_hash,
                wrap_width,
            )
            .into()
        }
    }
}

impl<'ui> Ui<'ui> {
    /// Get height of a line of previously drawn text item
    pub fn get_text_line_height_with_spacing(&self) -> f32 {
        unsafe { sys::igGetTextLineHeightWithSpacing() }
    }
    /// Get previously drawn item's size
    pub fn get_item_rect_size(&self) -> [f32; 2] {
        let size = unsafe { sys::igGetItemRectSize_nonUDT2() };
        size.into()
    }
}

impl<'ui> Ui<'ui> {
    /// Creates a progress bar. Fraction is the progress level with 0.0 = 0% and 1.0 = 100%.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = Context::create();
    /// # let ui = imgui.frame();
    /// ui.progress_bar(0.6)
    ///     .size([100.0, 12.0])
    ///     .overlay_text(im_str!("Progress!"))
    ///     .build();
    /// ```
    pub fn progress_bar<'p>(&self, fraction: f32) -> ProgressBar<'ui, 'p> {
        ProgressBar::new(self, fraction)
    }
}

impl<'ui> Ui<'ui> {
    /// Creates a child frame. Size is size of child_frame within parent window.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = Context::create();
    /// # let ui = imgui.frame();
    /// ui.window(im_str!("ChatWindow"))
    ///     .title_bar(true)
    ///     .scrollable(false)
    ///     .build(|| {
    ///         ui.separator();
    ///
    ///         ui.child_frame(im_str!("child frame"), [400.0, 100.0])
    ///             .show_borders(true)
    ///             .always_show_vertical_scroll_bar(true)
    ///             .build(|| {
    ///                 ui.text_colored([1.0, 0.0, 0.0, 1.0], im_str!("hello mate!"));
    ///             });
    /// });
    pub fn child_frame<'p>(&self, name: &'p ImStr, size: [f32; 2]) -> ChildFrame<'ui, 'p> {
        ChildFrame::new(self, name, size)
    }
}

/// # Utilities
impl<'ui> Ui<'ui> {
    /// Returns `true` if the last item is being hovered by the mouse.
    ///
    /// # Examples
    ///
    /// ```
    /// # use imgui::*;
    /// fn user_interface(ui: &Ui) {
    ///     ui.text("Hover over me");
    ///     let is_hover_over_me_text_hovered = ui.is_item_hovered();
    /// }
    /// # fn main() {
    /// # }
    /// ```
    pub fn is_item_hovered(&self) -> bool {
        unsafe { sys::igIsItemHovered(ImGuiHoveredFlags::empty().bits()) }
    }

    pub fn is_item_hovered_with_flags(&self, flags: ImGuiHoveredFlags) -> bool {
        unsafe { sys::igIsItemHovered(flags.bits()) }
    }

    /// Return `true` if the current window is being hovered by the mouse.
    pub fn is_window_hovered(&self) -> bool {
        unsafe { sys::igIsWindowHovered(ImGuiHoveredFlags::empty().bits()) }
    }

    pub fn is_window_hovered_with_flags(&self, flags: ImGuiHoveredFlags) -> bool {
        unsafe { sys::igIsWindowHovered(flags.bits()) }
    }

    /// Return `true` if the current window is currently focused.
    pub fn is_window_focused(&self) -> bool {
        unsafe { sys::igIsWindowFocused(ImGuiFocusedFlags::RootAndChildWindows.bits()) }
    }

    /// Return `true` if the current root window is currently focused.
    pub fn is_root_window_focused(&self) -> bool {
        unsafe { sys::igIsWindowFocused(ImGuiFocusedFlags::RootWindow.bits()) }
    }

    /// Return `true` if the current child window is currently focused.
    pub fn is_child_window_focused(&self) -> bool {
        unsafe { sys::igIsWindowFocused(ImGuiFocusedFlags::ChildWindows.bits()) }
    }

    /// Returns `true` if the last item is being active.
    pub fn is_item_active(&self) -> bool {
        unsafe { sys::igIsItemActive() }
    }

    /// Allow last item to be overlapped by a subsequent item. Both may be activated during the same frame before the later one takes priority.
    pub fn set_item_allow_overlap(&self) {
        unsafe {
            sys::igSetItemAllowOverlap();
        }
    }

    /// Group items together as a single item.
    ///
    /// May be useful to handle the same mouse event on a group of items, for example.
    pub fn group<F: FnOnce()>(&self, f: F) {
        unsafe {
            sys::igBeginGroup();
        }
        f();
        unsafe {
            sys::igEndGroup();
        }
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
    pub fn get_window_draw_list(&'ui self) -> WindowDrawList<'ui> {
        WindowDrawList::new(self)
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
    Left = sys::ImGuiDir_Left,
    Right = sys::ImGuiDir_Right,
    Up = sys::ImGuiDir_Up,
    Down = sys::ImGuiDir_Down,
}
