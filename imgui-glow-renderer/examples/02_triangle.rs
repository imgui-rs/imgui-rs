//! A basic example showing imgui rendering together with some custom rendering.
//!
//! Note this example uses `RendererBuilder` rather than `auto_renderer` and
//! (because we're using the default "trivial" `ContextStateManager`)
//! therefore does not attempt to backup/restore OpenGL state.

use std::time::Instant;

use glow::HasContext;

mod utils;

use utils::Triangler;

fn main() {
    let (event_loop, window) = utils::create_window("Hello, triangle!", glutin::GlRequest::Latest);
    let (mut winit_platform, mut imgui_context) = utils::imgui_init(&window);
    let gl = utils::glow_context(&window);

    let mut ig_renderer = imgui_glow_renderer::RendererBuilder::new()
        .build_owning(gl, &mut imgui_context)
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
                {
                    let gl = ig_renderer.gl_context();
                    // This is required because, without the `StateBackupCsm`
                    // (which is provided by `auto_renderer` but not
                    // `RendererBuilder` by default), the OpenGL context is left
                    // in an arbitrary, dirty state
                    unsafe { gl.disable(glow::SCISSOR_TEST) };
                    tri_renderer.render(gl);
                }

                let ui = imgui_context.frame();
                // Safety: internally, this reference just gets passed as a
                // pointer to imgui, which handles the null pointer properly.
                ui.show_demo_window(unsafe { &mut *std::ptr::null_mut() });

                winit_platform.prepare_render(&ui, window.window());
                let draw_data = ui.render();
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
