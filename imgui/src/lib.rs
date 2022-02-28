#![cfg_attr(test, allow(clippy::float_cmp))]
#![deny(rust_2018_idioms)]
// #![deny(missing_docs)]

pub extern crate imgui_sys as sys;

use std::cell;
use std::os::raw::c_char;

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
pub use self::input_widget::*;
pub use self::io::*;
pub use self::platform_io::*;
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

#[cfg(feature = "tables-api")]
pub use self::tables::*;
pub use self::utils::*;
pub use self::widget::color_editors::*;
pub use self::widget::combo_box::*;
pub use self::widget::drag::*;
pub use self::widget::image::*;
pub use self::widget::list_box::*;
pub use self::widget::menu::*;
pub use self::widget::misc::*;
pub use self::widget::progress_bar::*;
pub use self::widget::selectable::*;
pub use self::widget::slider::*;
pub use self::widget::tab::*;
pub use self::widget::tree::*;
pub use self::window::child_window::*;
pub use self::window::*;
use internal::RawCast;
use math::*;

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
mod platform_io;
mod layout;
mod list_clipper;
mod math;
mod plothistogram;
mod plotlines;
mod popups;
mod render;
mod stacks;
mod style;
#[cfg(feature = "tables-api")]
mod tables;
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
        let bytes = std::ffi::CStr::from_ptr(sys::igGetVersion()).to_bytes();
        std::str::from_utf8_unchecked(bytes)
    }
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

/// A reference for building the user interface for one frame
#[derive(Debug)]
pub struct Ui {
    /// our scratch sheet
    buffer: cell::UnsafeCell<string::UiBuffer>,
}

impl Ui {
    /// This provides access to the backing scratch buffer that we use to write
    /// strings, along with null-terminators, before we pass normal Rust strs to
    /// Dear ImGui.
    ///
    /// This is given as a get-out-of-jail free card if you need to handle the buffer,
    /// or, for example, resize it for some reason. Generally, you should never need this.
    ///
    /// ## Safety
    ///
    /// This uses a **static mut** and we assume it will *never* be passed between threads.
    /// Do not pass the raw pointer you get between threads at all -- Dear ImGui is single-threaded.
    /// We otherwise make no assumptions about the size or keep state in this buffer between calls,
    /// so editing the `UiBuffer` is fine.
    pub unsafe fn scratch_buffer(&self) -> &cell::UnsafeCell<string::UiBuffer> {
        &self.buffer
    }

    /// Internal method to push a single text to our scratch buffer.
    fn scratch_txt(&self, txt: impl AsRef<str>) -> *const sys::cty::c_char {
        unsafe {
            let handle = &mut *self.buffer.get();
            handle.scratch_txt(txt)
        }
    }

    /// Internal method to push an option text to our scratch buffer.
    fn scratch_txt_opt(&self, txt: Option<impl AsRef<str>>) -> *const sys::cty::c_char {
        unsafe {
            let handle = &mut *self.buffer.get();
            handle.scratch_txt_opt(txt)
        }
    }

    fn scratch_txt_two(
        &self,
        txt_0: impl AsRef<str>,
        txt_1: impl AsRef<str>,
    ) -> (*const sys::cty::c_char, *const sys::cty::c_char) {
        unsafe {
            let handle = &mut *self.buffer.get();
            handle.scratch_txt_two(txt_0, txt_1)
        }
    }

    fn scratch_txt_with_opt(
        &self,
        txt_0: impl AsRef<str>,
        txt_1: Option<impl AsRef<str>>,
    ) -> (*const sys::cty::c_char, *const sys::cty::c_char) {
        unsafe {
            let handle = &mut *self.buffer.get();
            handle.scratch_txt_with_opt(txt_0, txt_1)
        }
    }

    /// Returns an immutable reference to the inputs/outputs object
    #[doc(alias = "GetIO")]
    pub fn io(&self) -> &Io {
        unsafe { &*(sys::igGetIO() as *const Io) }
    }

    /// Returns an immutable reference to the font atlas.
    pub fn fonts(&self) -> &FontAtlas {
        unsafe { &*(self.io().fonts as *const FontAtlas) }
    }

    /// Returns a clone of the user interface style
    pub fn clone_style(&self) -> Style {
        unsafe { *self.style() }
    }

