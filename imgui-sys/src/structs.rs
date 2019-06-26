use std::os::raw::{c_char, c_float, c_int, c_uint, c_void};
use std::slice;

use crate::enums::{ImGuiCol, ImGuiDir, ImGuiKey, ImGuiMouseCursor, ImGuiNavInput};
use crate::flags::{ImDrawCornerFlags, ImDrawListFlags};
use crate::{
    ImDrawCallback, ImDrawIdx, ImDrawListSharedData, ImDrawListSplitter, ImDrawVert, ImFont,
    ImGuiID, ImGuiStorage, ImTextureID, ImU32, ImVec2, ImVec4,
};

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
    pub vtx_offset: c_uint,
    pub idx_offset: c_uint,
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
    pub framebuffer_scale: ImVec2,
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
    vtx_current_offset: c_uint,
    vtx_current_idx: c_uint,
    vtx_write_ptr: *mut ImDrawVert,
    idx_write_ptr: *mut ImDrawIdx,
    clip_rect_stack: ImVector<ImVec4>,
    texture_id_stack: ImVector<ImTextureID>,
    path: ImVector<ImVec2>,
    splitter: ImDrawListSplitter,
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
    pub fn ImDrawList_GetClipRectMin_nonUDT2(this: *mut ImDrawList) -> ImVec2;
    pub fn ImDrawList_GetClipRectMax_nonUDT2(this: *mut ImDrawList) -> ImVec2;

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
