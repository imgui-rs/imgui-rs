//! A basic example showing imgui rendering on top of a simple custom scene.

use std::{num::NonZeroU32, time::Instant};

mod utils;

use glutin::surface::GlSurface;
use utils::Triangler;

fn main() {
    let (event_loop, window, surface, context) = utils::create_window("Hello, triangle!", None);
    let (mut winit_platform, mut imgui_context) = utils::imgui_init(&window);
    let gl = utils::glow_context(&context);

    let mut ig_renderer = imgui_glow_renderer::AutoRenderer::initialize(gl, &mut imgui_context)
        .expect("failed to create renderer");
    let tri_renderer = Triangler::new(ig_renderer.gl_context(), "#version 330");

    let mut last_frame = Instant::now();
    event_loop
        .run(move |event, window_target| {
            match event {
                winit::event::Event::NewEvents(_) => {
                    let now = Instant::now();
                    imgui_context
                        .io_mut()
                        .update_delta_time(now.duration_since(last_frame));
                    last_frame = now;
                }
                winit::event::Event::AboutToWait => {
                    winit_platform
                        .prepare_frame(imgui_context.io_mut(), &window)
                        .unwrap();

                    window.request_redraw();
                }
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::RedrawRequested,
                    ..
                } => {
                    // Render your custom scene, note we need to borrow the OpenGL
                    // context from the `AutoRenderer`, which takes ownership of it.
                    tri_renderer.render(ig_renderer.gl_context());

                    let ui = imgui_context.frame();
                    ui.show_demo_window(&mut true);

                    winit_platform.prepare_render(ui, &window);
                    let draw_data = imgui_context.render();

                    // Render imgui on top of it
                    ig_renderer
                        .render(draw_data)
                        .expect("error rendering imgui");

                    surface
                        .swap_buffers(&context)
                        .expect("Failed to swap buffers");
                }
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    window_target.exit();
                }
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::Resized(new_size),
                    ..
                } => {
                    if new_size.width > 0 && new_size.height > 0 {
                        surface.resize(
                            &context,
                            NonZeroU32::new(new_size.width).unwrap(),
                            NonZeroU32::new(new_size.height).unwrap(),
                        );
                    }
                    winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
                }
                winit::event::Event::LoopExiting => {
                    let gl = ig_renderer.gl_context();
                    tri_renderer.destroy(gl);
                }
                event => {
                    winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
                }
            }
        })
        .expect("EventLoop error");
}
