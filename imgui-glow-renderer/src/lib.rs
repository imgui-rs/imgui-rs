use std::{borrow::Cow, error::Error, fmt::Display, marker::PhantomData, mem::size_of};

use imgui::{internal::RawWrapper, TextureId};

use crate::versions::{GlVersion, GlslVersion};

pub mod versions;

// TODO: document all the things!

// This may be generics overkill, but I have to assume there's some reason that
// `glow` hid the functions of `[glow::Context]` behind the `[glow::Context]`
// trait. The specified types required here are necessary because otherwise
// required initialisations or conversions in this crate can't be performed. In
// any case, the OpenGL standard specifies these types, so there should be no
// loss of generality.
pub trait Gl:
    glow::HasContext<
    Texture = glow::Texture,
    UniformLocation = glow::UniformLocation,
    Program = glow::Program,
    Buffer = glow::Buffer,
    Sampler = glow::Sampler,
    VertexArray = glow::VertexArray,
>
{
}
impl<
        G: glow::HasContext<
            Texture = glow::Texture,
            UniformLocation = glow::UniformLocation,
            Program = glow::Program,
            Buffer = glow::Buffer,
            Sampler = glow::Sampler,
            VertexArray = glow::VertexArray,
        >,
    > Gl for G
{
}

/// Convenience function to get you going quickly. In most larger programs
/// you probably want to take more control (including ownership of the GL
/// context). In those cases, construct an appropriate renderer with
/// `[RendererBuilder]`.
///
/// By default, it constructs a renderer which owns the OpenGL context and
/// attempts to backup the OpenGL state before rendering and restore it after
/// rendering.
///
/// # Errors
/// Any error initialising the OpenGL objects (including shaders) will
/// result in an error.
pub fn auto_renderer<G: Gl>(
    gl: G,
    imgui_context: &mut imgui::Context,
) -> Result<OwningRenderer<G, TrivialTextureMap, AutoShaderProvider<G>, StateBackupCsm>, InitError>
{
    RendererBuilder::new()
        .with_context_state_manager(StateBackupCsm::default())
        .build_owning(gl, imgui_context)
}

pub struct RendererBuilder<G, T, S, C>
where
    G: Gl,
    T: TextureMap,
    S: ShaderProvider<G>,
    C: ContextStateManager<G>,
{
    texture_map: T,
    shader_provider: S,
    context_state_manager: C,
    phantom_gl: PhantomData<G>,
}

impl<G: Gl> RendererBuilder<G, TrivialTextureMap, AutoShaderProvider<G>, TrivialCsm> {
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Self {
            texture_map: TrivialTextureMap(),
            shader_provider: <AutoShaderProvider<G> as Default>::default(),
            context_state_manager: TrivialCsm(),
            phantom_gl: PhantomData::default(),
        }
    }
}

impl<G, T, S, C> RendererBuilder<G, T, S, C>
where
    G: Gl,
    T: TextureMap,
    S: ShaderProvider<G>,
    C: ContextStateManager<G>,
{
    pub fn with_texture_map<T2: TextureMap>(self, texture_map: T2) -> RendererBuilder<G, T2, S, C> {
        RendererBuilder {
            texture_map,
            shader_provider: self.shader_provider,
            context_state_manager: self.context_state_manager,
            phantom_gl: self.phantom_gl,
        }
    }

    pub fn with_shader_provider<S2: ShaderProvider<G>>(
        self,
        shader_provider: S2,
    ) -> RendererBuilder<G, T, S2, C> {
        RendererBuilder {
            texture_map: self.texture_map,
            shader_provider,
            context_state_manager: self.context_state_manager,
            phantom_gl: self.phantom_gl,
        }
    }

    pub fn with_context_state_manager<C2: ContextStateManager<G>>(
        self,
        context_state_manager: C2,
    ) -> RendererBuilder<G, T, S, C2> {
        RendererBuilder {
            texture_map: self.texture_map,
            shader_provider: self.shader_provider,
            context_state_manager,
            phantom_gl: self.phantom_gl,
        }
    }
    /// Build a renderer which owns the OpenGL context (which can be borrowed
    /// from the renderer, but not taken).
    ///
    /// # Errors
    /// Any error initialising the OpenGL objects (including shaders) will
    /// result in an error.
    pub fn build_owning(
        self,
        gl: G,
        imgui_context: &mut imgui::Context,
    ) -> Result<OwningRenderer<G, T, S, C>, InitError> {
        let renderer = self.build_borrowing(&gl, imgui_context)?;
        Ok(OwningRenderer::<G, T, S, C> { gl, renderer })
    }

    /// Build a renderer which needs to borrow a context in order to render.
    ///
    /// # Errors
    /// Any error initialising the OpenGL objects (including shaders) will
    /// result in an error.
    pub fn build_borrowing(
        self,
        gl: &G,
        imgui_context: &mut imgui::Context,
    ) -> Result<Renderer<G, T, S, C>, InitError> {
        Renderer::<G, T, S, C>::initialize(
            gl,
            imgui_context,
            self.texture_map,
            self.shader_provider,
            self.context_state_manager,
        )
    }
}

/// Renderer which owns the OpenGL context. Useful for simple applications, but
/// more complicated applications may prefer to keep control of their own
/// OpenGL context, or even change that context at runtime.
///
/// OpenGL context is still available to the rest of the application through
/// the `[gl_context]` method.
pub struct OwningRenderer<G, T = TrivialTextureMap, S = AutoShaderProvider<G>, C = TrivialCsm>
where
    G: Gl,
    T: TextureMap,
    S: ShaderProvider<G>,
    C: ContextStateManager<G>,
{
    gl: G,
    renderer: Renderer<G, T, S, C>,
}

