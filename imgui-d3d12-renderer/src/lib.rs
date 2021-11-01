#![cfg(windows)]

use imgui::internal::RawWrapper;
use imgui::{
    BackendFlags, DrawCmd, DrawCmdParams, DrawData, DrawIdx, DrawVert, TextureId, Textures,
};
use log::{info, trace, warn};
use memoffset::offset_of;

use core::num::NonZeroI32;
use core::ptr;
use core::slice;
use core::{mem, ops};

use rusty_d3d12::*;

use thiserror::Error;

// IDR is for imgui-d3d12-renderer
#[derive(Error, Debug)]
pub enum IDRError {
    #[error("wrong index size")]
    WrongIndexSize,
    #[error("D3D12 error: {}", .0)]
    D3d12Error(#[from] DxError),
}

pub type IDRResult<T> = Result<T, IDRError>;

const VERTEX_BUF_ADD_CAPACITY: usize = 5000;
const INDEX_BUF_ADD_CAPACITY: usize = 10000;

#[repr(C)]
struct VertexConstantBuffer {
    mvp: [[f32; 4]; 4],
}

fn create_shaders() -> IDRResult<(Vec<u8>, Vec<u8>)> {
    let vertex_shader = compile_shader(
        "VertexShader",
        r"#
cbuffer vertexBuffer: register(b0) {
            float4x4 ProjectionMatrix;
        };

        struct VS_INPUT {
            float2 pos: POSITION;
            float4 col: COLOR0;
            float2 uv: TEXCOORD0;
        };

        struct PS_INPUT {
            float4 pos: SV_POSITION;
            float4 col: COLOR0;
            float2 uv: TEXCOORD0;
        };

        PS_INPUT main(VS_INPUT input) {
            PS_INPUT output;
            output.pos = mul(ProjectionMatrix, float4(input.pos.xy, 0.f, 1.f));
            output.col = input.col;
            output.uv = input.uv;
            return output;
        }
#",
        "main",
        "vs_6_0",
        &[],
        &[],
    )?;

    let pixel_shader = compile_shader(
        "PixelShader",
        r"#
struct PS_INPUT {
    float4 pos: SV_POSITION;
    float4 col: COLOR0;
    float2 uv: TEXCOORD0;
};

sampler sampler0;
Texture2D texture0;

float4 main(PS_INPUT input): SV_Target {
    float4 out_col = input.col * texture0.Sample(sampler0, input.uv);
    return out_col;
}
#",
        "main",
        "ps_6_0",
        &[],
        &[],
    )?;

    Ok((vertex_shader, pixel_shader))
}

fn create_pipeline_state(
    input_layout: Vec<InputElementDesc>,
    root_signature: &RootSignature,
    vertex_shader: Vec<u8>,
    pixel_shader: Vec<u8>,
    device: &Device,
) -> IDRResult<PipelineState> {
    let vs_bytecode = ShaderBytecode::from_bytes(&vertex_shader);
    let ps_bytecode = ShaderBytecode::from_bytes(&pixel_shader);

    let input_layout = InputLayoutDesc::default().from_input_elements(&input_layout);
    let pso_desc = GraphicsPipelineStateDesc::default()
        .set_input_layout(&input_layout)
        .set_root_signature(root_signature)
        .set_vs_bytecode(&vs_bytecode)
        .set_ps_bytecode(&ps_bytecode)
        .set_rasterizer_state(
            &RasterizerDesc::default()
                .set_fill_mode(FillMode::Solid)
                .set_depth_clip_enable(true),
        )
        .set_blend_state(
            &BlendDesc::default().set_render_targets(&[RenderTargetBlendDesc::default()
                .set_blend_enable(true)
                .set_src_blend(Blend::SrcAlpha)
                .set_dest_blend(Blend::InvSrcAlpha)
                .set_blend_op(BlendOp::Add)
                .set_src_blend_alpha(Blend::InvDestAlpha)
                .set_dest_blend_alpha(Blend::One)
                .set_blend_op_alpha(BlendOp::Add)
                .set_render_target_write_mask(ColorWriteEnable::EnableAll)]),
        )
        .set_depth_stencil_state(&DepthStencilDesc::default())
        .set_primitive_topology_type(PrimitiveTopologyType::Triangle)
        .set_rtv_formats(&[Format::R8G8B8A8_UNorm])
        .set_dsv_format(Format::D32_Float);

    device
        .create_graphics_pipeline_state(&pso_desc)
        .map_err(|err| err.into())
}

fn create_input_layout() -> Vec<InputElementDesc<'static>> {
    vec![
        InputElementDesc::default()
            // ToDo: "POSITION\0" on lib side would allow to get rid of allocations
            .set_name("POSITION")
            .unwrap()
            .set_format(Format::R32G32B32_Float)
            .set_input_slot(0)
            .set_offset(Bytes::from(offset_of!(DrawVert, pos))),
        InputElementDesc::default()
            .set_name("TEXCOORD")
            .unwrap()
            .set_format(Format::R32G32_Float)
            .set_input_slot(0)
            .set_offset(Bytes::from(offset_of!(DrawVert, uv))),
        InputElementDesc::default()
            .set_name("COLOR")
            .unwrap()
            .set_format(Format::R8G8B8A8_UNorm)
            .set_input_slot(0)
            .set_offset(Bytes::from(offset_of!(DrawVert, col))),
    ]
}

