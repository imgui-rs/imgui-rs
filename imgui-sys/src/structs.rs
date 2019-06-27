use std::os::raw::{c_char, c_float, c_int, c_void};
use std::slice;

use crate::enums::{ImGuiCol, ImGuiDir};
use crate::{ImGuiID, ImGuiStorage, ImVec2, ImVec4};

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
    data_type: [c_char; 32 + 1],
    /// Set when AcceptDragDropPayload() was called and mouse has been hovering the target item (nb: handle overlapping drag targets)
    preview: bool,
    /// Set when AcceptDragDropPayload() was called and mouse button is released over the target item.
    delivery: bool,
}

/// Callback data for size constraint callback
#[repr(C)]
pub struct ImGuiSizeCallbackData {
    pub user_data: *mut c_void,
    pub pos: ImVec2,
    pub current_size: ImVec2,
    pub desired_size: ImVec2,
}

/// Runtime data for styling/colors
#[repr(C)]
#[derive(Clone)]
pub struct ImGuiStyle {
    /// Global alpha applies to everything in ImGui.
    pub alpha: c_float,
    /// Padding within a window.
    pub window_padding: ImVec2,
    /// Radius of window corners rounding. Set to 0.0 to have rectangular windows.
    pub window_rounding: c_float,
    /// Thickness of border around windows. Generally set to 0.0 or 1.0. (Other values are not well
    /// tested and more CPU/GPU costly).
    pub window_border_size: c_float,
    /// Minimum window size. This is a global setting. If you want to constraint individual
    /// windows, use igSetNextWindowSizeConstraints().
    pub window_min_size: ImVec2,
    /// Alignment for title bar text. Defaults to (0.0, 0.5) for left-aligned, vertically centered.
    pub window_title_align: ImVec2,
    pub window_menu_button_position: ImGuiDir,
    /// Radius of child window corners rounding. Set to 0.0 to have rectangular windows.
    pub child_rounding: c_float,
    /// Thickness of border around child windows. Generally set to 0.0 or 1.0. (Other values are
    /// not well tested and more CPU/GPU costly).
    pub child_border_size: c_float,
    /// Radius of popup window corners rounding. (Note that tooltip windows use window_rounding)
    pub popup_rounding: c_float,
    /// Thickness of border around popup/tooltip windows. Generally set to 0.0 or 1.0. (Other
    /// values are not well tested and more CPU/GPU costly).
    pub popup_border_size: c_float,
    /// Padding within a framed rectangle (used by most widgets).
    pub frame_padding: ImVec2,
    /// Radius of frame corners rounding. Set to 0.0 to have rectangular frame (used by most
    /// widgets).
    pub frame_rounding: c_float,
    /// Thickness of border around frames. Generally set to 0.0 or 1.0. (Other values are not well
    /// tested and more CPU/GPU costly).
    pub frame_border_size: c_float,
    /// Horizontal and vertical spacing between widgets/lines.
    pub item_spacing: ImVec2,
    /// Horizontal and vertical spacing between within elements of a composed widget (e.g. a slider
    /// and its label).
    pub item_inner_spacing: ImVec2,
    /// Expand reactive bounding box for touch-based system where touch position is not accurate
    /// enough. Unfortunately we don't sort widgets so priority on overlap will always be given to
    /// the first widget. So don't grow this too much!
    pub touch_extra_padding: ImVec2,
    /// Horizontal indentation when e.g. entering a tree node. Generally == (FontSize +
    /// FramePadding.x*2).
    pub indent_spacing: c_float,
    /// Minimum horizontal spacing between two columns.
    pub columns_min_spacing: c_float,
    /// Width of the vertical scrollbar, Height of the horizontal scrollbar.
    pub scrollbar_size: c_float,
    /// Radius of grab corners for scrollbar.
    pub scrollbar_rounding: c_float,
    /// Minimum width/height of a grab box for slider/scrollbar.
    pub grab_min_size: c_float,
    /// Radius of grabs corners rounding. Set to 0.0 to have rectangular slider grabs.
    pub grab_rounding: c_float,
    pub tab_rounding: c_float,
    pub tab_border_size: c_float,
    /// Alignment of button text when button is larger than text. Defaults to (0.5, 0.5) for
    /// horizontally+vertically centered.
    pub button_text_align: ImVec2,
    pub selectable_text_align: ImVec2,
    /// Window position are clamped to be visible within the display area by at least this amount.
    /// Only applies to regular windows.
    pub display_window_padding: ImVec2,
    /// If you cannot see the edges of your screen (e.g. on a TV) increase the safe area padding.
    /// Apply to popups/tooltips as well regular windows. NB: Prefer configuring your TV sets
    /// correctly!
    pub display_safe_area_padding: ImVec2,
    /// Scale software rendered mouse cursor (when io.mouse_draw_cursor is enabled). May be removed
    /// later.
    pub mouse_cursor_scale: c_float,
    /// Enable anti-aliasing on lines/borders. Disable if you are really tight on CPU/GPU.
    pub anti_aliased_lines: bool,
    /// Enable anti-aliasing on filled shapes (rounded rectangles, circles, etc.)
    pub anti_aliased_fill: bool,
    /// Tessellation tolerance when using igPathBezierCurveTo() without a specific number of
    /// segments. Decrease for highly tessellated curves (higher quality, more polygons), increase
    /// to reduce quality.
    pub curve_tessellation_tol: c_float,
    /// Colors for the user interface
    pub colors: [ImVec4; ImGuiCol::COUNT],
}

/// Text buffer for logging/accumulating text
#[repr(C)]
pub struct ImGuiTextBuffer {
    pub buf: ImVector<c_char>,
}

