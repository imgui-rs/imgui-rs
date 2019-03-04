use glium::backend::{Context, Facade};
use glium::index::{self, PrimitiveType};
use glium::program::ProgramChooserCreationError;
use glium::texture::{ClientFormat, MipmapsOption, RawImage2d, TextureCreationError};
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use glium::{
    program, uniform, vertex, Blend, DrawError, DrawParameters, IndexBuffer, Program, Rect,
    Surface, Texture2d, VertexBuffer,
};
use imgui::{DrawCmd, ImString, TextureId, Textures, Ui};
use std::borrow::Cow;
use std::fmt;
use std::rc::Rc;
use std::usize;

#[derive(Clone, Debug)]
pub enum GliumRendererError {
    Vertex(vertex::BufferCreationError),
    Index(index::BufferCreationError),
    Program(ProgramChooserCreationError),
    Texture(TextureCreationError),
    Draw(DrawError),
    BadTexture(TextureId),
}

impl fmt::Display for GliumRendererError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::GliumRendererError::*;
        match *self {
            Vertex(_) => write!(f, "Vertex buffer creation failed"),
            Index(_) => write!(f, "Index buffer creation failed"),
            Program(ref e) => write!(f, "Program creation failed: {}", e),
            Texture(_) => write!(f, "Texture creation failed"),
            Draw(ref e) => write!(f, "Drawing failed: {}", e),
            BadTexture(ref t) => write!(f, "Bad texture ID: {}", t.id()),
        }
    }
}

impl From<vertex::BufferCreationError> for GliumRendererError {
    fn from(e: vertex::BufferCreationError) -> GliumRendererError {
        GliumRendererError::Vertex(e)
    }
}

impl From<index::BufferCreationError> for GliumRendererError {
    fn from(e: index::BufferCreationError) -> GliumRendererError {
        GliumRendererError::Index(e)
    }
}

impl From<ProgramChooserCreationError> for GliumRendererError {
    fn from(e: ProgramChooserCreationError) -> GliumRendererError {
        GliumRendererError::Program(e)
    }
}

impl From<TextureCreationError> for GliumRendererError {
    fn from(e: TextureCreationError) -> GliumRendererError {
        GliumRendererError::Texture(e)
    }
}

impl From<DrawError> for GliumRendererError {
    fn from(e: DrawError) -> GliumRendererError {
        GliumRendererError::Draw(e)
    }
}

pub struct GliumRenderer {
    ctx: Rc<Context>,
    program: Program,
    font_texture: Texture2d,
    textures: Textures<Rc<Texture2d>>,
}

