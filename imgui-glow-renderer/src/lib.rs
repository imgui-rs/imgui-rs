//! Renderer for [`imgui-rs`][imgui] using the [`glow`] library for OpenGL.
//!
//! This is heavily influenced by the
//! [example from upstream](https://github.com/ocornut/imgui/blob/fe245914114588f272b0924538fdd43f6c127a26/backends/imgui_impl_opengl3.cpp).
//!
//! # Basic usage
//!
//! A few code [examples] are provided in the source.
//!
//! [examples]: https://github.com/imgui-rs/imgui-rs/tree/main/imgui-glow-renderer/examples
//!
//! In short, create either an [`AutoRenderer`] (for basic usage) or [`Renderer`]
//! (for slightly more customizable operation), then call the `render(...)`
//! method with draw data from [`imgui`].
//!
//! # OpenGL (ES) versions
//!
//! This renderer is expected to work with OpenGL version 3.3 and above, and
//! OpenGL ES version 3.0 or above. This should cover the vast majority of even
//! fairly dated hardware. Please submit an issue for any incompatibilities
//! found with these OpenGL versions, pull requests to extend support to earlier
//! versions are welcomed.
//!
//! # Scope
//!
//! Consider this an example renderer. It is intended to be sufficent for simple
//! applications running imgui-rs as the final rendering step. If your application
//! has more specific needs, it's probably best to write your own renderer, in
//! which case this can be a useful starting point. This renderer is also not
//! foolproof (largely due to the global nature of the OpenGL state). For example,
//! a few "internal" functions are marked `pub` to allow the user more
//! fine-grained control at the cost of allowing potential rendering errors.
//!
//! # sRGB
//!
//! When outputting colors to a screen, colors need to be converted from a
//! linear color space to a non-linear space matching the monitor (e.g. sRGB).
//! When using the [`AutoRenderer`], this library will convert colors to sRGB
//! as a step in the shader. When initialising a [`Renderer`], you can choose
//! whether or not to include this step in the shader or not when calling
//! [`Renderer::initialize`].
//!
//! This library also assumes that textures have their internal format
//! set appropriately when uploaded to OpenGL. That is, assuming your texture
//! is sRGB (if you don't know, it probably is) the `internal_format` is
//! one of the `SRGB*` values.

use std::{borrow::Cow, error::Error, fmt::Display, mem::size_of, num::NonZeroU32, rc::Rc};

use imgui::{internal::RawWrapper, DrawCmd, DrawData, DrawVert};

use crate::versions::{GlVersion, GlslVersion};

// Re-export glow to make it easier for users to use the correct version.
pub use glow;
use glow::{Context, HasContext};

pub mod versions;

pub type GlBuffer = <Context as HasContext>::Buffer;
pub type GlTexture = <Context as HasContext>::Texture;
pub type GlVertexArray = <Context as HasContext>::VertexArray;
type GlProgram = <Context as HasContext>::Program;
type GlUniformLocation = <Context as HasContext>::UniformLocation;

/// Renderer which owns the OpenGL context and handles textures itself. Also
/// converts all output colors to sRGB for display. Useful for simple applications,
/// but more complicated applications may prefer to use [`Renderer`], or even
/// write their own renderer based on this code.
///
/// OpenGL context is still available to the rest of the application through
/// the [`gl_context`](Self::gl_context) method.
pub struct AutoRenderer {
    gl: Rc<glow::Context>,
    texture_map: SimpleTextureMap,
    renderer: Renderer,
}

impl AutoRenderer {
    /// # Errors
    /// Any error initialising the OpenGL objects (including shaders) will
    /// result in an error.
    pub fn initialize(
        gl: glow::Context,
        imgui_context: &mut imgui::Context,
    ) -> Result<Self, InitError> {
        let mut texture_map = SimpleTextureMap::default();
        let renderer = Renderer::initialize(&gl, imgui_context, &mut texture_map, true)?;
        Ok(Self {
            gl: Rc::new(gl),
            texture_map,
            renderer,
        })
    }

    /// Note: no need to provide a `mut` version of this, as all methods on
    /// [`glow::HasContext`] are immutable.
    #[inline]
    pub fn gl_context(&self) -> &Rc<glow::Context> {
        &self.gl
    }

    #[inline]
    pub fn texture_map(&self) -> &SimpleTextureMap {
        &self.texture_map
    }

