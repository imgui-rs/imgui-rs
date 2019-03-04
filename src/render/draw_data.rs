use std::slice;

use crate::render::renderer::TextureId;
use crate::sys;

#[repr(C)]
pub struct DrawData {
    pub valid: bool,
    cmd_lists: *mut *mut DrawList,
    cmd_lists_count: i32,
    pub total_idx_count: i32,
    pub total_vtx_count: i32,
    pub display_pos: [f32; 2],
    pub display_size: [f32; 2],
    pub framebuffer_scale: [f32; 2],
}

impl DrawData {
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
    pub fn deindex_all_buffers(&mut self) {
        unsafe {
            sys::ImDrawData_DeIndexAllBuffers(self.raw_mut());
        }
    }
    pub fn scale_clip_rects(&mut self, fb_scale: [f32; 2]) {
        unsafe {
            sys::ImDrawData_ScaleClipRects(self.raw_mut(), fb_scale.into());
        }
    }
}

impl DrawData {
    pub unsafe fn from_raw(raw: &sys::ImDrawData) -> &Self {
        &*(raw as *const _ as *const DrawData)
    }
    pub unsafe fn from_raw_mut(raw: &mut sys::ImDrawData) -> &mut Self {
        &mut *(raw as *mut _ as *mut DrawData)
    }
    /// Returns an immutable reference to the underlying raw Dear ImGui draw data
    pub unsafe fn raw(&self) -> &sys::ImDrawData {
        &*(self as *const _ as *const sys::ImDrawData)
    }
    /// Returns a mutable reference to the underlying raw Dear ImGui draw data
    pub unsafe fn raw_mut(&mut self) -> &mut sys::ImDrawData {
        &mut *(self as *mut _ as *mut sys::ImDrawData)
    }
}

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

#[repr(transparent)]
pub struct DrawList(sys::ImDrawList);

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
        self.iter.next().map(|&cmd| DrawCmd::Elements {
            count: cmd.ElemCount as usize,
            clip_rect: cmd.ClipRect.into(),
            texture_id: TextureId::from(cmd.TextureId),
        })
    }
}

pub type DrawIdx = sys::ImDrawIdx;

pub enum DrawCmd {
    Elements {
        count: usize,
        clip_rect: [f32; 4],
        texture_id: TextureId,
    },
    // TODO: Support for draw callbacks
}

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