impl<G, T, S, C> OwningRenderer<G, T, S, C>
where
    G: Gl,
    T: TextureMap,
    S: ShaderProvider<G>,
    C: ContextStateManager<G>,
{
    /// Note: no need to provide a `mut` version of this, as all methods on
    /// `[glow::HasContext]` are immutable.
    #[inline]
    pub fn gl_context(&self) -> &G {
        &self.gl
    }

    #[inline]
    pub fn renderer(&self) -> &Renderer<G, T, S, C> {
        &self.renderer
    }

    /// # Errors
    /// Some OpenGL errors trigger an error (few are explicitly checked,
    /// however)
    #[inline]
    pub fn render(&mut self, draw_data: &imgui::DrawData) -> Result<(), RenderError> {
        self.renderer.render(&self.gl, draw_data)
    }
}

impl<G, T, S, C> Drop for OwningRenderer<G, T, S, C>
where
    G: Gl,
    T: TextureMap,
    S: ShaderProvider<G>,
    C: ContextStateManager<G>,
{
    fn drop(&mut self) {
        self.renderer.destroy(&self.gl);
    }
}

pub struct Renderer<G, T = TrivialTextureMap, S = AutoShaderProvider<G>, C = TrivialCsm>
where
    G: Gl,
    T: TextureMap,
    S: ShaderProvider<G>,
    C: ContextStateManager<G>,
{
    pub texture_map: T,
    pub shader_provider: S,
    pub context_state_manager: C,
    pub vbo_handle: G::Buffer,
    pub ebo_handle: G::Buffer,
    pub font_atlas_texture: G::Texture,
    #[cfg(feature = "bind_vertex_array_support")]
    pub vertex_array_object: G::VertexArray,
    pub gl_version: GlVersion,
    pub has_clip_origin_support: bool,
    pub is_destroyed: bool,
}