fn setup_root_signature(device: &Device) -> IDRResult<RootSignature> {
    let ranges = [DescriptorRange::default()
        .set_range_type(DescriptorRangeType::Srv)
        .set_num_descriptors(1)
        .set_flags(DescriptorRangeFlags::DataVolatile)];

    let static_sampler_desc = StaticSamplerDesc::default()
        .set_filter(Filter::MinMagMipLinear)
        .set_address_u(TextureAddressMode::Wrap)
        .set_address_v(TextureAddressMode::Wrap)
        .set_address_w(TextureAddressMode::Wrap)
        .set_comparison_func(ComparisonFunc::Always)
        .set_border_color(StaticBorderColor::TransparentBlack)
        .set_shader_visibility(ShaderVisibility::Pixel);

    let descriptor_table = RootDescriptorTable::default().set_descriptor_ranges(&ranges);

    let root_parameters = [
        RootParameter::default()
            .new_constants(&RootConstants::default().set_num_32_bit_values(16))
            .set_shader_visibility(ShaderVisibility::Vertex),
        RootParameter::default()
            .new_descriptor_table(&descriptor_table)
            .set_shader_visibility(ShaderVisibility::All),
    ];
    let root_signature_desc = VersionedRootSignatureDesc::default().set_desc_1_1(
        &RootSignatureDesc::default()
            .set_parameters(&root_parameters)
            .set_static_samplers(slice::from_ref(&static_sampler_desc))
            .set_flags(RootSignatureFlags::AllowInputAssemblerInputLayout),
    );

    let (serialized_signature, serialization_result) =
        RootSignature::serialize_versioned(&root_signature_desc);
    assert!(
        serialization_result.is_ok(),
        "Result: {}",
        &serialization_result.err().unwrap()
    );

    let root_signature = device.create_root_signature(
        0,
        &ShaderBytecode::from_bytes(serialized_signature.get_buffer()),
    )?;

    root_signature.set_name("ImGUI Root Signature")?;

    Ok(root_signature)
}

fn create_font_texture(
    mut fonts: imgui::FontAtlasRefMut<'_>,
    device: &Device,
    font_tex_cpu_descriptor_handle: CpuDescriptorHandle,
    font_tex_gpu_descriptor_handle: GpuDescriptorHandle,
) -> IDRResult<(Resource, Resource)> {
    let fa_tex = fonts.build_rgba32_texture();

    let texture_desc = ResourceDesc::default()
        .set_dimension(ResourceDimension::Texture2D)
        .set_width(fa_tex.width as u64)
        .set_height(fa_tex.height)
        .set_mip_levels(1)
        .set_format(Format::R8G8B8A8_UNorm);

    let (staging_resource, texture_resource) = upload_texture(device, &texture_desc, fa_tex.data)?;

    device.create_shader_resource_view(&texture_resource, None, font_tex_cpu_descriptor_handle);

    fonts.tex_id = TextureId::from(font_tex_gpu_descriptor_handle.hw_handle.ptr as usize);

    Ok((staging_resource, texture_resource))
}

