#![allow(non_upper_case_globals)]

#[macro_use]
extern crate bitflags;

#[cfg(feature = "gfx")]
#[macro_use]
extern crate gfx;

#[cfg(feature = "glium")]
extern crate glium;

use std::convert::From;
use std::marker::PhantomData;
use std::mem;
use std::os::raw::{c_char, c_float, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};
use std::ptr;
use std::slice;

#[cfg(feature = "gfx")]
mod gfx_support;

#[cfg(feature = "glium")]
mod glium_support;

/// ImGui context (opaque)
pub enum ImGuiContext { }

/// 32-bit unsigned integer (typically used to store packed colors)
pub type ImU32 = c_uint;

/// Character for keyboard input/display
pub type ImWchar = c_ushort;

/// User data to identify a texture
pub type ImTextureID = *mut c_void;

/// Unique ID used by widgets (typically hashed from a stack of string)
pub type ImGuiID = ImU32;

/// A color identifier for styling
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImGuiCol {
    Text,
    TextDisabled,
    WindowBg,
    ChildWindowBg,
    PopupBg,
    Border,
    BorderShadow,
    FrameBg,
    FrameBgHovered,
    FrameBgActive,
    TitleBg,
    TitleBgActive,
    TitleBgCollapsed,
    MenuBarBg,
    ScrollbarBg,
    ScrollbarGrab,
    ScrollbarGrabHovered,
    ScrollbarGrabActive,
    ComboBg,
    CheckMark,
    SliderGrab,
    SliderGrabActive,
    Button,
    ButtonHovered,
    ButtonActive,
    Header,
    HeaderHovered,
    HeaderActive,
    Separator,
    SeparatorHovered,
    SeparatorActive,
    ResizeGrip,
    ResizeGripHovered,
    ResizeGripActive,
    CloseButton,
    CloseButtonHovered,
    CloseButtonActive,
    PlotLines,
    PlotLinesHovered,
    PlotHistogram,
    PlotHistogramHovered,
    TextSelectedBg,
    ModalWindowDarkening,
}
impl ImGuiCol {
    pub fn values() -> &'static [ImGuiCol] {
        use ImGuiCol::*;
        static values: &'static [ImGuiCol] = &[
            Text,
            TextDisabled,
            WindowBg,
            ChildWindowBg,
            PopupBg,
            Border,
            BorderShadow,
            FrameBg,
            FrameBgHovered,
            FrameBgActive,
            TitleBg,
            TitleBgActive,
            TitleBgCollapsed,
            MenuBarBg,
            ScrollbarBg,
            ScrollbarGrab,
            ScrollbarGrabHovered,
            ScrollbarGrabActive,
            ComboBg,
            CheckMark,
            SliderGrab,
            SliderGrabActive,
            Button,
            ButtonHovered,
            ButtonActive,
            Header,
            HeaderHovered,
            HeaderActive,
            Separator,
            SeparatorHovered,
            SeparatorActive,
            ResizeGrip,
            ResizeGripHovered,
            ResizeGripActive,
            CloseButton,
            CloseButtonHovered,
            CloseButtonActive,
            PlotLines,
            PlotLinesHovered,
            PlotHistogram,
            PlotHistogramHovered,
            TextSelectedBg,
            ModalWindowDarkening,
        ];
        values
    }
}
pub const ImGuiCol_COUNT: usize = 43;

/// A variable identifier for styling
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImGuiStyleVar {
    Alpha,
    WindowPadding,
    WindowRounding,
    WindowMinSize,
    ChildWindowRounding,
    FramePadding,
    FrameRounding,
    ItemSpacing,
    ItemInnerSpacing,
    IndentSpacing,
    GrabMinSize,
    ButtonTextAlign,
}
pub const ImGuiStyleVar_COUNT: usize = 12;

/// A key identifier (ImGui-side enum)
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImGuiKey {
    Tab,
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
    PageUp,
    PageDown,
    Home,
    End,
    Delete,
    Backspace,
    Enter,
    Escape,
    A,
    C,
    V,
    X,
    Y,
    Z,
}
pub const ImGuiKey_COUNT: usize = 19;

bitflags!(
    /// Color edit flags
    #[repr(C)]
    pub struct ImGuiColorEditFlags: c_int {
        const NoAlpha          = 1 << 1;
        const NoPicker         = 1 << 2;
        const NoOptions        = 1 << 3;
        const NoSmallPreview   = 1 << 4;
        const NoInputs         = 1 << 5;
        const NoTooltip        = 1 << 6;
        const NoLabel          = 1 << 7;
        const NoSidePreview    = 1 << 8;
        const AlphaBar         = 1 << 9;
        const AlphaPreview     = 1 << 10;
        const AlphaPreviewHalf = 1 << 11;
        const HDR              = 1 << 12;
        const RGB              = 1 << 13;
        const HSV              = 1 << 14;
        const HEX              = 1 << 15;
        const Uint8            = 1 << 16;
        const Float            = 1 << 17;
        const PickerHueBar     = 1 << 18;
        const PickerHueWheel   = 1 << 19;
    }
);

/// A mouse cursor identifier
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImGuiMouseCursor {
    None = -1,
    Arrow,
    TextInput,
    Move,
    ResizeNS,
    ResizeEW,
    ResizeNESW,
    ResizeNWSE,
}
pub const ImGuiMouseCursor_COUNT: usize = 7;

bitflags!(
    /// Window flags
    #[repr(C)]
    pub struct ImGuiWindowFlags: c_int {
        const NoTitleBar                = 1;
        const NoResize                  = 1 << 1;
        const NoMove                    = 1 << 2;
        const NoScrollbar               = 1 << 3;
        const NoScrollWithMouse         = 1 << 4;
        const NoCollapse                = 1 << 5;
        const AlwaysAutoResize          = 1 << 6;
        const ShowBorders               = 1 << 7;
        const NoSavedSettings           = 1 << 8;
        const NoInputs                  = 1 << 9;
        const MenuBar                   = 1 << 10;
        const HorizontalScrollbar       = 1 << 11;
        const NoFocusOnAppearing        = 1 << 12;
        const NoBringToFrontOnFocus     = 1 << 13;
        const AlwaysVerticalScrollbar   = 1 << 14;
        const AlwaysHorizontalScrollbar = 1 << 15;
        const AlwaysUseWindowPadding    = 1 << 16;
    }
);

bitflags!(
    /// Condition flags
    #[repr(C)]
    pub struct ImGuiCond: c_int {
        const Always       = 1;
        const Once         = 1 << 1;
        const FirstUseEver = 1 << 2;
        const Appearing    = 1 << 3;
    }
);

bitflags!(
    /// Flags for text inputs
    #[repr(C)]
    pub struct ImGuiInputTextFlags: c_int {
        const CharsDecimal        = 1;
        const CharsHexadecimal    = 1 << 1;
        const CharsUppercase      = 1 << 2;
        const CharsNoBlank        = 1 << 3;
        const AutoSelectAll       = 1 << 4;
        const EnterReturnsTrue    = 1 << 5;
        const CallbackCompletion  = 1 << 6;
        const CallbackHistory     = 1 << 7;
        const CallbackAlways      = 1 << 8;
        const CallbackCharFilter  = 1 << 9;
        const AllowTabInput       = 1 << 10;
        const CtrlEnterForNewLine = 1 << 11;
        const NoHorizontalScroll  = 1 << 12;
        const AlwaysInsertMode    = 1 << 13;
        const ReadOnly            = 1 << 14;
        const Password            = 1 << 15;
    }
);

bitflags!(
    /// Flags for selectables
    #[repr(C)]
    pub struct ImGuiSelectableFlags: c_int {
        const DontClosePopups  = 1;
        const SpanAllColumns   = 1 << 1;
        const AllowDoubleClick = 1 << 2;
    }
);

bitflags!(
    /// Flags for trees and collapsing headers
    #[repr(C)]
    pub struct ImGuiTreeNodeFlags: c_int {
        const Selected          = 1;
        const Framed            = 1 << 1;
        const AllowOverlapMode  = 1 << 2;
        const NoTreePushOnOpen  = 1 << 3;
        const NoAutoOpenOnLog   = 1 << 4;
        const DefaultOpen       = 1 << 5;
        const OpenOnDoubleClick = 1 << 6;
        const OpenOnArrow       = 1 << 7;
        const Leaf              = 1 << 8;
        const Bullet            = 1 << 9;
        const FramePadding      = 1 << 10;
        const CollapsingHeader  =
            ImGuiTreeNodeFlags::Framed.bits | ImGuiTreeNodeFlags::NoAutoOpenOnLog.bits;
    }
);

bitflags!(
    /// Flags for hover checks
    #[repr(C)]
    pub struct ImGuiHoveredFlags: c_int {
        const AllowWhenBlockedByPopup      = 1;
        const AllowWhenBlockedByActiveItem = 1 << 2;
        const AllowWhenOverlapped          = 1 << 3;
        const RectOnly = ImGuiHoveredFlags::AllowWhenBlockedByPopup.bits
            | ImGuiHoveredFlags::AllowWhenBlockedByActiveItem.bits
            | ImGuiHoveredFlags::AllowWhenOverlapped.bits;
    }
);

pub type ImGuiTextEditCallback = Option<
    extern "C" fn(data: *mut ImGuiTextEditCallbackData) -> c_int,
>;

pub type ImGuiSizeConstraintCallback = Option<
    extern "C" fn(data: *mut ImGuiSizeConstraintCallbackData),
>;

/// A tuple of 2 floating-point values
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ImVec2 {
    pub x: c_float,
    pub y: c_float,
}

impl ImVec2 {
    pub fn new(x: f32, y: f32) -> ImVec2 {
        ImVec2 {
            x: x as c_float,
            y: y as c_float,
        }
    }
    pub fn zero() -> ImVec2 {
        ImVec2 {
            x: 0.0 as c_float,
            y: 0.0 as c_float,
        }
    }
}

impl From<[f32; 2]> for ImVec2 {
    fn from(array: [f32; 2]) -> ImVec2 { ImVec2::new(array[0], array[1]) }
}

impl From<(f32, f32)> for ImVec2 {
    fn from((x, y): (f32, f32)) -> ImVec2 { ImVec2::new(x, y) }
}

