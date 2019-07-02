use std::mem;
use std::os::raw::c_char;
use std::slice;

use crate::internal::{closure_callback_marker, fn_callback_marker, RawCast, RawWrapper};
use crate::render::renderer::TextureId;
use crate::sys;
use crate::Ui;

/// Wrap `ImU32` (a type typically used by ImGui to store packed colors)
/// This type is used to represent the color of drawing primitives in ImGui's
/// custom drawing API.
///
/// The type implements `From<ImU32>`, `From<ImVec4>`, `From<[f32; 4]>`,
/// `From<[f32; 3]>`, `From<(f32, f32, f32, f32)>` and `From<(f32, f32, f32)>`
/// for convenience. If alpha is not provided, it is assumed to be 1.0 (255).
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ImColor(sys::ImU32);

impl From<ImColor> for sys::ImU32 {
    fn from(color: ImColor) -> Self {
        color.0
    }
}

impl From<sys::ImU32> for ImColor {
    fn from(color: sys::ImU32) -> Self {
        ImColor(color)
    }
}

impl From<[f32; 4]> for ImColor {
    fn from(v: [f32; 4]) -> Self {
        ImColor(unsafe { sys::igColorConvertFloat4ToU32(v.into()) })
    }
}

impl From<(f32, f32, f32, f32)> for ImColor {
    fn from(v: (f32, f32, f32, f32)) -> Self {
        ImColor(unsafe { sys::igColorConvertFloat4ToU32(v.into()) })
    }
}

impl From<[f32; 3]> for ImColor {
    fn from(v: [f32; 3]) -> Self {
        [v[0], v[1], v[2], 1.0].into()
    }
}

impl From<(f32, f32, f32)> for ImColor {
    fn from(v: (f32, f32, f32)) -> Self {
        [v.0, v.1, v.2, 1.0].into()
    }
}

/// All draw data required to render a frame.
#[repr(C)]
pub struct DrawData {
    valid: bool,
    cmd_lists: *mut *mut DrawList,
    cmd_lists_count: i32,
    /// For convenience, sum of all draw list index buffer sizes
    pub total_idx_count: i32,
    /// For convenience, sum of all draw list vertex buffer sizes
    pub total_vtx_count: i32,
    /// Upper-left position of the viewport to render.
    ///
    /// (= upper-left corner of the orthogonal projection matrix to use)
    pub display_pos: [f32; 2],
    /// Size of the viewport to render.
    ///
    /// (= display_pos + display_size == lower-right corner of the orthogonal matrix to use)
    pub display_size: [f32; 2],
    /// Amount of pixels for each unit of display_size.
    ///
    /// Based on io.display_frame_buffer_scale. Typically [1.0, 1.0] on normal displays, and
    /// [2.0, 2.0] on Retina displays, but fractional values are also possible.
    pub framebuffer_scale: [f32; 2],
}

unsafe impl RawCast<sys::ImDrawData> for DrawData {}

impl DrawData {
    /// Returns an iterator over the draw lists included in the draw data
    pub fn draw_lists(&self) -> DrawListIterator {
        unsafe {
            DrawListIterator {
                iter: self.cmd_lists().iter(),
            }
        }
    }
    pub(crate) unsafe fn cmd_lists(&self) -> &[*const DrawList] {
        slice::from_raw_parts(
            self.cmd_lists as *const *const DrawList,
            self.cmd_lists_count as usize,
        )
    }
    /// Converts all buffers from indexed to non-indexed, in case you cannot render indexed
    /// buffers.
    ///
    /// **This is slow and most likely a waste of resources. Always prefer indexed rendering!**
    pub fn deindex_all_buffers(&mut self) {
        unsafe {
            sys::ImDrawData_DeIndexAllBuffers(self.raw_mut());
        }
    }
    /// Scales the clip rect of each draw command.
    ///
    /// Can be used if your final output buffer is at a different scale than imgui-rs expects, or
    /// if there is a difference between your window resolution and framebuffer resolution.
    pub fn scale_clip_rects(&mut self, fb_scale: [f32; 2]) {
        unsafe {
            sys::ImDrawData_ScaleClipRects(self.raw_mut(), fb_scale.into());
        }
    }
}