impl<G, T, S, C> Renderer<G, T, S, C>
where
    G: Gl,
    T: TextureMap,
    S: ShaderProvider<G>,
    C: ContextStateManager<G>,
{
    /// # Errors
    /// Any error initialising the OpenGL objects (including shaders) will
    /// result in an error.
    pub fn initialize(
        gl: &G,
        imgui_context: &mut imgui::Context,
        texture_map: T,
        shader_provider: S,
        context_state_manager: C,
    ) -> Result<Self, InitError> {
        #![allow(
            clippy::similar_names,
            clippy::cast_sign_loss,
            clippy::shadow_unrelated
        )]

        let gl_version = GlVersion::read(gl);

        #[cfg(feature = "clip_origin_support")]
        let has_clip_origin_support = {
            let support = gl_version.clip_origin_support();

            #[cfg(feature = "gl_extensions_support")]
            if support {
                support
            } else {
                let extensions_count = unsafe { gl.get_parameter_i32(glow::NUM_EXTENSIONS) } as u32;
                (0..extensions_count).any(|index| {
                    let extension_name =
                        unsafe { gl.get_parameter_indexed_string(glow::EXTENSIONS, index) };
                    extension_name == "GL_ARB_clip_control"
                })
            }
            #[cfg(not(feature = "gl_extensions_support"))]
            support
        };
        #[cfg(not(feature = "clip_origin_support"))]
        let has_clip_origin_support = false;

        let mut context_state_manager = context_state_manager;
        context_state_manager.pre_init(gl, gl_version)?;

        let font_atlas_texture = prepare_font_atlas(gl, imgui_context.fonts())?;

        let mut shader_provider = shader_provider;
        shader_provider.initialize(gl, gl_version)?;
        let vbo_handle = unsafe { gl.create_buffer() }.map_err(InitError::CreateBufferObject)?;
        let ebo_handle = unsafe { gl.create_buffer() }.map_err(InitError::CreateBufferObject)?;

        context_state_manager.post_init(gl, gl_version)?;

        let out = Self {
            texture_map,
            shader_provider,
            context_state_manager,
            vbo_handle,
            ebo_handle,
            font_atlas_texture,
            #[cfg(feature = "bind_vertex_array_support")]
            vertex_array_object: 0,
            gl_version,
            has_clip_origin_support,
            is_destroyed: false,
        };

        // Leave this until the end of the function to avoid changing state if
        // there was ever an error above
        out.configure_imgui_context(imgui_context);

        Ok(out)
    }

    pub fn destroy(&mut self, gl: &G) {
        if self.is_destroyed {
            return;
        }

        let gl_version = self.gl_version;
        self.context_state_manager.pre_destroy(gl, gl_version);

        if self.vbo_handle != 0 {
            unsafe { gl.delete_buffer(self.vbo_handle) };
            self.vbo_handle = 0;
        }
        if self.ebo_handle != 0 {
            unsafe { gl.delete_buffer(self.vbo_handle) };
            self.ebo_handle = 0;
        }
        let program = self.shader_provider.data().program;
        if program != 0 {
            unsafe { gl.delete_program(program) };
        }
        if self.font_atlas_texture != 0 {
            unsafe { gl.delete_texture(self.font_atlas_texture) };
            self.font_atlas_texture = 0;
        }

        self.context_state_manager.post_destroy(gl, gl_version);

        self.is_destroyed = true;
    }

    /// # Errors
    /// Some OpenGL errors trigger an error (few are explicitly checked,
    /// however)
    pub fn render(&mut self, gl: &G, draw_data: &imgui::DrawData) -> Result<(), RenderError> {
        if self.is_destroyed {
            return Err(Self::renderer_destroyed());
        }

        let fb_width = draw_data.display_size[0] * draw_data.framebuffer_scale[0];
        let fb_height = draw_data.display_size[1] * draw_data.framebuffer_scale[1];
        if !(fb_width > 0.0 && fb_height > 0.0) {
            return Ok(());
        }

        gl_debug_message(gl, "imgui-rs-glow: start render");
        self.context_state_manager.pre_render(gl, self.gl_version)?;

        self.set_up_render_state(gl, draw_data, fb_width, fb_height)?;

        gl_debug_message(gl, "start loop over draw lists");
        for draw_list in draw_data.draw_lists() {
            unsafe {
                gl.buffer_data_u8_slice(
                    glow::ARRAY_BUFFER,
                    to_byte_slice(draw_list.vtx_buffer()),
                    glow::STREAM_DRAW,
                );
                gl.buffer_data_u8_slice(
                    glow::ELEMENT_ARRAY_BUFFER,
                    to_byte_slice(draw_list.idx_buffer()),
                    glow::STREAM_DRAW,
                );
            }

            gl_debug_message(gl, "start loop over commands");
            for command in draw_list.commands() {
                match command {
                    imgui::DrawCmd::Elements { count, cmd_params } => {
                        self.render_elements(gl, count, cmd_params, draw_data, fb_width, fb_height)
                    }
                    imgui::DrawCmd::RawCallback { callback, raw_cmd } => unsafe {
                        callback(draw_list.raw(), raw_cmd)
                    },
                    imgui::DrawCmd::ResetRenderState => {
                        self.set_up_render_state(gl, draw_data, fb_width, fb_height)?
                    }
                }
            }
        }

        #[cfg(feature = "bind_vertex_array_support")]
        if self.gl_version.bind_vertex_array_support() {
            unsafe { gl.delete_vertex_array(self.vertex_array_object) };
        }

        self.context_state_manager
            .post_render(gl, self.gl_version)?;
        gl_debug_message(gl, "imgui-rs-glow: complete render");
        Ok(())
    }

    /// # Errors
    /// Few GL calls are checked for errors, but any that are found will result
    /// in an error. Errors from the state manager lifecycle callbacks will also
    /// result in an error.
    pub fn set_up_render_state(
        &mut self,
        gl: &G,
        draw_data: &imgui::DrawData,
        fb_width: f32,
        fb_height: f32,
    ) -> Result<(), RenderError> {
        #![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]

        if self.is_destroyed {
            return Err(Self::renderer_destroyed());
        }

        self.context_state_manager
            .pre_setup_render(gl, self.gl_version)?;

        unsafe {
            gl.active_texture(glow::TEXTURE0);
            gl.enable(glow::BLEND);
            gl.blend_equation(glow::FUNC_ADD);
            gl.blend_func_separate(
                glow::SRC_ALPHA,
                glow::ONE_MINUS_SRC_ALPHA,
                glow::ONE,
                glow::ONE_MINUS_SRC_ALPHA,
            );
            gl.disable(glow::CULL_FACE);
            gl.disable(glow::DEPTH_TEST);
            gl.disable(glow::STENCIL_TEST);
            gl.enable(glow::SCISSOR_TEST);

            #[cfg(feature = "primitive_restart_support")]
            if self.gl_version.primitive_restart_support() {
                gl.disable(glow::PRIMITIVE_RESTART);
            }

            #[cfg(feature = "polygon_mode_support")]
            if self.gl_version.polygon_mode_support() {
                gl.polygon_mode(glow::FRONT_AND_BACK, glow::FILL);
            }

            gl.viewport(0, 0, fb_width as _, fb_height as _);
        }

        #[cfg(feature = "clip_origin_support")]
        let clip_origin_is_lower_left = if self.has_clip_origin_support {
            unsafe { gl.get_parameter_i32(glow::CLIP_ORIGIN) != glow::UPPER_LEFT as i32 }
        } else {
            true
        };
        #[cfg(not(feature = "clip_origin_support"))]
        let clip_origin_is_lower_left = true;

        let projection_matrix = calculate_matrix(draw_data, clip_origin_is_lower_left);
        let shader_data = self.shader_provider.data();

        unsafe {
            gl.use_program(Some(shader_data.program));
            gl.uniform_1_i32(Some(&shader_data.texture_uniform_location), 0);
            gl.uniform_matrix_4_f32_slice(
                Some(&shader_data.matrix_uniform_location),
                false,
                &projection_matrix,
            );
        }

        #[cfg(feature = "bind_sampler_support")]
        if self.gl_version.bind_sampler_support() {
            unsafe { gl.bind_sampler(0, None) };
        }

        #[cfg(feature = "bind_vertex_array_support")]
        if self.gl_version.bind_vertex_array_support() {
            unsafe {
                self.vertex_array_object = gl
                    .create_vertex_array()
                    .map_err(|err| format!("Error creating vertex array object: {}", err))?;
                gl.bind_vertex_array(Some(self.vertex_array_object));
            }
        }

        // TODO: soon it should be possible for these to be `const` functions
        let position_field_offset = memoffset::offset_of!(imgui::DrawVert, pos) as _;
        let uv_field_offset = memoffset::offset_of!(imgui::DrawVert, uv) as _;
        let color_field_offset = memoffset::offset_of!(imgui::DrawVert, col) as _;

        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo_handle));
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo_handle));
            gl.enable_vertex_attrib_array(shader_data.position_attribute_index);
            gl.vertex_attrib_pointer_f32(
                shader_data.position_attribute_index,
                2,
                glow::FLOAT,
                false,
                size_of::<imgui::DrawVert>() as _,
                position_field_offset,
            );
            gl.enable_vertex_attrib_array(shader_data.uv_attribute_index);
            gl.vertex_attrib_pointer_f32(
                shader_data.uv_attribute_index,
                2,
                glow::FLOAT,
                false,
                size_of::<imgui::DrawVert>() as _,
                uv_field_offset,
            );
            gl.enable_vertex_attrib_array(shader_data.color_attribute_index);
            gl.vertex_attrib_pointer_f32(
                shader_data.color_attribute_index,
                4,
                glow::UNSIGNED_BYTE,
                true,
                size_of::<imgui::DrawVert>() as _,
                color_field_offset,
            );
        }

        self.context_state_manager
            .post_setup_render(gl, self.gl_version)
    }

    fn render_elements(
        &self,
        gl: &G,
        element_count: usize,
        element_params: imgui::DrawCmdParams,
        draw_data: &imgui::DrawData,
        fb_width: f32,
        fb_height: f32,
    ) {
        #![allow(
            clippy::similar_names,
            clippy::cast_possible_truncation,
            clippy::cast_possible_wrap
        )]

        let imgui::DrawCmdParams {
            clip_rect,
            texture_id,
            vtx_offset,
            idx_offset,
        } = element_params;
        let clip_off = draw_data.display_pos;
        let scale = draw_data.framebuffer_scale;

        let clip_x1 = (clip_rect[0] - clip_off[0]) * scale[0];
        let clip_y1 = (clip_rect[1] - clip_off[1]) * scale[1];
        let clip_x2 = (clip_rect[2] - clip_off[0]) * scale[0];
        let clip_y2 = (clip_rect[3] - clip_off[1]) * scale[1];

        if clip_x1 >= fb_width || clip_y1 >= fb_height || clip_x2 < 0.0 || clip_y2 < 0.0 {
            return;
        }

        unsafe {
            gl.scissor(
                clip_x1 as i32,
                (fb_height - clip_y2) as i32,
                (clip_x2 - clip_x1) as i32,
                (clip_y2 - clip_y1) as i32,
            );
            gl.bind_texture(glow::TEXTURE_2D, self.texture_map.gl_texture(texture_id));

            #[cfg(feature = "vertex_offset_support")]
            let with_offset = self.gl_version.vertex_offset_support();
            #[cfg(not(feature = "vertex_offset_support"))]
            let with_offset = false;

            if with_offset {
                gl.draw_elements_base_vertex(
                    glow::TRIANGLES,
                    element_count as _,
                    imgui_index_type_as_gl(),
                    (idx_offset * size_of::<imgui::DrawIdx>()) as _,
                    vtx_offset as _,
                );
            } else {
                gl.draw_elements(
                    glow::TRIANGLES,
                    element_count as _,
                    imgui_index_type_as_gl(),
                    (idx_offset * size_of::<imgui::DrawIdx>()) as _,
                );
            }
        }
    }

    fn configure_imgui_context(&self, imgui_context: &mut imgui::Context) {
        imgui_context.set_renderer_name(Some(
            format!("imgui-rs-glow-render {}", env!("CARGO_PKG_VERSION")).into(),
        ));

        #[cfg(feature = "vertex_offset_support")]
        if self.gl_version.vertex_offset_support() {
            imgui_context
                .io_mut()
                .backend_flags
                .insert(imgui::BackendFlags::RENDERER_HAS_VTX_OFFSET);
        }
    }

    fn renderer_destroyed() -> RenderError {
        "Renderer is destroyed".into()
    }
}

