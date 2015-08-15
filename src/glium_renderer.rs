use glium::{
    index, program, texture, vertex,
    DrawError, DrawParameters, IndexBuffer, Program, Rect, Surface, Texture2d, VertexBuffer
};
use glium::backend::{Context, Facade};
use glium::draw_parameters::{BlendingFunction, LinearBlendingFactor};
use glium::index::PrimitiveType;
use glium::texture::{ClientFormat, RawImage2d};
use glium::vertex::{Attribute, AttributeType, Vertex, VertexFormat};
use libc::c_float;
use std::borrow::Cow;
use std::mem;
use std::rc::Rc;

use super::{DrawList, Frame, ImDrawIdx, ImDrawVert, ImGui, ImVec2, ImVec4};

pub type RendererResult<T> = Result<T, RendererError>;

pub enum RendererError {
    Vertex(vertex::BufferCreationError),
    Index(index::BufferCreationError),
    Program(program::ProgramCreationError),
    Texture(texture::TextureCreationError),
    Draw(DrawError)
}

impl From<vertex::BufferCreationError> for RendererError {
    fn from(e: vertex::BufferCreationError) -> RendererError {
        RendererError::Vertex(e)
    }
}

impl From<index::BufferCreationError> for RendererError {
    fn from(e: index::BufferCreationError) -> RendererError {
        RendererError::Index(e)
    }
}

impl From<program::ProgramCreationError> for RendererError {
    fn from(e: program::ProgramCreationError) -> RendererError {
        RendererError::Program(e)
    }
}

impl From<texture::TextureCreationError> for RendererError {
    fn from(e: texture::TextureCreationError) -> RendererError {
        RendererError::Texture(e)
    }
}

impl From<DrawError> for RendererError {
    fn from(e: DrawError) -> RendererError {
        RendererError::Draw(e)
    }
}

unsafe impl Attribute for ImVec2 {
    fn get_type() -> AttributeType { <(c_float, c_float) as Attribute>::get_type() }
}

unsafe impl Attribute for ImVec4 {
    fn get_type() -> AttributeType {
        <(c_float, c_float, c_float, c_float) as Attribute>::get_type()
    }
}

impl Vertex for ImDrawVert {
    fn build_bindings() -> VertexFormat {
        unsafe {
            let dummy: &ImDrawVert = mem::transmute(0usize);
            Cow::Owned(vec![
                       ("pos".into(), mem::transmute(&dummy.pos), <ImVec2 as Attribute>::get_type()),
                       ("uv".into(), mem::transmute(&dummy.uv), <ImVec2 as Attribute>::get_type()),
                       ("col".into(), mem::transmute(&dummy.col), AttributeType::U8U8U8U8)
            ])
        }
    }
}

pub struct Renderer {
    ctx: Rc<Context>,
    device_objects: DeviceObjects
}