fn upload_texture(
    device: &Device,
    texture_desc: &ResourceDesc,
    init_data: &[u8],
) -> IDRResult<(Resource, Resource)> {
    let command_queue = device.create_command_queue(
        &CommandQueueDesc::default()
            .set_queue_type(CommandListType::Direct)
            .set_flags(CommandQueueFlags::None),
    )?;

    let command_allocator = device.create_command_allocator(CommandListType::Direct)?;

    let command_list =
        device.create_command_list(CommandListType::Direct, &command_allocator, None)?;

    let mut fence_value = 0;
    let fence = device.create_fence(fence_value, FenceFlags::None)?;
    let event = Win32Event::default();

    let staging_buffer_desc = ResourceDesc::default()
        .set_dimension(ResourceDimension::Buffer)
        .set_layout(TextureLayout::RowMajor)
        .set_width(texture_desc.width() * texture_desc.height() as u64 * 4); // RGBA8

    let staging_buffer = device.create_committed_resource(
        &HeapProperties::default().set_heap_type(HeapType::Upload),
        HeapFlags::None,
        &staging_buffer_desc,
        ResourceStates::GenericRead,
        None,
    )?;

    let staging_data = staging_buffer.map(0, None)?;

    unsafe {
        std::ptr::copy_nonoverlapping(init_data.as_ptr(), staging_data, init_data.len());
    }

    staging_buffer.unmap(0, None);

    let texture_resource = device.create_committed_resource(
        &HeapProperties::default().set_heap_type(HeapType::Default),
        HeapFlags::None,
        texture_desc,
        ResourceStates::CopyDest,
        None,
    )?;

    let source_location = TextureCopyLocation::new_placed_footprint(
        &staging_buffer,
        PlacedSubresourceFootprint::default()
            .set_offset(Bytes(0))
            .set_footprint(
                SubresourceFootprint::default()
                    .set_width(texture_desc.width() as u32)
                    .set_height(texture_desc.height())
                    .set_depth(1)
                    .set_format(Format::R8G8B8A8_UNorm)
                    .set_row_pitch(Bytes(align_to_multiple(
                        texture_desc.width() as u64 * 4,
                        TEXTURE_DATA_PITCH_ALIGNMENT.0,
                    ))),
            ),
    );

    let dest_location = TextureCopyLocation::new_subresource_index(&texture_resource, 0);

    command_list.copy_texture_region(dest_location, 0, 0, 0, source_location, None);

    command_list.resource_barrier(std::slice::from_ref(&ResourceBarrier::new_transition(
        &ResourceTransitionBarrier::default()
            .set_resource(&texture_resource)
            .set_state_before(ResourceStates::CopyDest)
            .set_state_after(ResourceStates::PixelShaderResource),
    )));

    command_list.close()?;
    command_queue.execute_command_lists(slice::from_ref(&command_list));

    fence_value += 1;
    command_queue.signal(&fence, fence_value)?;

    fence.set_event_on_completion(fence_value, &event)?;
    event.wait(None);

    info!("uploaded font texture");

    Ok((staging_buffer, texture_resource))
}

fn create_vertex_buffer(
    device: &Device,
    vertex_count: usize,
) -> IDRResult<(Resource, VertexBufferView, *mut u8)> {
    let vertex_buffer_size = (vertex_count + VERTEX_BUF_ADD_CAPACITY) * size_of!(DrawVert);

    let vertex_buffer = device.create_committed_resource(
        &HeapProperties::default().set_heap_type(HeapType::Upload),
        HeapFlags::None,
        &ResourceDesc::default()
            .set_dimension(ResourceDimension::Buffer)
            .set_layout(TextureLayout::RowMajor)
            .set_width(vertex_buffer_size.0),
        ResourceStates::GenericRead,
        None,
    )?;

    vertex_buffer.set_name("ImGUI vertex buffer")?;

    let vertex_buffer_view = VertexBufferView::default()
        .set_buffer_location(vertex_buffer.get_gpu_virtual_address())
        .set_size_in_bytes(vertex_buffer_size)
        .set_stride_in_bytes(Bytes::from(std::mem::size_of::<DrawVert>()));

    let mapped_data = vertex_buffer.map(0, None)?;

    Ok((vertex_buffer, vertex_buffer_view, mapped_data))
}

