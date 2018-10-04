#[macro_use] extern crate vulkano;
#[macro_use] extern crate vulkano_shader_derive;
extern crate imgui;

use std::fmt;
use std::sync::Arc;
use imgui::{DrawList, FrameSize, ImDrawIdx, ImDrawVert, ImGui, Ui};

use vulkano::instance::QueueFamily;
use vulkano::sampler::*;
use vulkano::pipeline::*;
use vulkano::pipeline::viewport::*;
use vulkano::framebuffer::*;
use vulkano::device::{Device, Queue};
use vulkano::buffer::cpu_access::*;
use vulkano::buffer::*;
use vulkano::command_buffer::*;
use vulkano::image::*;
use vulkano::sync::*;
use vulkano::format::*;
use vulkano::descriptor::descriptor_set::*;

pub type RendererResult<T> = Result<T, RendererError>;

#[derive(Clone, Debug)]
pub enum RendererError {
    Unknown,
}

impl fmt::Display for RendererError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::RendererError::*;
        match *self {
            Unknown => write!(f, "Unknown error"),
        }
    }
}

pub struct Image {
    /// The actual image.
    pub image_access: Arc<ImmutableImage<R8G8B8A8Unorm>>,
    /// The width of the image.
    pub dimensions: vulkano::image::Dimensions,
}

#[derive(Debug, Clone)]
pub struct Vertex {
    position: [f32; 2],
    uv: [f32; 2],
    color: [f32; 4],
}

impl From<ImDrawVert> for Vertex {
    fn from(v: ImDrawVert) -> Self {
        Vertex {
            position: [v.pos.x, v.pos.y],
            uv: [v.uv.x, v.uv.y],
            color: normalize(v.col),
        }
    }
}

#[inline(always)]
pub fn normalize(src: u32) -> [f32; 4] {
    [
        (src & 0x000000ff >> 0) as f32 / 255.0,
        (src & 0x0000ff00 >> 8) as f32 / 255.0,
        (src & 0x00ff0000 >> 16) as f32 / 255.0,
        (src & 0xff000000 >> 24) as f32 / 255.0,

    ]
}

pub struct Renderer {
    dimensions: vulkano::image::Dimensions,
    dpi_factor: f64,
    pipeline: Box<Arc<GraphicsPipelineAbstract+Send+Sync>>,
    textures: Vec<Image>,
    sampler: Arc<Sampler>,
    tex_descs: FixedSizeDescriptorSetsPool<Arc<GraphicsPipelineAbstract+Send+Sync>>,
}

impl_vertex!(Vertex, position, uv, color);

