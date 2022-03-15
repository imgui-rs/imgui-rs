use std::mem::size_of;

use glow::HasContext;
use glutin::{PossiblyCurrent, event_loop::ControlFlow, event::WindowEvent};
use imgui::DrawVert;


fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();

    let mut imgui = imgui::Context::create();

    let mut main_viewport = Viewport::new(&event_loop, &mut imgui);

    let mut winit_platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
    // imgui_winit_support::WinitPlatform::init_viewports(&mut imgui, main_viewport.window.window(), &event_loop);

    event_loop.run(move |event, window_target, control_flow| {
        winit_platform.handle_event(imgui.io_mut(), main_viewport.window.window(), &event);

        match event {
            glutin::event::Event::WindowEvent { window_id, event: WindowEvent::CloseRequested } => {
                if window_id == main_viewport.window.window().id() {
                    *control_flow = ControlFlow::Exit;
                }
            },
            glutin::event::Event::MainEventsCleared => {
                main_viewport.window.window().request_redraw();
            },
            glutin::event::Event::RedrawRequested(window_id) => {
                if window_id == main_viewport.window.window().id() {
                    winit_platform.prepare_frame(imgui.io_mut(), main_viewport.window.window()).unwrap();
                    render(&mut imgui, &mut main_viewport);
                }
            },
            _ => {}
        }
    });
}

fn render(imgui: &mut imgui::Context, viewport: &mut Viewport) {
    let ui = imgui.new_frame();

    let mut open = true;
    ui.show_demo_window(&mut open);

    let draw_data = imgui.render();

    unsafe {
        //viewport.window = viewport.window.make_current().unwrap();
        viewport.context.disable(glow::SCISSOR_TEST);
        viewport.context.clear(glow::COLOR_BUFFER_BIT);
        viewport.context.enable(glow::SCISSOR_TEST);

        viewport.context.bind_vertex_array(Some(viewport.vao));
        viewport.context.use_program(Some(viewport.shader));

        let left = draw_data.display_pos[0];
        let right = draw_data.display_pos[0] + draw_data.display_size[0];
        let top = draw_data.display_pos[1];
        let bottom = draw_data.display_pos[1] + draw_data.display_size[1];
        let matrix = [
            (2.0 / (right - left)), 0.0, 0.0, 0.0,
            0.0, (2.0 / (top - bottom)), 0.0, 0.0,
            0.0, 0.0, -1.0, 0.0,
            
            (right + left) / (left - right),
            (top + bottom) / (bottom - top),
            0.0,
            1.0,
        ];

        let loc = viewport.context.get_uniform_location(viewport.shader, "u_Matrix").unwrap();
        viewport.context.uniform_matrix_4_f32_slice(Some(&loc), false, &matrix);

        viewport.context.blend_func_separate(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA, glow::ONE, glow::ZERO);
        viewport.context.enable(glow::BLEND);

        viewport.context.bind_buffer(glow::ARRAY_BUFFER, Some(viewport.vbo));
        viewport.context.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(viewport.ibo));

        viewport.context.bind_texture(glow::TEXTURE_2D, Some(viewport.font_tex));

        viewport.context.viewport(0, 0, viewport.window.window().inner_size().width as i32, viewport.window.window().inner_size().height as i32);
    }

    for draw_list in draw_data.draw_lists() {
        unsafe {
            viewport.context.buffer_data_u8_slice(glow::ARRAY_BUFFER, std::slice::from_raw_parts(draw_list.vtx_buffer().as_ptr() as *const u8, draw_list.vtx_buffer().len() * size_of::<DrawVert>()), glow::STREAM_DRAW);
            viewport.context.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, std::slice::from_raw_parts(draw_list.idx_buffer().as_ptr() as *const u8, draw_list.idx_buffer().len() * size_of::<u16>()), glow::STREAM_DRAW);
            viewport.context.bind_vertex_buffer(0, Some(viewport.vbo), 0, size_of::<DrawVert>() as i32);
        }

        for cmd in draw_list.commands() {
            match cmd {
                imgui::DrawCmd::Elements { count, cmd_params } => {
                    unsafe {
                        let window_height = viewport.window.window().inner_size().height as i32;

                        let x = cmd_params.clip_rect[0] as i32;
                        let y = cmd_params.clip_rect[1] as i32;
                        let width = (cmd_params.clip_rect[2] - cmd_params.clip_rect[0]) as i32;
                        let height = (cmd_params.clip_rect[3] - cmd_params.clip_rect[1]) as i32;

                        viewport.context.scissor(
                            x,
                            window_height - (y + height),
                            width,
                            height
                        );
                        viewport.context.enable(glow::SCISSOR_TEST);
                        viewport.context.draw_elements_base_vertex(glow::TRIANGLES, count as i32, glow::UNSIGNED_SHORT, (cmd_params.idx_offset * size_of::<u16>()) as i32, cmd_params.vtx_offset as i32);
                    }
                },
                _ => {},
            }
        }
    }

    viewport.window.swap_buffers().unwrap();
}