fn create_index_buffer(
    device: &Device,
    index_count: usize,
) -> IDRResult<(Resource, IndexBufferView, *mut u8)> {
    let index_buffer_size = (index_count + INDEX_BUF_ADD_CAPACITY) * size_of!(DrawIdx);

    let index_buffer = device.create_committed_resource(
        &HeapProperties::default().set_heap_type(HeapType::Upload),
        HeapFlags::None,
        &ResourceDesc::default()
            .set_dimension(ResourceDimension::Buffer)
            .set_layout(TextureLayout::RowMajor)
            .set_width(index_buffer_size.0),
        ResourceStates::GenericRead,
        None,
    )?;

    index_buffer.set_name("ImGUI index buffer")?;

    let index_buffer_view = IndexBufferView::default()
        .set_buffer_location(index_buffer.get_gpu_virtual_address())
        .set_size_in_bytes(index_buffer_size)
        .set_format(match size_of!(DrawIdx) {
            Bytes(2) => Format::R16_UInt,
            Bytes(4) => Format::R32_UInt,
            _ => return Err(IDRError::WrongIndexSize),
        });

    let mapped_data = index_buffer.map(0, None)?;

    Ok((index_buffer, index_buffer_view, mapped_data))
}

fn create_gpu_handle_from_texture_id(handle_size: u32, id: TextureId) -> GpuDescriptorHandle {
    GpuDescriptorHandle {
        hw_handle: D3D12_GPU_DESCRIPTOR_HANDLE {
            ptr: id.id() as u64,
        },
        handle_size,
    }
}

#[derive(Debug)]
struct FrameResources {
    vertex_buffer: Resource,
    vertex_buffer_view: VertexBufferView,
    vertex_buffer_data: *mut u8,
    vertex_count: usize,

    index_buffer: Resource,
    index_buffer_view: IndexBufferView,
    index_buffer_data: *mut u8,
    index_count: usize,
}

impl FrameResources {
    fn new(device: &Device, vertex_count: usize, index_count: usize) -> IDRResult<Self> {
        let (vertex_buffer, vertex_buffer_view, vertex_buffer_data) =
            create_vertex_buffer(device, 0)?;
        let (index_buffer, index_buffer_view, index_buffer_data) = create_index_buffer(device, 0)?;

        Ok(Self {
            vertex_buffer,
            vertex_buffer_view,
            vertex_buffer_data,
            vertex_count,
            index_buffer,
            index_buffer_view,
            index_buffer_data,
            index_count,
        })
    }
}

#[derive(Debug)]
pub struct Renderer {
    device: Device,
    frame_count: usize,
    current_frame_index: usize,
    frame_resources: Vec<FrameResources>,
    root_signature: RootSignature,
    pipeline_state: PipelineState,
    staging_resource: Resource,
    texture_resource: Resource,
    font_tex_cpu_descriptor_handle: CpuDescriptorHandle,
    font_tex_gpu_descriptor_handle: GpuDescriptorHandle,
    textures: Textures<GpuDescriptorHandle>,
}