impl Renderer {
    pub fn init<F: Facade>(imgui: &mut ImGui, ctx: &F) -> RendererResult<Renderer> {
        let device_objects = try!(DeviceObjects::init(imgui, ctx));
        Ok(Renderer {
            ctx: ctx.get_context().clone(),
            device_objects: device_objects
        })
    }
    pub fn render<'a, S: Surface>(&mut self, surface: &mut S,
                                  frame: Frame<'a>) -> RendererResult<()> {
        frame.render(|draw_list| self.render_draw_list(surface, draw_list))
    }
    fn render_draw_list<'a, S: Surface>(&mut self, surface: &mut S,
                                        draw_list: DrawList<'a>) -> RendererResult<()> {
        try!(self.device_objects.upload_vertex_buffer(&self.ctx, draw_list.vtx_buffer));
        try!(self.device_objects.upload_index_buffer(&self.ctx, draw_list.idx_buffer));

        let (width, height) = surface.get_dimensions();

        let mut idx_start = 0;
        for cmd in draw_list.cmd_buffer {
            let matrix = [
                [ 2.0 / (width as f32), 0.0, 0.0, 0.0 ],
                [ 0.0, 2.0 / -(height as f32), 0.0, 0.0 ],
                [ 0.0, 0.0, -1.0, 0.0 ],
                    [ -1.0, 1.0, 0.0, 1.0 ]
            ];
            let uniforms = uniform! {
                matrix: matrix,
                texture: self.device_objects.texture.sampled()
            };
            let draw_params = DrawParameters {
                blending_function: Some(BlendingFunction::Addition {
                    source: LinearBlendingFactor::SourceAlpha,
                    destination: LinearBlendingFactor::OneMinusSourceAlpha
                }),
                scissor: Some(Rect {
                    left: cmd.clip_rect.x as u32,
                    bottom: (height as f32 - cmd.clip_rect.w) as u32,
                    width: (cmd.clip_rect.z - cmd.clip_rect.x) as u32,
                    height: (cmd.clip_rect.w - cmd.clip_rect.y) as u32
                }),
                .. Default::default()
            };
            let idx_end = idx_start + cmd.elem_count as usize;
            try!(surface.draw(&self.device_objects.vertex_buffer,
                              &self.device_objects.index_buffer.slice(idx_start ..idx_end)
                                  .expect("Invalid index buffer range"),
                              &self.device_objects.program,
                              &uniforms,
                              &draw_params));
            idx_start = idx_end;
        }
        Ok(())
    }
}

pub struct DeviceObjects {
    vertex_buffer: VertexBuffer<ImDrawVert>,
    index_buffer: IndexBuffer<ImDrawIdx>,
    program: Program,
    texture: Texture2d
}

fn compile_default_program<F: Facade>(ctx: &F) -> Result<Program, program::ProgramCreationError> {
    program!(
        ctx,
        140 => {
            vertex: include_str!("shader/vert_140.glsl"),
            fragment: include_str!("shader/frag_140.glsl"),
            outputs_srgb: true
        },
        110 => {
            vertex: include_str!("shader/vert_110.glsl"),
            fragment: include_str!("shader/frag_110.glsl"),
            outputs_srgb: true
        }
    )
}

impl DeviceObjects {
    pub fn init<F: Facade>(im_gui: &mut ImGui, ctx: &F) -> RendererResult<DeviceObjects> {
        let vertex_buffer = try!(VertexBuffer::empty_dynamic(ctx, 0));
        let index_buffer = try!(IndexBuffer::empty_dynamic(ctx, PrimitiveType::TrianglesList, 0));

        let program = try!(compile_default_program(ctx));
        let texture = try!(im_gui.prepare_texture(|handle| {
            let data = RawImage2d {
                data: Cow::Borrowed(handle.pixels),
                width: handle.width,
                height: handle.height,
                format: ClientFormat::U8U8U8U8
            };
            Texture2d::new(ctx, data)
        }));

        Ok(DeviceObjects {
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            program: program,
            texture: texture
        })
    }
    pub fn upload_vertex_buffer<F: Facade>(&mut self, ctx: &F,
                                           vtx_buffer: &[ImDrawVert]) -> RendererResult<()> {
        self.vertex_buffer.invalidate();
        if let Some(slice) = self.vertex_buffer.slice_mut(0 .. vtx_buffer.len()) {
            slice.write(vtx_buffer);
            return Ok(());
        }
        self.vertex_buffer = try!(VertexBuffer::dynamic(ctx, vtx_buffer));
        Ok(())
    }
    pub fn upload_index_buffer<F: Facade>(&mut self, ctx: &F,
                                          idx_buffer: &[ImDrawIdx]) -> RendererResult<()> {
        self.index_buffer.invalidate();
        if let Some(slice) = self.index_buffer.slice_mut(0 .. idx_buffer.len()) {
            slice.write(idx_buffer);
            return Ok(());
        }
        self.index_buffer =
            try!(IndexBuffer::dynamic(ctx, PrimitiveType::TrianglesList, idx_buffer));
        Ok(())
    }
}