pub trait TextureMap {
    fn gl_texture(&self, imgui_texture: imgui::TextureId) -> Option<glow::Texture>;
}

#[derive(Default)]
pub struct TrivialTextureMap();

impl TextureMap for TrivialTextureMap {
    fn gl_texture(&self, imgui_texture: imgui::TextureId) -> Option<glow::Texture> {
        #[allow(clippy::cast_possible_truncation)]
        Some(imgui_texture.id() as _)
    }
}

pub trait ContextStateManager<G: Gl> {
    #![allow(unused_variables, clippy::missing_errors_doc)]

    fn pre_init(&mut self, gl: &G, gl_version: GlVersion) -> Result<(), InitError> {
        Ok(())
    }

    fn post_init(&mut self, gl: &G, gl_version: GlVersion) -> Result<(), InitError> {
        Ok(())
    }

    fn pre_render(&mut self, gl: &G, gl_version: GlVersion) -> Result<(), RenderError> {
        Ok(())
    }

    fn pre_setup_render(&mut self, gl: &G, gl_version: GlVersion) -> Result<(), RenderError> {
        Ok(())
    }

    fn post_setup_render(&mut self, gl: &G, gl_version: GlVersion) -> Result<(), RenderError> {
        Ok(())
    }

    fn post_render(&mut self, gl: &G, gl_version: GlVersion) -> Result<(), RenderError> {
        Ok(())
    }