impl Renderer {
    pub fn new(
        im_ctx: &mut imgui::Context,
        device: Device,
        frame_count: usize,
        font_tex_cpu_descriptor_handle: CpuDescriptorHandle,
        font_tex_gpu_descriptor_handle: GpuDescriptorHandle,
    ) -> IDRResult<Self> {
        let (vertex_shader, pixel_shader) = create_shaders()?;

        let input_layout = create_input_layout();
        let root_signature = setup_root_signature(&device)?;
        let pipeline_state = create_pipeline_state(
            input_layout,
            &root_signature,
            vertex_shader,
            pixel_shader,
            &device,
        )?;

        let (staging_resource, texture_resource) = create_font_texture(
            im_ctx.fonts(),
            &device,
            font_tex_cpu_descriptor_handle,
            font_tex_gpu_descriptor_handle,
        )?;

        let frame_resources: Vec<FrameResources> = (0..frame_count)
            .map(|idx| FrameResources::new(&device, 0, 0))
            .collect::<IDRResult<Vec<_>>>()?;

        im_ctx.io_mut().backend_flags |= BackendFlags::RENDERER_HAS_VTX_OFFSET;
        im_ctx.set_renderer_name(Some(
            concat!("imgui_d3d12_renderer@", env!("CARGO_PKG_VERSION")).to_owned(),
        ));

        Ok(Renderer {
            device,
            frame_count,
            current_frame_index: 0,
            frame_resources,
            root_signature,
            pipeline_state,
            staging_resource,
            texture_resource,
            font_tex_cpu_descriptor_handle,
            font_tex_gpu_descriptor_handle,
            textures: Textures::new(),
        })
    }

    pub fn textures_mut(&mut self) -> &mut Textures<GpuDescriptorHandle> {
        &mut self.textures
    }

    pub fn textures(&self) -> &Textures<GpuDescriptorHandle> {
        &self.textures
    }

    pub fn render(&mut self, draw_data: &DrawData, command_list: &CommandList) -> IDRResult<()> {
        if draw_data.display_size[0] <= 0.0 || draw_data.display_size[1] <= 0.0 {
            return Ok(());
        }

        if self.frame_resources[self.current_frame_index].vertex_count
            < draw_data.total_vtx_count as usize
            || self.frame_resources[self.current_frame_index].index_count
                < draw_data.total_idx_count as usize
        {
            self.frame_resources[self.current_frame_index] = FrameResources::new(
                &self.device,
                draw_data.total_vtx_count as usize,
                draw_data.total_idx_count as usize,
            )?;
        }

        // let _state_guard = StateBackup::backup(&self.context);

        self.update_buffers(draw_data, command_list)?;
        self.setup_render_state(draw_data, command_list);
        self.render_impl(draw_data, command_list)?;

        self.current_frame_index = (self.current_frame_index + 1) % self.frame_count;

        Ok(())
    }

    fn render_impl(&self, draw_data: &DrawData, command_list: &CommandList) -> IDRResult<()> {
        trace!("render_impl call");

        let clip_off = draw_data.display_pos;
        let clip_scale = draw_data.framebuffer_scale;
        let mut vertex_offset = 0;
        let mut index_offset = 0;

        let mut last_tex =
            TextureId::from(self.font_tex_gpu_descriptor_handle.hw_handle.ptr as usize);
        command_list.set_graphics_root_descriptor_table(1, self.font_tex_gpu_descriptor_handle);

        for draw_list in draw_data.draw_lists() {
            for cmd in draw_list.commands() {
                match cmd {
                    DrawCmd::Elements {
                        count,
                        cmd_params:
                            DrawCmdParams {
                                clip_rect,
                                texture_id,
                                ..
                            },
                    } => {
                        if texture_id != last_tex {
                            command_list.set_graphics_root_descriptor_table(
                                1,
                                create_gpu_handle_from_texture_id(
                                    self.font_tex_gpu_descriptor_handle.handle_size,
                                    texture_id,
                                ),
                            );

                            last_tex = texture_id;
                        }

                        let scissor_rect = Rect(D3D12_RECT {
                            left: ((clip_rect[0] - clip_off[0]) * clip_scale[0]) as i32,
                            top: ((clip_rect[1] - clip_off[1]) * clip_scale[1]) as i32,
                            right: ((clip_rect[2] - clip_off[0]) * clip_scale[0]) as i32,
                            bottom: ((clip_rect[3] - clip_off[1]) * clip_scale[1]) as i32,
                        });

                        command_list.set_scissor_rects(slice::from_ref(&scissor_rect));

                        command_list.draw_indexed_instanced(
                            count as u32,
                            1,
                            index_offset as u32,
                            vertex_offset as i32,
                            0,
                        );
                        index_offset += count;
                    }
                    DrawCmd::ResetRenderState => {
                        warn!("ResetRenderState was requested but is not currently implemented");
                        // self.setup_render_state(draw_data, command_list),
                    }
                    DrawCmd::RawCallback { callback, raw_cmd } => unsafe {
                        callback(draw_list.raw(), raw_cmd)
                    },
                }
            }
            vertex_offset += draw_list.vtx_buffer().len();
        }
        Ok(())
    }

