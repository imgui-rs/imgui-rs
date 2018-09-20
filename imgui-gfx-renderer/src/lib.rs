#[macro_use]
extern crate gfx;
extern crate imgui;

use gfx::handle::{Buffer, RenderTargetView};
use gfx::memory::Bind;
use gfx::texture::{FilterMethod, SamplerInfo, WrapMode};
use gfx::traits::FactoryExt;
use gfx::{CommandBuffer, Encoder, Factory, IntoIndexBuffer, Rect, Resources, Slice};
use gfx::pso::{PipelineData, PipelineState};
use imgui::{DrawList, FrameSize, ImDrawIdx, ImDrawVert, ImGui, Ui, Textures};

pub type RendererResult<T> = Result<T, RendererError>;

#[derive(Clone, Debug)]
pub enum RendererError {
    Update(gfx::UpdateError<usize>),
    Buffer(gfx::buffer::CreationError),
    Pipeline(gfx::PipelineStateError<String>),
    Combined(gfx::CombinedError),
}

impl From<gfx::UpdateError<usize>> for RendererError {
    fn from(e: gfx::UpdateError<usize>) -> RendererError {
        RendererError::Update(e)
    }
}

impl From<gfx::buffer::CreationError> for RendererError {
    fn from(e: gfx::buffer::CreationError) -> RendererError {
        RendererError::Buffer(e)
    }
}

impl From<gfx::PipelineStateError<String>> for RendererError {
    fn from(e: gfx::PipelineStateError<String>) -> RendererError {
        RendererError::Pipeline(e)
    }
}

impl From<gfx::CombinedError> for RendererError {
    fn from(e: gfx::CombinedError) -> RendererError {
        RendererError::Combined(e)
    }
}

// Based on gfx_defines! / gfx_pipeline!, to allow for not having to clone Arcs
// every draw call when selecting which texture is going to be shown.
macro_rules! extended_defines {
    (pipeline $module:ident { $( $field:ident: $ty:ty = $value:expr, )* }) => {
        #[allow(missing_docs)]
        mod $module {
            #[allow(unused_imports)]
            use super::*;
            #[allow(unused_imports)]
            use super::gfx;
            gfx_pipeline_inner!{ $(
                $field: $ty,
            )*}

            pub fn new() -> Init<'static> {
                Init {
                    $( $field: $value, )*
                }
            }

            pub struct BorrowedData<'a, R: Resources> {
                $(pub $field: &'a <$ty as DataBind<R>>::Data,)*
            }

            impl<'a, R: Resources> gfx::pso::PipelineData<R> for BorrowedData<'a, R> {
                type Meta = pipe::Meta;

                fn bake_to(&self, out: &mut RawDataSet<R>, meta: &Self::Meta, man: &mut gfx::handle::Manager<R>, access: &mut AccessInfo<R>) {
                    $(meta.$field.bind_to(out, &self.$field, man, access);)*
                }
            }
        }
    }
}