impl Into<[f32; 2]> for ImVec2 {
    fn into(self) -> [f32; 2] { [self.x, self.y] }
}

impl Into<(f32, f32)> for ImVec2 {
    fn into(self) -> (f32, f32) { (self.x, self.y) }
}

/// A tuple of 4 floating-point values
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ImVec4 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
    pub w: c_float,
}

impl ImVec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> ImVec4 {
        ImVec4 {
            x: x as c_float,
            y: y as c_float,
            z: z as c_float,
            w: w as c_float,
        }
    }
    pub fn zero() -> ImVec4 {
        ImVec4 {
            x: 0.0 as c_float,
            y: 0.0 as c_float,
            z: 0.0 as c_float,
            w: 0.0 as c_float,
        }
    }
}

impl From<[f32; 4]> for ImVec4 {
    fn from(array: [f32; 4]) -> ImVec4 { ImVec4::new(array[0], array[1], array[2], array[3]) }
}

impl From<(f32, f32, f32, f32)> for ImVec4 {
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> ImVec4 { ImVec4::new(x, y, z, w) }
}

impl Into<[f32; 4]> for ImVec4 {
    fn into(self) -> [f32; 4] { [self.x, self.y, self.z, self.w] }
}

impl Into<(f32, f32, f32, f32)> for ImVec4 {
    fn into(self) -> (f32, f32, f32, f32) { (self.x, self.y, self.z, self.w) }
}

/// Runtime data for styling/colors
#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// String slice for use with ig* functions
pub struct ImStr<'p> {
    pub begin: *const c_char,
    pub end: *const c_char,
    phantom: PhantomData<&'p str>
}

impl<'p> ImStr<'p> {
    pub fn null() -> ImStr<'p> {
        ImStr {
            begin: ptr::null(),
            end: ptr::null(),
            phantom: PhantomData,
        }
    }
}

impl<'p> From<&'p str> for ImStr<'p> {
    fn from(s: &'p str) -> ImStr<'p> {
        ImStr {
            begin: s.as_ptr() as *const c_char,
            end: unsafe { s.as_ptr().offset(s.len() as isize) as *const c_char },
            phantom: PhantomData,
        }
    }
}

#[repr(C)]
pub struct ImGuiStyle {
    /// Global alpha applies to everything in ImGui
    pub alpha: c_float,
    /// Padding within a window
    pub window_padding: ImVec2,
    /// Minimum window size
    pub window_min_size: ImVec2,
    /// Radius of window corners rounding. Set to 0.0f to have rectangular windows
    pub window_rounding: c_float,
    /// Alignment for title bar text. Defaults to (0.0f, 0.5f) for left-aligned, vertically centered
    pub window_title_align: ImVec2,
    /// Radius of child window corners rounding. Set to 0.0f to have rectangular child windows
    pub child_window_rounding: c_float,
    /// Padding within a framed rectangle (used by most widgets)
    pub frame_padding: ImVec2,
    /// Radius of frame corners rounding. Set to 0.0f to have rectangular frames (used by most
    /// widgets).
    pub frame_rounding: c_float,
    /// Horizontal and vertical spacing between widgets/lines
    pub item_spacing: ImVec2,
    /// Horizontal and vertical spacing between within elements of a composed
    /// widget (e.g. a slider and its label)
    pub item_inner_spacing: ImVec2,
    /// Expand reactive bounding box for touch-based system where touch position is not accurate
    /// enough. Unfortunately we don't sort widgets so priority on overlap will always be given
    /// to the first widget. So don't grow this too much!
    pub touch_extra_padding: ImVec2,
    /// Horizontal spacing when e.g. entering a tree node.
    /// Generally == (FontSize + FramePadding.x*2).
    pub indent_spacing: c_float,
    /// Minimum horizontal spacing between two columns
    pub columns_min_spacing: c_float,
    /// Width of the vertical scrollbar, Height of the horizontal scrollbar
    pub scrollbar_size: c_float,
    /// Width of the vertical scrollbar, Height of the horizontal scrollbar
    pub scrollbar_rounding: c_float,
    /// Minimum width/height of a grab box for slider/scrollbar
    pub grab_min_size: c_float,
    /// Radius of grabs corners rounding. Set to 0.0f to have rectangular slider grabs.
    pub grab_rounding: c_float,
    /// Alignment of button text when button is larger than text. Defaults to (0.5f, 0.5f)
    /// for horizontally + vertically centered
    pub button_text_align: ImVec2,
    /// Window positions are clamped to be visible within the display area by at least this
    /// amount. Only covers regular windows.
    pub display_window_padding: ImVec2,
    /// If you cannot see the edge of your screen (e.g. on a TV) increase the safe area padding.
    /// Covers popups/tooltips as well regular windows.
    pub display_safe_area_padding: ImVec2,
    /// Enable anti-aliasing on lines/borders. Disable if you are really short on CPU/GPU.
    pub anti_aliased_lines: bool,
    /// Enable anti-aliasing on filled shapes (rounded rectangles, circles, etc.)
    pub anti_aliased_shapes: bool,
    /// Tessellation tolerance. Decrease for highly tessellated curves (higher quality, more
    /// polygons), increase to reduce quality.
    pub curve_tessellation_tol: c_float,
    /// Colors for the user interface
    pub colors: [ImVec4; ImGuiCol_COUNT],
}

/// Main configuration and I/O between your application and ImGui
#[repr(C)]
pub struct ImGuiIO {
    pub display_size: ImVec2,
    pub delta_time: c_float,
    pub ini_saving_rate: c_float,
    pub ini_filename: *const c_char,
    pub log_filename: *const c_char,
    pub mouse_double_click_time: c_float,
    pub mouse_double_click_max_dist: c_float,
    pub mouse_drag_threshold: c_float,
    pub key_map: [c_int; ImGuiKey_COUNT],
    pub key_repeat_delay: c_float,
    pub key_repeat_rate: c_float,
    pub user_data: *mut c_void,

    pub fonts: *mut ImFontAtlas,
    pub font_global_scale: c_float,
    pub font_allow_user_scaling: bool,
    pub font_default: *mut ImFont,
    pub display_framebuffer_scale: ImVec2,
    pub display_visible_min: ImVec2,
    pub display_visible_max: ImVec2,

    pub osx_behaviors: bool,

    pub render_draw_lists_fn: Option<extern "C" fn(data: *mut ImDrawData)>,

    pub get_clipboard_text_fn: Option<extern "C" fn(user_data: *mut c_void) -> *const c_char>,
    pub set_clipboard_text_fn: Option<extern "C" fn(user_data: *mut c_void, text: *const c_char)>,
    pub clipboard_user_data: *mut c_void,

    pub mem_alloc_fn: Option<extern "C" fn(sz: usize) -> *mut c_void>,
    pub mem_free_fn: Option<extern "C" fn(ptr: *mut c_void)>,

    pub ime_set_input_screen_pos_fn: Option<extern "C" fn(x: c_int, y: c_int)>,
    pub ime_window_handle: *mut c_void,

    pub mouse_pos: ImVec2,
    pub mouse_down: [bool; 5],
    pub mouse_wheel: c_float,
    pub mouse_draw_cursor: bool,
    pub key_ctrl: bool,
    pub key_shift: bool,
    pub key_alt: bool,
    pub key_super: bool,
    pub keys_down: [bool; 512],
    pub input_characters: [ImWchar; 16 + 1],

    pub want_capture_mouse: bool,
    pub want_capture_keyboard: bool,
    pub want_text_input: bool,
    pub want_move_mouse: bool,
    pub framerate: c_float,
    pub metrics_allocs: c_int,
    pub metrics_render_vertices: c_int,
    pub metrics_render_indices: c_int,
    pub metrics_active_windows: c_int,
    pub mouse_delta: ImVec2,

    mouse_pos_prev: ImVec2,
    mouse_clicked_pos: [ImVec2; 5],
    mouse_clicked_time: [c_float; 5],
    mouse_clicked: [bool; 5],
    mouse_double_clicked: [bool; 5],
    mouse_released: [bool; 5],
    mouse_down_owned: [bool; 5],
    mouse_down_duration: [c_float; 5],
    mouse_down_duration_prev: [c_float; 5],
    mouse_drag_max_distance_sqr: [c_float; 5],
    keys_down_duration: [c_float; 512],
    keys_down_duration_prev: [c_float; 512],
}

/// Lightweight vector struct
#[repr(C)]
pub struct ImVector<T> {
    pub size: c_int,
    pub capacity: c_int,
    pub data: *mut T,
}

impl<T> ImVector<T> {
    pub unsafe fn as_slice(&self) -> &[T] { slice::from_raw_parts(self.data, self.size as usize) }
}

#[repr(C)]
pub struct TextRange {
    pub begin: *const c_char,
    pub end: *const c_char,
}

#[repr(C)]
pub struct ImGuiTextFilter {
    pub input_buf: [c_char; 256],
    pub filters: ImVector<TextRange>,
    pub count_grep: c_int,
}

#[repr(C)]
pub struct ImGuiTextBuffer {
    pub buf: ImVector<c_char>,
}

#[repr(C)]
pub struct Pair {
    pub key: ImGuiID,
    pub value: PairValue,
}

#[repr(C)]
pub union PairValue {
    val_i: c_int,
    val_f: c_float,
    val_p: *mut c_void,
}

#[repr(C)]
pub struct ImGuiStorage {
    pub data: ImVector<Pair>,
}

#[repr(C)]
pub struct ImGuiTextEditCallbackData {
    pub event_flag: ImGuiInputTextFlags,
    pub flags: ImGuiInputTextFlags,
    pub user_data: *mut c_void,
    pub read_only: bool,

    pub event_char: ImWchar,

    pub event_key: ImGuiKey,
    pub buf: *mut c_char,
    pub buf_text_len: c_int,
    pub buf_size: c_int,
    pub buf_dirty: bool,
    pub cursor_pos: c_int,
    pub selection_start: c_int,
    pub selection_end: c_int,
}

