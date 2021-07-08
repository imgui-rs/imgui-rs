//! A basic example showing imgui rendering on top of a simple custom scene.

use std::time::Instant;

mod utils;

use utils::Triangler;

fn main() {
    let (event_loop, window) = utils::create_window("Hello, triangle!", glutin::GlRequest::Latest);
    let (mut winit_platform, mut imgui_context) = utils::imgui_init(&window);
    let gl = utils::glow_context(&window);

    let mut ig_renderer = imgui_glow_renderer::AutoRenderer::initialize(gl, &mut imgui_context)
        .expect("failed to create renderer");
    let tri_renderer = Triangler::new(ig_renderer.gl_context(), "#version 330");

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
                // Render your custom scene, note we need to borrow the OpenGL
                // context from the `AutoRenderer`, which takes ownership of it.
                tri_renderer.render(ig_renderer.gl_context());

                let ui = imgui_context.frame();
                ui.show_demo_window(&mut true);

                winit_platform.prepare_render(&ui, window.window());
                let draw_data = ui.render();

                // Render imgui on top of it
                ig_renderer
                    .render(&draw_data)
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
                let gl = ig_renderer.gl_context();
                tri_renderer.destroy(gl);
            }
            event => {
                winit_platform.handle_event(imgui_context.io_mut(), window.window(), &event);
            }
        }
    });
}