impl Renderer {
    pub fn init<'a, L>(imgui: &mut ImGui,
                   device: Arc<Device>,
                   subpass: Subpass<L>,
                   queue: Arc<Queue>,
                   width: u32,
                   height: u32,
                   dpi_factor: f64,
    ) -> RendererResult<(Self, Box<GpuFuture>)>
        where L: RenderPassDesc + RenderPassAbstract + Send + Sync + 'static
    {

        let dimensions = vulkano::image::Dimensions::Dim2d { width, height };

        let mut textures = Vec::new();

        let (texture, texture_future) = imgui.prepare_texture(|handle| {
            let r = vulkano::image::immutable::ImmutableImage::from_iter(
                handle.pixels.iter().cloned(),
                vulkano::image::Dimensions::Dim2d { width: handle.width, height: handle.height },
                vulkano::format::R8G8B8A8Unorm,
                queue.clone()).unwrap();

            textures.push(Image {
                image_access: r.0.clone(),
                dimensions: vulkano::image::Dimensions::Dim2d { width: handle.width, height: handle.height }
            });

            r
        });

        let vertex_shader =
            vs::Shader::load(device.clone())
                .expect("failed to create shader module");

        let fragment_shader =
            fs::Shader::load(device.clone())
                .expect("failed to create shader module");

        let pipeline = Arc::new(GraphicsPipeline::start()
            .vertex_input_single_buffer::<Vertex>()
            .vertex_shader(vertex_shader.main_entry_point(), ())
            //.depth_stencil_simple_depth()
            .triangle_list()
            .front_face_clockwise()
            //.cull_mode_back()
            .viewports_scissors_dynamic(1)
            .fragment_shader(fragment_shader.main_entry_point(), ())
            .blend_alpha_blending()
            .render_pass(subpass)
            .build(device.clone())
            .unwrap());

        let tex_descs = FixedSizeDescriptorSetsPool::new(pipeline.clone() as Arc<_>, 0);

        let future = Box::new(texture_future) as Box<GpuFuture>;

        let sampler = Sampler::new(
            device.clone(),
            Filter::Linear,
            Filter::Linear,
            MipmapMode::Linear,
            SamplerAddressMode::ClampToEdge,
            SamplerAddressMode::ClampToEdge,
            SamplerAddressMode::ClampToEdge,
            0.0, 1.0, 0.0, 0.0
        ).unwrap();

        Ok((Self{
            dpi_factor,
            pipeline: Box::new(pipeline),
            textures,
            sampler,
            tex_descs,
            dimensions,
        }, future))
    }

    pub fn render<'a>(&mut self,
                      ui: Ui<'a>,
                      mut command_buffer: AutoCommandBufferBuilder,
                      device: Arc<Device>,
                      viewport: [f32; 4]
    ) -> RendererResult<AutoCommandBufferBuilder>
    {
        let current_viewport = Viewport {
            origin: [viewport[0], viewport[1]],
            dimensions: [viewport[2]-viewport[0], viewport[3]-viewport[1]],
            depth_range: 0.0 .. 1.0,
        };

        let imgui_desc = Arc::new(self.tex_descs.next()
            .add_sampled_image(self.textures.get(0).unwrap().image_access.clone(), self.sampler.clone()).unwrap()
            .build().unwrap());

        let mut draw_call_collection = Arc::new(Vec::new());

        // Push constant
        let push_constants = vs::ty::PushConstants {
            scale: [2.0 / self.dimensions.width() as f32, 2.0 / self.dimensions.height() as f32],
            translate: [-1.0, -1.0],
        };

        let render_result: RendererResult<()> = ui.render(|ui, mut draw_data| {
            draw_data.scale_clip_rects(ui.imgui().display_framebuffer_scale());

            struct DrawCall {
                vbuf: Arc<CpuAccessibleBuffer<[Vertex]>>,
                ibuf: Arc<CpuAccessibleBuffer<[u16]>>,
                state: DynamicState,
            }

            // draw shit
            for draw_list in draw_data.into_iter() {
                let mut dc = Arc::get_mut(&mut draw_call_collection).unwrap();
                let vert: Vec<Vertex> = draw_list.vtx_buffer.iter().map(|s| Vertex::from(*s)).collect();
                let vbuf = CpuAccessibleBuffer::from_iter(
                    device.clone(),
                    BufferUsage::vertex_buffer(),
                    vert.iter().cloned(),
                ).unwrap();

                let ibuf = CpuAccessibleBuffer::from_iter(
                    device.clone(),
                    BufferUsage::index_buffer(),
                    draw_list.idx_buffer.iter().cloned(),
                ).unwrap();

                for cmd in draw_list.cmd_buffer {

                    let state = DynamicState {
                        line_width: None,
                        viewports: Some(vec![current_viewport.clone()]),
                        scissors: Some(vec![vulkano::pipeline::viewport::Scissor {
                            origin: [std::cmp::max(cmd.clip_rect.x as i32, 0), std::cmp::max(cmd.clip_rect.y as i32, 0)],
                            dimensions: [(cmd.clip_rect.z - cmd.clip_rect.x) as u32, (cmd.clip_rect.w - cmd.clip_rect.y) as u32],
                        }]),
                    };

                    // Create draw call reference
                    dc.push(DrawCall {
                        vbuf: vbuf.clone(),
                        ibuf: ibuf.clone(),
                        state
                    });

                }
            }


            Ok(())
        });

        for call in draw_call_collection.iter() {
            command_buffer = command_buffer.draw_indexed(self.pipeline.clone(),
                                                         &call.state,
                                                         vec![call.vbuf.clone()],
                                                         call.ibuf.clone(),
                                                         imgui_desc.clone(),
                                                         push_constants)
                .unwrap();

        }

        Ok(command_buffer)
    }
}

mod vs {
#[derive(VulkanoShader)]
#[ty = "vertex"]
#[src = "
#version 450
layout (location = 0) in vec2 position;
layout (location = 1) in vec2 uv;
layout (location = 2) in vec4 color;

layout (push_constant) uniform PushConstants {
	vec2 scale;
	vec2 translate;
} pushConstants;

layout (location = 0) out vec2 outUV;
layout (location = 1) out vec4 outColor;

out gl_PerVertex
{
	vec4 gl_Position;
};

void main()
{
	outUV = uv;
	outColor = color;
	gl_Position = vec4(position * pushConstants.scale + pushConstants.translate, 0.0, 1.0);
}
"]
#[allow(dead_code)]
struct Dummy;
}

mod fs {
#[derive(VulkanoShader)]
#[ty = "fragment"]
#[src = "
#version 450
layout (set=0, binding = 0) uniform sampler2D fontSampler;

layout (location = 0) in vec2 inUV;
layout (location = 1) in vec4 inColor;

layout (location = 0) out vec4 outColor;

void main()
{
	outColor = inColor * texture(fontSampler, inUV);
}
"]
#[allow(dead_code)]
struct Dummy;
}