/// Iterator over draw lists
pub struct DrawListIterator<'a> {
    iter: std::slice::Iter<'a, *const DrawList>,
}

impl<'a> Iterator for DrawListIterator<'a> {
    type Item = &'a DrawList;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|&ptr| unsafe { &*ptr })
    }
}

#[test]
fn test_drawdata_memory_layout() {
    use std::mem;
    assert_eq!(
        mem::size_of::<DrawData>(),
        mem::size_of::<sys::ImDrawData>()
    );
    assert_eq!(
        mem::align_of::<DrawData>(),
        mem::align_of::<sys::ImDrawData>()
    );
    use memoffset::offset_of;
    macro_rules! assert_field_offset {
        ($l:ident, $r:ident) => {
            assert_eq!(offset_of!(DrawData, $l), offset_of!(sys::ImDrawData, $r));
        };
    };
    assert_field_offset!(valid, Valid);
    assert_field_offset!(cmd_lists, CmdLists);
    assert_field_offset!(cmd_lists_count, CmdListsCount);
    assert_field_offset!(total_idx_count, TotalIdxCount);
    assert_field_offset!(total_vtx_count, TotalVtxCount);
    assert_field_offset!(display_pos, DisplayPos);
    assert_field_offset!(display_size, DisplaySize);
    assert_field_offset!(framebuffer_scale, FramebufferScale);
}

/// Draw command list
#[repr(transparent)]
pub struct DrawList(sys::ImDrawList);

impl RawWrapper for DrawList {
    type Raw = sys::ImDrawList;
    unsafe fn raw(&self) -> &sys::ImDrawList {
        &self.0
    }
    unsafe fn raw_mut(&mut self) -> &mut sys::ImDrawList {
        &mut self.0
    }
}

impl DrawList {
    pub(crate) unsafe fn cmd_buffer(&self) -> &[sys::ImDrawCmd] {
        slice::from_raw_parts(
            self.0.CmdBuffer.Data as *const sys::ImDrawCmd,
            self.0.CmdBuffer.Size as usize,
        )
    }
    pub fn idx_buffer(&self) -> &[DrawIdx] {
        unsafe {
            slice::from_raw_parts(
                self.0.IdxBuffer.Data as *const DrawIdx,
                self.0.IdxBuffer.Size as usize,
            )
        }
    }
    pub fn vtx_buffer(&self) -> &[DrawVert] {
        unsafe {
            slice::from_raw_parts(
                self.0.VtxBuffer.Data as *const DrawVert,
                self.0.VtxBuffer.Size as usize,
            )
        }
    }
    pub fn commands(&self) -> DrawCmdIterator {
        unsafe {
            DrawCmdIterator {
                iter: self.cmd_buffer().iter(),
            }
        }
    }
}

/// Wrap `ptr` with a temporary lifetime. It's likely that `ptr` will live longer than `'a`, but the caller must ensure the returned value is only used when valid.
unsafe fn wrap_draw_list<'a>(ptr: *mut sys::ImDrawList) -> &'a mut DrawList {
    &mut *(ptr as *mut DrawList)
}

/// Functions for getting a `DrawList`.
impl<'ui> Ui<'ui> {
    pub fn get_cursor_screen_pos(&self) -> sys::ImVec2 {
        unsafe { sys::igGetCursorScreenPos_nonUDT2().into() }
    }

    pub fn get_window_draw_list<'a>(&self) -> &'a mut DrawList {
        unsafe { wrap_draw_list(sys::igGetWindowDrawList()) }
    }

    pub fn get_background_draw_list<'a>(&self) -> &'a mut DrawList {
        unsafe { wrap_draw_list(sys::igGetBackgroundDrawList()) }
    }

    pub fn get_foreground_draw_list<'a>(&self) -> &'a mut DrawList {
        unsafe { wrap_draw_list(sys::igGetForegroundDrawList()) }
    }
}

