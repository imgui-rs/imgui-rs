#[macro_use]
extern crate glium;
extern crate imgui;

use glium::backend::{Context, Facade};
use glium::index::{self, PrimitiveType};
use glium::program;
use glium::texture;
use glium::vertex;
use glium::{DrawError, GlObject, IndexBuffer, Program, Surface, Texture2d, VertexBuffer};
use imgui::{DrawList, FrameSize, ImDrawIdx, ImDrawVert, ImGui, Ui};
use std::borrow::Cow;
use std::fmt;
use std::rc::Rc;

pub type RendererResult<T> = Result<T, RendererError>;

#[derive(Clone, Debug)]
pub enum RendererError {
    Vertex(vertex::BufferCreationError),
    Index(index::BufferCreationError),
    Program(program::ProgramChooserCreationError),
    Texture(texture::TextureCreationError),
    Draw(DrawError),
}

impl fmt::Display for RendererError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RendererError::*;
        match *self {
            Vertex(_) => write!(f, "Vertex buffer creation failed"),
            Index(_) => write!(f, "Index buffer creation failed"),
            Program(ref e) => write!(f, "Program creation failed: {}", e),
            Texture(_) => write!(f, "Texture creation failed"),
            Draw(ref e) => write!(f, "Drawing failed: {}", e),
        }
    }
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

impl From<program::ProgramChooserCreationError> for RendererError {
    fn from(e: program::ProgramChooserCreationError) -> RendererError {
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

pub struct Renderer {
    ctx: Rc<Context>,
    device_objects: DeviceObjects,
}

impl Renderer {
    pub fn init<F: Facade>(imgui: &mut ImGui, ctx: &F) -> RendererResult<Renderer> {
        let device_objects = DeviceObjects::init(imgui, ctx)?;
        Ok(Renderer {
            ctx: Rc::clone(ctx.get_context()),
            device_objects: device_objects,
        })
    }

    pub fn render<'a, S: Surface>(&mut self, surface: &mut S, ui: Ui<'a>) -> RendererResult<()> {
        let _ = self.ctx.insert_debug_marker("imgui-rs: starting rendering");
        let FrameSize {
            logical_size: (width, height),
            hidpi_factor,
        } = ui.frame_size();
        if !(width > 0.0 && height > 0.0) {
            return Ok(());
        }
        let fb_size = (
            (width * hidpi_factor) as f32,
            (height * hidpi_factor) as f32,
        );

        let matrix = [
            [(2.0 / width) as f32, 0.0, 0.0, 0.0],
            [0.0, (2.0 / -height) as f32, 0.0, 0.0],
            [0.0, 0.0, -1.0, 0.0],
            [-1.0, 1.0, 0.0, 1.0],
        ];
        let result = ui.render(|ui, mut draw_data| {
            draw_data.scale_clip_rects(ui.imgui().display_framebuffer_scale());
            for draw_list in draw_data.into_iter() {
                self.render_draw_list(surface, &draw_list, fb_size, matrix)?;
            }
            Ok(())
        });
        let _ = self.ctx.insert_debug_marker("imgui-rs: rendering finished");
        result
    }

    fn render_draw_list<'a, S: Surface>(
        &mut self,
        surface: &mut S,
        draw_list: &DrawList<'a>,
        fb_size: (f32, f32),
        matrix: [[f32; 4]; 4],
    ) -> RendererResult<()> {
        use glium::{Blend, DrawParameters, Rect};

        let (fb_width, fb_height) = fb_size;

        self.device_objects
            .upload_vertex_buffer(&self.ctx, draw_list.vtx_buffer)?;
        self.device_objects
            .upload_index_buffer(&self.ctx, draw_list.idx_buffer)?;

        let font_texture_id = self.device_objects.texture.get_id() as usize;

        let mut idx_start = 0;
        for cmd in draw_list.cmd_buffer {
            // We don't support custom textures...yet!
            assert!(cmd.texture_id as usize == font_texture_id);

            let idx_end = idx_start + cmd.elem_count as usize;

            surface.draw(
                &self.device_objects.vertex_buffer,
                &self
                    .device_objects
                    .index_buffer
                    .slice(idx_start..idx_end)
                    .expect("Invalid index buffer range"),
                &self.device_objects.program,
                &uniform! {
                    matrix: matrix,
                    tex: self.device_objects.texture.sampled()
                },
                &DrawParameters {
                    blend: Blend::alpha_blending(),
                    scissor: Some(Rect {
                        left: cmd.clip_rect.x.max(0.0).min(fb_width).round() as u32,
                        bottom: (fb_height - cmd.clip_rect.w).max(0.0).min(fb_width).round() as u32,
                        width: (cmd.clip_rect.z - cmd.clip_rect.x)
                            .abs()
                            .min(fb_width)
                            .round() as u32,
                        height: (cmd.clip_rect.w - cmd.clip_rect.y)
                            .abs()
                            .min(fb_height)
                            .round() as u32,
                    }),
                    ..DrawParameters::default()
                },
            )?;

            idx_start = idx_end;
        }

        Ok(())
    }
}

