use std::os::raw::{c_char, c_float, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};
use std::slice;

use super::enums::{ImGuiCol, ImGuiKey, ImGuiMouseCursor, ImGuiNavInput};
use super::flags::{
    ImDrawCornerFlags, ImDrawListFlags, ImFontAtlasFlags, ImGuiBackendFlags, ImGuiConfigFlags,
    ImGuiInputTextFlags,
};
use super::{ImDrawCallback, ImDrawIdx, ImGuiID, ImTextureID, ImU32, ImVec2, ImVec4, ImWchar};

/// Font atlas glyph range builder
#[repr(C)]
pub struct GlyphRangesBuilder {
    pub used_chars: ImVector<c_uchar>,
}

/// Font atlas custom rectangle
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

/// Temporary storage for outputting drawing commands out of order
#[repr(C)]
pub struct ImDrawChannel {
    pub cmd_buffer: ImVector<ImDrawCmd>,
    pub idx_buffer: ImVector<ImDrawIdx>,
}

/// A single draw command within a parent ImDrawList (generally maps to 1 GPU draw call)
#[repr(C)]
pub struct ImDrawCmd {
    pub elem_count: c_uint,
    pub clip_rect: ImVec4,
    pub texture_id: ImTextureID,
    pub user_callback: ImDrawCallback,
    pub user_callback_data: *mut c_void,
}

/// All draw command lists required to render the frame
#[repr(C)]
pub struct ImDrawData {
    pub valid: bool,
    pub cmd_lists: *mut *mut ImDrawList,
    pub cmd_lists_count: c_int,
    pub total_idx_count: c_int,
    pub total_vtx_count: c_int,
    pub display_pos: ImVec2,
    pub display_size: ImVec2,
}

impl ImDrawData {
    pub unsafe fn cmd_lists(&self) -> &[*const ImDrawList] {
        let cmd_lists = self.cmd_lists as *const *const ImDrawList;
        slice::from_raw_parts(cmd_lists, self.cmd_lists_count as usize)
    }
}

/// A single draw command list (generally one per window)
#[repr(C)]
pub struct ImDrawList {
    pub cmd_buffer: ImVector<ImDrawCmd>,
    pub idx_buffer: ImVector<ImDrawIdx>,
    pub vtx_buffer: ImVector<ImDrawVert>,
    pub flags: ImDrawListFlags,

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

/// Data shared among multiple draw lists
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

/// A single vertex
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct ImDrawVert {
    pub pos: ImVec2,
    pub uv: ImVec2,
    pub col: ImU32,
}

/// Runtime data for a single font within a parent ImFontAtlas
#[repr(C)]
pub struct ImFont {
    pub font_size: c_float,
    pub scale: c_float,
    pub display_offset: ImVec2,
    pub glyphs: ImVector<ImFontGlyph>,
    pub index_advance_x: ImVector<c_float>,
    pub index_lookup: ImVector<c_ushort>,
    pub fallback_glyph: *const ImFontGlyph,
    pub fallback_advance_x: c_float,
    pub fallback_char: ImWchar,

    pub config_data_count: c_short,
    pub config_data: *mut ImFontConfig,
    pub container_font_atlas: *mut ImFontAtlas,
    pub ascent: c_float,
    pub descent: c_float,
    pub dirty_lookup_tables: bool,
    pub metrics_total_surface: c_int,
}

/// Runtime data for multiple fonts, bake multiple fonts into a single texture, TTF/OTF font loader
#[repr(C)]
pub struct ImFontAtlas {
    pub locked: bool,
    pub flags: ImFontAtlasFlags,
    pub tex_id: ImTextureID,
    pub tex_desired_width: c_int,
    pub tex_glyph_padding: c_int,

    pub tex_pixels_alpha8: *mut c_uchar,
    pub tex_pixels_rgba32: *mut c_uint,
    pub tex_width: c_int,
    pub tex_height: c_int,
    pub tex_uv_scale: ImVec2,
    pub tex_uv_white_pixel: ImVec2,
    pub fonts: ImVector<*mut ImFont>,
    pub custom_rects: ImVector<CustomRect>,
    pub config_data: ImVector<ImFontConfig>,
    pub custom_rect_ids: [c_int; 1],
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
    pub glyph_min_advance_x: c_float,
    pub glyph_max_advance_x: c_float,
    pub merge_mode: bool,
    pub rasterizer_flags: c_uint,
    pub rasterizer_multiply: c_float,

