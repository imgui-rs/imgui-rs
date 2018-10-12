use std::os::raw::{c_char, c_float, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};
use std::slice;

use super::enums::{ImGuiCol, ImGuiKey, ImGuiNavInput};
use super::flags::{
    ImDrawListFlags, ImFontAtlasFlags, ImGuiBackendFlags, ImGuiConfigFlags, ImGuiInputTextFlags,
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
    dirty_lookup_tables: bool,
    metrics_total_surface: c_int,
}

/// Runtime data for multiple fonts, bake multiple fonts into a single texture, TTF/OTF font loader
#[repr(C)]
pub struct ImFontAtlas {
    pub locked: bool,
    pub flags: ImFontAtlasFlags,
    pub tex_id: ImTextureID,
    pub tex_desired_width: c_int,
    pub tex_glyph_padding: c_int,

    tex_pixels_alpha8: *mut c_uchar,
    tex_pixels_rgba32: *mut c_uint,
    tex_width: c_int,
    tex_height: c_int,
    tex_uv_scale: ImVec2,
    tex_uv_white_pixel: ImVec2,
    fonts: ImVector<*mut ImFont>,
    custom_rects: ImVector<CustomRect>,
    config_data: ImVector<ImFontConfig>,
    custom_rect_ids: [c_int; 1],
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
    pub nav_inputs: [c_float; ImGuiNavInput::COUNT],

    pub want_capture_mouse: bool,
    pub want_capture_keyboard: bool,
    pub want_text_input: bool,
    pub want_move_mouse: bool,
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
    nav_inputs_down_duration: [c_float; ImGuiNavInput::COUNT],
    nav_inputs_down_duration_prev: [c_float; ImGuiNavInput::COUNT],
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
