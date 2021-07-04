//! A basic example showing imgui rendering together with some custom rendering
//! using OpenGL ES, rather than full-fat OpenGL.
//!
//! Note this example uses `Renderer` rather than `OwningRenderer` and
//! therefore requries more lifetime-management of the OpenGL context.

use std::time::Instant;

mod utils;

use utils::Triangler;

fn main() {
    let (event_loop, window) = utils::create_window(
        "Hello, triangle! (GLES 3.0)",
        glutin::GlRequest::Specific(glutin::Api::OpenGlEs, (3, 0)),
    );
    let (mut winit_platform, mut imgui_context) = utils::imgui_init(&window);
    let gl = utils::glow_context(&window);

    let mut texture_map = imgui_glow_renderer::SimpleTextureMap::default();
    let mut ig_renderer =
        imgui_glow_renderer::Renderer::initialize(&gl, &mut imgui_context, &mut texture_map)
            .expect("failed to create renderer");
    // Note the shader header now needs a precision specifier
    let tri_renderer = Triangler::new(
        &gl,
        "#version 300 es\nprecision mediump float;\n#define IS_GLES",
    );

    let mut last_frame = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Wait;
        match event {
            glutin::event::Event::NewEvents(_) => {
                let now = Instant::now();
                imgui_context
                    .io_mut()
                    .update_delta_time(now.duration_since(last_frame));
                last_frame = now;
            }
            glutin::event::Event::MainEventsCleared => {
                winit_platform
                    .prepare_frame(imgui_context.io_mut(), window.window())
                    .unwrap();

                window.window().request_redraw();
            }
            glutin::event::Event::RedrawRequested(_) => {
                tri_renderer.render(&gl);

                let ui = imgui_context.frame();
                ui.show_demo_window(&mut true);

                winit_platform.prepare_render(&ui, window.window());
                let draw_data = ui.render();
                ig_renderer
                    .render(&gl, &texture_map, &draw_data)
                    .expect("error rendering imgui");

                window.swap_buffers().unwrap();
            }
            glutin::event::Event::WindowEvent {
                event: glutin::event::WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            }
            glutin::event::Event::LoopDestroyed => {
                tri_renderer.destroy(&gl);
                // Note, to be good citizens we should manually call destroy
                // when the renderer does not own the GL context
                ig_renderer.destroy(&gl);
            }
            event => {
                winit_platform.handle_event(imgui_context.io_mut(), window.window(), &event);
            }
        }
    });
}