pub struct DeviceObjects {
    vertex_buffer: VertexBuffer<ImDrawVert>,
    index_buffer: IndexBuffer<ImDrawIdx>,
    program: Program,
    texture: Texture2d,
}

fn compile_default_program<F: Facade>(
    ctx: &F,
) -> Result<Program, program::ProgramChooserCreationError> {
    program!(
        ctx,
        400 => {
            vertex: include_str!("shader/glsl_400.vert"),
            fragment: include_str!("shader/glsl_400.frag"),
            outputs_srgb: true,
        },
        130 => {
            vertex: include_str!("shader/glsl_130.vert"),
            fragment: include_str!("shader/glsl_130.frag"),
            outputs_srgb: true,
        },
        110 => {
            vertex: include_str!("shader/glsl_110.vert"),
            fragment: include_str!("shader/glsl_110.frag"),
            outputs_srgb: true,
        },
        300 es => {
            vertex: include_str!("shader/glsles_300.vert"),
            fragment: include_str!("shader/glsles_300.frag"),
            outputs_srgb: true,
        },
        100 es => {
            vertex: include_str!("shader/glsles_100.vert"),
            fragment: include_str!("shader/glsles_100.frag"),
            outputs_srgb: true,
        },
    )
}

impl DeviceObjects {
    pub fn init<F: Facade>(im_gui: &mut ImGui, ctx: &F) -> RendererResult<DeviceObjects> {
        use glium::texture::{ClientFormat, RawImage2d};

        let vertex_buffer = VertexBuffer::empty_dynamic(ctx, 0)?;
        let index_buffer = IndexBuffer::empty_dynamic(ctx, PrimitiveType::TrianglesList, 0)?;

        let program = compile_default_program(ctx)?;
        let texture = im_gui.prepare_texture(|handle| {
            let data = RawImage2d {
                data: Cow::Borrowed(handle.pixels),
                width: handle.width,
                height: handle.height,
                format: ClientFormat::U8U8U8U8,
            };
            Texture2d::new(ctx, data)
        })?;
        im_gui.set_texture_id(texture.get_id() as usize);

        Ok(DeviceObjects {
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            program: program,
            texture: texture,
        })
    }
    pub fn upload_vertex_buffer<F: Facade>(
        &mut self,
        ctx: &F,
        vtx_buffer: &[ImDrawVert],
    ) -> RendererResult<()> {
        self.vertex_buffer.invalidate();
        if let Some(slice) = self.vertex_buffer.slice_mut(0..vtx_buffer.len()) {
            slice.write(vtx_buffer);
            return Ok(());
        }
        self.vertex_buffer = VertexBuffer::dynamic(ctx, vtx_buffer)?;
        let _ = ctx.get_context().insert_debug_marker(&format!(
            "imgui-rs: resized vertex buffer to {} bytes",
            self.vertex_buffer.get_size()
        ));
        Ok(())
    }
    pub fn upload_index_buffer<F: Facade>(
        &mut self,
        ctx: &F,
        idx_buffer: &[ImDrawIdx],
    ) -> RendererResult<()> {
        self.index_buffer.invalidate();
        if let Some(slice) = self.index_buffer.slice_mut(0..idx_buffer.len()) {
            slice.write(idx_buffer);
            return Ok(());
        }
        self.index_buffer = IndexBuffer::dynamic(ctx, PrimitiveType::TrianglesList, idx_buffer)?;
        let _ = ctx.get_context().insert_debug_marker(&format!(
            "imgui-rs: resized index buffer to {} bytes",
            self.index_buffer.get_size()
        ));
        Ok(())
    }
}