    fn pre_destroy(&mut self, gl: &G, gl_version: GlVersion) {}

    fn post_destroy(&mut self, gl: &G, gl_version: GlVersion) {}
}

#[derive(Default)]
pub struct TrivialCsm();

impl<G: Gl> ContextStateManager<G> for TrivialCsm {}

/// This `[ContextStateManager]` is based on the upstream OpenGL example from
/// imgui, where an attempt is made to save and restore the OpenGL context state
/// before and after rendering.
///
/// It is unlikely that any such attempt will be comprehensive for all possible
/// applications, due to the complexity of OpenGL and the possibility of
/// arbitrary extensions. However, it remains as a useful tool for quickly
/// getting started, and a good example of how to use a `[ContextStateManager]` to
/// customise the renderer.
#[allow(clippy::struct_excessive_bools)]
#[derive(Default)]
pub struct StateBackupCsm {
    active_texture: i32,
    program: i32,
    texture: i32,
    #[cfg(feature = "bind_sampler_support")]
    sampler: Option<i32>,
    array_buffer: i32,
    #[cfg(feature = "polygon_mode_support")]
    polygon_mode: Option<[i32; 2]>,
    viewport: [i32; 4],
    scissor_box: [i32; 4],
    blend_src_rgb: i32,
    blend_dst_rgb: i32,
    blend_src_alpha: i32,
    blend_dst_alpha: i32,
    blend_equation_rgb: i32,
    blend_equation_alpha: i32,
    blend_enabled: bool,
    cull_face_enabled: bool,
    depth_test_enabled: bool,
    stencil_test_enabled: bool,
    scissor_test_enabled: bool,
    #[cfg(feature = "primitive_restart_support")]
    primitive_restart_enabled: Option<bool>,
    #[cfg(feature = "bind_vertex_array_support")]
    vertex_array_object: Option<glow::VertexArray>,
}

impl<G: Gl> ContextStateManager<G> for StateBackupCsm {
    fn pre_init(&mut self, gl: &G, _gl_version: GlVersion) -> Result<(), InitError> {
        self.texture = unsafe { gl.get_parameter_i32(glow::TEXTURE_BINDING_2D) };
        Ok(())
    }

    fn post_init(&mut self, gl: &G, _gl_version: GlVersion) -> Result<(), InitError> {
        #[allow(clippy::clippy::cast_sign_loss)]
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(self.texture as _));
        }
        Ok(())
    }

    fn pre_render(&mut self, gl: &G, gl_version: GlVersion) -> Result<(), RenderError> {
        #[allow(clippy::cast_sign_loss)]
        unsafe {
            self.active_texture = gl.get_parameter_i32(glow::ACTIVE_TEXTURE);
            self.program = gl.get_parameter_i32(glow::CURRENT_PROGRAM);
            self.texture = gl.get_parameter_i32(glow::TEXTURE_BINDING_2D);
            #[cfg(feature = "bind_sampler_support")]
            if gl_version.bind_sampler_support() {
                self.sampler = Some(gl.get_parameter_i32(glow::SAMPLER_BINDING));
            } else {
                self.sampler = None;
            }
            self.array_buffer = gl.get_parameter_i32(glow::ARRAY_BUFFER_BINDING);

            #[cfg(feature = "bind_vertex_array_support")]
            if gl_version.bind_vertex_array_support() {
                self.vertex_array_object =
                    Some(gl.get_parameter_i32(glow::VERTEX_ARRAY_BINDING) as _);
            }

            #[cfg(feature = "polygon_mode_support")]
            if gl_version.polygon_mode_support() {
                if self.polygon_mode.is_none() {
                    self.polygon_mode = Some(Default::default());
                }
                gl.get_parameter_i32_slice(glow::POLYGON_MODE, self.polygon_mode.as_mut().unwrap());
            } else {
                self.polygon_mode = None;
            }
            gl.get_parameter_i32_slice(glow::VIEWPORT, &mut self.viewport);
            gl.get_parameter_i32_slice(glow::SCISSOR_BOX, &mut self.scissor_box);
            self.blend_src_rgb = gl.get_parameter_i32(glow::BLEND_SRC_RGB);
            self.blend_dst_rgb = gl.get_parameter_i32(glow::BLEND_DST_RGB);
            self.blend_src_alpha = gl.get_parameter_i32(glow::BLEND_SRC_ALPHA);
            self.blend_dst_alpha = gl.get_parameter_i32(glow::BLEND_DST_ALPHA);
            self.blend_equation_rgb = gl.get_parameter_i32(glow::BLEND_EQUATION_RGB);
            self.blend_equation_alpha = gl.get_parameter_i32(glow::BLEND_EQUATION_ALPHA);
            self.blend_enabled = gl.is_enabled(glow::BLEND);
            self.cull_face_enabled = gl.is_enabled(glow::CULL_FACE);
            self.depth_test_enabled = gl.is_enabled(glow::DEPTH_TEST);
            self.stencil_test_enabled = gl.is_enabled(glow::STENCIL_TEST);
            self.scissor_test_enabled = gl.is_enabled(glow::SCISSOR_TEST);
            #[cfg(feature = "primitive_restart_support")]
            if gl_version.primitive_restart_support() {
                self.primitive_restart_enabled = Some(gl.is_enabled(glow::PRIMITIVE_RESTART));
            } else {
                self.primitive_restart_enabled = None;
            }
        }
        Ok(())
    }

    fn post_render(&mut self, gl: &G, _gl_version: GlVersion) -> Result<(), RenderError> {
        #![allow(clippy::cast_sign_loss)]
        unsafe {
            gl.use_program(Some(self.program as _));
            gl.bind_texture(glow::TEXTURE_2D, Some(self.texture as _));
            #[cfg(feature = "bind_sampler_support")]
            if let Some(sampler) = self.sampler {
                gl.bind_sampler(0, Some(sampler as _));
            }
            gl.active_texture(self.active_texture as _);
            #[cfg(feature = "bind_vertex_array_support")]
            if let Some(vao) = self.vertex_array_object {
                gl.bind_vertex_array(Some(vao));
            }
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.array_buffer as _));
            gl.blend_equation_separate(
                self.blend_equation_rgb as _,
                self.blend_equation_alpha as _,
            );
            gl.blend_func_separate(
                self.blend_src_rgb as _,
                self.blend_dst_rgb as _,
                self.blend_src_alpha as _,
                self.blend_dst_alpha as _,
            );
            if self.blend_enabled {
                gl.enable(glow::BLEND)
            } else {
                gl.disable(glow::BLEND);
            }
            if self.cull_face_enabled {
                gl.enable(glow::CULL_FACE)
            } else {
                gl.disable(glow::CULL_FACE)
            }
            if self.depth_test_enabled {
                gl.enable(glow::DEPTH_TEST)
            } else {
                gl.disable(glow::DEPTH_TEST)
            }
            if self.stencil_test_enabled {
                gl.enable(glow::STENCIL_TEST)
            } else {
                gl.disable(glow::STENCIL_TEST)
            }
            if self.scissor_test_enabled {
                gl.enable(glow::SCISSOR_TEST)
            } else {
                gl.disable(glow::SCISSOR_TEST)
            }
            #[cfg(feature = "primitive_restart_support")]
            if let Some(restart_enabled) = self.primitive_restart_enabled {
                if restart_enabled {
                    gl.enable(glow::PRIMITIVE_RESTART)
                } else {
                    gl.disable(glow::PRIMITIVE_RESTART)
                }
            }
            #[cfg(feature = "polygon_mode_support")]
            if let Some([mode, _]) = self.polygon_mode {
                gl.polygon_mode(glow::FRONT_AND_BACK, mode as _);
            }
            gl.viewport(
                self.viewport[0],
                self.viewport[1],
                self.viewport[2],
                self.viewport[3],
            );
            gl.scissor(
                self.scissor_box[0],
                self.scissor_box[1],
                self.scissor_box[2],
                self.scissor_box[3],
            );
        }
        Ok(())
    }
}