/// Functions for drawing into a `DrawList`.
impl DrawList {
    pub fn push_clip_rect(
        &mut self,
        clip_rect_min: sys::ImVec2,
        clip_rect_max: sys::ImVec2,
        intersect_with_current_clip_rect: bool,
    ) {
        unsafe {
            sys::ImDrawList_PushClipRect(
                self.raw_mut(),
                clip_rect_min,
                clip_rect_max,
                intersect_with_current_clip_rect,
            );
        }
    }

    pub fn push_clip_rect_full_screen(&mut self) {
        unsafe {
            sys::ImDrawList_PushClipRectFullScreen(self.raw_mut());
        }
    }

    pub fn pop_clip_rect(&mut self) {
        unsafe {
            sys::ImDrawList_PopClipRect(self.raw_mut());
        }
    }

    pub fn push_texture_id(&mut self, texture_id: TextureId) {
        unsafe {
            sys::ImDrawList_PushTextureID(self.raw_mut(), texture_id.id() as sys::ImTextureID);
        }
    }

    pub fn pop_texture_id(&mut self) {
        unsafe {
            sys::ImDrawList_PopTextureID(self.raw_mut());
        }
    }

    pub fn get_clip_rect_min(&mut self) -> sys::ImVec2 {
        unsafe { sys::ImDrawList_GetClipRectMin_nonUDT2(self.raw_mut()).into() }
    }

    pub fn get_clip_rect_max(&mut self) -> sys::ImVec2 {
        unsafe { sys::ImDrawList_GetClipRectMax_nonUDT2(self.raw_mut()).into() }
    }

    pub fn add_line(&mut self, a: sys::ImVec2, b: sys::ImVec2, col: ImColor, thickness: f32) {
        unsafe {
            sys::ImDrawList_AddLine(self.raw_mut(), a, b, col.into(), thickness);
        }
    }

    pub fn add_rect(
        &mut self,
        a: sys::ImVec2,
        b: sys::ImVec2,
        col: ImColor,
        rounding: f32,
        rounding_corners_flags: sys::ImDrawCornerFlags_,
        thickness: f32,
    ) {
        unsafe {
            sys::ImDrawList_AddRect(
                self.raw_mut(),
                a,
                b,
                col.into(),
                rounding,
                rounding_corners_flags as i32,
                thickness,
            );
        }
    }

    pub fn add_rect_filled(
        &mut self,
        a: sys::ImVec2,
        b: sys::ImVec2,
        col: ImColor,
        rounding: f32,
        rounding_corners_flags: sys::ImDrawCornerFlags_,
    ) {
        unsafe {
            sys::ImDrawList_AddRectFilled(
                self.raw_mut(),
                a,
                b,
                col.into(),
                rounding,
                rounding_corners_flags as i32,
            );
        }
    }

    pub fn add_rect_filled_multi_color(
        &mut self,
        a: sys::ImVec2,
        b: sys::ImVec2,
        col_upr_left: ImColor,
        col_upr_right: ImColor,
        col_bot_right: ImColor,
        col_bot_left: ImColor,
    ) {
        unsafe {
            sys::ImDrawList_AddRectFilledMultiColor(
                self.raw_mut(),
                a,
                b,
                col_upr_left.into(),
                col_upr_right.into(),
                col_bot_right.into(),
                col_bot_left.into(),
            );
        }
    }

    pub fn add_quad(
        &mut self,
        a: sys::ImVec2,
        b: sys::ImVec2,
        c: sys::ImVec2,
        d: sys::ImVec2,
        col: ImColor,
        thickness: f32,
    ) {
        unsafe {
            sys::ImDrawList_AddQuad(self.raw_mut(), a, b, c, d, col.into(), thickness);
        }
    }