    name: [c_char; 40],
    dst_font: *mut ImFont,
}

/// Font glyph
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct ImFontGlyph {
    pub codepoint: ImWchar,
    pub advance_x: c_float,
    pub x0: c_float,
    pub y0: c_float,
    pub x1: c_float,
    pub y1: c_float,
    pub u0: c_float,
    pub v0: c_float,
    pub u1: c_float,
    pub v1: c_float,
}

/// Shared state of input text callback
#[repr(C)]
pub struct ImGuiInputTextCallbackData {
    pub event_flag: ImGuiInputTextFlags,
    pub flags: ImGuiInputTextFlags,
    pub user_data: *mut c_void,

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

/// Main configuration and I/O between your application and ImGui
#[repr(C)]
pub struct ImGuiIO {
    pub config_flags: ImGuiConfigFlags,
    pub backend_flags: ImGuiBackendFlags,
    pub display_size: ImVec2,
    pub delta_time: c_float,
    pub ini_saving_rate: c_float,
    pub ini_filename: *const c_char,
    pub log_filename: *const c_char,
    pub mouse_double_click_time: c_float,
    pub mouse_double_click_max_dist: c_float,
    pub mouse_drag_threshold: c_float,
    pub key_map: [c_int; ImGuiKey::COUNT],
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
    pub mouse_draw_cursor: bool,
    pub config_mac_osx_behaviors: bool,
    pub config_input_text_cursor_blink: bool,
    pub config_resize_windows_from_edges: bool,

    pub get_clipboard_text_fn: Option<extern "C" fn(user_data: *mut c_void) -> *const c_char>,
    pub set_clipboard_text_fn: Option<extern "C" fn(user_data: *mut c_void, text: *const c_char)>,
    pub clipboard_user_data: *mut c_void,

    pub ime_set_input_screen_pos_fn: Option<extern "C" fn(x: c_int, y: c_int)>,
    pub ime_window_handle: *mut c_void,

    pub render_draw_lists_fn_unused: *mut c_void,

    pub mouse_pos: ImVec2,
    pub mouse_down: [bool; 5],
    pub mouse_wheel: c_float,
    pub mouse_wheel_h: c_float,
    pub key_ctrl: bool,
    pub key_shift: bool,
    pub key_alt: bool,
    pub key_super: bool,
    pub keys_down: [bool; 512],
    pub input_characters: [ImWchar; 16 + 1],
    pub nav_inputs: [c_float; ImGuiNavInput::COUNT_INTERNAL],

