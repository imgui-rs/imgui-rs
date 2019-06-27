use gfx::format::BlendFormat;
use gfx::handle::{Buffer, RenderTargetView};
use gfx::memory::Bind;
use gfx::pso::PipelineState;
use gfx::texture::{FilterMethod, SamplerInfo, WrapMode};
use gfx::traits::FactoryExt;
use gfx::{CommandBuffer, Encoder, Factory, IntoIndexBuffer, Rect, Resources, Slice};
use imgui::internal::RawWrapper;
use imgui::{DrawCmd, DrawCmdParams, DrawIdx, DrawVert, ImString, TextureId, Textures, Ui};
use std::usize;

#[derive(Clone, Debug)]
pub enum GfxRendererError {
    Update(gfx::UpdateError<usize>),
    Buffer(gfx::buffer::CreationError),
    Pipeline(gfx::PipelineStateError<String>),
    Combined(gfx::CombinedError),
    BadTexture(TextureId),
}

impl From<gfx::UpdateError<usize>> for GfxRendererError {
    fn from(e: gfx::UpdateError<usize>) -> GfxRendererError {
        GfxRendererError::Update(e)
    }
}

impl From<gfx::buffer::CreationError> for GfxRendererError {
    fn from(e: gfx::buffer::CreationError) -> GfxRendererError {
        GfxRendererError::Buffer(e)
    }
}

impl From<gfx::PipelineStateError<String>> for GfxRendererError {
    fn from(e: gfx::PipelineStateError<String>) -> GfxRendererError {
        GfxRendererError::Pipeline(e)
    }
}