    pub fn add_quad_filled(
        &mut self,
        a: sys::ImVec2,
        b: sys::ImVec2,
        c: sys::ImVec2,
        d: sys::ImVec2,
        col: ImColor,
    ) {
        unsafe {
            sys::ImDrawList_AddQuadFilled(self.raw_mut(), a, b, c, d, col.into());
        }
    }

    pub fn add_triangle(
        &mut self,
        a: sys::ImVec2,
        b: sys::ImVec2,
        c: sys::ImVec2,
        col: ImColor,
        thickness: f32,
    ) {
        unsafe {
            sys::ImDrawList_AddTriangle(self.raw_mut(), a, b, c, col.into(), thickness);
        }
    }

    pub fn add_triangle_filled(
        &mut self,
        a: sys::ImVec2,
        b: sys::ImVec2,
        c: sys::ImVec2,
        col: ImColor,
    ) {
        unsafe {
            sys::ImDrawList_AddTriangleFilled(self.raw_mut(), a, b, c, col.into());
        }
    }

    pub fn add_circle(
        &mut self,
        centre: sys::ImVec2,
        radius: f32,
        col: ImColor,
        num_segments: i32,
        thickness: f32,
    ) {
        unsafe {
            sys::ImDrawList_AddCircle(
                self.raw_mut(),
                centre,
                radius,
                col.into(),
                num_segments,
                thickness,
            );
        }
    }

    pub fn add_circle_filled(
        &mut self,
        centre: sys::ImVec2,
        radius: f32,
        col: ImColor,
        num_segments: i32,
    ) {
        unsafe {
            sys::ImDrawList_AddCircleFilled(
                self.raw_mut(),
                centre,
                radius,
                col.into(),
                num_segments,
            );
        }
    }

    pub fn add_text<T: AsRef<str>>(&mut self, pos: sys::ImVec2, col: ImColor, text: T) {
        let s = text.as_ref();
        unsafe {
            let start = s.as_ptr();
            let end = start.add(s.len());
            sys::ImDrawList_AddText(
                self.raw_mut(),
                pos,
                col.into(),
                start as *const c_char,
                end as *const c_char,
            );
        }
    }

    pub fn add_text_font_ptr<T: AsRef<str>>(
        &mut self,
        font: *mut sys::ImFont,
        font_size: f32,
        pos: sys::ImVec2,
        col: ImColor,
        text: T,
        wrap_width: f32,
        cpu_fine_clip_rect: &[f32; 4],
    ) {
        let s = text.as_ref();
        unsafe {
            let start = s.as_ptr();
            let end = start.add(s.len());
            sys::ImDrawList_AddTextFontPtr(
                self.raw_mut(),
                font,
                font_size,
                pos,
                col.into(),
                start as *const c_char,
                end as *const c_char,
                wrap_width,
                cpu_fine_clip_rect.as_ptr() as *const sys::ImVec4,
            );
        }
    }

    pub fn add_image(
        &mut self,
        user_texture_id: TextureId,
        a: sys::ImVec2,
        b: sys::ImVec2,
        uv_a: sys::ImVec2,
        uv_b: sys::ImVec2,
        col: ImColor,
    ) {
        unsafe {
            sys::ImDrawList_AddImage(
                self.raw_mut(),
                user_texture_id.id() as sys::ImTextureID,
                a,
                b,
                uv_a,
                uv_b,
                col.into(),
            );
        }
    }

    pub fn add_image_quad(
        &mut self,
        user_texture_id: TextureId,
        a: sys::ImVec2,
        b: sys::ImVec2,
        c: sys::ImVec2,
        d: sys::ImVec2,
        uv_a: sys::ImVec2,
        uv_b: sys::ImVec2,
        uv_c: sys::ImVec2,
        uv_d: sys::ImVec2,
        col: ImColor,
    ) {
        unsafe {
            sys::ImDrawList_AddImageQuad(
                self.raw_mut(),
                user_texture_id.id() as sys::ImTextureID,
                a,
                b,
                c,
                d,
                uv_a,
                uv_b,
                uv_c,
                uv_d,
                col.into(),
            );
        }
    }

