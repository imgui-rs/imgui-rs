#![allow(non_upper_case_globals)]

#[macro_use]
extern crate bitflags;

extern crate libc;

#[cfg(feature = "gfx")]
#[macro_use]
extern crate gfx;

#[cfg(feature = "glium")]
extern crate glium;

use std::convert::From;
use std::mem;
use std::os::raw::{c_char, c_float, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};
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
    ChildBg,
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
    DragDropTarget,
}
impl ImGuiCol {
    #[deprecated(since = "0.0.19", note = "ComboBg has been merged with PopupBg. Please use PopupBg instead")]
    pub const ComboBg: ImGuiCol = ImGuiCol::PopupBg;
    #[deprecated(since = "0.0.19", note = "please use ChildBg instead")]
    pub const ChildWindowBg: ImGuiCol = ImGuiCol::ChildBg;

    pub fn values() -> &'static [ImGuiCol] {
        use ImGuiCol::*;
        static values: &'static [ImGuiCol] = &[
            Text,
            TextDisabled,
            WindowBg,
            ChildBg,
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
            DragDropTarget,
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
    WindowBorderSize,
    WindowMinSize,
    ChildRounding,
    ChildBorderSize,
    PopupRounding,
    PopupBorderSize,
    FramePadding,
    FrameRounding,
    FrameBorderSize,
    ItemSpacing,
    ItemInnerSpacing,
    IndentSpacing,
    GrabMinSize,
    ButtonTextAlign,
}
pub const ImGuiStyleVar_COUNT: usize = 17;

impl ImGuiStyleVar {
    #[deprecated(since = "0.0.19", note = "please use ChildRounding instead")]
    pub const ChildWindowRounding: ImGuiStyleVar = ImGuiStyleVar::ChildRounding;
}

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
        const NoSavedSettings           = 1 << 8;
        const NoInputs                  = 1 << 9;
        const MenuBar                   = 1 << 10;
        const HorizontalScrollbar       = 1 << 11;
        const NoFocusOnAppearing        = 1 << 12;
        const NoBringToFrontOnFocus     = 1 << 13;
        const AlwaysVerticalScrollbar   = 1 << 14;
        const AlwaysHorizontalScrollbar = 1 << 15;
        const AlwaysUseWindowPadding    = 1 << 16;
        const ResizeFromAnySide         = 1 << 17;
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
        const NoUndoRedo          = 1 << 16;
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
        #[deprecated(since = "0.0.19", note = "please use AllowItemOverlap instead")]
        const AllowOverlapMode  = 1 << 2;
        const AllowItemOverlap  = 1 << 2;
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
    /// Flags for window focus check
    #[repr(C)]
    pub struct ImGuiFocusedFlags: c_int {
        const ChildWindows = 1 << 0;
        const RootWindow = 1 << 1;
        const RootAndChildWindows =
            ImGuiFocusedFlags::RootWindow.bits | ImGuiFocusedFlags::ChildWindows.bits;
    }
);

bitflags!(
    /// Flags for hover checks
    #[repr(C)]
    pub struct ImGuiHoveredFlags: c_int {
        const ChildWindows                 = 1 << 0;
        const RootWindow                   = 1 << 1;
        const AllowWhenBlockedByPopup      = 1 << 2;
        const AllowWhenBlockedByActiveItem = 1 << 4;
        const AllowWhenOverlapped          = 1 << 5;
        const RectOnly = ImGuiHoveredFlags::AllowWhenBlockedByPopup.bits
            | ImGuiHoveredFlags::AllowWhenBlockedByActiveItem.bits
            | ImGuiHoveredFlags::AllowWhenOverlapped.bits;
        const RootAndChildWindows = ImGuiFocusedFlags::RootWindow.bits
            | ImGuiFocusedFlags::ChildWindows.bits;
    }
);

bitflags!(
    /// Flags for igBeginCombo
    #[repr(C)]
    pub struct ImGuiComboFlags: c_int {
        /// Align the popup toward the left by default
        const PopupAlignLeft = 1 << 0;
        /// Max ~4 items visible.
        /// Tip: If you want your combo popup to be a specific size you can use
        /// igSetNextWindowSizeConstraints() prior to calling igBeginCombo()
        const HeightSmall    = 1 << 1;
        /// Max ~8 items visible (default)
        const HeightRegular  = 1 << 2;
        /// Max ~20 items visible
        const HeightLarge    = 1 << 3;
        /// As many fitting items as possible
        const HeightLargest  = 1 << 4;
        const HeightMask     = ImGuiComboFlags::HeightSmall.bits
            | ImGuiComboFlags::HeightRegular.bits
            | ImGuiComboFlags::HeightLarge.bits
            | ImGuiComboFlags::HeightLargest.bits;
    }
);