impl GliumRenderer {
    pub fn init<F: Facade>(
        ctx: &mut imgui::Context,
        facade: &F,
    ) -> Result<GliumRenderer, GliumRendererError> {
        let program = compile_default_program(facade)?;
        let font_texture = upload_font_texture(ctx.fonts(), facade.get_context())?;
        ctx.set_renderer_name(Some(ImString::from(format!(
            "imgui-glium-renderer {}",
            env!("CARGO_PKG_VERSION")
        ))));
        Ok(GliumRenderer {
            ctx: Rc::clone(facade.get_context()),
            program,
            font_texture,
            textures: Textures::new(),
        })
    }
    pub fn reload_font_texture(
        &mut self,
        ctx: &mut imgui::Context,
    ) -> Result<(), GliumRendererError> {
        self.font_texture = upload_font_texture(ctx.fonts(), &self.ctx)?;
        Ok(())
    }
    pub fn textures(&mut self) -> &mut Textures<Rc<Texture2d>> {
        &mut self.textures
    }
    fn lookup_texture(&self, texture_id: TextureId) -> Result<&Texture2d, GliumRendererError> {
        if texture_id.id() == usize::MAX {
            Ok(&self.font_texture)
        } else if let Some(texture) = self.textures.get(texture_id) {
            Ok(texture)
        } else {
            Err(GliumRendererError::BadTexture(texture_id))
        }
    }
    pub fn render<'ui, T: Surface>(
        &mut self,
        target: &mut T,
        ui: Ui<'ui>,
    ) -> Result<(), GliumRendererError> {
        let draw_data = ui.render();
        let fb_width = draw_data.display_size[0] * draw_data.framebuffer_scale[0];
        let fb_height = draw_data.display_size[1] * draw_data.framebuffer_scale[1];
        if !(fb_width > 0.0 && fb_height > 0.0) {
            return Ok(());
        }
        let _ = self.ctx.insert_debug_marker("imgui-rs: starting rendering");
        let left = draw_data.display_pos[0];
        let right = draw_data.display_pos[0] + draw_data.display_size[0];
        let top = draw_data.display_pos[1];
        let bottom = draw_data.display_pos[1] + draw_data.display_size[1];
        let matrix = [
            [(2.0 / (right - left)), 0.0, 0.0, 0.0],
            [0.0, (2.0 / (top - bottom)), 0.0, 0.0],
            [0.0, 0.0, -1.0, 0.0],
            [
                (right + left) / (left - right),
                (top + bottom) / (bottom - top),
                0.0,
                1.0,
            ],
        ];
        let clip_off = draw_data.display_pos;
        let clip_scale = draw_data.framebuffer_scale;
        for draw_list in draw_data.draw_lists() {
            let vtx_buffer = VertexBuffer::immutable(&self.ctx, draw_list.vtx_buffer())?;
            let idx_buffer = IndexBuffer::immutable(
                &self.ctx,
                PrimitiveType::TrianglesList,
                draw_list.idx_buffer(),
            )?;
            let mut idx_start = 0;
            for cmd in draw_list.commands() {
                // TODO: Support for draw callbacks
                match cmd {
                    DrawCmd::Elements {
                        count,
                        clip_rect,
                        texture_id,
                    } => {
                        let idx_end = idx_start + count;
                        let clip_rect = [
                            (clip_rect[0] - clip_off[0]) * clip_scale[0],
                            (clip_rect[1] - clip_off[1]) * clip_scale[1],
                            (clip_rect[2] - clip_off[0]) * clip_scale[0],
                            (clip_rect[3] - clip_off[1]) * clip_scale[1],
                        ];

                        if clip_rect[0] < fb_width
                            && clip_rect[1] < fb_height
                            && clip_rect[2] >= 0.0
                            && clip_rect[3] >= 0.0
                        {
                            target.draw(
                                &vtx_buffer,
                                &idx_buffer
                                    .slice(idx_start..idx_end)
                                    .expect("Invalid index buffer range"),
                                &self.program,
                                &uniform! {
                                    matrix: matrix,
                                    tex: self.lookup_texture(texture_id)?.sampled()
                                        .minify_filter(MinifySamplerFilter::Linear)
                                        .magnify_filter(MagnifySamplerFilter::Linear)
                                },
                                &DrawParameters {
                                    blend: Blend::alpha_blending(),
                                    scissor: Some(Rect {
                                        left: f32::max(0.0, clip_rect[0]).floor() as u32,
                                        bottom: f32::max(0.0, fb_height - clip_rect[3]).floor()
                                            as u32,
                                        width: (clip_rect[2] - clip_rect[0]).abs().ceil() as u32,
                                        height: (clip_rect[3] - clip_rect[1]).abs().ceil() as u32,
                                    }),
                                    ..DrawParameters::default()
                                },
                            )?;
                        }

                        idx_start = idx_end;
                    }
                }
            }
        }
        let _ = self.ctx.insert_debug_marker("imgui-rs: rendering finished");
        Ok(())
    }
}

fn upload_font_texture(
    mut fonts: imgui::FontAtlasRefMut,
    ctx: &Rc<Context>,
) -> Result<Texture2d, GliumRendererError> {
    let texture = fonts.build_rgba32_texture();
    let data = RawImage2d {
        data: Cow::Borrowed(texture.data),
        width: texture.width,
        height: texture.height,
        format: ClientFormat::U8U8U8U8,
    };
    let font_texture = Texture2d::with_mipmaps(ctx, data, MipmapsOption::NoMipmap)?;
    fonts.set_texture_id(TextureId::from(usize::MAX));
    Ok(font_texture)
}

fn compile_default_program<F: Facade>(facade: &F) -> Result<Program, ProgramChooserCreationError> {
    program!(
        facade,
        400 => {
            vertex: include_str!("shader/glsl_400.vert"),
            fragment: include_str!("shader/glsl_400.frag"),
            outputs_srgb: true,
        },
        150 => {
            vertex: include_str!("shader/glsl_150.vert"),
            fragment: include_str!("shader/glsl_150.frag"),
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