    pub fn add_image_rounded(
        &mut self,
        user_texture_id: TextureId,
        a: sys::ImVec2,
        b: sys::ImVec2,
        uv_a: sys::ImVec2,
        uv_b: sys::ImVec2,
        col: ImColor,
        rounding: f32,
        rounding_corners: i32,
    ) {
        unsafe {
            sys::ImDrawList_AddImageRounded(
                self.raw_mut(),
                user_texture_id.id() as sys::ImTextureID,
                a,
                b,
                uv_a,
                uv_b,
                col.into(),
                rounding,
                rounding_corners,
            );
        }
    }

    pub fn add_polyline(
        &mut self,
        points: &[sys::ImVec2],
        col: ImColor,
        closed: bool,
        thickness: f32,
    ) {
        unsafe {
            sys::ImDrawList_AddPolyline(
                self.raw_mut(),
                points.as_ptr() as *const sys::ImVec2,
                points.len() as i32,
                col.into(),
                closed,
                thickness,
            );
        }
    }

    pub fn add_convex_poly_filled(&mut self, points: &[sys::ImVec2], col: ImColor) {
        unsafe {
            sys::ImDrawList_AddConvexPolyFilled(
                self.raw_mut(),
                points.as_ptr() as *const sys::ImVec2,
                points.len() as i32,
                col.into(),
            );
        }
    }

    pub fn add_bezier_curve(
        &mut self,
        pos0: sys::ImVec2,
        cp0: sys::ImVec2,
        cp1: sys::ImVec2,
        pos1: sys::ImVec2,
        col: ImColor,
        thickness: f32,
        num_segments: i32,
    ) {
        unsafe {
            sys::ImDrawList_AddBezierCurve(
                self.raw_mut(),
                pos0,
                cp0,
                cp1,
                pos1,
                col.into(),
                thickness,
                num_segments,
            );
        }
    }

    pub fn path_clear(&mut self) {
        unsafe {
            sys::ImDrawList_PathClear(self.raw_mut());
        }
    }

    pub fn path_line_to(&mut self, pos: sys::ImVec2) {
        unsafe {
            sys::ImDrawList_PathLineTo(self.raw_mut(), pos);
        }
    }

    pub fn path_line_to_merge_duplicate(&mut self, pos: sys::ImVec2) {
        unsafe {
            sys::ImDrawList_PathLineToMergeDuplicate(self.raw_mut(), pos);
        }
    }

    pub fn path_fill_convex(&mut self, col: ImColor) {
        unsafe {
            sys::ImDrawList_PathFillConvex(self.raw_mut(), col.into());
        }
    }

    pub fn path_stroke(&mut self, col: ImColor, closed: bool, thickness: f32) {
        unsafe {
            sys::ImDrawList_PathStroke(self.raw_mut(), col.into(), closed, thickness);
        }
    }

    pub fn path_arc_to(
        &mut self,
        centre: sys::ImVec2,
        radius: f32,
        a_min: f32,
        a_max: f32,
        num_segments: i32,
    ) {
        unsafe {
            sys::ImDrawList_PathArcTo(self.raw_mut(), centre, radius, a_min, a_max, num_segments);
        }
    }

    pub fn path_arc_to_fast(
        &mut self,
        centre: sys::ImVec2,
        radius: f32,
        a_min_of_12: i32,
        a_max_of_12: i32,
    ) {
        unsafe {
            sys::ImDrawList_PathArcToFast(self.raw_mut(), centre, radius, a_min_of_12, a_max_of_12);
        }
    }

    pub fn path_bezier_curve_to(
        &mut self,
        p1: sys::ImVec2,
        p2: sys::ImVec2,
        p3: sys::ImVec2,
        num_segments: i32,
    ) {
        unsafe {
            sys::ImDrawList_PathBezierCurveTo(self.raw_mut(), p1, p2, p3, num_segments);
        }
    }