bitflags!(
    /// Flags for igBeginDragDropSource(), igAcceptDragDropPayload()
    #[repr(C)]
    pub struct ImGuiDragDropFlags: c_int {
        // BeginDragDropSource() flags
        /// By default, a successful call to igBeginDragDropSource opens a
        /// tooltip so you can display a preview or description of the source
        /// contents. This flag disable this behavior.
        const SourceNoPreviewTooltip   = 1 << 0;
        /// By default, when dragging we clear data so that igIsItemHovered()
        /// will return true, to avoid subsequent user code submitting tooltips.
        /// This flag disable this behavior so you can still call
        /// igIsItemHovered() on the source item.
        const SourceNoDisableHover     = 1 << 1;
        /// Disable the behavior that allows to open tree nodes and collapsing
        /// header by holding over them while dragging a source item.
        const SourceNoHoldToOpenOthers = 1 << 2;
        /// Allow items such as igText(), igImage() that have no unique
        /// identifier to be used as drag source, by manufacturing a temporary
        /// identifier based on their window-relative position. This is
        /// extremely unusual within the dear imgui ecosystem and so we made it
        /// explicit.
        const SourceAllowNullID        = 1 << 3;
        /// External source (from outside of imgui), won't attempt to read
        /// current item/window info. Will always return true. Only one Extern
        /// source can be active simultaneously.
        const SourceExtern             = 1 << 4;
        // AcceptDragDropPayload() flags
        /// igAcceptDragDropPayload() will returns true even before the mouse
        /// button is released. You can then call igIsDelivery() to test if the
        /// payload needs to be delivered.
        const AcceptBeforeDelivery     = 1 << 10;
        /// Do not draw the default highlight rectangle when hovering over target.
        const AcceptNoDrawDefaultRect  = 1 << 11;
        /// For peeking ahead and inspecting the payload before delivery.
        const AcceptPeekOnly           = ImGuiDragDropFlags::AcceptBeforeDelivery.bits
            | ImGuiDragDropFlags::AcceptNoDrawDefaultRect.bits;
    }
);

bitflags!(
    /// Flags for indictating which corner of a rectangle should be rounded
    #[repr(C)]
    pub struct ImDrawCornerFlags: c_int {
        const TopLeft  = 1 << 0;
        const TopRight = 1 << 1;
        const BotLeft  = 1 << 2;
        const BotRight = 1 << 3;
        const Top      = ImDrawCornerFlags::TopLeft.bits
                       | ImDrawCornerFlags::TopRight.bits;
        const Bot      = ImDrawCornerFlags::BotLeft.bits
                       | ImDrawCornerFlags::BotRight.bits;
        const Left     = ImDrawCornerFlags::TopLeft.bits
                       | ImDrawCornerFlags::BotLeft.bits;
        const Right    = ImDrawCornerFlags::TopRight.bits
                       | ImDrawCornerFlags::BotRight.bits;
        const All      = 0xF;
    }
);

bitflags!(
    #[repr(C)]
    pub struct ImDrawListFlags: c_int {
        const AntiAliasedLines = 1 << 0;
        const AntiAliasedFill  = 1 << 1;
    }
);

pub type ImGuiTextEditCallback = Option<
    extern "C" fn(data: *mut ImGuiTextEditCallbackData) -> c_int,
>;

pub type ImGuiSizeConstraintCallback =
    Option<extern "C" fn(data: *mut ImGuiSizeConstraintCallbackData)>;

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
pub struct ImGuiStyle {
    /// Global alpha applies to everything in ImGui
    pub alpha: c_float,
    /// Padding within a window
    pub window_padding: ImVec2,
    /// Radius of window corners rounding. Set to 0.0f to have rectangular windows
    pub window_rounding: c_float,
    /// Thickness of border around windows. Generally set to 0.0f or 1.0f. Other values not well tested.
    pub window_border_size: c_float,
    /// Minimum window size
    pub window_min_size: ImVec2,
    /// Alignment for title bar text. Defaults to (0.0f, 0.5f) for left-aligned, vertically centered
    pub window_title_align: ImVec2,
    /// Radius of child window corners rounding. Set to 0.0f to have rectangular child windows
    pub child_rounding: c_float,
    /// Thickness of border around child windows. Generally set to 0.0f or 1.0f. Other values not well tested.
    pub child_border_size: c_float,
    /// Radius of popup window corners rounding. Set to 0.0f to have rectangular child windows
    pub popup_rounding: c_float,
    /// Thickness of border around popup or tooltip windows. Generally set to 0.0f or 1.0f. Other values not well tested.
    pub popup_border_size: c_float,
    /// Padding within a framed rectangle (used by most widgets)
    pub frame_padding: ImVec2,
    /// Radius of frame corners rounding. Set to 0.0f to have rectangular frames (used by most
    /// widgets).
    pub frame_rounding: c_float,
    /// Thickness of border around frames. Generally set to 0.0f or 1.0f. Other values not well tested.
    pub frame_border_size: c_float,
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
    pub anti_aliased_fill: bool,
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