    /// This function, and the library's api, has been changed as of `0.9`!
    /// Do not use this function! Instead, use [`Context::render`],
    /// which does what this function in `0.8` used to do.
    ///
    /// This function right now simply **ends** the current frame, but does not
    /// return draw data. If you want to end the frame without generated draw data,
    /// and thus save some CPU time, use [`end_frame_early`].
    ///
    /// [`end_frame_early`]: Self::end_frame_early
    #[deprecated(
        since = "0.9.0",
        note = "use `Context::render` to render frames, or `end_frame_early` to not render at all"
    )]
    pub fn render(&mut self) {
        self.end_frame_early();
    }

    /// Use this function to end the frame early.
    /// After this call, you should **stop using the `Ui` object till `new_frame` has been called.**
    ///
    /// You probably *don't want this function.* If you want to render your data, use `Context::render` now.
    pub fn end_frame_early(&mut self) {
        unsafe {
            sys::igEndFrame();
        }
    }
}

/// # Demo, debug, information
impl Ui {
    /// Renders a demo window (previously called a test window), which demonstrates most
    /// Dear Imgui features.
    #[doc(alias = "ShowDemoWindow")]
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
        unsafe { sys::igShowStyleEditor(std::ptr::null_mut()) };
    }
    /// Renders a basic help/info block (not a window)
    #[doc(alias = "ShowUserGuide")]
    pub fn show_user_guide(&self) {
        unsafe { sys::igShowUserGuide() };
    }
}

/// Unique ID used by widgets.
///
/// This represents a hash of the current stack of Ids used in ImGui + the
/// input provided. It is only used in a few places directly in the
/// codebase, but you can think of it as effectively allowing you to
/// run your Id hashing yourself.
///
/// Previously, this was erroneously constructed with `From` implementations.
/// Now, however, it is made from the `Ui` object directly, with a few
/// deprecated helper methods here.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Id(pub(crate) u32);

impl Id {
    #[allow(non_snake_case)]
    pub fn Int(input: i32, ui: &Ui) -> Self {
        ui.new_id_int(input)
    }

    #[allow(non_snake_case)]
    pub fn Str(input: impl AsRef<str>, ui: &Ui) -> Self {
        ui.new_id_str(input)
    }

    #[allow(non_snake_case)]
    pub fn Ptr<T>(input: &T, ui: &Ui) -> Self {
        ui.new_id_ptr(input)
    }
}

impl Ui {
    pub fn new_id(&self, input: usize) -> Id {
        let p = input as *const std::os::raw::c_void;
        let value = unsafe { sys::igGetIDPtr(p) };

        Id(value)
    }

    pub fn new_id_int(&self, input: i32) -> Id {
        let p = input as *const std::os::raw::c_void;
        let value = unsafe { sys::igGetIDPtr(p) };
        Id(value)
    }

    pub fn new_id_ptr<T>(&self, input: &T) -> Id {
        let p = input as *const T as *const sys::cty::c_void;
        let value = unsafe { sys::igGetIDPtr(p) };
        Id(value)
    }

    pub fn new_id_str(&self, s: impl AsRef<str>) -> Id {
        let s = s.as_ref();

        let s1 = s.as_ptr() as *const std::os::raw::c_char;
        let value = unsafe {
            let s2 = s1.add(s.len());
            sys::igGetIDStrStr(s1, s2)
        };
        Id(value)
    }
}

// /// Unique ID used by widgets
// pub enum Id<'a> {
//     Int(i32),
//     Str(&'a str),
//     Ptr(*const c_void),
// }

// impl From<i32> for Id<'static> {
//     #[inline]
//     fn from(i: i32) -> Self {
//         Id::Int(i)
//     }
// }

// impl<'a, T: ?Sized + AsRef<str>> From<&'a T> for Id<'a> {
//     #[inline]
//     fn from(s: &'a T) -> Self {
//         Id::Str(s.as_ref())
//     }
// }

// impl<T> From<*const T> for Id<'static> {
//     #[inline]
//     fn from(p: *const T) -> Self {
//         Id::Ptr(p as *const c_void)
//     }
// }

// impl<T> From<*mut T> for Id<'static> {
//     #[inline]
//     fn from(p: *mut T) -> Self {
//         Id::Ptr(p as *const T as *const c_void)
//     }
// }

// impl<'a> Id<'a> {
//     // this is used in the tables-api and possibly elsewhere,
//     // but not with just default features...
//     #[allow(dead_code)]
//     fn as_imgui_id(&self) -> sys::ImGuiID {
//         unsafe {
//             match self {
//                 Id::Ptr(p) => sys::igGetID_Ptr(*p),
//                 Id::Str(s) => {
//                     let s1 = s.as_ptr() as *const std::os::raw::c_char;
//                     let s2 = s1.add(s.len());
//                     sys::igGetID_StrStr(s1, s2)
//                 }
//                 Id::Int(i) => {
//                     let p = *i as *const std::os::raw::c_void;
//                     sys::igGetID_Ptr(p)
//                 } // Id::ImGuiID(n) => *n,
//             }
//         }
//     }
// }