#[repr(C)]
pub struct ImGuiSizeConstraintCallbackData {
    pub user_data: *mut c_void,
    pub pos: ImVec2,
    pub current_size: ImVec2,
    pub desired_size: ImVec2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct ImColor {
    pub value: ImVec4,
}

/// Helper to manually clip large list of items
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImGuiListClipper {
    pub start_pos_y: c_float,
    pub items_height: c_float,
    pub items_count: c_int,
    pub step_no: c_int,
    pub display_start: c_int,
    pub display_end: c_int,
}

pub type ImDrawCallback = Option<
    extern "C" fn(parent_list: *const ImDrawList,
                  cmd: *const ImDrawCmd),
>;

/// A single draw command within a parent ImDrawList (generally maps to 1 GPU draw call)
#[repr(C)]
pub struct ImDrawCmd {
    pub elem_count: c_uint,
    pub clip_rect: ImVec4,
    pub texture_id: ImTextureID,
    pub user_callback: ImDrawCallback,
    pub user_callback_data: *mut c_void,
}

/// Vertex index
pub type ImDrawIdx = c_ushort;

/// A single vertex
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct ImDrawVert {
    pub pos: ImVec2,
    pub uv: ImVec2,
    pub col: ImU32,
}

/// Temporary storage for outputting drawing commands out of order
#[repr(C)]
pub struct ImDrawChannel {
    pub cmd_buffer: ImVector<ImDrawCmd>,
    pub idx_buffer: ImVector<ImDrawIdx>,
}

/// A single draw command list (generally one per window)
#[repr(C)]
pub struct ImDrawList {
    pub cmd_buffer: ImVector<ImDrawCmd>,
    pub idx_buffer: ImVector<ImDrawIdx>,
    pub vtx_buffer: ImVector<ImDrawVert>,

    owner_name: *const c_char,
    vtx_current_idx: c_uint,
    vtx_write_ptr: *mut ImDrawVert,
    idx_write_ptr: *mut ImDrawIdx,
    clip_rect_stack: ImVector<ImVec4>,
    texture_id_stack: ImVector<ImTextureID>,
    path: ImVector<ImVec2>,
    channels_current: c_int,
    channels_count: c_int,
    channels: ImVector<ImDrawChannel>,
}

/// All draw command lists required to render the frame
#[repr(C)]
pub struct ImDrawData {
    pub valid: bool,
    pub cmd_lists: *mut *mut ImDrawList,
    pub cmd_lists_count: c_int,
    pub total_vtx_count: c_int,
    pub total_idx_count: c_int,
}

impl ImDrawData {
    pub unsafe fn cmd_lists(&self) -> &[*const ImDrawList] {
        let cmd_lists: *const *const ImDrawList = mem::transmute(self.cmd_lists);
        slice::from_raw_parts(cmd_lists, self.cmd_lists_count as usize)
    }
}

/// Configuration data when adding a font or merging fonts
#[repr(C)]
pub struct ImFontConfig {
    pub font_data: *mut c_void,
    pub font_data_size: c_int,
    pub font_data_owned_by_atlas: bool,
    pub font_no: c_int,
    pub size_pixels: c_float,
    pub oversample_h: c_int,
    pub oversample_v: c_int,
    pub pixel_snap_h: bool,
    pub glyph_extra_spacing: ImVec2,
    pub glyph_offset: ImVec2,
    pub glyph_ranges: *const ImWchar,
    pub merge_mode: bool,
    pub rasterizer_flags: c_uint,
    pub rasterizer_multiply: c_float,

    name: [c_char; 32],
    dst_font: *mut ImFont,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct ImFontGlyph {
    codepoint: ImWchar,
    advance_x: c_float,
    x0: c_float,
    y0: c_float,
    x1: c_float,
    y1: c_float,
    u0: c_float,
    v0: c_float,
    u1: c_float,
    v1: c_float,
}

#[repr(C)]
pub struct CustomRect {
    pub id: c_uint,
    pub width: c_ushort,
    pub height: c_ushort,
    pub x: c_ushort,
    pub y: c_ushort,
    pub glyph_advance_x: c_float,
    pub glyph_offset: ImVec2,
    pub font: *mut ImFont,
}

/// Runtime data for multiple fonts, bake multiple fonts into a single texture, TTF/OTF font loader
#[repr(C)]
pub struct ImFontAtlas {
    pub tex_id: *mut c_void,
    pub tex_desired_width: c_int,
    pub tex_glyph_padding: c_int,

    tex_pixels_alpha8: *mut c_uchar,
    tex_pixels_rgba32: *mut c_uint,
    tex_width: c_int,
    tex_height: c_int,
    tex_uv_white_pixel: ImVec2,
    fonts: ImVector<*mut ImFont>,
    custom_rects: ImVector<CustomRect>,
    config_data: ImVector<ImFontConfig>,
    custom_rect_ids: [c_int; 1],
}

/// Runtime data for a single font within a parent ImFontAtlas
#[repr(C)]
pub struct ImFont {
    font_size: c_float,
    scale: c_float,
    display_offset: ImVec2,
    glyphs: ImVector<ImFontGlyph>,
    index_advance_x: ImVector<c_float>,
    index_lookup: ImVector<c_ushort>,
    fallback_glyph: *const ImFontGlyph,
    fallback_advance_x: c_float,
    fallback_char: ImWchar,