    pub opt_mac_osx_behaviors: bool,
    pub opt_cursor_blink: bool,

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
    mouse_drag_max_distance_abs: [ImVec2; 5],
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

/// Data payload for Drag and Drop operations
#[repr(C)]
pub struct ImGuiPayload {
    /// Data (copied and owned by dear imgui)
    pub data: *const c_void,
    /// Data size
    pub data_size: c_int,

    /// Source item id
    source_id: ImGuiID,
    /// Source parent id (if available)
    source_parent_id: ImGuiID,
    /// Data timestamp
    data_frame_count: c_int,
    /// Data type tag (short user-supplied string)
    data_type: [c_char; 8 + 1],
    /// Set when AcceptDragDropPayload() was called and mouse has been hovering the target item (nb: handle overlapping drag targets)
    preview: bool,
    /// Set when AcceptDragDropPayload() was called and mouse button is released over the target item.
    delivery: bool,
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

    flags: ImDrawListFlags,
    data: *const ImDrawListSharedData,
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

#[repr(C)]
pub struct ImDrawListSharedData {
    /// UV of white pixel in the atlas
    tex_uv_white_pixel: ImVec2,
    /// Current/default font (optional, for simplified AddText overload)
    font: *mut ImFont,
    /// Current/default font size (optional, for simplified AddText overload)
    font_size: c_float,
    curve_tessellation_tol: c_float,
    /// Value for PushClipRectFullscreen()
    clip_rect_fullscreen: ImVec4,
    circle_vtx12: [ImVec2; 12],
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
    pub fn igEndFrame();
    pub fn igShutdown();
}

// Demo/Debug/Info
extern "C" {
    pub fn igShowDemoWindow(opened: *mut bool);
    pub fn igShowMetricsWindow(opened: *mut bool);
    pub fn igShowStyleEditor(style: *mut ImGuiStyle);
    pub fn igShowStyleSelector(label: *const c_char);
    pub fn igShowFontSelector(label: *const c_char);
    pub fn igShowUserGuide();
}

#[allow(non_snake_case)]
#[deprecated(since = "0.0.19", note = "please use igShowDemoWindow instead")]
pub unsafe fn igShowTestWindow(opened: *mut bool) {
   igShowDemoWindow(opened)
}

// Window
extern "C" {
    pub fn igBegin(name: *const c_char, open: *mut bool, flags: ImGuiWindowFlags) -> bool;
    pub fn igEnd();
    pub fn igBeginChild(
        str_id: *const c_char,
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
    pub fn igSetNextWindowCollapsed(collapsed: bool, cond: ImGuiCond);
    pub fn igSetNextWindowFocus();
    pub fn igSetWindowPos(pos: ImVec2, cond: ImGuiCond);
    pub fn igSetWindowSize(size: ImVec2, cond: ImGuiCond);
    pub fn igSetWindowCollapsed(collapsed: bool, cond: ImGuiCond);
    pub fn igSetWindowFocus();
    pub fn igSetWindowPosByName(name: *const c_char, pos: ImVec2, cond: ImGuiCond);
    pub fn igSetWindowSize2(name: *const c_char, size: ImVec2, cond: ImGuiCond);
    pub fn igSetWindowCollapsed2(name: *const c_char, collapsed: bool, cond: ImGuiCond);
    pub fn igSetWindowFocus2(name: *const c_char);

    pub fn igGetScrollX() -> c_float;
    pub fn igGetScrollY() -> c_float;
    pub fn igGetScrollMaxX() -> c_float;
    pub fn igGetScrollMaxY() -> c_float;
    pub fn igSetScrollX(scroll_x: c_float);
    pub fn igSetScrollY(scroll_y: c_float);
    pub fn igSetScrollHere(center_y_ratio: c_float);
    pub fn igSetScrollFromPosY(pos_y: c_float, center_y_ratio: c_float);
    pub fn igSetStateStorage(tree: *mut ImGuiStorage);
    pub fn igGetStateStorage() -> *mut ImGuiStorage;
}

/// Set next window content's width.
///
/// Original non-deprecated version preserved last Y value set by
/// [`igSetNextWindowContentSize`].
#[allow(non_snake_case)]
#[deprecated(since = "0.0.19", note = "please use igSetNextWindowContentSize instead")]
pub unsafe fn igSetNextWindowContentWidth(width: c_float) {
    igSetNextWindowContentSize(ImVec2 { x: width, y: 0.0 })
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
    pub fn igGetFrameHeight() -> c_float;
    pub fn igGetFrameHeightWithSpacing() -> c_float;
}

#[allow(non_snake_case)]
#[deprecated(since = "0.0.19", note = "please use igGetFrameHeightWithSpacing instead")]
pub unsafe fn igGetItemsLineHeightWithSpacing() -> c_float {
    igGetFrameHeightWithSpacing()
}

// Columns
extern "C" {
    pub fn igColumns(count: c_int, id: *const c_char, border: bool);
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
    pub fn igPushIDStr(str_id: *const c_char);
    pub fn igPushIDStrRange(str_begin: *const c_char, str_end: *const c_char);
    pub fn igPushIDPtr(ptr_id: *const c_void);
    pub fn igPushIDInt(int_id: c_int);
    pub fn igPopID();
    pub fn igGetIDStr(str_id: *const c_char) -> ImGuiID;
    pub fn igGetIDStrRange(str_begin: *const c_char, str_end: *const c_char) -> ImGuiID;
    pub fn igGetIDPtr(ptr_id: *const c_void) -> ImGuiID;
}

// Widgets
extern "C" {
    pub fn igTextUnformatted(text: *const c_char, text_end: *const c_char);
    pub fn igText(fmt: *const c_char, ...);
    // pub fn igTextV(fmt: *const c_char, args: va_list);
    pub fn igTextColored(col: ImVec4, fmt: *const c_char, ...);
    // pub fn igTextColoredV(col: ImVec4, fmt: *const c_char, args: va_list);
    pub fn igTextDisabled(fmt: *const c_char, ...);
    // pub fn igTextDisabledV(fmt: *const c_char, args: va_list);
    pub fn igTextWrapped(fmt: *const c_char, ...);
    // pub fn igTextWrappedV(fmt: *const c_char, args: va_list);
    pub fn igLabelText(label: *const c_char, fmt: *const c_char, ...);
    // pub fn igLabelTextV(label: *const c_char, fmt: *const c_char, args: va_list);
    pub fn igBulletText(fmt: *const c_char, ...);
    // pub fn igBulletTextV(fmt: *const c_char, args: va_list);
    pub fn igBullet();
    pub fn igButton(label: *const c_char, size: ImVec2) -> bool;
    pub fn igSmallButton(label: *const c_char) -> bool;
    pub fn igInvisibleButton(str_id: *const c_char, size: ImVec2) -> bool;
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
    pub fn igCheckbox(label: *const c_char, v: *mut bool) -> bool;
    pub fn igCheckboxFlags(label: *const c_char, flags: *mut c_uint, flags_value: c_uint) -> bool;
    pub fn igRadioButtonBool(label: *const c_char, active: bool) -> bool;
    pub fn igRadioButton(label: *const c_char, v: *mut c_int, v_button: c_int) -> bool;
    pub fn igPlotLines(
        label: *const c_char,
        values: *const c_float,
        values_count: c_int,
        values_offset: c_int,
        overlay_text: *const c_char,
        scale_min: c_float,
        scale_max: c_float,
        graph_size: ImVec2,
        stride: c_int,
    );
    pub fn igPlotLines2(
        label: *const c_char,
        values_getter: extern "C" fn(data: *mut c_void, idx: c_int) -> c_float,
        data: *mut c_void,
        values_count: c_int,
        values_offset: c_int,
        overlay_text: *const c_char,
        scale_min: c_float,
        scale_max: c_float,
        graph_size: ImVec2,
    );
    pub fn igPlotHistogram(
        label: *const c_char,
        values: *const c_float,
        values_count: c_int,
        values_offset: c_int,
        overlay_text: *const c_char,
        scale_min: c_float,
        scale_max: c_float,
        graph_size: ImVec2,
        stride: c_int,
    );
    pub fn igPlotHistogram2(
        label: *const c_char,
        values_getter: extern "C" fn(data: *mut c_void, idx: c_int) -> c_float,
        data: *mut c_void,
        values_count: c_int,
        values_offset: c_int,
        overlay_text: *const c_char,
        scale_min: c_float,
        scale_max: c_float,
        graph_size: ImVec2,
    );
    pub fn igProgressBar(fraction: c_float, size_arg: *const ImVec2, overlay: *const c_char);
}

// Combo
extern "C" {
    pub fn igBeginCombo(
        label: *const c_char,
        preview_value: *const c_char,
        flags: ImGuiComboFlags,
    ) -> bool;
    pub fn igEndCombo();
    pub fn igCombo(
        label: *const c_char,
        current_item: *mut c_int,
        items: *const *const c_char,
        items_count: c_int,
        height_in_items: c_int,
    ) -> bool;
    pub fn igCombo2(
        label: *const c_char,
        current_item: *mut c_int,
        items_separated_by_zeros: *const c_char,
        height_in_items: c_int,
    ) -> bool;
    pub fn igCombo3(
        label: *const c_char,
        current_item: *mut c_int,
        items_getter: extern "C" fn(data: *mut c_void, idx: c_int, out_text: *mut *const c_char) -> bool,
        data: *mut c_void,
        items_count: c_int,
        height_in_items: c_int,
    ) -> bool;
}

// Widgets: Color Editor/Picker
extern "C" {
    pub fn igColorEdit3(
        label: *const c_char,
        col: *mut c_float,
        flags: ImGuiColorEditFlags,
    ) -> bool;
    pub fn igColorEdit4(
        label: *const c_char,
        col: *mut c_float,
        flags: ImGuiColorEditFlags,
    ) -> bool;
    pub fn igColorPicker3(
        label: *const c_char,
        col: *mut c_float,
        flags: ImGuiColorEditFlags,
    ) -> bool;
    pub fn igColorPicker4(
        label: *const c_char,
        col: *mut c_float,
        flags: ImGuiColorEditFlags,
        ref_col: *const c_float,
    ) -> bool;
    pub fn igColorButton(
        desc_id: *const c_char,
        col: ImVec4,
        flags: ImGuiColorEditFlags,
        size: ImVec2,
    ) -> bool;
    pub fn igSetColorEditOptions(flags: ImGuiColorEditFlags);
}

// Widgets: Drags
extern "C" {
    pub fn igDragFloat(
        label: *const c_char,
        v: *mut c_float,
        v_speed: c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igDragFloat2(
        label: *const c_char,
        v: *mut c_float,
        v_speed: c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igDragFloat3(
        label: *const c_char,
        v: *mut c_float,
        v_speed: c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igDragFloat4(
        label: *const c_char,
        v: *mut c_float,
        v_speed: c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igDragFloatRange2(
        label: *const c_char,
        v_current_min: *mut c_float,
        v_current_max: *mut c_float,
        v_speed: c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: *const c_char,
        display_format_max: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igDragInt(
        label: *const c_char,
        v: *mut c_int,
        v_speed: c_float,
        v_min: c_int,
        v_max: c_int,
        display_format: *const c_char,
    ) -> bool;
    pub fn igDragInt2(
        label: *const c_char,
        v: *mut c_int,
        v_speed: c_float,
        v_min: c_int,
        v_max: c_int,
        display_format: *const c_char,
    ) -> bool;
    pub fn igDragInt3(
        label: *const c_char,
        v: *mut c_int,
        v_speed: c_float,
        v_min: c_int,
        v_max: c_int,
        display_format: *const c_char,
    ) -> bool;
    pub fn igDragInt4(
        label: *const c_char,
        v: *mut c_int,
        v_speed: c_float,
        v_min: c_int,
        v_max: c_int,
        display_format: *const c_char,
    ) -> bool;
    pub fn igDragIntRange2(
        label: *const c_char,
        v_current_min: *mut c_int,
        v_current_max: *mut c_int,
        v_speed: c_float,
        v_min: c_int,
        v_max: c_int,
        display_format: *const c_char,
        display_format_max: *const c_char,
    ) -> bool;
}

// Widgets: Input with Keyboard
extern "C" {
    pub fn igInputText(
        label: *const c_char,
        buf: *mut c_char,
        buf_size: usize,
        flags: ImGuiInputTextFlags,
        callback: ImGuiTextEditCallback,
        user_data: *mut c_void,
    ) -> bool;
    pub fn igInputTextMultiline(
        label: *const c_char,
        buf: *mut c_char,
        buf_size: usize,
        size: ImVec2,
        flags: ImGuiInputTextFlags,
        callback: ImGuiTextEditCallback,
        user_data: *mut c_void,
    ) -> bool;
    pub fn igInputFloat(
        label: *const c_char,
        v: *mut c_float,
        step: c_float,
        step_fast: c_float,
        decimal_precision: c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputFloat2(
        label: *const c_char,
        v: *mut c_float,
        decimal_precision: c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputFloat3(
        label: *const c_char,
        v: *mut c_float,
        decimal_precision: c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputFloat4(
        label: *const c_char,
        v: *mut c_float,
        decimal_precision: c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputInt(
        label: *const c_char,
        v: *mut c_int,
        step: c_int,
        step_fast: c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputInt2(
        label: *const c_char,
        v: *mut c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputInt3(
        label: *const c_char,
        v: *mut c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputInt4(
        label: *const c_char,
        v: *mut c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
}

// Widgets: Sliders
extern "C" {
    pub fn igSliderFloat(
        label: *const c_char,
        v: *mut c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igSliderFloat2(
        label: *const c_char,
        v: *mut c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igSliderFloat3(
        label: *const c_char,
        v: *mut c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igSliderFloat4(
        label: *const c_char,
        v: *mut c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igSliderAngle(
        label: *const c_char,
        v_rad: *mut c_float,
        v_degrees_min: c_float,
        v_degrees_max: c_float,
    ) -> bool;
    pub fn igSliderInt(
        label: *const c_char,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        display_format: *const c_char,
    ) -> bool;
    pub fn igSliderInt2(
        label: *const c_char,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        display_format: *const c_char,
    ) -> bool;
    pub fn igSliderInt3(
        label: *const c_char,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        display_format: *const c_char,
    ) -> bool;
    pub fn igSliderInt4(
        label: *const c_char,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        display_format: *const c_char,
    ) -> bool;
    pub fn igVSliderFloat(
        label: *const c_char,
        size: ImVec2,
        v: *mut c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igVSliderInt(
        label: *const c_char,
        size: ImVec2,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        display_format: *const c_char,
    ) -> bool;
}

// Widgets: Trees
extern "C" {
    pub fn igTreeNode(label: *const c_char) -> bool;
    pub fn igTreeNodeStr(str_id: *const c_char, fmt: *const c_char, ...) -> bool;
    pub fn igTreeNodePtr(ptr_id: *const c_void, fmt: *const c_char, ...) -> bool;
    // pub fn igTreeNodeStrV(str_id: *const c_char, fmt: *const c_char, args: va_list) -> bool;
    // pub fn igTreeNodePtrV(ptr_id: *const c_void, fmt: *const c_char, args: va_list) -> bool;
    pub fn igTreeNodeEx(label: *const c_char, flags: ImGuiTreeNodeFlags) -> bool;
    pub fn igTreeNodeExStr(
        str_id: *const c_char,
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
    pub fn igTreePushStr(str_id: *const c_char);
    pub fn igTreePushPtr(ptr_id: *const c_void);
    pub fn igTreePop();
    pub fn igTreeAdvanceToLabelPos();
    pub fn igGetTreeNodeToLabelSpacing() -> c_float;
    pub fn igSetNextTreeNodeOpen(opened: bool, cond: ImGuiCond);
    pub fn igCollapsingHeader(label: *const c_char, flags: ImGuiTreeNodeFlags) -> bool;
    pub fn igCollapsingHeaderEx(
        label: *const c_char,
        open: *mut bool,
        flags: ImGuiTreeNodeFlags,
    ) -> bool;
}

// Widgets: Selectable / Lists
extern "C" {
    pub fn igSelectable(
        label: *const c_char,
        selected: bool,
        flags: ImGuiSelectableFlags,
        size: ImVec2,
    ) -> bool;
    pub fn igSelectableEx(
        label: *const c_char,
        p_selected: *mut bool,
        flags: ImGuiSelectableFlags,
        size: ImVec2,
    ) -> bool;
    pub fn igListBox(
        label: *const c_char,
        current_item: *mut c_int,
        items: *const *const c_char,
        items_count: c_int,
        height_in_items: c_int,
    ) -> bool;
    pub fn igListBox2(
        label: *const c_char,
        current_item: *mut c_int,
        items_getter: extern "C" fn(data: *mut c_void, idx: c_int, out_text: *mut *const c_char)
                                    -> bool,
        data: *mut c_void,
        items_count: c_int,
        height_in_items: c_int,
    ) -> bool;
    pub fn igListBoxHeader(label: *const c_char, size: ImVec2) -> bool;
    pub fn igListBoxHeader2(
        label: *const c_char,
        items_count: c_int,
        height_in_items: c_int,
    ) -> bool;
    pub fn igListBoxFooter();
}

// Widgets: Value() Helpers
extern "C" {
    pub fn igValueBool(prefix: *const c_char, b: bool);
    pub fn igValueInt(prefix: *const c_char, v: c_int);
    pub fn igValueUInt(prefix: *const c_char, v: c_uint);
    pub fn igValueFloat(prefix: *const c_char, v: c_float, float_format: *const c_char);
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
    pub fn igBeginMenu(label: *const c_char, enabled: bool) -> bool;
    pub fn igEndMenu();
    pub fn igMenuItem(
        label: *const c_char,
        shortcut: *const c_char,
        selected: bool,
        enabled: bool,
    ) -> bool;
    pub fn igMenuItemPtr(
        label: *const c_char,
        shortcut: *const c_char,
        p_selected: *mut bool,
        enabled: bool,
    ) -> bool;
}

// Popup
extern "C" {
    pub fn igOpenPopup(str_id: *const c_char);
    pub fn igOpenPopupOnItemClick(str_id: *const c_char, mouse_button: c_int) -> bool;
    pub fn igBeginPopup(str_id: *const c_char) -> bool;
    pub fn igBeginPopupModal(
        name: *const c_char,
        open: *mut bool,
        extra_flags: ImGuiWindowFlags,
    ) -> bool;
    pub fn igBeginPopupContextItem(str_id: *const c_char, mouse_button: c_int) -> bool;
    pub fn igBeginPopupContextWindow(
        str_id: *const c_char,
        mouse_button: c_int,
        also_over_items: bool,
    ) -> bool;
    pub fn igBeginPopupContextVoid(str_id: *const c_char, mouse_button: c_int) -> bool;
    pub fn igEndPopup();
    pub fn igIsPopupOpen(str_id: *const c_char) -> bool;
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

// DragDrop
extern "C" {
    /// Call when current ID is active.
    ///
    /// When this returns true you need to:
    ///
    /// 1. call [`igSetDragDropPayload`] exactly once,
    /// 2. you may render the payload visual/description,
    /// 3. pcall [`igEndDragDropSource`]
    pub fn igBeginDragDropSource(flags: ImGuiDragDropFlags, mouse_button: c_int) -> bool;
    /// Use 'cond' to choose to submit payload on drag start or every frame
    pub fn igSetDragDropPayload(type_: *const c_char, data: *const c_void, size: libc::size_t, cond: ImGuiCond) -> bool;
    pub fn igEndDragDropSource();
    pub fn igBeginDragDropTarget() -> bool;
    pub fn igAcceptDragDropPayload(type_: *const c_char, flags: ImGuiDragDropFlags) -> *const ImGuiPayload;
    pub fn igEndDragDropTarget();
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
    pub fn igStyleColorsDark(dst: *mut ImGuiStyle);
    pub fn igStyleColorsLight(dst: *mut ImGuiStyle);
}

// Focus
extern "C" {
    pub fn igSetItemDefaultFocus();
    pub fn igSetKeyboardFocusHere(offset: c_int);
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
    pub fn igIsWindowFocused(flags: ImGuiFocusedFlags) -> bool;
    pub fn igIsWindowHovered(flags: ImGuiHoveredFlags) -> bool;
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
        text: *const c_char,
        text_end: *const c_char,
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

#[allow(non_snake_case)]
#[deprecated(since = "0.0.19", note = "please use igIsWindowFocused(ImGuiFocusedFlags::RootWindow) instead")]
pub unsafe fn igIsRootWindowFocused() -> bool {
    igIsWindowFocused(ImGuiFocusedFlags::RootWindow)
}
#[allow(non_snake_case)]
#[deprecated(since = "0.0.19", note = "please use igIsWindowFocused(ImGuiFocusedFlags::RootAndChildWindows) instead")]
pub unsafe fn igIsRootWindowOrAnyChildFocused() -> bool {
    igIsWindowFocused(ImGuiFocusedFlags::RootAndChildWindows)
}
#[allow(non_snake_case)]
#[deprecated(since = "0.0.19", note = "please use igIsWindowFocused(ImGuiFocusedFlags::RootAndChildWindows) instead")]
pub unsafe fn igIsRootWindowOrAnyChildHovered(_flags: ImGuiHoveredFlags) -> bool {
    igIsWindowHovered(ImGuiHoveredFlags::RootAndChildWindows)
}

// DrawList
extern "C" {
    pub fn igGetOverlayDrawList() -> *mut ImDrawList;
    pub fn igGetDrawListSharedData() -> *mut ImDrawListSharedData;
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

extern "C" {
    pub fn ImFontConfig_DefaultConstructor(config: *mut ImFontConfig);
}

// ImGuiIO
extern "C" {
    pub fn ImGuiIO_AddInputCharacter(c: c_ushort);
    pub fn ImGuiIO_AddInputCharactersUTF8(utf8_chars: *const c_char);
    pub fn ImGuiIO_ClearInputCharacters();
}

// ImGuiTextFilter
extern "C" {
    pub fn ImGuiTextFilter_Create(default_filter: *const c_char) -> *mut ImGuiTextFilter;
    pub fn ImGuiTextFilter_Destroy(filter: *mut ImGuiTextFilter);
    pub fn ImGuiTextFilter_Clear(filter: *mut ImGuiTextFilter);
    pub fn ImGuiTextFilter_Draw(
        filter: *mut ImGuiTextFilter,
        label: *const c_char,
        width: c_float,
    ) -> bool;
    pub fn ImGuiTextFilter_PassFilter(
        filter: *const ImGuiTextFilter,
        text: *const c_char,
        text_end: *const c_char,
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
    pub fn ImGuiTextBuffer_appendf(buffer: *const ImGuiTextBuffer, fmt: *const c_char, ...);
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
        text: *const c_char,
        text_end: *const c_char,
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
        rounding_corners_flags: ImDrawCornerFlags,
        thickness: c_float,
    );
    pub fn ImDrawList_AddRectFilled(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        col: ImU32,
        rounding: c_float,
        rounding_corners_flags: ImDrawCornerFlags,
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
        thickness: c_float,
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
        text_begin: *const c_char,
        text_end: *const c_char,
    );
    pub fn ImDrawList_AddTextExt(
        list: *mut ImDrawList,
        font: *const ImFont,
        font_size: c_float,
        pos: ImVec2,
        col: ImU32,
        text_begin: *const c_char,
        text_end: *const c_char,
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
    pub fn ImDrawList_AddImageRounded(
        list: *mut ImDrawList,
        user_texture_id: ImTextureID,
        a: ImVec2,
        b: ImVec2,
        uv_a: ImVec2,
        uv_b: ImVec2,
        col: ImU32,
        rounding: c_float,
        rounding_corners: c_int,
    );
    pub fn ImDrawList_AddPolyLine(
        list: *mut ImDrawList,
        points: *const ImVec2,
        num_points: c_int,
        col: ImU32,
        closed: bool,
        thickness: c_float,
    );
    pub fn ImDrawList_AddConvexPolyFilled(
        list: *mut ImDrawList,
        points: *const ImVec2,
        num_points: c_int,
        col: ImU32,
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
    pub fn ImFont_GetDebugName(font: *const ImFont) -> *const c_char;
    pub fn ImFont_CalcTextSizeA(
        font: *const ImFont,
        out: *mut ImVec2,
        size: c_float,
        max_width: c_float,
        wrap_width: c_float,
        text_begin: *const c_char,
        text_end: *const c_char,
        remaining: *mut *const c_char,
    );
    pub fn ImFont_CalcWordWrapPositionA(
        font: *const ImFont,
        scale: c_float,
        text: *const c_char,
        text_end: *const c_char,
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
        text_begin: *const c_char,
        text_end: *const c_char,
        wrap_width: c_float,
        cpu_fine_clip: bool,
    );
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
    assert_eq!(style.window_rounding, 7.0);
    assert_eq!(style.window_border_size, 0.0);
    assert_eq!(style.window_min_size, ImVec2::new(32.0, 32.0));
    assert_eq!(style.window_title_align, ImVec2::new(0.0, 0.5));
    assert_eq!(style.popup_rounding, 0.0);
    assert_eq!(style.popup_border_size, 1.0);
    assert_eq!(style.frame_padding, ImVec2::new(4.0, 3.0));
    assert_eq!(style.frame_rounding, 0.0);
    assert_eq!(style.frame_border_size, 0.0);
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
    assert_eq!(style.anti_aliased_fill, true);
    assert_eq!(style.curve_tessellation_tol, 1.25);
}