    #[inline]
    pub fn texture_map_mut(&mut self) -> &mut SimpleTextureMap {
        &mut self.texture_map
    }

    #[inline]
    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    /// # Errors
    /// Some OpenGL errors trigger an error (few are explicitly checked,
    /// however)
    #[inline]
    pub fn render(&mut self, draw_data: &DrawData) -> Result<(), RenderError> {
        self.renderer.render(&self.gl, &self.texture_map, draw_data)
    }
}

impl Drop for AutoRenderer {
    fn drop(&mut self) {
        self.renderer.destroy(&self.gl);
    }
}

/// Main renderer. Borrows the OpenGL context and [texture map](TextureMap)
/// when required.
pub struct Renderer {
    shaders: Shaders,
    state_backup: GlStateBackup,
    pub vbo_handle: Option<GlBuffer>,
    pub ebo_handle: Option<GlBuffer>,
    pub font_atlas_texture: Option<GlTexture>,
    #[cfg(feature = "bind_vertex_array_support")]
    pub vertex_array_object: Option<GlVertexArray>,
    pub gl_version: GlVersion,
    pub has_clip_origin_support: bool,
    pub is_destroyed: bool,
}

impl Renderer {
    /// Create the renderer, initialising OpenGL objects and shaders.
    ///
    /// `output_srgb` controls whether the shader outputs sRGB colors, or linear
    /// RGB colors. In short:
    /// - If you're outputting to a display and haven't specified the framebuffer
    ///   is sRGB (e.g. with `gl.enable(glow::FRAMEBUFFER_SRGB)`), then you probably
    ///   want `output_srgb=true`.
    /// - OpenGL ES doesn't support sRGB framebuffers, so you almost always
    ///   want `output_srgb=true` if you're using OpenGL ES and you're outputting
    ///   to a display.
    /// - If you're outputting to a display with an sRGB framebuffer (e.g. with
    ///   `gl.enable(glow::FRAMEBUFFER_SRGB)`), then you probably want
    ///   `output_srgb=false`, as OpenGL will convert to sRGB itself.
    /// - If you're not outputting to a display, but instead to some intermediate
    ///   framebuffer, then you probably want `output_srgb=false` to keep the
    ///   colors in linear color space, and then convert them to sRGB at some
    ///   later stage.
    ///
    /// # Errors
    /// Any error initialising the OpenGL objects (including shaders) will
    /// result in an error.
    pub fn initialize<T: TextureMap>(
        gl: &Context,
        imgui_context: &mut imgui::Context,
        texture_map: &mut T,
        output_srgb: bool,
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

        let mut state_backup = GlStateBackup::default();
        state_backup.pre_init(gl);

        let font_atlas_texture = prepare_font_atlas(gl, imgui_context.fonts(), texture_map)?;

        let shaders = Shaders::new(gl, gl_version, output_srgb)?;
        let vbo_handle = unsafe { gl.create_buffer() }.map_err(InitError::CreateBufferObject)?;
        let ebo_handle = unsafe { gl.create_buffer() }.map_err(InitError::CreateBufferObject)?;

        state_backup.post_init(gl);

        let out = Self {
            shaders,
            state_backup,
            vbo_handle: Some(vbo_handle),
            ebo_handle: Some(ebo_handle),
            font_atlas_texture: Some(font_atlas_texture),
            #[cfg(feature = "bind_vertex_array_support")]
            vertex_array_object: None,
            gl_version,
            has_clip_origin_support,
            is_destroyed: false,
        };

        // Leave this until the end of the function to avoid changing state if
        // there was ever an error above
        out.configure_imgui_context(imgui_context);

        Ok(out)
    }

    /// This must be called before being dropped to properly free OpenGL
    /// resources.
    pub fn destroy(&mut self, gl: &Context) {
        if self.is_destroyed {
            return;
        }

        if let Some(h) = self.vbo_handle {
            unsafe { gl.delete_buffer(h) };
            self.vbo_handle = None;
        }
        if let Some(h) = self.ebo_handle {
            unsafe { gl.delete_buffer(h) };
            self.ebo_handle = None;
        }
        if let Some(p) = self.shaders.program {
            unsafe { gl.delete_program(p) };
            self.shaders.program = None;
        }
        if let Some(h) = self.font_atlas_texture {
            unsafe { gl.delete_texture(h) };
            self.font_atlas_texture = None;
        }

        self.is_destroyed = true;
    }