pub trait ShaderProvider<G: Gl> {
    /// Called during renderer initialization, before this call the shader
    /// provide should be in a neutral state and have not interacted with the
    /// OpenGL context.
    ///
    /// Implementors should use this opporunity to check whether the version of
    /// the GL context (found with `[GlVersion::read]`) is compatible with the
    /// shader they provide.
    ///
    /// # Errors
    /// Any error creating the GL objects, compiling or linking or loading the
    /// shaders, or an GL context with an incompatible OpenGL version will
    /// result in an error.
    fn initialize(&mut self, gl: &G, gl_version: GlVersion) -> Result<(), ShaderError>;

    fn data(&self) -> &GenericShaderData<G>;
}

/// A generic shader provider that parses `GL_VERSION` and
/// `GL_SHADING_LANGUAGE_VERSION` at runtime in order to generate shaders which
/// should work on a wide variety of modern devices (GL >= 3.3 and GLES >= 2.0
/// are expected to work).
pub struct AutoShaderProvider<G: Gl>(GenericShaderData<G>);

impl<G: Gl> ShaderProvider<G> for AutoShaderProvider<G> {
    fn initialize(&mut self, gl: &G, gl_version: GlVersion) -> Result<(), ShaderError> {
        const VERTEX_BODY: &str = r#"
layout (location = 0) in vec2 position;
layout (location = 1) in vec2 uv;
layout (location = 2) in vec4 color;

uniform mat4 matrix;
out vec2 fragment_uv;
out vec4 fragment_color;

void main() {
    fragment_uv = uv;
    fragment_color = color;
    gl_Position = matrix * vec4(position.xy, 0, 1);
}
"#;
        const FRAGMENT_BODY: &str = r#"
in vec2 fragment_uv;
in vec4 fragment_color;

uniform sampler2D tex;
layout (location = 0) out vec4 out_color;

void main() {
    out_color = fragment_color * texture(tex, fragment_uv.st);
}
"#;

        let glsl_version = GlslVersion::read(gl);

        // Find the lowest common denominator version
        let is_gles = gl_version.is_gles || glsl_version.is_gles;
        let (major, minor) = if let std::cmp::Ordering::Less = gl_version
            .major
            .cmp(&glsl_version.major)
            .then(gl_version.minor.cmp(&glsl_version.minor))
        {
            (gl_version.major, gl_version.minor)
        } else {
            (glsl_version.major, glsl_version.minor)
        };

        if is_gles && major < 2 {
            return Err(ShaderError::IncompatibleVersion(format!(
                "This auto-shader OpenGL version 3.0 or OpenGL ES version 2.0 or higher, found: ES {}.{}",
                major, minor
            )));
        }
        if !is_gles && major < 3 {
            return Err(ShaderError::IncompatibleVersion(format!(
                "This auto-shader OpenGL version 3.0 or OpenGL ES version 2.0 or higher, found: {}.{}",
                major, minor
            )));
        }

        let vertex_source = format!(
            "#version {major}{minor}{es_extras}\n{body}",
            major = major,
            minor = minor * 10,
            es_extras = if is_gles {
                " es\nprecision mediump float;"
            } else {
                ""
            },
            body = VERTEX_BODY,
        );
        let fragment_source = format!(
            "#version {major}{minor}{es_extras}\n{body}",
            major = major,
            minor = minor * 10,
            es_extras = if is_gles {
                " es\nprecision mediump float;"
            } else {
                ""
            },
            body = FRAGMENT_BODY,
        );

        create_shaders(&mut self.0, gl, &vertex_source, &fragment_source)
    }