impl Ui {
    /// # Windows
    /// Start constructing a window.
    ///
    /// This, like many objects in the library, uses the builder
    /// pattern to set optional arguments (like window size, flags,
    /// etc). Once all desired options are set, you must call either
    /// [`Window::build`] or [`Window::begin`] to
    /// actually create the window.
    ///
    /// # Examples
    ///
    /// Create a window using the closure based [`Window::build`]:
    /// ```no_run
    /// # let mut ctx = imgui::Context::create();
    /// # let ui = ctx.frame();
    /// ui.window("Example Window")
    ///     .size([100.0, 50.0], imgui::Condition::FirstUseEver)
    ///     .build(|| {
    ///         ui.text("An example");
    ///     });
    /// ```
    ///
    /// Same as [`Ui::window`] but using the "token based" `.begin()` approach.
    ///
    /// ```no_run
    /// # let mut ctx = imgui::Context::create();
    /// # let ui = ctx.frame();
    /// if let Some(wt) = ui
    ///     .window("Example Window")
    ///     .size([100.0, 50.0], imgui::Condition::FirstUseEver)
    ///     .begin()
    /// {
    ///     ui.text("Window is visible");
    ///     // Window ends where where wt is dropped,
    ///     // or you could call
    ///     // if you want to let it drop on its own, name it `_wt`.
    ///     // never name it `_`, as this will drop it *immediately*.
    ///     wt.end();
    /// };
    /// ```
    pub fn window<Label: AsRef<str>>(&self, name: Label) -> Window<'_, '_, Label> {
        #[allow(deprecated)]
        Window::new(self, name)
    }

    /// Begins constructing a child window with the given name.
    ///
    /// Use child windows to begin into a self-contained independent scrolling/clipping
    /// regions within a host window. Child windows can embed their own child.
    pub fn child_window<Label: AsRef<str>>(&self, name: Label) -> ChildWindow<'_> {
        #[allow(deprecated)]
        ChildWindow::new(self, name)
    }

    /// Begins constructing a child window with the given name.
    ///
    /// Use child windows to begin into a self-contained independent scrolling/clipping
    /// regions within a host window. Child windows can embed their own child.
    pub fn child_window_id(&self, id: Id) -> ChildWindow<'_> {
        ChildWindow::new_id(self, id)
    }
}