    /// # Errors
    /// Some OpenGL errors trigger an error (few are explicitly checked,
    /// however)
    pub fn render<T: TextureMap>(
        &mut self,
        gl: &Context,
        texture_map: &T,
        draw_data: &DrawData,
    ) -> Result<(), RenderError> {
        if self.is_destroyed {
            return Err(Self::renderer_destroyed());
        }

        let fb_width = draw_data.display_size[0] * draw_data.framebuffer_scale[0];
        let fb_height = draw_data.display_size[1] * draw_data.framebuffer_scale[1];
        if !(fb_width > 0.0 && fb_height > 0.0) {
            return Ok(());
        }

        gl_debug_message(gl, "imgui-rs-glow: start render");
        self.state_backup.pre_render(gl, self.gl_version);

        #[cfg(feature = "bind_vertex_array_support")]
        if self.gl_version.bind_vertex_array_support() {
            unsafe {
                self.vertex_array_object = Some(
                    gl.create_vertex_array()
                        .map_err(|err| format!("Error creating vertex array object: {}", err))?,
                );
                gl.bind_vertex_array(self.vertex_array_object);
            }
        }

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
                    DrawCmd::Elements { count, cmd_params } => self.render_elements(
                        gl,
                        texture_map,
                        count,
                        cmd_params,
                        draw_data,
                        fb_width,
                        fb_height,
                    ),
                    DrawCmd::RawCallback { callback, raw_cmd } => unsafe {
                        callback(draw_list.raw(), raw_cmd)
                    },
                    DrawCmd::ResetRenderState => {
                        self.set_up_render_state(gl, draw_data, fb_width, fb_height)?
                    }
                }
            }
        }

        #[cfg(feature = "bind_vertex_array_support")]
        if self.gl_version.bind_vertex_array_support() {
            unsafe { gl.delete_vertex_array(self.vertex_array_object.unwrap()) };
            self.vertex_array_object = None;
        }

        self.state_backup.post_render(gl, self.gl_version);
        gl_debug_message(gl, "imgui-rs-glow: complete render");
        Ok(())
    }

    /// # Errors
    /// Few GL calls are checked for errors, but any that are found will result
    /// in an error. Errors from the state manager lifecycle callbacks will also
    /// result in an error.
    pub fn set_up_render_state(
        &mut self,
        gl: &Context,
        draw_data: &DrawData,
        fb_width: f32,
        fb_height: f32,
    ) -> Result<(), RenderError> {
        #![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]

        if self.is_destroyed {
            return Err(Self::renderer_destroyed());
        }

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

        unsafe {
            gl.use_program(self.shaders.program);
            gl.uniform_1_i32(Some(&self.shaders.texture_uniform_location), 0);
            gl.uniform_matrix_4_f32_slice(
                Some(&self.shaders.matrix_uniform_location),
                false,
                &projection_matrix,
            );
        }

        #[cfg(feature = "bind_sampler_support")]
        if self.gl_version.bind_sampler_support() {
            unsafe { gl.bind_sampler(0, None) };
        }

        // TODO: soon it should be possible for these to be `const` functions
        let position_field_offset = memoffset::offset_of!(DrawVert, pos) as _;
        let uv_field_offset = memoffset::offset_of!(DrawVert, uv) as _;
        let color_field_offset = memoffset::offset_of!(DrawVert, col) as _;

        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, self.vbo_handle);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, self.ebo_handle);
            gl.enable_vertex_attrib_array(self.shaders.position_attribute_index);
            gl.vertex_attrib_pointer_f32(
                self.shaders.position_attribute_index,
                2,
                glow::FLOAT,
                false,
                size_of::<imgui::DrawVert>() as _,
                position_field_offset,
            );
            gl.enable_vertex_attrib_array(self.shaders.uv_attribute_index);
            gl.vertex_attrib_pointer_f32(
                self.shaders.uv_attribute_index,
                2,
                glow::FLOAT,
                false,
                size_of::<imgui::DrawVert>() as _,
                uv_field_offset,
            );
            gl.enable_vertex_attrib_array(self.shaders.color_attribute_index);
            gl.vertex_attrib_pointer_f32(
                self.shaders.color_attribute_index,
                4,
                glow::UNSIGNED_BYTE,
                true,
                size_of::<imgui::DrawVert>() as _,
                color_field_offset,
            );
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn render_elements<T: TextureMap>(
        &self,
        gl: &Context,
        texture_map: &T,
        element_count: usize,
        element_params: imgui::DrawCmdParams,
        draw_data: &DrawData,
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
            gl.bind_texture(glow::TEXTURE_2D, texture_map.gl_texture(texture_id));

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
        imgui_context.set_renderer_name(Some(format!(
            "imgui-rs-glow-render {}",
            env!("CARGO_PKG_VERSION")
        )));

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

/// Trait for mapping imgui texture IDs to OpenGL textures.
///
/// [`register`] should be called after uploading a texture to OpenGL to get a
/// [`imgui::TextureId`] corresponding to it.
///
/// [`register`]: Self::register
///
/// Then [`gl_texture`] can be called to find the OpenGL texture corresponding to
/// that [`imgui::TextureId`].
///
/// [`gl_texture`]: Self::gl_texture
pub trait TextureMap {
    fn register(&mut self, gl_texture: GlTexture) -> Option<imgui::TextureId>;

    fn gl_texture(&self, imgui_texture: imgui::TextureId) -> Option<GlTexture>;
}

/// Texture map where the imgui texture ID is simply numerically equal to the
/// OpenGL texture ID.
#[derive(Default)]
pub struct SimpleTextureMap();

impl TextureMap for SimpleTextureMap {
    #[inline(always)]
    fn register(&mut self, gl_texture: glow::Texture) -> Option<imgui::TextureId> {
        Some(imgui::TextureId::new(gl_texture.0.get() as _))
    }

    #[inline(always)]
    fn gl_texture(&self, imgui_texture: imgui::TextureId) -> Option<glow::Texture> {
        #[allow(clippy::cast_possible_truncation)]
        Some(glow::NativeTexture(
            NonZeroU32::new(imgui_texture.id() as _).unwrap(),
        ))
    }
}

/// [`imgui::Textures`] is a simple choice for a texture map.
impl TextureMap for imgui::Textures<glow::Texture> {
    fn register(&mut self, gl_texture: glow::Texture) -> Option<imgui::TextureId> {
        Some(self.insert(gl_texture))
    }

    fn gl_texture(&self, imgui_texture: imgui::TextureId) -> Option<glow::Texture> {
        self.get(imgui_texture).copied()
    }
}

/// This OpenGL state backup is based on the upstream OpenGL example from
/// imgui, where an attempt is made to save and restore the OpenGL context state
/// before and after rendering.
///
/// If you're writing your own renderer, you can likely streamline most of this.
///
/// It is unlikely that any such attempt will be comprehensive for all possible
/// applications, due to the complexity of OpenGL and the possibility of
/// arbitrary extensions. However, it remains as a useful tool for quickly
/// getting started. If your application needs more state to be backed up and
/// restored, it is probably best to do this manually before/after calling
/// the render method rather than opening an issue to add more to this
/// struct.
#[allow(clippy::struct_excessive_bools)]
#[derive(Default)]
pub struct GlStateBackup {
    active_texture: u32,
    program: u32,
    texture: u32,
    #[cfg(feature = "bind_sampler_support")]
    sampler: Option<u32>,
    array_buffer: u32,
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
    vertex_array_object: Option<u32>,
}

fn to_native_gl<T>(handle: u32, constructor: fn(NonZeroU32) -> T) -> Option<T> {
    if handle != 0 {
        Some(constructor(NonZeroU32::new(handle).unwrap()))
    } else {
        None
    }
}

impl GlStateBackup {
    fn pre_init(&mut self, gl: &Context) {
        self.texture = unsafe { gl.get_parameter_i32(glow::TEXTURE_BINDING_2D) as _ };
    }

    fn post_init(&mut self, gl: &Context) {
        #[allow(clippy::cast_sign_loss)]
        unsafe {
            gl.bind_texture(
                glow::TEXTURE_2D,
                to_native_gl(self.texture, glow::NativeTexture),
            );
        }
    }

    fn pre_render(&mut self, gl: &Context, gl_version: GlVersion) {
        #[allow(clippy::cast_sign_loss)]
        unsafe {
            self.active_texture = gl.get_parameter_i32(glow::ACTIVE_TEXTURE) as _;
            self.program = gl.get_parameter_i32(glow::CURRENT_PROGRAM) as _;
            self.texture = gl.get_parameter_i32(glow::TEXTURE_BINDING_2D) as _;
            #[cfg(feature = "bind_sampler_support")]
            if gl_version.bind_sampler_support() {
                self.sampler = Some(gl.get_parameter_i32(glow::SAMPLER_BINDING) as _);
            } else {
                self.sampler = None;
            }
            self.array_buffer = gl.get_parameter_i32(glow::ARRAY_BUFFER_BINDING) as _;

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
    }

    fn post_render(&mut self, gl: &Context, _gl_version: GlVersion) {
        #![allow(clippy::cast_sign_loss)]
        unsafe {
            gl.use_program(to_native_gl(self.program, glow::NativeProgram));
            gl.bind_texture(
                glow::TEXTURE_2D,
                to_native_gl(self.texture, glow::NativeTexture),
            );
            #[cfg(feature = "bind_sampler_support")]
            if let Some(sampler) = self.sampler {
                gl.bind_sampler(0, to_native_gl(sampler, glow::NativeSampler));
            }
            gl.active_texture(self.active_texture as _);
            #[cfg(feature = "bind_vertex_array_support")]
            if let Some(vao) = self.vertex_array_object {
                gl.bind_vertex_array(to_native_gl(vao, glow::NativeVertexArray));
            }
            gl.bind_buffer(
                glow::ARRAY_BUFFER,
                to_native_gl(self.array_buffer, glow::NativeBuffer),
            );
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
    }
}

/// Parses `GL_VERSION` and `GL_SHADING_LANGUAGE_VERSION` at runtime in order to
/// generate shaders which should work on a wide variety of modern devices
/// (GL >= 3.3 and GLES >= 2.0 are expected to work).
struct Shaders {
    program: Option<GlProgram>,
    texture_uniform_location: GlUniformLocation,
    matrix_uniform_location: GlUniformLocation,
    position_attribute_index: u32,
    uv_attribute_index: u32,
    color_attribute_index: u32,
}

impl Shaders {
    fn new(gl: &Context, gl_version: GlVersion, output_srgb: bool) -> Result<Self, ShaderError> {
        let (vertex_source, fragment_source) =
            Self::get_shader_sources(gl, gl_version, output_srgb)?;

        let vertex_shader =
            unsafe { gl.create_shader(glow::VERTEX_SHADER) }.map_err(ShaderError::CreateShader)?;
        unsafe {
            gl.shader_source(vertex_shader, &vertex_source);
            gl.compile_shader(vertex_shader);
            if !gl.get_shader_compile_status(vertex_shader) {
                return Err(ShaderError::CompileShader(
                    gl.get_shader_info_log(vertex_shader),
                ));
            }
        }

        let fragment_shader = unsafe { gl.create_shader(glow::FRAGMENT_SHADER) }
            .map_err(ShaderError::CreateShader)?;
        unsafe {
            gl.shader_source(fragment_shader, &fragment_source);
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

        Ok(unsafe {
            Self {
                program: Some(program),
                texture_uniform_location: gl
                    .get_uniform_location(program, "tex")
                    .ok_or_else(|| ShaderError::UniformNotFound("tex".into()))?,
                matrix_uniform_location: gl
                    .get_uniform_location(program, "matrix")
                    .ok_or_else(|| ShaderError::UniformNotFound("matrix".into()))?,
                position_attribute_index: gl
                    .get_attrib_location(program, "position")
                    .ok_or_else(|| ShaderError::AttributeNotFound("position".into()))?,
                uv_attribute_index: gl
                    .get_attrib_location(program, "uv")
                    .ok_or_else(|| ShaderError::AttributeNotFound("uv".into()))?,
                color_attribute_index: gl
                    .get_attrib_location(program, "color")
                    .ok_or_else(|| ShaderError::AttributeNotFound("color".into()))?,
            }
        })
    }

    fn get_shader_sources(
        gl: &Context,
        gl_version: GlVersion,
        output_srgb: bool,
    ) -> Result<(String, String), ShaderError> {
        const VERTEX_BODY: &str = r#"
layout (location = 0) in vec2 position;
layout (location = 1) in vec2 uv;
layout (location = 2) in vec4 color;

uniform mat4 matrix;
out vec2 fragment_uv;
out vec4 fragment_color;

// Because imgui only specifies sRGB colors
vec4 srgb_to_linear(vec4 srgb_color) {
    // Calcuation as documented by OpenGL
    vec3 srgb = srgb_color.rgb;
    vec3 selector = ceil(srgb - 0.04045);
    vec3 less_than_branch = srgb / 12.92;
    vec3 greater_than_branch = pow((srgb + 0.055) / 1.055, vec3(2.4));
    return vec4(
        mix(less_than_branch, greater_than_branch, selector),
        srgb_color.a
    );
}

void main() {
    fragment_uv = uv;
    fragment_color = srgb_to_linear(color);
    gl_Position = matrix * vec4(position.xy, 0, 1);
}
"#;
        const FRAGMENT_BODY: &str = r#"
in vec2 fragment_uv;
in vec4 fragment_color;

uniform sampler2D tex;
layout (location = 0) out vec4 out_color;

vec4 linear_to_srgb(vec4 linear_color) {
    vec3 linear = linear_color.rgb;
    vec3 selector = ceil(linear - 0.0031308);
    vec3 less_than_branch = linear * 12.92;
    vec3 greater_than_branch = pow(linear, vec3(1.0/2.4)) * 1.055 - 0.055;
    return vec4(
        mix(less_than_branch, greater_than_branch, selector),
        linear_color.a
    );
}

void main() {
    vec4 linear_color = fragment_color * texture(tex, fragment_uv.st);
#ifdef OUTPUT_SRGB
    out_color = linear_to_srgb(linear_color);
#else
    out_color = linear_color;
#endif
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
            "#version {version}{es_extras}\n{body}",
            version = major * 100 + minor * 10,
            es_extras = if is_gles {
                " es\nprecision mediump float;"
            } else {
                ""
            },
            body = VERTEX_BODY,
        );
        let fragment_source = format!(
            "#version {version}{es_extras}{defines}\n{body}",
            version = major * 100 + minor * 10,
            es_extras = if is_gles {
                " es\nprecision mediump float;"
            } else {
                ""
            },
            defines = if output_srgb {
                "\n#define OUTPUT_SRGB"
            } else {
                ""
            },
            body = FRAGMENT_BODY,
        );

        Ok((vertex_source, fragment_source))
    }
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
    RegisterTexture,
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
            Self::RegisterTexture => write!(f, "Error registering texture in texture map"),
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

fn prepare_font_atlas<T: TextureMap>(
    gl: &Context,
    fonts: &mut imgui::FontAtlas,
    texture_map: &mut T,
) -> Result<GlTexture, InitError> {
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
            glow::SRGB8_ALPHA8 as _,
            atlas_texture.width as _,
            atlas_texture.height as _,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            Some(atlas_texture.data),
        );
    }

    fonts.tex_id = texture_map
        .register(gl_texture)
        .ok_or(InitError::RegisterTexture)?;

    Ok(gl_texture)
}

// this CFG guard disables apple usage of this function -- apple only has supported up to opengl 3.3
#[cfg(all(not(target_vendor = "apple"), feature = "debug_message_insert_support"))]
fn gl_debug_message<G: glow::HasContext>(gl: &G, message: impl AsRef<str>) {
    unsafe {
        gl.debug_message_insert(
            glow::DEBUG_SOURCE_APPLICATION,
            glow::DEBUG_TYPE_MARKER,
            0,
            glow::DEBUG_SEVERITY_NOTIFICATION,
            message,
        )
    };
}

#[cfg(any(target_vendor = "apple", not(feature = "debug_message_insert_support")))]
fn gl_debug_message<G: glow::HasContext>(_gl: &G, _message: impl AsRef<str>) {}

fn calculate_matrix(draw_data: &DrawData, clip_origin_is_lower_left: bool) -> [f32; 16] {
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
    std::slice::from_raw_parts(slice.as_ptr().cast(), std::mem::size_of_val(slice))
}

const fn imgui_index_type_as_gl() -> u32 {
    match size_of::<imgui::DrawIdx>() {
        1 => glow::UNSIGNED_BYTE,
        2 => glow::UNSIGNED_SHORT,
        _ => glow::UNSIGNED_INT,
    }
}