struct Viewport {
    window: glutin::ContextWrapper<PossiblyCurrent, glutin::window::Window>,
    context: glow::Context,
    vao: glow::VertexArray,
    vbo: glow::Buffer,
    ibo: glow::Buffer,
    shader: glow::Program,
    font_tex: glow::Texture,
}

impl Viewport {
    fn new<T>(event_loop: &glutin::event_loop::EventLoopWindowTarget<T>, imgui: &mut imgui::Context) -> Self {
        let wb = glutin::window::WindowBuilder::new()
            .with_inner_size(glutin::dpi::LogicalSize::new(800.0, 600.0))
            .with_resizable(true)
            .with_title("Viewports")
            .with_visible(true);
        let window = unsafe{glutin::ContextBuilder::new()
            .with_double_buffer(Some(true))
            .with_vsync(true)
            .build_windowed(wb, &event_loop)
            .unwrap()
            .make_current()
            .unwrap()};

        let context = unsafe{glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _)};

        let (vao, vbo, ibo, shader, font_tex) = unsafe {
            let vao = context.create_vertex_array().unwrap();
            let vbo = context.create_buffer().unwrap();
            let ibo = context.create_buffer().unwrap();

            context.bind_vertex_array(Some(vao));
            context.vertex_attrib_binding(0, 0);
            context.vertex_attrib_binding(1, 0);
            context.vertex_attrib_binding(2, 0);
            context.vertex_attrib_format_f32(0, 2, glow::FLOAT, false, 0);
            context.vertex_attrib_format_f32(1, 2, glow::FLOAT, false, 8);
            context.vertex_attrib_format_f32(2, 4, glow::UNSIGNED_BYTE, true, 16);
            context.enable_vertex_attrib_array(0);
            context.enable_vertex_attrib_array(1);
            context.enable_vertex_attrib_array(2);
            context.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
            context.bind_vertex_array(None);
            
            let vertex_shader = context.create_shader(glow::VERTEX_SHADER).unwrap();
            context.shader_source(vertex_shader, VERTEX_SHADER);
            context.compile_shader(vertex_shader);

            let fragment_shader = context.create_shader(glow::FRAGMENT_SHADER).unwrap();
            context.shader_source(fragment_shader, FRAGMENT_SHADER);
            context.compile_shader(fragment_shader);

            let program = context.create_program().unwrap();
            context.attach_shader(program, vertex_shader);
            context.attach_shader(program, fragment_shader);
            context.link_program(program);
            
            context.delete_shader(vertex_shader);
            context.delete_shader(fragment_shader);

            let font_tex = context.create_texture().unwrap();
            let data = imgui.fonts().build_rgba32_texture();
            context.bind_texture(glow::TEXTURE_2D, Some(font_tex));
            context.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA as i32, data.width as i32, data.height as i32, 0, glow::RGBA, glow::UNSIGNED_BYTE, Some(data.data));
            context.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
            context.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
            context.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
            context.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);

            (vao, vbo, ibo, program, font_tex)
        };

        Self {
            window,
            context,
            vao,
            vbo,
            ibo,
            shader,
            font_tex,
        }
    }
}

const VERTEX_SHADER: &'static str = "#version 450 core

layout(location = 0) in vec2 in_Position;
layout(location = 1) in vec2 in_UV;
layout(location = 2) in vec4 in_Color;

out vec2 v2f_UV;
out vec4 v2f_Color;

uniform mat4 u_Matrix;

void main() {
    gl_Position = u_Matrix * vec4(in_Position, 0.0, 1.0);
    v2f_UV = in_UV;
    v2f_Color = in_Color;
}

";

const FRAGMENT_SHADER: &'static str = "#version 450 core

in vec2 v2f_UV;
in vec4 v2f_Color;

layout(location = 0) uniform sampler2D u_FontTexture;

out vec4 out_Color;

void main() {
    vec4 texColor = texture(u_FontTexture, v2f_UV);
    vec4 finalColor = texColor * v2f_Color;

    out_Color = finalColor;
}

";