    config_data_count: c_short,
    config_data: *mut ImFontConfig,
    container_atlas: *mut ImFontAtlas,
    ascent: c_float,
    descent: c_float,
    metrics_total_surface: c_int,
}

// Main
extern "C" {
    pub fn igGetIO() -> *mut ImGuiIO;
    pub fn igGetStyle() -> *mut ImGuiStyle;
    pub fn igGetDrawData() -> *mut ImDrawData;
    pub fn igNewFrame();
    pub fn igRender();
    pub fn igShutdown();
}

// Demo/Debug/Info
extern "C" {
    pub fn igShowTestWindow(opened: *mut bool);
    pub fn igShowMetricsWindow(opened: *mut bool);
    pub fn igShowStyleEditor(style: *mut ImGuiStyle);
    pub fn igShowUserGuide();
}

// Window
extern "C" {
    pub fn igBegin(name: ImStr, open: *mut bool, flags: ImGuiWindowFlags) -> bool;
    pub fn igEnd();
    pub fn igBeginChild(
        str_id: ImStr,
        size: ImVec2,
        border: bool,
        extra_flags: ImGuiWindowFlags,
    ) -> bool;
    pub fn igBeginChildEx(
        id: ImGuiID,
        size: ImVec2,
        border: bool,
        extra_flags: ImGuiWindowFlags,
    ) -> bool;
    pub fn igEndChild();
    pub fn igGetContentRegionMax(out: *mut ImVec2);
    pub fn igGetContentRegionAvail(out: *mut ImVec2);
    pub fn igGetContentRegionAvailWidth() -> c_float;
    pub fn igGetWindowContentRegionMin(out: *mut ImVec2);
    pub fn igGetWindowContentRegionMax(out: *mut ImVec2);
    pub fn igGetWindowContentRegionWidth() -> c_float;
    pub fn igGetWindowDrawList() -> *mut ImDrawList;
    pub fn igGetWindowPos(out: *mut ImVec2);
    pub fn igGetWindowSize(out: *mut ImVec2);
    pub fn igGetWindowWidth() -> c_float;
    pub fn igGetWindowHeight() -> c_float;
    pub fn igIsWindowCollapsed() -> bool;
    pub fn igIsWindowAppearing() -> bool;
    pub fn igSetWindowFontScale(scale: c_float);

    pub fn igSetNextWindowPos(pos: ImVec2, cond: ImGuiCond, pivot: ImVec2);
    pub fn igSetNextWindowSize(size: ImVec2, cond: ImGuiCond);
    pub fn igSetNextWindowConstraints(
        size_min: ImVec2,
        size_max: ImVec2,
        custom_callback: ImGuiSizeConstraintCallback,
        custom_callback_data: *mut c_void,
    );
    pub fn igSetNextWindowContentSize(size: ImVec2);
    pub fn igSetNextWindowContentWidth(width: c_float);
    pub fn igSetNextWindowCollapsed(collapsed: bool, cond: ImGuiCond);
    pub fn igSetNextWindowFocus();
    pub fn igSetWindowPos(pos: ImVec2, cond: ImGuiCond);
    pub fn igSetWindowSize(size: ImVec2, cond: ImGuiCond);
    pub fn igSetWindowCollapsed(collapsed: bool, cond: ImGuiCond);
    pub fn igSetWindowFocus();
    pub fn igSetWindowPosByName(name: ImStr, pos: ImVec2, cond: ImGuiCond);
    pub fn igSetWindowSize2(name: ImStr, size: ImVec2, cond: ImGuiCond);
    pub fn igSetWindowCollapsed2(name: ImStr, collapsed: bool, cond: ImGuiCond);
    pub fn igSetWindowFocus2(name: ImStr);

    pub fn igGetScrollX() -> c_float;
    pub fn igGetScrollY() -> c_float;
    pub fn igGetScrollMaxX() -> c_float;
    pub fn igGetScrollMaxY() -> c_float;
    pub fn igSetScrollX(scroll_x: c_float);
    pub fn igSetScrollY(scroll_y: c_float);
    pub fn igSetScrollHere(center_y_ratio: c_float);
    pub fn igSetScrollFromPosY(pos_y: c_float, center_y_ratio: c_float);
    pub fn igSetKeyboardFocusHere(offset: c_int);
    pub fn igSetStateStorage(tree: *mut ImGuiStorage);
    pub fn igGetStateStorage() -> *mut ImGuiStorage;
}

#[allow(non_snake_case)]
#[deprecated(since = "0.0.18", note = "please use igSetNextWindowPos instead")]
pub unsafe fn igSetNextWindowPosCenter(cond: ImGuiCond) {
    let io = igGetIO();
    let pos = ImVec2::new((*io).display_size.x * 0.5, (*io).display_size.y * 0.5);
    let pivot = ImVec2::new(0.5, 0.5);
    igSetNextWindowPos(pos, cond, pivot);
}

// Parameter stack (shared)
extern "C" {
    pub fn igPushFont(font: *mut ImFont);
    pub fn igPopFont();
    pub fn igPushStyleColorU32(idx: ImGuiCol, col: ImU32);
    pub fn igPushStyleColor(idx: ImGuiCol, col: ImVec4);
    pub fn igPopStyleColor(count: c_int);
    pub fn igPushStyleVar(idx: ImGuiStyleVar, val: c_float);
    pub fn igPushStyleVarVec(idx: ImGuiStyleVar, val: ImVec2);
    pub fn igPopStyleVar(count: c_int);
    pub fn igGetStyleColorVec4(out: *mut ImVec4, idx: ImGuiCol);
    pub fn igGetFont() -> *mut ImFont;
    pub fn igGetFontSize() -> c_float;
    pub fn igGetFontTexUvWhitePixel(out: *mut ImVec2);
    pub fn igGetColorU32(idx: ImGuiCol, alpha_mul: c_float) -> ImU32;
    pub fn igGetColorU32Vec(col: *const ImVec4) -> ImU32;
    pub fn igGetColorU32U32(col: ImU32) -> ImU32;
}

// Parameter stack (current window)
extern "C" {
    pub fn igPushItemWidth(item_width: c_float);
    pub fn igPopItemWidth();
    pub fn igCalcItemWidth() -> c_float;
    pub fn igPushTextWrapPos(wrap_pos_x: c_float);
    pub fn igPopTextWrapPos();
    pub fn igPushAllowKeyboardFocus(allow_keyboard_focus: bool);
    pub fn igPopAllowKeyboardFocus();
    pub fn igPushButtonRepeat(repeat: bool);
    pub fn igPopButtonRepeat();
}

// Cursor / Layout
extern "C" {
    pub fn igSeparator();
    pub fn igSameLine(pos_x: c_float, spacing_w: c_float);
    pub fn igNewLine();
    pub fn igSpacing();
    pub fn igDummy(size: *const ImVec2);
    pub fn igIndent(indent_w: c_float);
    pub fn igUnindent(indent_w: c_float);
    pub fn igBeginGroup();
    pub fn igEndGroup();
    pub fn igGetCursorPos(out: *mut ImVec2);
    pub fn igGetCursorPosX() -> c_float;
    pub fn igGetCursorPosY() -> c_float;
    pub fn igSetCursorPos(local_pos: ImVec2);
    pub fn igSetCursorPosX(x: c_float);
    pub fn igSetCursorPosY(y: c_float);
    pub fn igGetCursorStartPos(out: *mut ImVec2);
    pub fn igGetCursorScreenPos(out: *mut ImVec2);
    pub fn igSetCursorScreenPos(pos: ImVec2);
    pub fn igAlignTextToFramePadding();
    pub fn igGetTextLineHeight() -> c_float;
    pub fn igGetTextLineHeightWithSpacing() -> c_float;
    pub fn igGetItemsLineHeightWithSpacing() -> c_float;
}

#[allow(non_snake_case)]
#[deprecated(since = "0.0.18", note = "please use igAlignTextToFramePadding instead")]
pub unsafe fn igAlignFirstTextHeightToWidgets() { igAlignTextToFramePadding(); }

// Columns
extern "C" {
    pub fn igColumns(count: c_int, id: ImStr, border: bool);
    pub fn igNextColumn();
    pub fn igGetColumnIndex() -> c_int;
    pub fn igGetColumnWidth(column_index: c_int) -> c_float;
    pub fn igSetColumnWidth(column_index: c_int, width: c_float);
    pub fn igGetColumnOffset(column_index: c_int) -> c_float;
    pub fn igSetColumnOffset(column_index: c_int, offset_x: c_float);
    pub fn igGetColumnsCount() -> c_int;
}

// ID scopes
extern "C" {
    pub fn igPushIDStr(str_id: ImStr);
    pub fn igPushIDStrRange(str_begin: *const c_char, str_end: *const c_char);
    pub fn igPushIDPtr(ptr_id: *const c_void);
    pub fn igPushIDInt(int_id: c_int);
    pub fn igPopID();
    pub fn igGetIDStr(str_id: ImStr) -> ImGuiID;
    pub fn igGetIDStrRange(str_begin: *const c_char, str_end: *const c_char) -> ImGuiID;
    pub fn igGetIDPtr(ptr_id: *const c_void) -> ImGuiID;
}

// Widgets
extern "C" {
    pub fn igTextUnformatted(text: *const c_char, text_end: *const c_char);
    pub fn igTextUnformatted1(text: ImStr);
    pub fn igText(fmt: *const c_char, ...);
    // pub fn igTextV(fmt: *const c_char, args: va_list);
    pub fn igText1(text: ImStr);
    pub fn igTextColored(col: ImVec4, fmt: *const c_char, ...);
    // pub fn igTextColoredV(col: ImVec4, fmt: *const c_char, args: va_list);
    pub fn igTextColored1(col: ImVec4, text: ImStr);
    pub fn igTextDisabled(fmt: *const c_char, ...);
    // pub fn igTextDisabledV(fmt: *const c_char, args: va_list);
    pub fn igTextDisabled1(text: ImStr);
    pub fn igTextWrapped(fmt: *const c_char, ...);
    pub fn igTextWrapped1(text: ImStr);
    // pub fn igTextWrappedV(fmt: *const c_char, args: va_list);
    pub fn igLabelText(label: ImStr, fmt: *const c_char, ...);
    pub fn igLabelText1(label: ImStr, value: ImStr);
    // pub fn igLabelTextV(label: *const c_char, fmt: *const c_char, args: va_list);
    pub fn igBulletText(fmt: *const c_char, ...);
    pub fn igBulletText1(text: ImStr);
    // pub fn igBulletTextV(fmt: *const c_char, args: va_list);
    pub fn igBullet();
    pub fn igButton(label: ImStr, size: ImVec2) -> bool;
    pub fn igSmallButton(label: ImStr) -> bool;
    pub fn igInvisibleButton(str_id: ImStr, size: ImVec2) -> bool;
    pub fn igImage(
        user_texture_id: ImTextureID,
        size: ImVec2,
        uv0: ImVec2,
        uv1: ImVec2,
        tint_col: ImVec4,
        border_col: ImVec4,
    );
    pub fn igImageButton(
        user_texture_id: ImTextureID,
        size: ImVec2,
        uv0: ImVec2,
        uv1: ImVec2,
        frame_padding: c_int,
        bg_col: ImVec4,
        tint_col: ImVec4,
    ) -> bool;
    pub fn igCheckbox(label: ImStr, v: *mut bool) -> bool;
    pub fn igCheckboxFlags(label: ImStr, flags: *mut c_uint, flags_value: c_uint) -> bool;
    pub fn igRadioButtonBool(label: ImStr, active: bool) -> bool;
    pub fn igRadioButton(label: ImStr, v: *mut c_int, v_button: c_int) -> bool;
    pub fn igCombo(
        label: ImStr,
        current_item: *mut c_int,
        items: *const ImStr,
        items_count: c_int,
        height_in_items: c_int,
    ) -> bool;
    pub fn igCombo2(
        label: ImStr,
        current_item: *mut c_int,
        items_separated_by_zeros: *const c_char,
        height_in_items: c_int,
    ) -> bool;
    pub fn igCombo3(
        label: ImStr,
        current_item: *mut c_int,
        items_getter: extern "C" fn(data: *mut c_void, idx: c_int, out_text: *mut ImStr)
                                    -> bool,
        data: *mut c_void,
        items_count: c_int,
        height_in_items: c_int,
    ) -> bool;
    pub fn igPlotLines(
        label: ImStr,
        values: *const c_float,
        values_count: c_int,
        values_offset: c_int,
        overlay_text: ImStr,
        scale_min: c_float,
        scale_max: c_float,
        graph_size: ImVec2,
        stride: c_int,
    );
    pub fn igPlotLines2(
        label: ImStr,
        values_getter: extern "C" fn(data: *mut c_void, idx: c_int) -> c_float,
        data: *mut c_void,
        values_count: c_int,
        values_offset: c_int,
        overlay_text: ImStr,
        scale_min: c_float,
        scale_max: c_float,
        graph_size: ImVec2,
    );
    pub fn igPlotHistogram(
        label: ImStr,
        values: *const c_float,
        values_count: c_int,
        values_offset: c_int,
        overlay_text: ImStr,
        scale_min: c_float,
        scale_max: c_float,
        graph_size: ImVec2,
        stride: c_int,
    );
    pub fn igPlotHistogram2(
        label: ImStr,
        values_getter: extern "C" fn(data: *mut c_void, idx: c_int) -> c_float,
        data: *mut c_void,
        values_count: c_int,
        values_offset: c_int,
        overlay_text: ImStr,
        scale_min: c_float,
        scale_max: c_float,
        graph_size: ImVec2,
    );
    pub fn igProgressBar(fraction: c_float, size_arg: *const ImVec2, overlay: ImStr);
}

// Widgets: Color Editor/Picker
extern "C" {
    pub fn igColorEdit3(
        label: ImStr,
        col: *mut c_float,
        flags: ImGuiColorEditFlags,
    ) -> bool;
    pub fn igColorEdit4(
        label: ImStr,
        col: *mut c_float,
        flags: ImGuiColorEditFlags,
    ) -> bool;
    pub fn igColorPicker3(
        label: ImStr,
        col: *mut c_float,
        flags: ImGuiColorEditFlags,
    ) -> bool;
    pub fn igColorPicker4(
        label: ImStr,
        col: *mut c_float,
        flags: ImGuiColorEditFlags,
        ref_col: *const c_float,
    ) -> bool;
    pub fn igColorButton(
        desc_id: ImStr,
        col: ImVec4,
        flags: ImGuiColorEditFlags,
        size: ImVec2,
    ) -> bool;
    pub fn igSetColorEditOptions(flags: ImGuiColorEditFlags);
}

// Widgets: Drags
extern "C" {
    pub fn igDragFloat(
        label: ImStr,
        v: *mut c_float,
        v_speed: c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: ImStr,
        power: c_float,
    ) -> bool;
    pub fn igDragFloat2(
        label: ImStr,
        v: *mut c_float,
        v_speed: c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: ImStr,
        power: c_float,
    ) -> bool;
    pub fn igDragFloat3(
        label: ImStr,
        v: *mut c_float,
        v_speed: c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: ImStr,
        power: c_float,
    ) -> bool;
    pub fn igDragFloat4(
        label: ImStr,
        v: *mut c_float,
        v_speed: c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: ImStr,
        power: c_float,
    ) -> bool;
    pub fn igDragFloatRange2(
        label: ImStr,
        v_current_min: *mut c_float,
        v_current_max: *mut c_float,
        v_speed: c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: ImStr,
        display_format_max: ImStr,
        power: c_float,
    ) -> bool;
    pub fn igDragInt(
        label: ImStr,
        v: *mut c_int,
        v_speed: c_float,
        v_min: c_int,
        v_max: c_int,
        display_format: ImStr,
    ) -> bool;
    pub fn igDragInt2(
        label: ImStr,
        v: *mut c_int,
        v_speed: c_float,
        v_min: c_int,
        v_max: c_int,
        display_format: ImStr,
    ) -> bool;
    pub fn igDragInt3(
        label: ImStr,
        v: *mut c_int,
        v_speed: c_float,
        v_min: c_int,
        v_max: c_int,
        display_format: ImStr,
    ) -> bool;
    pub fn igDragInt4(
        label: ImStr,
        v: *mut c_int,
        v_speed: c_float,
        v_min: c_int,
        v_max: c_int,
        display_format: ImStr,
    ) -> bool;
    pub fn igDragIntRange2(
        label: ImStr,
        v_current_min: *mut c_int,
        v_current_max: *mut c_int,
        v_speed: c_float,
        v_min: c_int,
        v_max: c_int,
        display_format: ImStr,
        display_format_max: ImStr,
    ) -> bool;
}

// Widgets: Input with Keyboard
extern "C" {
    pub fn igInputText(
        label: ImStr,
        buf: *mut c_char,
        buf_size: usize,
        flags: ImGuiInputTextFlags,
        callback: ImGuiTextEditCallback,
        user_data: *mut c_void,
    ) -> bool;
    pub fn igInputTextMultiline(
        label: ImStr,
        buf: *mut c_char,
        buf_size: usize,
        size: ImVec2,
        flags: ImGuiInputTextFlags,
        callback: ImGuiTextEditCallback,
        user_data: *mut c_void,
    ) -> bool;
    pub fn igInputFloat(
        label: ImStr,
        v: *mut c_float,
        step: c_float,
        step_fast: c_float,
        decimal_precision: c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputFloat2(
        label: ImStr,
        v: *mut c_float,
        decimal_precision: c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputFloat3(
        label: ImStr,
        v: *mut c_float,
        decimal_precision: c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputFloat4(
        label: ImStr,
        v: *mut c_float,
        decimal_precision: c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputInt(
        label: ImStr,
        v: *mut c_int,
        step: c_int,
        step_fast: c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputInt2(
        label: ImStr,
        v: *mut c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputInt3(
        label: ImStr,
        v: *mut c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputInt4(
        label: ImStr,
        v: *mut c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
}

// Widgets: Sliders
extern "C" {
    pub fn igSliderFloat(
        label: ImStr,
        v: *mut c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: ImStr,
        power: c_float,
    ) -> bool;
    pub fn igSliderFloat2(
        label: ImStr,
        v: *mut c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: ImStr,
        power: c_float,
    ) -> bool;
    pub fn igSliderFloat3(
        label: ImStr,
        v: *mut c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: ImStr,
        power: c_float,
    ) -> bool;
    pub fn igSliderFloat4(
        label: ImStr,
        v: *mut c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: ImStr,
        power: c_float,
    ) -> bool;
    pub fn igSliderAngle(
        label: ImStr,
        v_rad: *mut c_float,
        v_degrees_min: c_float,
        v_degrees_max: c_float,
    ) -> bool;
    pub fn igSliderInt(
        label: ImStr,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        display_format: ImStr,
    ) -> bool;
    pub fn igSliderInt2(
        label: ImStr,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        display_format: ImStr,
    ) -> bool;
    pub fn igSliderInt3(
        label: ImStr,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        display_format: ImStr,
    ) -> bool;
    pub fn igSliderInt4(
        label: ImStr,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        display_format: ImStr,
    ) -> bool;
    pub fn igVSliderFloat(
        label: ImStr,
        size: ImVec2,
        v: *mut c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: ImStr,
        power: c_float,
    ) -> bool;
    pub fn igVSliderInt(
        label: ImStr,
        size: ImVec2,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        display_format: ImStr,
    ) -> bool;
}

// Widgets: Trees
extern "C" {
    pub fn igTreeNode(label: ImStr) -> bool;
    pub fn igTreeNodeStr(str_id: ImStr, fmt: *const c_char, ...) -> bool;
    pub fn igTreeNodeStr1(str_id: ImStr, text: ImStr) -> bool;
    pub fn igTreeNodePtr(ptr_id: *const c_void, fmt: *const c_char, ...) -> bool;
    // pub fn igTreeNodeStrV(str_id: *const c_char, fmt: *const c_char, args: va_list) -> bool;
    // pub fn igTreeNodePtrV(ptr_id: *const c_void, fmt: *const c_char, args: va_list) -> bool;
    pub fn igTreeNodeEx(label: ImStr, flags: ImGuiTreeNodeFlags) -> bool;
    pub fn igTreeNodeExStr(
        str_id: ImStr,
        flags: ImGuiTreeNodeFlags,
        fmt: *const c_char,
        ...
    ) -> bool;
    pub fn igTreeNodeExPtr(
        ptr_id: *const c_void,
        flags: ImGuiTreeNodeFlags,
        fmt: *const c_char,
        ...
    ) -> bool;
    // pub fn igTreeNodeExV(str_id: *const c_char, flags: ImGuiTreeNodeFlags,
    //                      fmt: *const c_char, args: va_list) -> bool;
    // pub fn igTreeNodeExVPtr(ptr_id: *const c_void, flags: ImGuiTreeNodeFlags,
    //                      fmt: *const c_char, args: va_list) -> bool;
    pub fn igTreePushPtr(ptr_id: *const c_void);
    pub fn igTreePop();
    pub fn igTreeAdvanceToLabelPos();
    pub fn igGetTreeNodeToLabelSpacing() -> c_float;
    pub fn igSetNextTreeNodeOpen(opened: bool, cond: ImGuiCond);
    pub fn igCollapsingHeader(label: ImStr, flags: ImGuiTreeNodeFlags) -> bool;
    pub fn igCollapsingHeaderEx(
        label: ImStr,
        open: *mut bool,
        flags: ImGuiTreeNodeFlags,
    ) -> bool;
}

// Widgets: Selectable / Lists
extern "C" {
    pub fn igSelectable(
        label: ImStr,
        selected: bool,
        flags: ImGuiSelectableFlags,
        size: ImVec2,
    ) -> bool;
    pub fn igSelectableEx(
        label: ImStr,
        p_selected: *mut bool,
        flags: ImGuiSelectableFlags,
        size: ImVec2,
    ) -> bool;
    pub fn igListBox(
        label: ImStr,
        current_item: *mut c_int,
        items: *const ImStr,
        items_count: c_int,
        height_in_items: c_int,
    ) -> bool;
    pub fn igListBox2(
        label: ImStr,
        current_item: *mut c_int,
        items_getter: extern "C" fn(data: *mut c_void, idx: c_int, out_text: *mut ImStr)
                                    -> bool,
        data: *mut c_void,
        items_count: c_int,
        height_in_items: c_int,
    ) -> bool;
    pub fn igListBoxHeader(label: ImStr, size: ImVec2) -> bool;
    pub fn igListBoxHeader2(
        label: ImStr,
        items_count: c_int,
        height_in_items: c_int,
    ) -> bool;
    pub fn igListBoxFooter();
}

// Widgets: Value() Helpers
extern "C" {
    pub fn igValueBool(prefix: ImStr, b: bool);
    pub fn igValueInt(prefix: ImStr, v: c_int);
    pub fn igValueUInt(prefix: ImStr, v: c_uint);
    pub fn igValueFloat(prefix: ImStr, v: c_float, float_format: ImStr);
}

// Tooltip
extern "C" {
    pub fn igSetTooltip(fmt: *const c_char, ...);
    // pub fn igSetTooltipV(fmt: *const c_char, args: va_list);
    pub fn igBeginTooltip();
    pub fn igEndTooltip();
}

// Widgets: Menus
extern "C" {
    pub fn igBeginMainMenuBar() -> bool;
    pub fn igEndMainMenuBar();
    pub fn igBeginMenuBar() -> bool;
    pub fn igEndMenuBar();
    pub fn igBeginMenu(label: ImStr, enabled: bool) -> bool;
    pub fn igEndMenu();
    pub fn igMenuItem(
        label: ImStr,
        shortcut: ImStr,
        selected: bool,
        enabled: bool,
    ) -> bool;
    pub fn igMenuItemPtr(
        label: ImStr,
        shortcut: ImStr,
        p_selected: *mut bool,
        enabled: bool,
    ) -> bool;
}

// Popup
extern "C" {
    pub fn igOpenPopup(str_id: ImStr);
    pub fn igOpenPopupOnItemClick(str_id: ImStr, mouse_button: c_int) -> bool;
    pub fn igBeginPopup(str_id: ImStr) -> bool;
    pub fn igBeginPopupModal(
        name: ImStr,
        open: *mut bool,
        extra_flags: ImGuiWindowFlags,
    ) -> bool;
    pub fn igBeginPopupContextItem(str_id: ImStr, mouse_button: c_int) -> bool;
    pub fn igBeginPopupContextWindow(
        str_id: ImStr,
        mouse_button: c_int,
        also_over_items: bool,
    ) -> bool;
    pub fn igBeginPopupContextVoid(str_id: ImStr, mouse_button: c_int) -> bool;
    pub fn igEndPopup();
    pub fn igIsPopupOpen(str_id: ImStr) -> bool;
    pub fn igCloseCurrentPopup();
}

// Logging
extern "C" {
    pub fn igLogToTTY(max_depth: c_int);
    pub fn igLogToFile(max_depth: c_int, filename: *const c_char);
    pub fn igLogToClipboard(max_depth: c_int);
    pub fn igLogFinish();
    pub fn igLogButtons();
    pub fn igLogText(fmt: *const c_char, ...);
}

// Clipping
extern "C" {
    pub fn igPushClipRect(
        clip_rect_min: ImVec2,
        clip_rect_max: ImVec2,
        intersect_with_current_clip_rect: bool,
    );
    pub fn igPopClipRect();
}

// Styles
extern "C" {
    pub fn igStyleColorsClassic(dst: *mut ImGuiStyle);
}

// Utilities
extern "C" {
    pub fn igIsItemHovered(flags: ImGuiHoveredFlags) -> bool;
    pub fn igIsItemActive() -> bool;
    pub fn igIsItemClicked(mouse_button: c_int) -> bool;
    pub fn igIsItemVisible() -> bool;
    pub fn igIsAnyItemHovered() -> bool;
    pub fn igIsAnyItemActive() -> bool;
    pub fn igGetItemRectMin(out: *mut ImVec2);
    pub fn igGetItemRectMax(out: *mut ImVec2);
    pub fn igGetItemRectSize(out: *mut ImVec2);
    pub fn igSetItemAllowOverlap();
    pub fn igIsWindowFocused() -> bool;
    pub fn igIsWindowHovered(flags: ImGuiHoveredFlags) -> bool;
    pub fn igIsRootWindowFocused() -> bool;
    pub fn igIsRootWindowOrAnyChildFocused() -> bool;
    pub fn igIsRootWindowOrAnyChildHovered(flags: ImGuiHoveredFlags) -> bool;
    pub fn igIsAnyWindowHovered() -> bool;
    pub fn igIsRectVisible(item_size: ImVec2) -> bool;
    pub fn igIsRectVisible2(rect_min: *const ImVec2, rect_max: *const ImVec2) -> bool;
    pub fn igGetTime() -> c_float;
    pub fn igGetFrameCount() -> c_int;
    pub fn igGetStyleColorName(idx: ImGuiCol) -> *const c_char;
    pub fn igCalcItemRectClosestPoint(
        out: *mut ImVec2,
        pos: ImVec2,
        on_edge: bool,
        outward: c_float,
    );
    pub fn igCalcTextSize(
        out: *mut ImVec2,
        text: ImStr,
        hide_text_after_double_hash: bool,
        wrap_width: c_float,
    );
    pub fn igCalcListClipping(
        items_count: c_int,
        items_height: c_float,
        out_items_display_start: *mut c_int,
        out_items_display_end: *mut c_int,
    );

    pub fn igBeginChildFrame(id: ImGuiID, size: ImVec2, extra_flags: ImGuiWindowFlags) -> bool;
    pub fn igEndChildFrame();

    pub fn igColorConvertU32ToFloat4(out: *mut ImVec4, color: ImU32);
    pub fn igColorConvertFloat4ToU32(color: ImVec4) -> ImU32;
    pub fn igColorConvertRGBtoHSV(
        r: c_float,
        g: c_float,
        b: c_float,
        out_h: *mut c_float,
        out_s: *mut c_float,
        out_v: *mut c_float,
    );
    pub fn igColorConvertHSVtoRGB(
        h: c_float,
        s: c_float,
        v: c_float,
        out_r: *mut c_float,
        out_g: *mut c_float,
        out_b: *mut c_float,
    );
}

// Inputs
extern "C" {
    pub fn igGetKeyIndex(imgui_key: ImGuiKey) -> c_int;
    pub fn igIsKeyDown(user_key_index: c_int) -> bool;
    pub fn igIsKeyPressed(user_key_index: c_int, repeat: bool) -> bool;
    pub fn igIsKeyReleased(user_key_index: c_int) -> bool;
    pub fn igGetKeyPressedAmount(key_index: c_int, repeat_delay: c_float, rate: c_float) -> c_int;
    pub fn igIsMouseDown(button: c_int) -> bool;
    pub fn igIsMouseClicked(button: c_int, repeat: bool) -> bool;
    pub fn igIsMouseDoubleClicked(button: c_int) -> bool;
    pub fn igIsMouseReleased(button: c_int) -> bool;
    pub fn igIsMouseDragging(button: c_int, lock_threshold: c_float) -> bool;
    pub fn igIsMouseHoveringRect(r_min: ImVec2, r_max: ImVec2, clip: bool) -> bool;
    pub fn igIsMousePosValid(mouse_pos: *const ImVec2) -> bool;
    pub fn igGetMousePos(out: *mut ImVec2);
    pub fn igGetMousePosOnOpeningCurrentPopup(out: *mut ImVec2);
    pub fn igGetMouseDragDelta(out: *mut ImVec2, button: c_int, lock_threshold: c_float);
    pub fn igResetMouseDragDelta(button: c_int);
    pub fn igGetMouseCursor() -> ImGuiMouseCursor;
    pub fn igSetMouseCursor(cursor: ImGuiMouseCursor);
    pub fn igCaptureKeyboardFromApp(capture: bool);
    pub fn igCaptureMouseFromApp(capture: bool);
}

#[allow(non_snake_case)]
#[deprecated(since = "0.0.18", note = "please use igIsItemHovered instead")]
pub unsafe fn igIsItemRectHovered() -> bool { igIsItemHovered(ImGuiHoveredFlags::RectOnly) }
#[allow(non_snake_case)]
#[deprecated(since = "0.0.18", note = "please use igIsWindowHovered instead")]
pub unsafe fn igIsWindowRectHovered() -> bool {
    igIsWindowHovered(
        ImGuiHoveredFlags::AllowWhenBlockedByPopup | ImGuiHoveredFlags::AllowWhenBlockedByActiveItem,
    )
}


// Helpers functions to access functions pointers in ImGui::GetIO()
extern "C" {
    pub fn igMemAlloc(sz: usize) -> *mut c_void;
    pub fn igMemFree(ptr: *mut c_void);
    pub fn igGetClipboardText() -> *const c_char;
    pub fn igSetClipboardText(text: *const c_char);
}

// Internal state access
extern "C" {
    pub fn igGetVersion() -> *const c_char;
    pub fn igCreateContext(
        malloc_fn: Option<extern "C" fn(size: usize) -> *mut c_void>,
        free_fn: Option<extern "C" fn(ptr: *mut c_void)>,
    ) -> *mut ImGuiContext;
    pub fn igDestroyContext(ctx: *mut ImGuiContext);
    pub fn igGetCurrentContext() -> *mut ImGuiContext;
    pub fn igSetCurrentContext(ctx: *mut ImGuiContext);
}

// ImGuiIO
extern "C" {
    pub fn ImGuiIO_AddInputCharacter(c: c_ushort);
    pub fn ImGuiIO_AddInputCharactersUTF8(utf8_chars: ImStr);
    pub fn ImGuiIO_ClearInputCharacters();
}

// ImGuiTextFilter
extern "C" {
    pub fn ImGuiTextFilter_Create(default_filter: ImStr) -> *mut ImGuiTextFilter;
    pub fn ImGuiTextFilter_Destroy(filter: *mut ImGuiTextFilter);
    pub fn ImGuiTextFilter_Clear(filter: *mut ImGuiTextFilter);
    pub fn ImGuiTextFilter_Draw(
        filter: *mut ImGuiTextFilter,
        label: ImStr,
        width: c_float,
    ) -> bool;
    pub fn ImGuiTextFilter_PassFilter(
        filter: *const ImGuiTextFilter,
        text: ImStr,
    ) -> bool;
    pub fn ImGuiTextFilter_IsActive(filter: *const ImGuiTextFilter) -> bool;
    pub fn ImGuiTextFilter_Build(filter: *const ImGuiTextFilter);
    pub fn ImGuiTextFilter_GetInputBuf(filter: *mut ImGuiTextFilter) -> *const c_char;
}

// ImGuiTextBuffer
extern "C" {
    pub fn ImGuiTextBuffer_Create() -> *mut ImGuiTextBuffer;
    pub fn ImGuiTextBuffer_Destroy(buffer: *mut ImGuiTextBuffer);
    pub fn ImGuiTextBuffer_index(buffer: *mut ImGuiTextBuffer, i: c_int) -> c_char;
    pub fn ImGuiTextBuffer_begin(buffer: *const ImGuiTextBuffer) -> *const c_char;
    pub fn ImGuiTextBuffer_end(buffer: *const ImGuiTextBuffer) -> *const c_char;
    pub fn ImGuiTextBuffer_size(buffer: *const ImGuiTextBuffer) -> c_int;
    pub fn ImGuiTextBuffer_empty(buffer: *mut ImGuiTextBuffer) -> bool;
    pub fn ImGuiTextBuffer_clear(buffer: *mut ImGuiTextBuffer);
    pub fn ImGuiTextBuffer_c_str(buffer: *const ImGuiTextBuffer) -> *const c_char;
    pub fn ImGuiTextBuffer_append(buffer: *const ImGuiTextBuffer, fmt: *const c_char, ...);
// pub fn ImGuiTextBuffer_appendv(
//     buffer: *const ImGuiTextBuffer,
//     fmt: *const c_char,
//     args: va_list
// );
}

// ImGuiStorage
extern "C" {
    pub fn ImGuiStorage_Create() -> *mut ImGuiStorage;
    pub fn ImGuiStorage_Destroy(storage: *mut ImGuiStorage);
    pub fn ImGuiStorage_GetInt(
        storage: *mut ImGuiStorage,
        key: ImGuiID,
        default_val: c_int,
    ) -> c_int;
    pub fn ImGuiStorage_SetInt(storage: *mut ImGuiStorage, key: ImGuiID, val: c_int);
    pub fn ImGuiStorage_GetBool(
        storage: *mut ImGuiStorage,
        key: ImGuiID,
        default_val: bool,
    ) -> bool;
    pub fn ImGuiStorage_SetBool(storage: *mut ImGuiStorage, key: ImGuiID, val: bool);
    pub fn ImGuiStorage_GetFloat(
        storage: *mut ImGuiStorage,
        key: ImGuiID,
        default_val: c_float,
    ) -> c_float;
    pub fn ImGuiStorage_SetFloat(storage: *mut ImGuiStorage, key: ImGuiID, val: c_float);
    pub fn ImGuiStorage_GetVoidPtr(storage: *mut ImGuiStorage, key: ImGuiID);
    pub fn ImGuiStorage_SetVoidPtr(storage: *mut ImGuiStorage, key: ImGuiID, val: *mut c_void);
    pub fn ImGuiStorage_GetIntRef(
        storage: *mut ImGuiStorage,
        key: ImGuiID,
        default_val: c_int,
    ) -> *mut c_int;
    pub fn ImGuiStorage_GetBoolRef(
        storage: *mut ImGuiStorage,
        key: ImGuiID,
        default_val: bool,
    ) -> *mut bool;
    pub fn ImGuiStorage_GetFloatRef(
        storage: *mut ImGuiStorage,
        key: ImGuiID,
        default_val: c_float,
    ) -> *mut c_float;
    pub fn ImGuiStorage_GetVoidPtrRef(
        storage: *mut ImGuiStorage,
        key: ImGuiID,
        default_val: *mut c_void,
    ) -> *mut *mut c_void;
    pub fn ImGuiStorage_SetAllInt(storage: *mut ImGuiStorage, val: c_int);
}

// ImGuiTextEditCallbackData
extern "C" {
    pub fn ImGuiTextEditCallbackData_DeleteChars(
        data: *mut ImGuiTextEditCallbackData,
        pos: c_int,
        bytes_count: c_int,
    );
    pub fn ImGuiTextEditCallbackData_InsertChars(
        data: *mut ImGuiTextEditCallbackData,
        pos: c_int,
        text: ImStr,
    );
    pub fn ImGuiTextEditCallbackData_HasSelection(data: *mut ImGuiTextEditCallbackData) -> bool;
}

// ImGuiListClipper
extern "C" {
    pub fn ImGuiListClipper_Step(clipper: *mut ImGuiListClipper) -> bool;
    pub fn ImGuiListClipper_Begin(
        clipper: *mut ImGuiListClipper,
        count: c_int,
        items_height: c_float,
    );
    pub fn ImGuiListClipper_End(clipper: *mut ImGuiListClipper);
    pub fn ImGuiListClipper_GetDisplayStart(clipper: *mut ImGuiListClipper) -> c_int;
    pub fn ImGuiListClipper_GetDisplayEnd(clipper: *mut ImGuiListClipper) -> c_int;
}

// ImDrawList
extern "C" {
    pub fn ImDrawList_GetVertexBufferSize(list: *mut ImDrawList) -> c_int;
    pub fn ImDrawList_GetVertexPtr(list: *mut ImDrawList, n: c_int) -> *mut ImDrawVert;
    pub fn ImDrawList_GetIndexBufferSize(list: *mut ImDrawList) -> c_int;
    pub fn ImDrawList_GetIndexPtr(list: *mut ImDrawList, n: c_int) -> *mut ImDrawIdx;
    pub fn ImDrawList_GetCmdSize(list: *mut ImDrawList) -> c_int;
    pub fn ImDrawList_GetCmdPtr(list: *mut ImDrawList, n: c_int) -> *mut ImDrawCmd;

    pub fn ImDrawList_Clear(list: *mut ImDrawList);
    pub fn ImDrawList_ClearFreeMemory(list: *mut ImDrawList);
    pub fn ImDrawList_PushClipRect(
        list: *mut ImDrawList,
        clip_rect_min: ImVec2,
        clip_rect_max: ImVec2,
        intersect_with_current_: bool,
    );
    pub fn ImDrawList_PushClipRectFullScreen(list: *mut ImDrawList);
    pub fn ImDrawList_PopClipRect(list: *mut ImDrawList);
    pub fn ImDrawList_PushTextureID(list: *mut ImDrawList, texture_id: ImTextureID);
    pub fn ImDrawList_PopTextureID(list: *mut ImDrawList);
    pub fn ImDrawList_GetClipRectMin(out: *mut ImVec2, list: *mut ImDrawList);
    pub fn ImDrawList_GetClipRectMax(out: *mut ImVec2, list: *mut ImDrawList);

    pub fn ImDrawList_AddLine(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        col: ImU32,
        thickness: c_float,
    );
    pub fn ImDrawList_AddRect(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        col: ImU32,
        rounding: c_float,
        rounding_corners_flags: c_int,
        thickness: c_float,
    );
    pub fn ImDrawList_AddRectFilled(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        col: ImU32,
        rounding: c_float,
        rounding_corners_flags: c_int,
    );
    pub fn ImDrawList_AddRectFilledMultiColor(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        col_upr_left: ImU32,
        col_upr_right: ImU32,
        col_bot_right: ImU32,
        col_bot_left: ImU32,
    );
    pub fn ImDrawList_AddQuad(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        d: ImVec2,
        col: ImU32,
        thickness: c_float,
    );
    pub fn ImDrawList_AddQuadFilled(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        d: ImVec2,
        col: ImU32,
    );
    pub fn ImDrawList_AddTriangle(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        col: ImU32,
        thickness: c_float,
    );
    pub fn ImDrawList_AddTriangleFilled(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        col: ImU32,
    );
    pub fn ImDrawList_AddCircle(
        list: *mut ImDrawList,
        centre: ImVec2,
        radius: c_float,
        col: ImU32,
        num_segments: c_int,
    );
    pub fn ImDrawList_AddCircleFilled(
        list: *mut ImDrawList,
        centre: ImVec2,
        radius: c_float,
        col: ImU32,
        num_segments: c_int,
    );
    pub fn ImDrawList_AddText(
        list: *mut ImDrawList,
        pos: ImVec2,
        col: ImU32,
        text: ImStr,
    );
    pub fn ImDrawList_AddTextExt(
        list: *mut ImDrawList,
        font: *const ImFont,
        font_size: c_float,
        pos: ImVec2,
        col: ImU32,
        text: ImStr,
        wrap_width: c_float,
        cpu_fine_clip_rect: *const ImVec4,
    );
    pub fn ImDrawList_AddImage(
        list: *mut ImDrawList,
        user_texture_id: ImTextureID,
        a: ImVec2,
        b: ImVec2,
        uv_a: ImVec2,
        uv_b: ImVec2,
        col: ImU32,
    );
    pub fn ImDrawList_AddImageQuad(
        list: *mut ImDrawList,
        user_texture_id: ImTextureID,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        d: ImVec2,
        uv_a: ImVec2,
        uv_b: ImVec2,
        uv_c: ImVec2,
        uv_d: ImVec2,
        col: ImU32,
    );
    pub fn ImDrawList_AddPolyLine(
        list: *mut ImDrawList,
        points: *const ImVec2,
        num_points: c_int,
        col: ImU32,
        closed: bool,
        thickness: c_float,
        anti_aliased: bool,
    );
    pub fn ImDrawList_AddConvexPolyFilled(
        list: *mut ImDrawList,
        points: *const ImVec2,
        num_points: c_int,
        col: ImU32,
        anti_aliased: bool,
    );
    pub fn ImDrawList_AddBezierCurve(
        list: *mut ImDrawList,
        pos0: ImVec2,
        cp0: ImVec2,
        cp1: ImVec2,
        pos1: ImVec2,
        col: ImU32,
        thickness: c_float,
        num_segments: c_int,
    );

    pub fn ImDrawList_PathClear(list: *mut ImDrawList);
    pub fn ImDrawList_PathLineTo(list: *mut ImDrawList, pos: ImVec2);
    pub fn ImDrawList_PathLineToMergeDuplicate(list: *mut ImDrawList, pos: ImVec2);
    pub fn ImDrawList_PathFillConvex(list: *mut ImDrawList, col: ImU32);
    pub fn ImDrawList_PathStroke(
        list: *mut ImDrawList,
        col: ImU32,
        closed: bool,
        thickness: c_float,
    );
    pub fn ImDrawList_PathArcTo(
        list: *mut ImDrawList,
        centre: ImVec2,
        radius: c_float,
        a_min: c_float,
        a_max: c_float,
        num_segments: c_int,
    );
    pub fn ImDrawList_PathArcToFast(
        list: *mut ImDrawList,
        centre: ImVec2,
        radius: c_float,
        a_min_of_12: c_int,
        a_max_of_12: c_int,
    );
    pub fn ImDrawList_PathBezierCurveTo(
        list: *mut ImDrawList,
        p1: ImVec2,
        p2: ImVec2,
        p3: ImVec2,
        num_segments: c_int,
    );
    pub fn ImDrawList_PathRect(
        list: *mut ImDrawList,
        rect_min: ImVec2,
        rect_max: ImVec2,
        rounding: c_float,
        rounding_corners_flags: c_int,
    );

    pub fn ImDrawList_ChannelsSplit(list: *mut ImDrawList, channels_count: c_int);
    pub fn ImDrawList_ChannelsMerge(list: *mut ImDrawList);
    pub fn ImDrawList_ChannelsSetCurrent(list: *mut ImDrawList, channel_index: c_int);

    pub fn ImDrawList_AddCallback(
        list: *mut ImDrawList,
        callback: ImDrawCallback,
        callback_data: *mut c_void,
    );
    pub fn ImDrawList_AddDrawCmd(list: *mut ImDrawList);

    pub fn ImDrawList_PrimReserve(list: *mut ImDrawList, idx_count: c_int, vtx_count: c_int);
    pub fn ImDrawList_PrimRect(list: *mut ImDrawList, a: ImVec2, b: ImVec2, col: ImU32);
    pub fn ImDrawList_PrimRectUV(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        uv_a: ImVec2,
        uv_b: ImVec2,
        col: ImU32,
    );
    pub fn ImDrawList_PrimQuadUV(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        d: ImVec2,
        uv_a: ImVec2,
        uv_b: ImVec2,
        uv_c: ImVec2,
        uv_d: ImVec2,
        col: ImU32,
    );
    pub fn ImDrawList_PrimWriteVtx(list: *mut ImDrawList, pos: ImVec2, uv: ImVec2, col: ImU32);
    pub fn ImDrawList_PrimWriteIdx(list: *mut ImDrawList, idx: ImDrawIdx);
    pub fn ImDrawList_PrimVtx(list: *mut ImDrawList, pos: ImVec2, uv: ImVec2, col: ImU32);
    pub fn ImDrawList_UpdateClipRect(list: *mut ImDrawList);
    pub fn ImDrawList_UpdateTextureID(list: *mut ImDrawList);
}

// ImDrawData
extern "C" {
    pub fn ImDrawData_DeIndexAllBuffers(drawData: *mut ImDrawData);
    pub fn ImDrawData_ScaleClipRects(drawData: *mut ImDrawData, sc: ImVec2);
}

extern "C" {
    pub fn ImFontAtlas_GetTexDataAsRGBA32(
        atlas: *mut ImFontAtlas,
        out_pixels: *mut *mut c_uchar,
        out_width: *mut c_int,
        out_height: *mut c_int,
        out_bytes_per_pixel: *mut c_int,
    );
    pub fn ImFontAtlas_GetTexDataAsAlpha8(
        atlas: *mut ImFontAtlas,
        out_pixels: *mut *mut c_uchar,
        out_width: *mut c_int,
        out_height: *mut c_int,
        out_bytes_per_pixel: *mut c_int,
    );
    pub fn ImFontAtlas_SetTexID(atlas: *mut ImFontAtlas, tex: ImTextureID);
    pub fn ImFontAtlas_AddFont(
        atlas: *mut ImFontAtlas,
        font_cfg: *const ImFontConfig,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontDefault(
        atlas: *mut ImFontAtlas,
        font_cfg: *const ImFontConfig,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontFromFileTTF(
        atlas: *mut ImFontAtlas,
        filename: *const c_char,
        size_pixels: c_float,
        font_cfg: *const ImFontConfig,
        glyph_ranges: *const ImWchar,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontFromMemoryTTF(
        atlas: *mut ImFontAtlas,
        font_data: *mut c_void,
        font_size: c_int,
        size_pixels: c_float,
        font_cfg: *const ImFontConfig,
        glyph_ranges: *const ImWchar,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontFromMemoryCompressedTTF(
        atlas: *mut ImFontAtlas,
        compressed_font_data: *const c_void,
        compressed_font_size: c_int,
        size_pixels: c_float,
        font_cfg: *const ImFontConfig,
        glyph_ranges: *const ImWchar,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontFromMemoryCompressedBase85TTF(
        atlas: *mut ImFontAtlas,
        compressed_font_data_base85: *const c_char,
        size_pixels: c_float,
        font_cfg: *const ImFontConfig,
        glyph_ranges: *const ImWchar,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_ClearTexData(atlas: *mut ImFontAtlas);
    pub fn ImFontAtlas_Clear(atlas: *mut ImFontAtlas);
    pub fn ImFontAtlas_GetGlyphRangesDefault(atlas: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesKorean(atlas: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesJapanese(atlas: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesChinese(atlas: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesCyrillic(atlas: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesThai(atlas: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetTexID(atlas: *mut ImFontAtlas) -> ImTextureID;
    pub fn ImFontAtlas_GetTexPixelsAlpha8(atlas: *mut ImFontAtlas) -> *mut c_uchar;
    pub fn ImFontAtlas_GetTexPixelsRGBA32(altas: *mut ImFontAtlas) -> *mut c_uint;
    pub fn ImFontAtlas_GetTexWidth(atlas: *mut ImFontAtlas) -> c_int;
    pub fn ImFontAtlas_GetTexHeight(atlas: *mut ImFontAtlas) -> c_int;
    pub fn ImFontAtlas_GetTexDesiredWidth(atlas: *mut ImFontAtlas) -> c_int;
    pub fn ImFontAtlas_SetTexDesiredWidth(atlas: *mut ImFontAtlas, TexDesiredWidth_: c_int);
    pub fn ImFontAtlas_GetTexGlyphPadding(atlas: *mut ImFontAtlas) -> c_int;
    pub fn ImFontAtlas_SetTexGlyphPadding(atlas: *mut ImFontAtlas, TexGlyphPadding_: c_int);
    pub fn ImFontAtlas_GetTexUvWhitePixel(atlas: *mut ImFontAtlas, out: *mut ImVec2);
}

// ImFontAtlas::Fonts
extern "C" {
    pub fn ImFontAtlas_Fonts_size(atlas: *mut ImFontAtlas) -> c_int;
    pub fn ImFontAtlas_Fonts_index(atlas: *mut ImFontAtlas, index: c_int) -> *mut ImFont;
}

// ImFont
extern "C" {
    pub fn ImFont_GetFontSize(font: *const ImFont) -> c_float;
    pub fn ImFont_SetFontSize(font: *mut ImFont, FontSize_: c_float);
    pub fn ImFont_GetScale(font: *const ImFont) -> c_float;
    pub fn ImFont_SetScale(font: *mut ImFont, Scale_: c_float);
    pub fn ImFont_GetDisplayOffset(font: *const ImFont, out: *mut ImVec2);
    pub fn ImFont_GetFallbackGlyph(font: *const ImFont) -> *const ImFontGlyph;
    pub fn ImFont_SetFallbackGlyph(font: *mut ImFont, FallbackGlyph: *const ImFontGlyph);
    pub fn ImFont_GetFallbackAdvanceX(font: *const ImFont) -> c_float;
    pub fn ImFont_GetFallbackChar(font: *const ImFont) -> ImWchar;
    pub fn ImFont_GetConfigDataCount(font: *const ImFont) -> c_short;
    pub fn ImFont_GetConfigData(font: *mut ImFont) -> *mut ImFontConfig;
    pub fn ImFont_GetContainerAtlas(font: *mut ImFont) -> *mut ImFontAtlas;
    pub fn ImFont_GetAscent(font: *const ImFont) -> c_float;
    pub fn ImFont_GetDescent(font: *const ImFont) -> c_float;
    pub fn ImFont_GetMetricsTotalSurface(font: *const ImFont) -> c_int;
    pub fn ImFont_ClearOutputData(font: *mut ImFont);
    pub fn ImFont_BuildLookupTable(font: *mut ImFont);
    pub fn ImFont_FindGlyph(font: *const ImFont, c: ImWchar) -> *const ImFontGlyph;
    pub fn ImFont_SetFallbackChar(font: *mut ImFont, c: ImWchar);
    pub fn ImFont_GetCharAdvance(font: *const ImFont, c: ImWchar) -> c_float;
    pub fn ImFont_IsLoaded(font: *const ImFont) -> bool;
    pub fn ImFont_CalcTextSizeA(
        font: *const ImFont,
        out: *mut ImVec2,
        size: c_float,
        max_width: c_float,
        wrap_width: c_float,
        text: ImStr,
        remaining: *mut *const c_char,
    );
    pub fn ImFont_CalcWordWrapPositionA(
        font: *const ImFont,
        scale: c_float,
        text: ImStr,
        wrap_width: c_float,
    ) -> *const c_char;
    pub fn ImFont_RenderChar(
        font: *const ImFont,
        draw_list: *mut ImDrawList,
        size: c_float,
        pos: ImVec2,
        col: ImU32,
        c: c_ushort,
    );
    pub fn ImFont_RenderText(
        font: *const ImFont,
        draw_list: *mut ImDrawList,
        size: c_float,
        pos: ImVec2,
        col: ImU32,
        clip_rect: *const ImVec4,
        text: ImStr,
        wrap_width: c_float,
        cpu_fine_clip: bool,
    );
}

#[allow(non_snake_case)]
#[deprecated(since = "0.0.18", note = "please use ImFont_ClearOutputData instead")]
pub unsafe fn ImFont_Clear(font: *mut ImFont) { ImFont_ClearOutputData(font); }
#[allow(non_snake_case)]
#[deprecated(since = "0.0.18", note = "please use ImFont_GetFallbackChar instead")]
pub unsafe fn ImFont_GetFallbackXAdvance(font: *const ImFont) -> c_float {
    ImFont_GetFallbackAdvanceX(font)
}

// ImFont::Glyph
extern "C" {
    pub fn ImFont_Glyphs_size(font: *const ImFont) -> c_int;
    pub fn ImFont_Glyphs_index(font: *mut ImFont, index: c_int) -> *mut ImFontGlyph;
}

// ImFont::IndexXAdvance
extern "C" {
    pub fn ImFont_IndexXAdvance_size(font: *const ImFont) -> c_int;
    pub fn ImFont_IndexXAdvance_index(font: *const ImFont, index: c_int) -> c_float;
}

// ImFont::IndexLookup
extern "C" {
    pub fn ImFont_IndexLookup_size(ofnt: *const ImFont) -> c_int;
    pub fn ImFont_IndexLookup_index(font: *const ImFont, index: c_int) -> c_ushort;
}

// Although this test is sensitive to ImGui updates, it's useful to reveal potential
// alignment errors
#[test]
fn test_default_style() {
    let style = unsafe { &*igGetStyle() };
    assert_eq!(style.alpha, 1.0);
    assert_eq!(style.window_padding, ImVec2::new(8.0, 8.0));
    assert_eq!(style.window_min_size, ImVec2::new(32.0, 32.0));
    assert_eq!(style.window_rounding, 9.0);
    assert_eq!(style.window_title_align, ImVec2::new(0.0, 0.5));
    assert_eq!(style.child_window_rounding, 0.0);
    assert_eq!(style.frame_padding, ImVec2::new(4.0, 3.0));
    assert_eq!(style.frame_rounding, 0.0);
    assert_eq!(style.item_spacing, ImVec2::new(8.0, 4.0));
    assert_eq!(style.item_inner_spacing, ImVec2::new(4.0, 4.0));
    assert_eq!(style.touch_extra_padding, ImVec2::new(0.0, 0.0));
    assert_eq!(style.indent_spacing, 21.0);
    assert_eq!(style.columns_min_spacing, 6.0);
    assert_eq!(style.scrollbar_size, 16.0);
    assert_eq!(style.scrollbar_rounding, 9.0);
    assert_eq!(style.grab_min_size, 10.0);
    assert_eq!(style.grab_rounding, 0.0);
    assert_eq!(style.button_text_align, ImVec2::new(0.5, 0.5));
    assert_eq!(style.display_window_padding, ImVec2::new(22.0, 22.0));
    assert_eq!(style.display_safe_area_padding, ImVec2::new(4.0, 4.0));
    assert_eq!(style.anti_aliased_lines, true);
    assert_eq!(style.anti_aliased_shapes, true);
    assert_eq!(style.curve_tessellation_tol, 1.25);
}