    pub want_capture_mouse: bool,
    pub want_capture_keyboard: bool,
    pub want_text_input: bool,
    pub want_set_mouse_pos: bool,
    pub want_save_ini_settings: bool,
    pub nav_active: bool,
    pub nav_visible: bool,
    pub framerate: c_float,
    pub metrics_render_vertices: c_int,
    pub metrics_render_indices: c_int,
    pub metrics_render_windows: c_int,
    pub metrics_active_windows: c_int,
    pub metrics_active_allocations: c_int,
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
    nav_inputs_down_duration: [c_float; ImGuiNavInput::COUNT_INTERNAL],
    nav_inputs_down_duration_prev: [c_float; ImGuiNavInput::COUNT_INTERNAL],
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

/// Key->value storage
#[repr(C)]
pub struct ImGuiStorage {
    pub data: ImVector<Pair>,
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
    /// Alignment of button text when button is larger than text. Defaults to (0.5, 0.5) for
    /// horizontally+vertically centered.
    pub button_text_align: ImVec2,
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

// ImGuiIO
extern "C" {
    pub fn ImGuiIO_AddInputCharacter(this: *mut ImGuiIO, c: c_ushort);
    pub fn ImGuiIO_AddInputCharactersUTF8(this: *mut ImGuiIO, utf8_chars: *const c_char);
    pub fn ImGuiIO_ClearInputCharacters(this: *mut ImGuiIO);
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

// ImGuiInputTextCallbackData
extern "C" {
    pub fn ImGuiInputTextCallbackData_DeleteChars(
        this: *mut ImGuiInputTextCallbackData,
        pos: c_int,
        bytes_count: c_int,
    );
    pub fn ImGuiInputTextCallbackData_InsertChars(
        this: *mut ImGuiInputTextCallbackData,
        pos: c_int,
        text: *const c_char,
        text_end: *const c_char,
    );
    pub fn ImGuiInputTextCallbackData_HasSelection(this: *mut ImGuiInputTextCallbackData) -> bool;
}

// ImGuiPayload
extern "C" {
    pub fn ImGuiPayload_Clear(this: *mut ImGuiPayload);
    pub fn ImGuiPayload_IsDataType(this: *mut ImGuiPayload, type_: *const c_char) -> bool;
    pub fn ImGuiPayload_IsPreview(this: *mut ImGuiPayload) -> bool;
    pub fn ImGuiPayload_IsDelivery(this: *mut ImGuiPayload) -> bool;
}

// ImGuiListClipper
extern "C" {
    pub fn ImGuiListClipper_Step(this: *mut ImGuiListClipper) -> bool;
    pub fn ImGuiListClipper_Begin(
        this: *mut ImGuiListClipper,
        items_count: c_int,
        items_height: c_float,
    );
    pub fn ImGuiListClipper_End(this: *mut ImGuiListClipper);
}

// ImDrawList
extern "C" {
    pub fn ImDrawList_PushClipRect(
        this: *mut ImDrawList,
        clip_rect_min: ImVec2,
        clip_rect_max: ImVec2,
        intersect_with_current_clip_rect: bool,
    );
    pub fn ImDrawList_PushClipRectFullScreen(this: *mut ImDrawList);
    pub fn ImDrawList_PopClipRect(this: *mut ImDrawList);
    pub fn ImDrawList_PushTextureID(this: *mut ImDrawList, texture_id: ImTextureID);
    pub fn ImDrawList_PopTextureID(this: *mut ImDrawList);
    pub fn ImDrawList_GetClipRectMin(this: *mut ImDrawList) -> ImVec2;
    pub fn ImDrawList_GetClipRectMax(this: *mut ImDrawList) -> ImVec2;

    pub fn ImDrawList_AddLine(
        this: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        col: ImU32,
        thickness: c_float,
    );
    pub fn ImDrawList_AddRect(
        this: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        col: ImU32,
        rounding: c_float,
        rounding_corners_flags: ImDrawCornerFlags,
        thickness: c_float,
    );
    pub fn ImDrawList_AddRectFilled(
        this: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        col: ImU32,
        rounding: c_float,
        rounding_corners_flags: ImDrawCornerFlags,
    );
    pub fn ImDrawList_AddRectFilledMultiColor(
        this: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        col_upr_left: ImU32,
        col_upr_right: ImU32,
        col_bot_right: ImU32,
        col_bot_left: ImU32,
    );
    pub fn ImDrawList_AddQuad(
        this: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        d: ImVec2,
        col: ImU32,
        thickness: c_float,
    );
    pub fn ImDrawList_AddQuadFilled(
        this: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        d: ImVec2,
        col: ImU32,
    );
    pub fn ImDrawList_AddTriangle(
        this: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        col: ImU32,
        thickness: c_float,
    );
    pub fn ImDrawList_AddTriangleFilled(
        this: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        col: ImU32,
    );
    pub fn ImDrawList_AddCircle(
        this: *mut ImDrawList,
        centre: ImVec2,
        radius: c_float,
        col: ImU32,
        num_segments: c_int,
        thickness: c_float,
    );
    pub fn ImDrawList_AddCircleFilled(
        this: *mut ImDrawList,
        centre: ImVec2,
        radius: c_float,
        col: ImU32,
        num_segments: c_int,
    );
    pub fn ImDrawList_AddText(
        this: *mut ImDrawList,
        pos: ImVec2,
        col: ImU32,
        text_begin: *const c_char,
        text_end: *const c_char,
    );
    pub fn ImDrawList_AddTextFontPtr(
        this: *mut ImDrawList,
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
        this: *mut ImDrawList,
        user_texture_id: ImTextureID,
        a: ImVec2,
        b: ImVec2,
        uv_a: ImVec2,
        uv_b: ImVec2,
        col: ImU32,
    );
    pub fn ImDrawList_AddImageQuad(
        this: *mut ImDrawList,
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
        this: *mut ImDrawList,
        user_texture_id: ImTextureID,
        a: ImVec2,
        b: ImVec2,
        uv_a: ImVec2,
        uv_b: ImVec2,
        col: ImU32,
        rounding: c_float,
        rounding_corners: ImDrawCornerFlags,
    );
    pub fn ImDrawList_AddPolyLine(
        this: *mut ImDrawList,
        points: *const ImVec2,
        num_points: c_int,
        col: ImU32,
        closed: bool,
        thickness: c_float,
    );
    pub fn ImDrawList_AddConvexPolyFilled(
        this: *mut ImDrawList,
        points: *const ImVec2,
        num_points: c_int,
        col: ImU32,
    );
    pub fn ImDrawList_AddBezierCurve(
        this: *mut ImDrawList,
        pos0: ImVec2,
        cp0: ImVec2,
        cp1: ImVec2,
        pos1: ImVec2,
        col: ImU32,
        thickness: c_float,
        num_segments: c_int,
    );

    pub fn ImDrawList_PathClear(this: *mut ImDrawList);
    pub fn ImDrawList_PathLineTo(this: *mut ImDrawList, pos: ImVec2);
    pub fn ImDrawList_PathLineToMergeDuplicate(this: *mut ImDrawList, pos: ImVec2);
    pub fn ImDrawList_PathFillConvex(this: *mut ImDrawList, col: ImU32);
    pub fn ImDrawList_PathStroke(
        this: *mut ImDrawList,
        col: ImU32,
        closed: bool,
        thickness: c_float,
    );
    pub fn ImDrawList_PathArcTo(
        this: *mut ImDrawList,
        centre: ImVec2,
        radius: c_float,
        a_min: c_float,
        a_max: c_float,
        num_segments: c_int,
    );
    pub fn ImDrawList_PathArcToFast(
        this: *mut ImDrawList,
        centre: ImVec2,
        radius: c_float,
        a_min_of_12: c_int,
        a_max_of_12: c_int,
    );
    pub fn ImDrawList_PathBezierCurveTo(
        this: *mut ImDrawList,
        p1: ImVec2,
        p2: ImVec2,
        p3: ImVec2,
        num_segments: c_int,
    );
    pub fn ImDrawList_PathRect(
        this: *mut ImDrawList,
        rect_min: ImVec2,
        rect_max: ImVec2,
        rounding: c_float,
        rounding_corners_flags: c_int,
    );

    pub fn ImDrawList_ChannelsSplit(this: *mut ImDrawList, channels_count: c_int);
    pub fn ImDrawList_ChannelsMerge(this: *mut ImDrawList);
    pub fn ImDrawList_ChannelsSetCurrent(this: *mut ImDrawList, channel_index: c_int);

    pub fn ImDrawList_AddCallback(
        this: *mut ImDrawList,
        callback: ImDrawCallback,
        callback_data: *mut c_void,
    );
    pub fn ImDrawList_AddDrawCmd(this: *mut ImDrawList);

    pub fn ImDrawList_CloneOutput(this: *mut ImDrawList) -> *mut ImDrawList;
    pub fn ImDrawList_Clear(this: *mut ImDrawList);
    pub fn ImDrawList_ClearFreeMemory(this: *mut ImDrawList);

    pub fn ImDrawList_PrimReserve(this: *mut ImDrawList, idx_count: c_int, vtx_count: c_int);
    pub fn ImDrawList_PrimRect(this: *mut ImDrawList, a: ImVec2, b: ImVec2, col: ImU32);
    pub fn ImDrawList_PrimRectUV(
        this: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        uv_a: ImVec2,
        uv_b: ImVec2,
        col: ImU32,
    );
    pub fn ImDrawList_PrimQuadUV(
        this: *mut ImDrawList,
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
    pub fn ImDrawList_PrimWriteVtx(this: *mut ImDrawList, pos: ImVec2, uv: ImVec2, col: ImU32);
    pub fn ImDrawList_PrimWriteIdx(this: *mut ImDrawList, idx: ImDrawIdx);
    pub fn ImDrawList_PrimVtx(this: *mut ImDrawList, pos: ImVec2, uv: ImVec2, col: ImU32);
    pub fn ImDrawList_UpdateClipRect(this: *mut ImDrawList);
    pub fn ImDrawList_UpdateTextureID(this: *mut ImDrawList);
}

// ImDrawData
extern "C" {
    pub fn ImDrawData_Clear(this: *mut ImDrawData);
    pub fn ImDrawData_DeIndexAllBuffers(this: *mut ImDrawData);
    pub fn ImDrawData_ScaleClipRects(this: *mut ImDrawData, sc: ImVec2);
}

// ImFontConfig
extern "C" {
    pub fn ImFontConfig_DefaultConstructor(config: *mut ImFontConfig);
}

// ImFontAtlas
extern "C" {
    pub fn ImFontAtlas_AddFont(
        this: *mut ImFontAtlas,
        font_cfg: *const ImFontConfig,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontDefault(
        this: *mut ImFontAtlas,
        font_cfg: *const ImFontConfig,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontFromFileTTF(
        this: *mut ImFontAtlas,
        filename: *const c_char,
        size_pixels: c_float,
        font_cfg: *const ImFontConfig,
        glyph_ranges: *const ImWchar,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontFromMemoryTTF(
        this: *mut ImFontAtlas,
        font_data: *mut c_void,
        font_size: c_int,
        size_pixels: c_float,
        font_cfg: *const ImFontConfig,
        glyph_ranges: *const ImWchar,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontFromMemoryCompressedTTF(
        this: *mut ImFontAtlas,
        compressed_font_data: *const c_void,
        compressed_font_size: c_int,
        size_pixels: c_float,
        font_cfg: *const ImFontConfig,
        glyph_ranges: *const ImWchar,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontFromMemoryCompressedBase85TTF(
        this: *mut ImFontAtlas,
        compressed_font_data_base85: *const c_char,
        size_pixels: c_float,
        font_cfg: *const ImFontConfig,
        glyph_ranges: *const ImWchar,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_ClearInputData(this: *mut ImFontAtlas);
    pub fn ImFontAtlas_ClearTexData(this: *mut ImFontAtlas);
    pub fn ImFontAtlas_ClearFonts(this: *mut ImFontAtlas);
    pub fn ImFontAtlas_Clear(this: *mut ImFontAtlas);
    pub fn ImFontAtlas_Build(this: *mut ImFontAtlas) -> bool;
    pub fn ImFontAtlas_IsBuilt(this: *mut ImFontAtlas) -> bool;
    pub fn ImFontAtlas_GetTexDataAsAlpha8(
        this: *mut ImFontAtlas,
        out_pixels: *mut *mut c_uchar,
        out_width: *mut c_int,
        out_height: *mut c_int,
        out_bytes_per_pixel: *mut c_int,
    );
    pub fn ImFontAtlas_GetTexDataAsRGBA32(
        this: *mut ImFontAtlas,
        out_pixels: *mut *mut c_uchar,
        out_width: *mut c_int,
        out_height: *mut c_int,
        out_bytes_per_pixel: *mut c_int,
    );
    pub fn ImFontAtlas_SetTexID(this: *mut ImFontAtlas, id: ImTextureID);
    pub fn ImFontAtlas_GetGlyphRangesDefault(this: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesKorean(this: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesJapanese(this: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesChineseFull(this: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesChineseSimplifiedCommon(
        this: *mut ImFontAtlas,
    ) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesCyrillic(this: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesThai(this: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_AddCustomRectRegular(
        this: *mut ImFontAtlas,
        id: c_uint,
        width: c_int,
        height: c_int,
    ) -> c_int;
    pub fn ImFontAtlas_AddCustomRectFontGlyph(
        this: *mut ImFontAtlas,
        font: *mut ImFont,
        id: ImWchar,
        width: c_int,
        height: c_int,
        advance_x: c_float,
        offset: ImVec2,
    ) -> c_int;
    pub fn ImFontAtlas_GetCustomRectByIndex(
        this: *mut ImFontAtlas,
        index: c_int,
    ) -> *const CustomRect;
    pub fn ImFontAtlas_CalcCustomRectUV(
        this: *mut ImFontAtlas,
        rect: *const CustomRect,
        out_uv_min: *mut ImVec2,
        out_uv_max: *mut ImVec2,
    );
    pub fn ImFontAtlas_GetMouseCursorTexData(
        this: *mut ImFontAtlas,
        cursor: ImGuiMouseCursor,
        out_offset: *mut ImVec2,
        out_size: *mut ImVec2,
        out_uv_border: *mut ImVec2,
        out_uv_fill: *mut ImVec2,
    ) -> bool;
}

// GlyphRangesBuilder
extern "C" {
    pub fn GlyphRangesBuilder_GetBit(this: *mut GlyphRangesBuilder, n: c_int) -> bool;
    pub fn GlyphRangesBuilder_SetBit(this: *mut GlyphRangesBuilder, n: c_int);
    pub fn GlyphRangesBuilder_AddChar(this: *mut GlyphRangesBuilder, c: ImWchar);
    pub fn GlyphRangesBuilder_AddText(
        this: *mut GlyphRangesBuilder,
        text: *const c_char,
        text_end: *const c_char,
    );
    pub fn GlyphRangesBuilder_AddRanges(this: *mut GlyphRangesBuilder, ranges: *const ImWchar);
    pub fn GlyphRangesBuilder_BuildRanges(
        this: *mut GlyphRangesBuilder,
        out_ranges: *mut ImVector<ImWchar>,
    );
}

// CustomRect
extern "C" {
    pub fn CustomRect_IsPacked(this: *mut CustomRect) -> bool;
}

// ImFont
extern "C" {
    pub fn ImFont_ClearOutputData(this: *mut ImFont);
    pub fn ImFont_BuildLookupTable(this: *mut ImFont);
    pub fn ImFont_FindGlyph(this: *mut ImFont, c: ImWchar) -> *const ImFontGlyph;
    pub fn ImFont_FindGlyphNoFallback(this: *mut ImFont, c: ImWchar) -> *const ImFontGlyph;
    pub fn ImFont_SetFallbackChar(this: *mut ImFont, c: ImWchar);
    pub fn ImFont_GetCharAdvance(this: *mut ImFont, c: ImWchar) -> c_float;
    pub fn ImFont_IsLoaded(this: *mut ImFont) -> bool;
    pub fn ImFont_GetDebugName(this: *mut ImFont) -> *const c_char;
    pub fn ImFont_CalcTextSizeA(
        this: *mut ImFont,
        size: c_float,
        max_width: c_float,
        wrap_width: c_float,
        text_begin: *const c_char,
        text_end: *const c_char,
        remaining: *mut *const c_char,
    ) -> ImVec2;
    pub fn ImFont_CalcWordWrapPositionA(
        this: *mut ImFont,
        scale: c_float,
        text: *const c_char,
        text_end: *const c_char,
        wrap_width: c_float,
    ) -> *const c_char;
    pub fn ImFont_RenderChar(
        this: *mut ImFont,
        draw_list: *mut ImDrawList,
        size: c_float,
        pos: ImVec2,
        col: ImU32,
        c: c_ushort,
    );
    pub fn ImFont_RenderText(
        this: *mut ImFont,
        draw_list: *mut ImDrawList,
        size: c_float,
        pos: ImVec2,
        col: ImU32,
        clip_rect: ImVec4,
        text_begin: *const c_char,
        text_end: *const c_char,
        wrap_width: c_float,
        cpu_fine_clip: bool,
    );
    pub fn ImFont_GrowIndex(this: *mut ImFont, new_size: c_int);
    pub fn ImFont_AddGlyph(
        this: *mut ImFont,
        c: ImWchar,
        x0: c_float,
        y0: c_float,
        x1: c_float,
        y1: c_float,
        u0: c_float,
        v0: c_float,
        u1: c_float,
        v1: c_float,
        advance_x: c_float,
    );
    pub fn ImFont_AddRemapChar(this: *mut ImFont, dst: ImWchar, src: ImWchar, overwrite_dst: bool);
}