extended_defines! {
    pipeline pipe {
        vertex_buffer: gfx::VertexBuffer<ImDrawVert> = (),
        matrix: gfx::Global<[[f32; 4]; 4]> = "matrix",
        tex: gfx::TextureSampler<[f32; 4]> = "tex",
        out: gfx::BlendTarget<gfx::format::Rgba8> = (
            "Target0",
            gfx::state::ColorMask::all(),
            gfx::preset::blend::ALPHA,
        ),
        scissor: gfx::Scissor = (),
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Shaders {
    GlSl400,   // OpenGL 4.0+
    GlSl130,   // OpenGL 3.0+
    GlSl110,   // OpenGL 2.0+
    GlSlEs300, // OpenGL ES 3.0+
    GlSlEs100, // OpenGL ES 2.0+
}

impl Shaders {
    fn get_program_code(self) -> (&'static [u8], &'static [u8]) {
        use Shaders::*;
        match self {
            GlSl400 => (
                include_bytes!("shader/glsl_400.vert"),
                include_bytes!("shader/glsl_400.frag"),
            ),
            GlSl130 => (
                include_bytes!("shader/glsl_130.vert"),
                include_bytes!("shader/glsl_130.frag"),
            ),
            GlSl110 => (
                include_bytes!("shader/glsl_110.vert"),
                include_bytes!("shader/glsl_110.frag"),
            ),
            GlSlEs300 => (
                include_bytes!("shader/glsles_300.vert"),
                include_bytes!("shader/glsles_300.frag"),
            ),
            GlSlEs100 => (
                include_bytes!("shader/glsles_100.vert"),
                include_bytes!("shader/glsles_100.frag"),
            ),
        }
    }
}

pub type Texture<R> = (gfx::handle::ShaderResourceView<R, [f32; 4]>, gfx::handle::Sampler<R>);

pub struct Renderer<R: Resources> {
    bundle: Bundle<R, pipe::Data<R>>,
    index_buffer: Buffer<R, u16>,
    textures: Textures<Texture<R>>,
    backup_texture: Texture<R>,
}

impl<R: Resources> Renderer<R> {
    pub fn init<F: Factory<R>>(
        imgui: &mut ImGui,
        factory: &mut F,
        shaders: Shaders,
        out: RenderTargetView<R, gfx::format::Rgba8>,
    ) -> RendererResult<Renderer<R>> {
        let (vs_code, ps_code) = shaders.get_program_code();
        let pso = factory.create_pipeline_simple(vs_code, ps_code, pipe::new())?;
        let vertex_buffer = factory.create_buffer::<ImDrawVert>(
            256,
            gfx::buffer::Role::Vertex,
            gfx::memory::Usage::Dynamic,
            Bind::empty(),
        )?;
        let index_buffer = factory.create_buffer::<ImDrawIdx>(
            256,
            gfx::buffer::Role::Index,
            gfx::memory::Usage::Dynamic,
            Bind::empty(),
        )?;
        let (_, texture) = imgui.prepare_texture(|handle| {
            factory.create_texture_immutable_u8::<gfx::format::Rgba8>(
                gfx::texture::Kind::D2(
                    handle.width as u16,
                    handle.height as u16,
                    gfx::texture::AaMode::Single,
                ),
                gfx::texture::Mipmap::Provided,
                &[handle.pixels],
            )
        })?;
        let sampler =
            factory.create_sampler(SamplerInfo::new(FilterMethod::Trilinear, WrapMode::Clamp));
        let pair = (texture, sampler);
        let mut textures = Textures::new();
        imgui.set_font_texture_id(textures.insert(pair.clone()));

        let slice = Slice {
            start: 0,
            end: 0,
            base_vertex: 0,
            instances: None,
            buffer: index_buffer.clone().into_index_buffer(factory),
        };
        Ok(Renderer {
            bundle: Bundle {
                slice: slice,
                pso: pso,
                vertex_buffer: vertex_buffer,
                out: out,
            },
            index_buffer: index_buffer,
            textures: textures,
            backup_texture: pair,
        })
    }

    pub fn update_render_target(&mut self, out: RenderTargetView<R, gfx::format::Rgba8>) {
        self.bundle.out = out;
    }

    pub fn textures(&mut self) -> &mut Textures<Texture<R>> {
        &mut self.textures
    }

    pub fn render<'a, F: Factory<R>, C: CommandBuffer<R>>(
        &mut self,
        ui: Ui<'a>,
        factory: &mut F,
        encoder: &mut Encoder<R, C>,
    ) -> RendererResult<()> {
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

        ui.render(|ui, mut draw_data| {
            draw_data.scale_clip_rects(ui.imgui().display_framebuffer_scale());
            for draw_list in &draw_data {
                self.render_draw_list(factory, encoder, &draw_list, fb_size, &matrix)?;
            }
            Ok(())
        })
    }
    fn render_draw_list<'a, F: Factory<R>, C: CommandBuffer<R>>(
        &mut self,
        factory: &mut F,
        encoder: &mut Encoder<R, C>,
        draw_list: &DrawList<'a>,
        fb_size: (f32, f32),
        matrix: &[[f32; 4]; 4],
    ) -> RendererResult<()> {
        let (fb_width, fb_height) = fb_size;

        self.upload_vertex_buffer(factory, encoder, draw_list.vtx_buffer)?;
        self.upload_index_buffer(factory, encoder, draw_list.idx_buffer)?;

        self.bundle.slice.start = 0;
        for cmd in draw_list.cmd_buffer {
            let tex = self.textures.get(cmd.texture_id.into()).unwrap_or(&self.backup_texture);

            self.bundle.slice.end = self.bundle.slice.start + cmd.elem_count;
            let scissor = Rect {
                x: cmd.clip_rect.x.max(0.0).min(fb_width).round() as u16,
                y: cmd.clip_rect.y.max(0.0).min(fb_height).round() as u16,
                w: (cmd.clip_rect.z - cmd.clip_rect.x)
                    .abs()
                    .min(fb_width)
                    .round() as u16,
                h: (cmd.clip_rect.w - cmd.clip_rect.y)
                    .abs()
                    .min(fb_height)
                    .round() as u16,
            };
            let data = pipe::BorrowedData {
                vertex_buffer: &self.bundle.vertex_buffer,
                matrix: matrix,
                tex: tex,
                out: &self.bundle.out,
                scissor: &scissor,
            };
            encoder.draw(&self.bundle.slice, &self.bundle.pso, &data);
            self.bundle.slice.start = self.bundle.slice.end;
        }
        Ok(())
    }
    fn upload_vertex_buffer<F: Factory<R>, C: CommandBuffer<R>>(
        &mut self,
        factory: &mut F,
        encoder: &mut Encoder<R, C>,
        vtx_buffer: &[ImDrawVert],
    ) -> RendererResult<()> {
        if self.bundle.vertex_buffer.len() < vtx_buffer.len() {
            self.bundle.vertex_buffer = factory.create_buffer::<ImDrawVert>(
                vtx_buffer.len(),
                gfx::buffer::Role::Vertex,
                gfx::memory::Usage::Dynamic,
                Bind::empty(),
            )?;
        }
        Ok(encoder.update_buffer(&self.bundle.vertex_buffer, vtx_buffer, 0)?)
    }
    fn upload_index_buffer<F: Factory<R>, C: CommandBuffer<R>>(
        &mut self,
        factory: &mut F,
        encoder: &mut Encoder<R, C>,
        idx_buffer: &[ImDrawIdx],
    ) -> RendererResult<()> {
        if self.index_buffer.len() < idx_buffer.len() {
            self.index_buffer = factory.create_buffer::<ImDrawIdx>(
                idx_buffer.len(),
                gfx::buffer::Role::Index,
                gfx::memory::Usage::Dynamic,
                Bind::empty(),
            )?;
            self.bundle.slice.buffer = self.index_buffer.clone().into_index_buffer(factory);
        }
        Ok(encoder.update_buffer(&self.index_buffer, idx_buffer, 0)?)
    }
}

struct Bundle<R: Resources, Data: PipelineData<R>> {
    slice: Slice<R>,
    pso: PipelineState<R, Data::Meta>,
    vertex_buffer: Buffer<R, ImDrawVert>,
    out: RenderTargetView<R, gfx::format::Rgba8>,
}