    pub fn path_rect(
        &mut self,
        rect_min: sys::ImVec2,
        rect_max: sys::ImVec2,
        rounding: f32,
        rounding_corners_flags: sys::ImDrawCornerFlags_,
    ) {
        unsafe {
            sys::ImDrawList_PathRect(
                self.raw_mut(),
                rect_min,
                rect_max,
                rounding,
                rounding_corners_flags as i32,
            );
        }
    }

    pub fn channels_split(&mut self, channels_count: i32) {
        unsafe {
            sys::ImDrawList_ChannelsSplit(self.raw_mut(), channels_count);
        }
    }

    pub fn channels_merge(&mut self) {
        unsafe {
            sys::ImDrawList_ChannelsMerge(self.raw_mut());
        }
    }

    pub fn channels_set_current(&mut self, channel_index: i32) {
        unsafe {
            sys::ImDrawList_ChannelsSetCurrent(self.raw_mut(), channel_index);
        }
    }

    // The remaining functions are advanced or internal helpers
    // CIMGUI_API void ImDrawList_AddCallback(ImDrawList* self,ImDrawCallback callback,void* callback_data);
    // CIMGUI_API void ImDrawList_AddDrawCmd(ImDrawList* self);
    // CIMGUI_API ImDrawList* ImDrawList_CloneOutput(ImDrawList* self);
    // CIMGUI_API void ImDrawList_Clear(ImDrawList* self);
    // CIMGUI_API void ImDrawList_ClearFreeMemory(ImDrawList* self);
    // CIMGUI_API void ImDrawList_PrimReserve(ImDrawList* self,int idx_count,int vtx_count);
    // CIMGUI_API void ImDrawList_PrimRect(ImDrawList* self,const ImVec2 a,const ImVec2 b,ImU32 col);
    // CIMGUI_API void ImDrawList_PrimRectUV(ImDrawList* self,const ImVec2 a,const ImVec2 b,const ImVec2 uv_a,const ImVec2 uv_b,ImU32 col);
    // CIMGUI_API void ImDrawList_PrimQuadUV(ImDrawList* self,const ImVec2 a,const ImVec2 b,const ImVec2 c,const ImVec2 d,const ImVec2 uv_a,const ImVec2 uv_b,const ImVec2 uv_c,const ImVec2 uv_d,ImU32 col);
    // CIMGUI_API void ImDrawList_PrimWriteVtx(ImDrawList* self,const ImVec2 pos,const ImVec2 uv,ImU32 col);
    // CIMGUI_API void ImDrawList_PrimWriteIdx(ImDrawList* self,ImDrawIdx idx);
    // CIMGUI_API void ImDrawList_PrimVtx(ImDrawList* self,const ImVec2 pos,const ImVec2 uv,ImU32 col);
    // CIMGUI_API void ImDrawList_UpdateClipRect(ImDrawList* self);
    // CIMGUI_API void ImDrawList_UpdateTextureID(ImDrawList* self);
}

pub struct DrawCmdIterator<'a> {
    iter: std::slice::Iter<'a, sys::ImDrawCmd>,
}

impl<'a> Iterator for DrawCmdIterator<'a> {
    type Item = DrawCmd;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|cmd| {
            let cmd_params = DrawCmdParams {
                clip_rect: cmd.ClipRect.into(),
                texture_id: TextureId::from(cmd.TextureId),
            };
            if let Some(raw_callback) = cmd.UserCallback {
                if raw_callback == fn_callback_marker {
                    assert!(!cmd.UserCallbackData.is_null());
                    let callback: FnCallback = unsafe { mem::transmute(cmd.UserCallbackData) };
                    DrawCmd::FnCallback {
                        callback,
                        cmd_params,
                    }
                } else if raw_callback == closure_callback_marker {
                    assert!(!cmd.UserCallbackData.is_null());
                    let callback =
                        unsafe { Box::from_raw(cmd.UserCallbackData as *mut ClosureCallback) };
                    DrawCmd::ClosureCallback {
                        callback,
                        cmd_params,
                    }
                } else {
                    DrawCmd::RawCallback {
                        callback: raw_callback,
                        raw_cmd: cmd,
                    }
                }
            } else {
                DrawCmd::Elements {
                    count: cmd.ElemCount as usize,
                    cmd_params,
                }
            }
        })
    }
}