/// Parse and apply text filters
#[repr(C)]
pub struct ImGuiTextFilter {
    pub input_buf: [c_char; 256],
    pub filters: ImVector<TextRange>,
    pub count_grep: c_int,
}

/// Lightweight vector struct
#[repr(C)]
pub struct ImVector<T> {
    pub size: c_int,
    pub capacity: c_int,
    pub data: *mut T,
}

impl<T> ImVector<T> {
    pub unsafe fn as_slice(&self) -> &[T] {
        slice::from_raw_parts(self.data, self.size as usize)
    }
}

/// ImGuiStorage key->value pair
#[repr(C)]
pub struct Pair {
    pub key: ImGuiID,
    pub value: PairValue,
}

/// ImGuiStorage value union
#[repr(C)]
pub union PairValue {
    pub val_i: c_int,
    pub val_f: c_float,
    pub val_p: *mut c_void,
}

/// ImGuiTextFilter text range
#[repr(C)]
pub struct TextRange {
    pub begin: *const c_char,
    pub end: *const c_char,
}

// ImGuiStyle
extern "C" {
    pub fn ImGuiStyle_ScaleAllSizes(this: *mut ImGuiStyle, scale_factor: c_float);
}

// ImGuiTextFilter
extern "C" {
    pub fn ImGuiTextFilter_Draw(
        this: *mut ImGuiTextFilter,
        label: *const c_char,
        width: c_float,
    ) -> bool;
    pub fn ImGuiTextFilter_PassFilter(
        this: *mut ImGuiTextFilter,
        text: *const c_char,
        text_end: *const c_char,
    ) -> bool;
    pub fn ImGuiTextFilter_Build(this: *mut ImGuiTextFilter);
    pub fn ImGuiTextFilter_Clear(this: *mut ImGuiTextFilter);
    pub fn ImGuiTextFilter_IsActive(this: *mut ImGuiTextFilter) -> bool;
}

// TextRange
extern "C" {
    pub fn TextRange_begin(this: *mut TextRange) -> *const c_char;
    pub fn TextRange_end(this: *mut TextRange) -> *const c_char;
    pub fn TextRange_empty(this: *mut TextRange) -> bool;
    pub fn TextRange_split(this: *mut TextRange, separator: c_char, out: *mut ImVector<TextRange>);
}

// ImGuiTextBuffer
extern "C" {
    pub fn ImGuiTextBuffer_begin(this: *mut ImGuiTextBuffer) -> *const c_char;
    pub fn ImGuiTextBuffer_end(this: *mut ImGuiTextBuffer) -> *const c_char;
    pub fn ImGuiTextBuffer_size(this: *mut ImGuiTextBuffer) -> c_int;
    pub fn ImGuiTextBuffer_empty(this: *mut ImGuiTextBuffer) -> bool;
    pub fn ImGuiTextBuffer_clear(this: *mut ImGuiTextBuffer);
    pub fn ImGuiTextBuffer_reserve(this: *mut ImGuiTextBuffer, capacity: c_int);
    pub fn ImGuiTextBuffer_c_str(this: *mut ImGuiTextBuffer) -> *const c_char;
    pub fn ImGuiTextBuffer_appendf(this: *mut ImGuiTextBuffer, fmt: *const c_char, ...);
}

// ImGuiStorage
extern "C" {
    pub fn ImGuiStorage_Clear(this: *mut ImGuiStorage);
    pub fn ImGuiStorage_GetInt(this: *mut ImGuiStorage, key: ImGuiID, default_val: c_int) -> c_int;
    pub fn ImGuiStorage_SetInt(this: *mut ImGuiStorage, key: ImGuiID, val: c_int);
    pub fn ImGuiStorage_GetBool(this: *mut ImGuiStorage, key: ImGuiID, default_val: bool) -> bool;
    pub fn ImGuiStorage_SetBool(this: *mut ImGuiStorage, key: ImGuiID, val: bool);
    pub fn ImGuiStorage_GetFloat(
        this: *mut ImGuiStorage,
        key: ImGuiID,
        default_val: c_float,
    ) -> c_float;
    pub fn ImGuiStorage_SetFloat(this: *mut ImGuiStorage, key: ImGuiID, val: c_float);
    pub fn ImGuiStorage_GetVoidPtr(this: *mut ImGuiStorage, key: ImGuiID);
    pub fn ImGuiStorage_SetVoidPtr(this: *mut ImGuiStorage, key: ImGuiID, val: *mut c_void);
    pub fn ImGuiStorage_GetIntRef(
        this: *mut ImGuiStorage,
        key: ImGuiID,
        default_val: c_int,
    ) -> *mut c_int;
    pub fn ImGuiStorage_GetBoolRef(
        this: *mut ImGuiStorage,
        key: ImGuiID,
        default_val: bool,
    ) -> *mut bool;
    pub fn ImGuiStorage_GetFloatRef(
        this: *mut ImGuiStorage,
        key: ImGuiID,
        default_val: c_float,
    ) -> *mut c_float;
    pub fn ImGuiStorage_GetVoidPtrRef(
        this: *mut ImGuiStorage,
        key: ImGuiID,
        default_val: *mut c_void,
    ) -> *mut *mut c_void;
    pub fn ImGuiStorage_SetAllInt(this: *mut ImGuiStorage, val: c_int);
    pub fn ImGuiStorage_BuildSortByKey(this: *mut ImGuiStorage);
}

// ImGuiPayload
extern "C" {
    pub fn ImGuiPayload_Clear(this: *mut ImGuiPayload);
    pub fn ImGuiPayload_IsDataType(this: *mut ImGuiPayload, type_: *const c_char) -> bool;
    pub fn ImGuiPayload_IsPreview(this: *mut ImGuiPayload) -> bool;
    pub fn ImGuiPayload_IsDelivery(this: *mut ImGuiPayload) -> bool;
}