    fn setup_render_state(&self, draw_data: &DrawData, command_list: &CommandList) {
        trace!("setup_render_state call");

        let current_resources = &self.frame_resources[self.current_frame_index];

        let viewport = Viewport(D3D12_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: draw_data.display_size[0] * draw_data.framebuffer_scale[0],
            Height: draw_data.display_size[1] * draw_data.framebuffer_scale[1],
            MinDepth: 0.0,
            MaxDepth: 1.0,
        });

        command_list.set_viewports(slice::from_ref(&viewport));

        command_list.set_vertex_buffers(0, slice::from_ref(&current_resources.vertex_buffer_view));
        command_list.set_index_buffer(&current_resources.index_buffer_view);

        trace!(
            "set VB/IB, IB view: {:?}",
            &current_resources.index_buffer_view
        );

        command_list.set_primitive_topology(PrimitiveTopology::TriangleList);

        let l = draw_data.display_pos[0];
        let r = draw_data.display_pos[0] + draw_data.display_size[0];
        let t = draw_data.display_pos[1];
        let b = draw_data.display_pos[1] + draw_data.display_size[1];
        let mvp = [
            [2.0 / (r - l), 0.0, 0.0, 0.0],
            [0.0, 2.0 / (t - b), 0.0, 0.0],
            [0.0, 0.0, 0.5, 0.0],
            [(r + l) / (l - r), (t + b) / (b - t), 0.5, 1.0],
        ];
        let constant_buffer_data = VertexConstantBuffer { mvp };
        let data_view = unsafe {
            slice::from_raw_parts(
                &constant_buffer_data as *const VertexConstantBuffer as *const u32,
                std::mem::size_of::<VertexConstantBuffer>() / std::mem::size_of::<u32>(),
            )
        };

        command_list.set_graphics_root_signature(&self.root_signature);

        command_list.set_graphics_root_32bit_constants(0, data_view, 0);

        command_list.set_pipeline_state(&self.pipeline_state);

        command_list.set_blend_factor([0., 0., 0., 0.]);
    }

    fn update_buffers(&self, draw_data: &DrawData, command_list: &CommandList) -> IDRResult<()> {
        trace!("update_buffers call");

        let mut current_vb_data = self.frame_resources[self.current_frame_index].vertex_buffer_data;
        let mut current_ib_data = self.frame_resources[self.current_frame_index].index_buffer_data;

        for (imgui_vb, imgui_ib) in draw_data
            .draw_lists()
            .map(|draw_list| (draw_list.vtx_buffer(), draw_list.idx_buffer()))
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    imgui_vb.as_ptr() as *mut u8,
                    current_vb_data,
                    imgui_vb.len() * std::mem::size_of::<DrawVert>(),
                );

                current_vb_data =
                    current_vb_data.add(imgui_vb.len() * std::mem::size_of::<DrawVert>());

                trace!(
                    "copied {} bytes to vertex buffer",
                    imgui_vb.len() * std::mem::size_of::<DrawVert>()
                );

                std::ptr::copy_nonoverlapping(
                    imgui_ib.as_ptr() as *mut u8,
                    current_ib_data,
                    imgui_ib.len() * std::mem::size_of::<DrawIdx>(),
                );

                trace!(
                    "copied {} bytes to index buffer",
                    imgui_ib.len() * std::mem::size_of::<DrawIdx>()
                );

                current_ib_data =
                    current_ib_data.add(imgui_ib.len() * std::mem::size_of::<DrawIdx>());
            }
        }

        Ok(())
    }
}

pub fn align_to_multiple(location: u64, alignment: u64) -> u64 {
    (location + (alignment - 1)) & (!(alignment - 1))
}