impl From<gfx::CombinedError> for GfxRendererError {
    fn from(e: gfx::CombinedError) -> GfxRendererError {
        GfxRendererError::Combined(e)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Shaders {
    /// OpenGL 4.0+
    GlSl400,
    /// OpenGL 3.2+
    GlSl150,
    /// OpenGL 3.0+
    GlSl130,
    /// OpenGL 2.0+
    GlSl110,
    /// OpenGL ES 3.0+
    GlSlEs300,
    /// OpenGL ES 2.0+
    GlSlEs100,
    /// HLSL Shader Model 4.0+
    HlslSm40,
}

impl Shaders {
    fn get_program_code(self) -> (&'static [u8], &'static [u8]) {
        use self::Shaders::*;
        match self {
            GlSl400 => (
                include_bytes!("shader/glsl_400.vert"),
                include_bytes!("shader/glsl_400.frag"),
            ),
            GlSl150 => (
                include_bytes!("shader/glsl_150.vert"),
                include_bytes!("shader/glsl_150.frag"),
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
            HlslSm40 => (
                include_bytes!("data/vertex.fx"),
                include_bytes!("data/pixel.fx"),
            ),
        }
    }
}

pub type Texture<R> = (
    gfx::handle::ShaderResourceView<R, [f32; 4]>,
    gfx::handle::Sampler<R>,
);

pub struct GfxRenderer<Cf: BlendFormat, R: Resources> {
    vertex_buffer: Buffer<R, DrawVert>,
    index_buffer: Buffer<R, DrawIdx>,
    slice: Slice<R>,
    pso: PipelineState<R, pipeline::Meta<Cf>>,
    font_texture: Texture<R>,
    textures: Textures<Texture<R>>,
    #[cfg(feature = "directx")]
    constants: Buffer<R, constants::Constants>,
}

impl<Cf, R> GfxRenderer<Cf, R>
where
    Cf: BlendFormat,
    R: Resources,
{
    pub fn init<F: Factory<R>>(
        ctx: &mut imgui::Context,
        factory: &mut F,
        shaders: Shaders,
    ) -> Result<GfxRenderer<Cf, R>, GfxRendererError> {
        let (vs_code, ps_code) = shaders.get_program_code();
        let pso = factory.create_pipeline_simple(vs_code, ps_code, pipeline::new::<Cf>())?;
        let vertex_buffer = factory.create_buffer::<DrawVert>(
            256,
            gfx::buffer::Role::Vertex,
            gfx::memory::Usage::Dynamic,
            Bind::empty(),
        )?;
        let index_buffer = factory.create_buffer::<DrawIdx>(
            256,
            gfx::buffer::Role::Index,
            gfx::memory::Usage::Dynamic,
            Bind::empty(),
        )?;
        let (_, texture) = ctx.prepare_texture(|handle| {
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
        ctx.set_font_texture_id(TextureId::from(usize::MAX));

        let slice = Slice {
            start: 0,
            end: 0,
            base_vertex: 0,
            instances: None,
            buffer: index_buffer.clone().into_index_buffer(factory),
        };
        ctx.set_renderer_name(Some(ImString::from(format!(
            "imgui-gfx-renderer {}",
            env!("CARGO_PKG_VERSION")
        ))));
        Ok(GfxRenderer {
            vertex_buffer,
            index_buffer,
            slice,
            pso,
            font_texture: (texture, sampler),
            textures: Textures::new(),
            #[cfg(feature = "directx")]
            constants: factory.create_constant_buffer(1),
        })
    }
    pub fn textures(&mut self) -> &mut Textures<Texture<R>> {
        &mut self.textures
    }
    pub fn render<'ui, F: Factory<R>, C: CommandBuffer<R>>(
        &mut self,
        factory: &mut F,
        encoder: &mut Encoder<R, C>,
        target: &mut RenderTargetView<R, Cf>,
        ui: Ui<'ui>,
    ) -> Result<(), GfxRendererError> {
        let draw_data = ui.render();
        let fb_width = draw_data.display_size[0] * draw_data.framebuffer_scale[0];
        let fb_height = draw_data.display_size[1] * draw_data.framebuffer_scale[1];
        if !(fb_width > 0.0 && fb_height > 0.0) {
            return Ok(());
        }
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
            self.upload_vertex_buffer(factory, encoder, draw_list.vtx_buffer())?;
            self.upload_index_buffer(factory, encoder, draw_list.idx_buffer())?;
            self.slice.start = 0;
            for cmd in draw_list.commands() {
                match cmd {
                    DrawCmd::Elements {
                        count,
                        cmd_params:
                            DrawCmdParams {
                                clip_rect,
                                texture_id,
                            },
                    } => {
                        let clip_rect = [
                            (clip_rect[0] - clip_off[0]) * clip_scale[0],
                            (clip_rect[1] - clip_off[1]) * clip_scale[1],
                            (clip_rect[2] - clip_off[0]) * clip_scale[0],
                            (clip_rect[3] - clip_off[1]) * clip_scale[1],
                        ];

                        self.slice.end = self.slice.start + count as u32;

                        if clip_rect[0] < fb_width
                            && clip_rect[1] < fb_height
                            && clip_rect[2] >= 0.0
                            && clip_rect[3] >= 0.0
                        {
                            let scissor = Rect {
                                x: f32::max(0.0, clip_rect[0]).floor() as u16,
                                y: f32::max(0.0, clip_rect[1]).floor() as u16,
                                w: (clip_rect[2] - clip_rect[0]).abs().ceil() as u16,
                                h: (clip_rect[3] - clip_rect[1]).abs().ceil() as u16,
                            };
                            let tex = self.lookup_texture(texture_id)?;
                            #[cfg(feature = "directx")]
                            {
                                let constants = constants::Constants { matrix };
                                encoder.update_constant_buffer(&self.constants, &constants);
                            }
                            let data = pipeline::Data {
                                vertex_buffer: &self.vertex_buffer,
                                #[cfg(not(feature = "directx"))]
                                matrix: &matrix,
                                #[cfg(feature = "directx")]
                                constants: &self.constants,
                                tex,
                                scissor: &scissor,
                                target,
                            };
                            encoder.draw(&self.slice, &self.pso, &data);
                        }
                        self.slice.start = self.slice.end;
                    }
                    DrawCmd::RawCallback { callback, raw_cmd } => unsafe {
                        callback(draw_list.raw(), raw_cmd)
                    },
                }
            }
        }
        Ok(())
    }
    fn upload_vertex_buffer<F: Factory<R>, C: CommandBuffer<R>>(
        &mut self,
        factory: &mut F,
        encoder: &mut Encoder<R, C>,
        vtx_buffer: &[DrawVert],
    ) -> Result<(), GfxRendererError> {
        if self.vertex_buffer.len() < vtx_buffer.len() {
            self.vertex_buffer = factory.create_buffer::<DrawVert>(
                vtx_buffer.len(),
                gfx::buffer::Role::Vertex,
                gfx::memory::Usage::Dynamic,
                Bind::empty(),
            )?;
        }
        encoder.update_buffer(&self.vertex_buffer, vtx_buffer, 0)?;
        Ok(())
    }
    fn upload_index_buffer<F: Factory<R>, C: CommandBuffer<R>>(
        &mut self,
        factory: &mut F,
        encoder: &mut Encoder<R, C>,
        idx_buffer: &[DrawIdx],
    ) -> Result<(), GfxRendererError> {
        if self.index_buffer.len() < idx_buffer.len() {
            self.index_buffer = factory.create_buffer::<DrawIdx>(
                idx_buffer.len(),
                gfx::buffer::Role::Index,
                gfx::memory::Usage::Dynamic,
                Bind::empty(),
            )?;
            self.slice.buffer = self.index_buffer.clone().into_index_buffer(factory);
        }
        encoder.update_buffer(&self.index_buffer, idx_buffer, 0)?;
        Ok(())
    }
    fn lookup_texture(&self, texture_id: TextureId) -> Result<&Texture<R>, GfxRendererError> {
        if texture_id.id() == usize::MAX {
            Ok(&self.font_texture)
        } else if let Some(texture) = self.textures.get(texture_id) {
            Ok(texture)
        } else {
            Err(GfxRendererError::BadTexture(texture_id))
        }
    }
}

#[cfg(feature = "directx")]
mod constants {
    use gfx::gfx_constant_struct_meta;
    use gfx::gfx_impl_struct_meta;

    gfx::gfx_constant_struct! {
        Constants {
            // `matrix` is a reserved keyword in HLSL
            matrix: [[f32; 4]; 4] = "matrix_",
        }
    }
}

// This is basically what gfx_pipeline generates, but with some changes:
//
// * Data struct contains references to avoid copying
// * Everything is parameterized with BlendFormat
// * Pipeline init is specialized for our structs
mod pipeline {
    use gfx::format::BlendFormat;
    use gfx::handle::Manager;
    use gfx::preset::blend;
    use gfx::pso::{
        AccessInfo, DataBind, DataLink, Descriptor, InitError, PipelineData, PipelineInit,
        RawDataSet,
    };
    use gfx::state::ColorMask;
    use gfx::{ProgramInfo, Resources};
    use imgui::DrawVert;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Data<'a, R: Resources, Cf: BlendFormat + 'a> {
        pub vertex_buffer: &'a <gfx::VertexBuffer<DrawVert> as DataBind<R>>::Data,
        #[cfg(not(feature = "directx"))]
        pub matrix: &'a <gfx::Global<[[f32; 4]; 4]> as DataBind<R>>::Data,
        #[cfg(feature = "directx")]
        pub constants: &'a <gfx::ConstantBuffer<super::constants::Constants> as DataBind<R>>::Data,
        pub tex: &'a <gfx::TextureSampler<[f32; 4]> as DataBind<R>>::Data,
        pub target: &'a <gfx::BlendTarget<Cf> as DataBind<R>>::Data,
        pub scissor: &'a <gfx::Scissor as DataBind<R>>::Data,
    }

    #[derive(Clone, Debug, Hash, PartialEq)]
    pub struct Meta<Cf: BlendFormat> {
        vertex_buffer: gfx::VertexBuffer<DrawVert>,
        #[cfg(not(feature = "directx"))]
        matrix: gfx::Global<[[f32; 4]; 4]>,
        #[cfg(feature = "directx")]
        constants: gfx::ConstantBuffer<super::constants::Constants>,
        tex: gfx::TextureSampler<[f32; 4]>,
        target: gfx::BlendTarget<Cf>,
        scissor: gfx::Scissor,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Init<'a, Cf: BlendFormat> {
        vertex_buffer: <gfx::VertexBuffer<DrawVert> as DataLink<'a>>::Init,
        #[cfg(not(feature = "directx"))]
        matrix: <gfx::Global<[[f32; 4]; 4]> as DataLink<'a>>::Init,
        #[cfg(feature = "directx")]
        constants: <gfx::ConstantBuffer<super::constants::Constants> as DataLink<'a>>::Init,
        tex: <gfx::TextureSampler<[f32; 4]> as DataLink<'a>>::Init,
        target: <gfx::BlendTarget<Cf> as DataLink<'a>>::Init,
        scissor: <gfx::Scissor as DataLink<'a>>::Init,
    }

    impl<'a, Cf: BlendFormat> PipelineInit for Init<'a, Cf> {
        type Meta = Meta<Cf>;
        fn link_to<'s>(
            &self,
            desc: &mut Descriptor,
            info: &'s ProgramInfo,
        ) -> Result<Meta<Cf>, InitError<&'s str>> {
            let mut meta = Meta {
                vertex_buffer: DataLink::new(),
                #[cfg(not(feature = "directx"))]
                matrix: DataLink::new(),
                #[cfg(feature = "directx")]
                constants: DataLink::new(),
                tex: DataLink::new(),
                target: DataLink::new(),
                scissor: DataLink::new(),
            };
            if let Some(d) = meta
                .vertex_buffer
                .link_vertex_buffer(0, &self.vertex_buffer)
            {
                assert!(meta.vertex_buffer.is_active());
                desc.vertex_buffers[0] = Some(d);
            }
            for at in &info.vertex_attributes {
                match meta.vertex_buffer.link_input(at, &self.vertex_buffer) {
                    Some(Ok(d)) => {
                        assert!(meta.vertex_buffer.is_active());
                        desc.attributes[at.slot as usize] = Some(d);
                        continue;
                    }
                    Some(Err(fm)) => return Err(InitError::VertexImport(&at.name, Some(fm))),
                    None => return Err(InitError::VertexImport(&at.name, None)),
                }
            }
            for gc in &info.globals {
                #[cfg(not(feature = "directx"))]
                {
                    match meta.matrix.link_global_constant(gc, &self.matrix) {
                        Some(Ok(())) => assert!(meta.matrix.is_active()),
                        Some(Err(e)) => return Err(InitError::GlobalConstant(&gc.name, Some(e))),
                        None => return Err(InitError::GlobalConstant(&gc.name, None)),
                    }
                }
                #[cfg(feature = "directx")]
                {
                    match meta.constants.link_global_constant(gc, &self.constants) {
                        Some(Ok(())) => assert!(meta.constants.is_active()),
                        Some(Err(e)) => return Err(InitError::GlobalConstant(&gc.name, Some(e))),
                        None => return Err(InitError::GlobalConstant(&gc.name, None)),
                    }
                }
            }
            for srv in &info.textures {
                match meta.tex.link_resource_view(srv, &self.tex) {
                    Some(Ok(d)) => {
                        assert!(meta.tex.is_active());
                        desc.resource_views[srv.slot as usize] = Some(d);
                    }
                    Some(Err(_)) => return Err(InitError::ResourceView(&srv.name, Some(()))),
                    None => return Err(InitError::ResourceView(&srv.name, None)),
                }
            }
            for sm in &info.samplers {
                match meta.tex.link_sampler(sm, &self.tex) {
                    Some(d) => {
                        assert!(meta.tex.is_active());
                        desc.samplers[sm.slot as usize] = Some(d);
                    }
                    None => return Err(InitError::Sampler(&sm.name, None)),
                }
            }
            for out in &info.outputs {
                match meta.target.link_output(out, &self.target) {
                    Some(Ok(d)) => {
                        assert!(meta.target.is_active());
                        desc.color_targets[out.slot as usize] = Some(d);
                    }
                    Some(Err(fm)) => return Err(InitError::PixelExport(&out.name, Some(fm))),
                    None => return Err(InitError::PixelExport(&out.name, None)),
                }
            }
            if !info.knows_outputs {
                use gfx::shade::core::*;
                let mut out = OutputVar {
                    name: String::new(),
                    slot: 0,
                    base_type: BaseType::F32,
                    container: ContainerType::Vector(4),
                };
                match meta.target.link_output(&out, &self.target) {
                    Some(Ok(d)) => {
                        assert!(meta.target.is_active());
                        desc.color_targets[out.slot as usize] = Some(d);
                        out.slot += 1;
                    }
                    Some(Err(fm)) => return Err(InitError::PixelExport(&"!known", Some(fm))),
                    None => (),
                }
            }
            if meta.scissor.link_scissor() {
                assert!(meta.scissor.is_active());
                desc.scissor = true;
            }
            Ok(meta)
        }
    }

    impl<'a, R: Resources, Cf: BlendFormat> PipelineData<R> for Data<'a, R, Cf> {
        type Meta = Meta<Cf>;
        fn bake_to(
            &self,
            out: &mut RawDataSet<R>,
            meta: &Meta<Cf>,
            man: &mut Manager<R>,
            access: &mut AccessInfo<R>,
        ) {
            meta.vertex_buffer
                .bind_to(out, self.vertex_buffer, man, access);
            #[cfg(not(feature = "directx"))]
            {
                meta.matrix.bind_to(out, self.matrix, man, access);
            }
            #[cfg(feature = "directx")]
            {
                meta.constants.bind_to(out, self.constants, man, access);
            }
            meta.tex.bind_to(out, self.tex, man, access);
            meta.target.bind_to(out, self.target, man, access);
            meta.scissor.bind_to(out, self.scissor, man, access);
        }
    }

    pub fn new<Cf: BlendFormat>() -> Init<'static, Cf> {
        Init {
            vertex_buffer: (),
            #[cfg(not(feature = "directx"))]
            matrix: "matrix",
            #[cfg(feature = "directx")]
            constants: "constants",
            tex: "tex",
            target: ("Target0", ColorMask::all(), blend::ALPHA),
            scissor: (),
        }
    }
}