// Widgets: Input
impl<'ui> Ui {
    #[doc(alias = "InputText", alias = "InputTextWithHint")]
    pub fn input_text<'p, L: AsRef<str>>(
        &'ui self,
        label: L,
        buf: &'p mut String,
    ) -> InputText<'ui, 'p, L> {
        InputText::new(self, label, buf)
    }
    #[doc(alias = "InputText", alias = "InputTextMultiline")]
    pub fn input_text_multiline<'p, L: AsRef<str>>(
        &'ui self,
        label: L,
        buf: &'p mut String,
        size: [f32; 2],
    ) -> InputTextMultiline<'ui, 'p, L> {
        InputTextMultiline::new(self, label, buf, size)
    }
    #[doc(alias = "InputFloat2")]
    pub fn input_float<'p, L: AsRef<str>>(
        &'ui self,
        label: L,
        value: &'p mut f32,
    ) -> InputScalar<'ui, 'p, f32, L> {
        self.input_scalar(label, value)
    }
    #[doc(alias = "InputFloat2")]
    pub fn input_float2<'p, L, T>(
        &'ui self,
        label: L,
        value: &'p mut T,
    ) -> InputFloat2<'ui, 'p, L, T>
    where
        L: AsRef<str>,
        T: Copy + Into<MintVec2>,
        MintVec2: Into<T> + Into<[f32; 2]>,
    {
        InputFloat2::new(self, label, value)
    }
    #[doc(alias = "InputFloat3")]
    pub fn input_float3<'p, L, T>(
        &'ui self,
        label: L,
        value: &'p mut T,
    ) -> InputFloat3<'ui, 'p, L, T>
    where
        L: AsRef<str>,
        T: Copy + Into<MintVec3>,
        MintVec3: Into<T> + Into<[f32; 3]>,
    {
        InputFloat3::new(self, label, value)
    }
    #[doc(alias = "InputFloat4")]
    pub fn input_float4<'p, L, T>(
        &'ui self,
        label: L,
        value: &'p mut T,
    ) -> InputFloat4<'ui, 'p, L, T>
    where
        L: AsRef<str>,
        T: Copy + Into<MintVec4>,
        MintVec4: Into<T> + Into<[f32; 4]>,
    {
        InputFloat4::new(self, label, value)
    }
    #[doc(alias = "InputInt")]
    pub fn input_int<'p, L: AsRef<str>>(
        &'ui self,
        label: L,
        value: &'p mut i32,
    ) -> InputScalar<'ui, 'p, i32, L> {
        self.input_scalar(label, value).step(1)
    }
    #[doc(alias = "InputInt2")]
    pub fn input_int2<'p, L, T>(&'ui self, label: L, value: &'p mut T) -> InputInt2<'ui, 'p, L, T>
    where
        L: AsRef<str>,
        T: Copy + Into<MintIVec2>,
        MintIVec2: Into<T> + Into<[i32; 2]>,
    {
        InputInt2::new(self, label, value)
    }
    #[doc(alias = "InputInt3")]
    pub fn input_int3<'p, L, T>(&'ui self, label: L, value: &'p mut T) -> InputInt3<'ui, 'p, L, T>
    where
        L: AsRef<str>,
        T: Copy + Into<MintIVec3>,
        MintIVec3: Into<T> + Into<[i32; 3]>,
    {
        InputInt3::new(self, label, value)
    }
    #[doc(alias = "InputInt4")]
    pub fn input_int4<'p, L, T>(&'ui self, label: L, value: &'p mut T) -> InputInt4<'ui, 'p, L, T>
    where
        L: AsRef<str>,
        T: Copy + Into<MintIVec4>,
        MintIVec4: Into<T> + Into<[i32; 4]>,
    {
        InputInt4::new(self, label, value)
    }
    /// Shows an input field for a scalar value. This is not limited to `f32` and `i32` and can be used with
    /// any primitive scalar type e.g. `u8` and `f64`.
    #[doc(alias = "InputScalar")]
    pub fn input_scalar<'p, L, T>(
        &'ui self,
        label: L,
        value: &'p mut T,
    ) -> InputScalar<'ui, 'p, T, L>
    where
        L: AsRef<str>,
        T: internal::DataTypeKind,
    {
        InputScalar::new(self, label, value)
    }
    /// Shows a horizontal array of scalar value input fields. See [`input_scalar`].
    ///
    /// [`input_scalar`]: Self::input_scalar
    #[doc(alias = "InputScalarN")]
    pub fn input_scalar_n<'p, L, T>(
        &'ui self,
        label: L,
        values: &'p mut [T],
    ) -> InputScalarN<'ui, 'p, T, L>
    where
        L: AsRef<str>,
        T: internal::DataTypeKind,
    {
        InputScalarN::new(self, label, values)
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
impl Ui {
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

create_token!(
    /// Starts a scope where interaction is disabled. Ends be calling `.end()` or when the token is dropped.
    pub struct DisabledToken<'ui>;

    /// Drops the layout tooltip manually. You can also just allow this token
    /// to drop on its own.
    drop { sys::igEndDisabled() }
);

/// # Disabling widgets
///
/// imgui can disable widgets so they don't react to mouse/keyboard
/// inputs, and are displayed differently (currently dimmed by an
/// amount set in [`Style::disabled_alpha`])
impl Ui {
    /// Creates a scope where interactions are disabled.
    ///
    /// Scope ends when returned token is dropped, or `.end()` is
    /// explicitly called
    ///
    /// # Examples
    ///
    /// ```
    /// # use imgui::*;
    /// fn user_interface(ui: &Ui) {
    ///     let disable_buttons = true;
    ///     let _d = ui.begin_disabled(disable_buttons);
    ///     ui.button(im_str!("Dangerous button"));
    /// }
    /// ```

    #[doc(alias = "BeginDisabled")]
    pub fn begin_disabled(&self, disabled: bool) -> DisabledToken<'_> {
        unsafe { sys::igBeginDisabled(disabled) };
        DisabledToken::new(self)
    }

    /// Identical to [`Ui::begin_disabled`] but exists to allow avoiding a
    /// double-negative, for example `begin_enabled(enable_buttons)`
    /// instead of `begin_disabled(!enable_buttons)`)
    #[doc(alias = "BeginDisabled")]
    pub fn begin_enabled(&self, enabled: bool) -> DisabledToken<'_> {
        self.begin_disabled(!enabled)
    }

    /// Helper to create a disabled section of widgets
    ///
    /// # Examples
    ///
    /// ```
    /// # use imgui::*;
    /// fn user_interface(ui: &Ui) {
    ///     let safe_mode = true;
    ///     ui.disabled(safe_mode, || {
    ///         ui.button(im_str!("Dangerous button"));
    ///     });
    /// }
    /// ```
    #[doc(alias = "BeginDisabled", alias = "EndDisabled")]
    pub fn disabled<F: FnOnce()>(&self, disabled: bool, f: F) {
        unsafe { sys::igBeginDisabled(disabled) };
        f();
        unsafe { sys::igEndDisabled() };
    }

    /// Same as [`Ui::disabled`] but with logic reversed. See
    /// [`Ui::begin_enabled`].
    #[doc(alias = "BeginDisabled", alias = "EndDisabled")]
    pub fn enabled<F: FnOnce()>(&self, enabled: bool, f: F) {
        unsafe { sys::igBeginDisabled(!enabled) };
        f();
        unsafe { sys::igEndDisabled() };
    }
}