    fn data(&self) -> &GenericShaderData<G> {
        &self.0
    }
}

impl<G: Gl> Default for AutoShaderProvider<G> {
    fn default() -> Self {
        Self(<GenericShaderData<G> as Default>::default())
    }
}

/// A shader provider for specific shaders for OpenGL ES (GLSL ES) version(s)
/// 3.0
#[derive(Default)]
pub struct Es3ShaderProvider<G: Gl>(GenericShaderData<G>);

impl<G: Gl> ShaderProvider<G> for Es3ShaderProvider<G> {
    fn initialize(&mut self, gl: &G, gl_version: GlVersion) -> Result<(), ShaderError> {
        const VERTEX_SOURCE: &str = r#"#version 300 es
precision mediump float;

layout (location = 0) in vec2 position;
layout (location = 1) in vec2 uv;
layout (location = 2) in vec4 color;

uniform mat4 matrix;
out vec2 fragment_uv;
out vec4 fragment_color;

void main() {
    fragment_uv = uv;
    fragment_color = color;
    gl_Position = matrix * vec4(position.xy, 0, 1);
}
"#;
        const FRAGMENT_SOURCE: &str = r#"#version 300 es
precision mediump float;

in vec2 fragment_uv;
in vec4 fragment_color;

uniform sampler2D tex;
layout (location = 0) out vec4 out_color;

void main() {
    out_color = fragment_color * texture(tex, fragment_uv.st);
}
"#;

        if !gl_version.is_gles {
            return Err(ShaderError::IncompatibleVersion(format!(
                "A version of OpenGL ES is required for this shader, found: {}.{}",
                gl_version.major, gl_version.minor
            )));
        }
        if gl_version < GlVersion::gles(3, 0) {
            return Err(ShaderError::IncompatibleVersion(format!(
                "This shader requires OpenGL ES version 3.0 or higher, found: ES {}.{}",
                gl_version.major, gl_version.minor
            )));
        }

        create_shaders(&mut self.0, gl, VERTEX_SOURCE, FRAGMENT_SOURCE)
    }

    fn data(&self) -> &GenericShaderData<G> {
        &self.0
    }
}

pub struct GenericShaderData<G: Gl> {
    pub program: G::Program,
    pub texture_uniform_location: G::UniformLocation,
    pub matrix_uniform_location: G::UniformLocation,
    pub position_attribute_index: u32,
    pub uv_attribute_index: u32,
    pub color_attribute_index: u32,
}

impl<G: Gl> Default for GenericShaderData<G> {
    fn default() -> Self {
        Self {
            program: Default::default(),
            texture_uniform_location: Default::default(),
            matrix_uniform_location: Default::default(),
            position_attribute_index: Default::default(),
            uv_attribute_index: Default::default(),
            color_attribute_index: Default::default(),
        }
    }
}

/// # Errors
/// Any error creating OpenGL objects, compiling, or linking the given shaders
/// results in an error.
pub fn create_shaders<G: Gl>(
    data: &mut GenericShaderData<G>,
    gl: &G,
    vertex_source: &str,
    fragment_source: &str,
) -> Result<(), ShaderError> {
    let vertex_shader =
        unsafe { gl.create_shader(glow::VERTEX_SHADER) }.map_err(ShaderError::CreateShader)?;
    unsafe {
        gl.shader_source(vertex_shader, vertex_source);
        gl.compile_shader(vertex_shader);
        if !gl.get_shader_compile_status(vertex_shader) {
            return Err(ShaderError::CompileShader(
                gl.get_shader_info_log(vertex_shader),
            ));
        }
    }

    let fragment_shader =
        unsafe { gl.create_shader(glow::FRAGMENT_SHADER) }.map_err(ShaderError::CreateShader)?;
    unsafe {
        gl.shader_source(fragment_shader, fragment_source);
        gl.compile_shader(fragment_shader);
        if !gl.get_shader_compile_status(fragment_shader) {
            return Err(ShaderError::CompileShader(
                gl.get_shader_info_log(fragment_shader),
            ));
        }
    }

    let program = unsafe { gl.create_program() }.map_err(ShaderError::CreateProgram)?;
    unsafe {
        gl.attach_shader(program, vertex_shader);
        gl.attach_shader(program, fragment_shader);
        gl.link_program(program);

        if !gl.get_program_link_status(program) {
            return Err(ShaderError::LinkProgram(gl.get_program_info_log(program)));
        }

        gl.detach_shader(program, vertex_shader);
        gl.detach_shader(program, fragment_shader);
        gl.delete_shader(vertex_shader);
        gl.delete_shader(fragment_shader);
    }

    data.program = program;
    unsafe {
        data.texture_uniform_location = gl
            .get_uniform_location(program, "tex")
            .ok_or_else(|| ShaderError::UniformNotFound("tex".into()))?;
        data.matrix_uniform_location = gl
            .get_uniform_location(program, "matrix")
            .ok_or_else(|| ShaderError::UniformNotFound("matrix".into()))?;
        data.position_attribute_index = gl
            .get_attrib_location(program, "position")
            .ok_or_else(|| ShaderError::AttributeNotFound("position".into()))?;
        data.uv_attribute_index = gl
            .get_attrib_location(program, "uv")
            .ok_or_else(|| ShaderError::AttributeNotFound("uv".into()))?;
        data.color_attribute_index = gl
            .get_attrib_location(program, "color")
            .ok_or_else(|| ShaderError::AttributeNotFound("color".into()))?;
    }

    Ok(())
}

