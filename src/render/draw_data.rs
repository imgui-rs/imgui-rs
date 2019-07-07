use std::slice;

use crate::internal::{RawCast, RawWrapper};
use crate::render::renderer::TextureId;
use crate::sys;

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
    use sys::ImDrawData;
    macro_rules! assert_field_offset {
        ($l:ident, $r:ident) => {
            assert_eq!(offset_of!(DrawData, $l), offset_of!(ImDrawData, $r));
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
                vtx_offset: cmd.VtxOffset as usize,
                idx_offset: cmd.IdxOffset as usize,
            };
            match cmd.UserCallback {
                Some(raw_callback) if raw_callback as isize == -1 => DrawCmd::ResetRenderState,
                Some(raw_callback) => DrawCmd::RawCallback {
                    callback: raw_callback,
                    raw_cmd: cmd,
                },
                None => DrawCmd::Elements {
                    count: cmd.ElemCount as usize,
                    cmd_params,
                },
            }
        })
    }
}

pub type DrawIdx = sys::ImDrawIdx;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DrawCmdParams {
    pub clip_rect: [f32; 4],
    pub texture_id: TextureId,
    pub vtx_offset: usize,
    pub idx_offset: usize,
}

pub enum DrawCmd {
    Elements {
        count: usize,
        cmd_params: DrawCmdParams,
    },
    ResetRenderState,
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
    use sys::ImDrawVert;
    macro_rules! assert_field_offset {
        ($l:ident, $r:ident) => {
            assert_eq!(offset_of!(DrawVert, $l), offset_of!(ImDrawVert, $r));
        };
    };
    assert_field_offset!(pos, pos);
    assert_field_offset!(uv, uv);
    assert_field_offset!(col, col);
}