// Widgets: ListBox
impl Ui {
    #[doc(alias = "ListBox")]
    pub fn list_box<'p, StringType: AsRef<str> + ?Sized>(
        &self,
        label: impl AsRef<str>,
        current_item: &mut i32,
        items: &'p [&'p StringType],
        height_in_items: i32,
    ) -> bool {
        let (label_ptr, items_inner) = unsafe {
            let handle = &mut *self.scratch_buffer().get();

            handle.refresh_buffer();
            let label_ptr = handle.push(label);

            let items_inner: Vec<_> = items.iter().map(|&v| handle.push(v)).collect();

            (label_ptr, items_inner)
        };

        unsafe {
            sys::igListBoxStr_arr(
                label_ptr,
                current_item,
                items_inner.as_ptr() as *mut *const c_char,
                items_inner.len() as i32,
                height_in_items,
            )
        }
    }

    // written out for the future times...
    // #[doc(alias = "ListBox")]
    // pub fn list_box_const<'p, StringType: AsRef<str> + ?Sized, const N: usize>(
    //     &self,
    //     label: impl AsRef<str>,
    //     current_item: &mut i32,
    //     items: [&'p StringType; N],
    //     height_in_items: i32,
    // ) -> bool {
    //     let (label_ptr, items_inner) = unsafe {
    //         let handle = &mut *self.buffer.get();

    //         handle.refresh_buffer();
    //         let label_ptr = handle.push(label);

    //         let mut items_inner: [*const i8; N] = [std::ptr::null(); N];

    //         for (i, item) in items.iter().enumerate() {
    //             items_inner[i] = handle.push(item);
    //         }

    //         (label_ptr, items_inner)
    //     };

    //     unsafe {
    //         sys::igListBoxStr_arr(
    //             label_ptr,
    //             current_item,
    //             items_inner.as_ptr() as *mut *const c_char,
    //             items_inner.len() as i32,
    //             height_in_items,
    //         )
    //     }
    // }
}

impl<'ui> Ui {
    #[doc(alias = "PlotLines")]
    pub fn plot_lines<'p, Label: AsRef<str>>(
        &'ui self,
        label: Label,
        values: &'p [f32],
    ) -> PlotLines<'ui, 'p, Label> {
        PlotLines::new(self, label, values)
    }

    #[doc(alias = "PlotHistogram")]
    pub fn plot_histogram<'p, Label: AsRef<str>>(
        &'ui self,
        label: Label,
        values: &'p [f32],
    ) -> PlotHistogram<'ui, 'p, Label> {
        PlotHistogram::new(self, label, values)
    }

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
impl Ui {
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
    pub fn get_window_draw_list(&self) -> DrawListMut<'_> {
        DrawListMut::window(self)
    }

    #[must_use]
    #[doc(alias = "GetBackgroundDrawList")]
    pub fn get_background_draw_list(&self) -> DrawListMut<'_> {
        DrawListMut::background(self)
    }

    #[must_use]
    #[doc(alias = "GetForegroundDrawList")]
    pub fn get_foreground_draw_list(&self) -> DrawListMut<'_> {
        DrawListMut::foreground(self)
    }
}

/// Condition for applying a setting
#[repr(i8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Condition {
    /// Never apply the setting
    Never = -1,

    /// Apply the setting every frame
    Always = sys::ImGuiCond_Always as i8,

    /// Apply the setting once per runtime session (only the first
    /// call will succeed). Will ignore any setting saved in `.ini`
    Once = sys::ImGuiCond_Once as i8,

    /// Apply the setting if the object/window has no persistently
    /// saved data (but otherwise use the setting from the .ini file)
    FirstUseEver = sys::ImGuiCond_FirstUseEver as i8,

    /// Apply the setting if the object/window is appearing after
    /// being hidden/inactive (or the first time)
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