#[derive(Debug)]
pub enum ShaderError {
    IncompatibleVersion(String),
    CreateShader(String),
    CreateProgram(String),
    CompileShader(String),
    LinkProgram(String),
    UniformNotFound(Cow<'static, str>),
    AttributeNotFound(Cow<'static, str>),
}

impl Error for ShaderError {}

impl Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncompatibleVersion(msg) => write!(
                f,
                "Shader not compatible with OpenGL version found in the context: {}",
                msg
            ),
            Self::CreateShader(msg) => write!(f, "Error creating shader object: {}", msg),
            Self::CreateProgram(msg) => write!(f, "Error creating program object: {}", msg),
            Self::CompileShader(msg) => write!(f, "Error compiling shader: {}", msg),
            Self::LinkProgram(msg) => write!(f, "Error linking shader program: {}", msg),
            Self::UniformNotFound(uniform_name) => {
                write!(f, "Uniform `{}` not found in shader program", uniform_name)
            }
            Self::AttributeNotFound(attribute_name) => {
                write!(
                    f,
                    "Attribute `{}` not found in shader program",
                    attribute_name
                )
            }
        }
    }
}

#[derive(Debug)]
pub enum InitError {
    Shader(ShaderError),
    CreateBufferObject(String),
    CreateTexture(String),
    UserError(String),
}

impl Error for InitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Shader(error) => Some(error),
            _ => None,
        }
    }
}

impl Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shader(error) => write!(f, "Shader initialisation error: {}", error),
            Self::CreateBufferObject(msg) => write!(f, "Error creating buffer object: {}", msg),
            Self::CreateTexture(msg) => write!(f, "Error creating texture object: {}", msg),
            Self::UserError(msg) => write!(f, "Initialization error: {}", msg),
        }
    }
}

impl From<ShaderError> for InitError {
    fn from(error: ShaderError) -> Self {
        Self::Shader(error)
    }
}

pub type RenderError = String;

fn prepare_font_atlas<G: Gl>(
    gl: &G,
    mut fonts: imgui::FontAtlasRefMut,
) -> Result<G::Texture, InitError> {
    #![allow(clippy::cast_possible_wrap)]

    let atlas_texture = fonts.build_rgba32_texture();

    let gl_texture = unsafe { gl.create_texture() }.map_err(InitError::CreateTexture)?;

    unsafe {
        gl.bind_texture(glow::TEXTURE_2D, Some(gl_texture));
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MIN_FILTER,
            glow::LINEAR as _,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MAG_FILTER,
            glow::LINEAR as _,
        );
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGBA as _,
            atlas_texture.width as _,
            atlas_texture.height as _,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            Some(atlas_texture.data),
        )
    }

    fonts.tex_id = TextureId::new(gl_texture as _);

    Ok(gl_texture)
}

fn gl_debug_message<G: glow::HasContext>(context: &G, message: impl AsRef<str>) {
    unsafe {
        context.debug_message_insert(
            glow::DEBUG_SOURCE_APPLICATION,
            glow::DEBUG_TYPE_MARKER,
            0,
            glow::DEBUG_SEVERITY_NOTIFICATION,
            message,
        )
    };
}

fn calculate_matrix(draw_data: &imgui::DrawData, clip_origin_is_lower_left: bool) -> [f32; 16] {
    #![allow(clippy::deprecated_cfg_attr)]

    let left = draw_data.display_pos[0];
    let right = draw_data.display_pos[0] + draw_data.display_size[0];
    let top = draw_data.display_pos[1];
    let bottom = draw_data.display_pos[1] + draw_data.display_size[1];

    #[cfg(feature = "clip_origin_support")]
    let (top, bottom) = if clip_origin_is_lower_left {
        (top, bottom)
    } else {
        (bottom, top)
    };

    #[cfg_attr(rustfmt, rustfmt::skip)]
    {
        [
        2.0 / (right - left)           , 0.0                            , 0.0 , 0.0,
        0.0                            , (2.0 / (top - bottom))         , 0.0 , 0.0,
        0.0                            , 0.0                            , -1.0, 0.0,
        (right + left) / (left - right), (top + bottom) / (bottom - top), 0.0 , 1.0,
        ]
    }
}

unsafe fn to_byte_slice<T>(slice: &[T]) -> &[u8] {
    std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len() * size_of::<T>())
}

const fn imgui_index_type_as_gl() -> u32 {
    match size_of::<imgui::DrawIdx>() {
        1 => glow::UNSIGNED_BYTE,
        2 => glow::UNSIGNED_SHORT,
        _ => glow::UNSIGNED_INT,
    }
}