pub type FnCallback = fn(&DrawList, &DrawCmdParams);
pub type ClosureCallback = Box<dyn FnMut(&DrawList, &DrawCmdParams)>;

pub type DrawIdx = sys::ImDrawIdx;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DrawCmdParams {
    pub clip_rect: [f32; 4],
    pub texture_id: TextureId,
}

pub enum DrawCmd {
    Elements {
        count: usize,
        cmd_params: DrawCmdParams,
    },
    FnCallback {
        callback: FnCallback,
        cmd_params: DrawCmdParams,
    },
    ClosureCallback {
        callback: Box<ClosureCallback>,
        cmd_params: DrawCmdParams,
    },
    RawCallback {
        callback: unsafe extern "C" fn(*const sys::ImDrawList, cmd: *const sys::ImDrawCmd),
        raw_cmd: *const sys::ImDrawCmd,
    },
}

/// A single vertex
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DrawVert {
    pub pos: [f32; 2],
    pub uv: [f32; 2],
    pub col: [u8; 4],
}

#[cfg(feature = "glium")]
use glium::implement_vertex;
#[cfg(feature = "glium")]
implement_vertex!(DrawVert, pos, uv, col);

#[cfg(feature = "gfx")]
unsafe impl gfx::traits::Pod for DrawVert {}
#[cfg(feature = "gfx")]
impl gfx::pso::buffer::Structure<gfx::format::Format> for DrawVert {
    fn query(name: &str) -> Option<gfx::pso::buffer::Element<gfx::format::Format>> {
        // array query hack from gfx_impl_struct_meta macro
        use gfx::format::Formatted;
        use gfx::pso::buffer::{ElemOffset, Element};
        use std::mem::{size_of, transmute};
        // using "1" here as a simple non-zero pointer addres
        let tmp: &DrawVert = unsafe { transmute(1usize) };
        let base = tmp as *const _ as usize;
        //HACK: special treatment of array queries
        let (sub_name, big_offset) = {
            let mut split = name.split(|c| c == '[' || c == ']');
            let _ = split.next().unwrap();
            match split.next() {
                Some(s) => {
                    let array_id: ElemOffset = s.parse().unwrap();
                    let sub_name = match split.next() {
                        Some(s) if s.starts_with('.') => &s[1..],
                        _ => name,
                    };
                    (sub_name, array_id * (size_of::<DrawVert>() as ElemOffset))
                }
                None => (name, 0),
            }
        };
        match sub_name {
            "pos" => Some(Element {
                format: <[f32; 2] as Formatted>::get_format(),
                offset: ((&tmp.pos as *const _ as usize) - base) as ElemOffset + big_offset,
            }),
            "uv" => Some(Element {
                format: <[f32; 2] as Formatted>::get_format(),
                offset: ((&tmp.uv as *const _ as usize) - base) as ElemOffset + big_offset,
            }),
            "col" => Some(Element {
                format: <[gfx::format::U8Norm; 4] as Formatted>::get_format(),
                offset: ((&tmp.col as *const _ as usize) - base) as ElemOffset + big_offset,
            }),
            _ => None,
        }
    }
}

#[test]
fn test_drawvert_memory_layout() {
    use std::mem;
    assert_eq!(
        mem::size_of::<DrawVert>(),
        mem::size_of::<sys::ImDrawVert>()
    );
    assert_eq!(
        mem::align_of::<DrawVert>(),
        mem::align_of::<sys::ImDrawVert>()
    );
    use memoffset::offset_of;
    macro_rules! assert_field_offset {
        ($l:ident, $r:ident) => {
            assert_eq!(offset_of!(DrawVert, $l), offset_of!(sys::ImDrawVert, $r));
        };
    };
    assert_field_offset!(pos, pos);
    assert_field_offset!(uv, uv);
    assert_field_offset!(col, col);
}
