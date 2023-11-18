//! A basic example showing imgui rendering on top of a simple custom scene
//! using OpenGL ES, rather than full-fat OpenGL.
//!
//! Note this example uses `Renderer` rather than `AutoRenderer` and
//! therefore requries more lifetime-management of the OpenGL context.

use std::{num::NonZeroU32, time::Instant};

mod utils;

use glutin::{
    context::{ContextApi, Version},
    surface::GlSurface,
};
use utils::Triangler;

fn main() {
    let (event_loop, window, surface, context) = utils::create_window(
        "Hello, triangle! (GLES 3.0)",
        Some(ContextApi::Gles(Some(Version::new(3, 0)))),
    );
    let (mut winit_platform, mut imgui_context) = utils::imgui_init(&window);
    let gl = utils::glow_context(&context);

    // When using `Renderer`, we need to create a texture map
    let mut texture_map = imgui_glow_renderer::SimpleTextureMap::default();

    // When using `Renderer`, we specify whether or not to output sRGB colors.
    // Since we're drawing to screen and using OpenGL ES (which doesn't support
    // `GL_FRAMEBUFFER_SRGB`) then we do need to convert to sRGB in the shader.
    let mut ig_renderer =
        imgui_glow_renderer::Renderer::initialize(&gl, &mut imgui_context, &mut texture_map, true)
            .expect("failed to create renderer");
    // Note the shader header now needs a precision specifier
    let tri_renderer = Triangler::new(&gl, "#version 300 es\nprecision mediump float;");

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
                    // Draw custom scene
                    tri_renderer.render(&gl);

                    let ui = imgui_context.frame();
                    ui.show_demo_window(&mut true);

                    winit_platform.prepare_render(ui, &window);
                    let draw_data = imgui_context.render();

                    // Render imgui on top
                    ig_renderer
                        .render(&gl, &texture_map, draw_data)
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
                    tri_renderer.destroy(&gl);
                    // Note, to be good citizens we should manually call destroy
                    // when the renderer does not own the GL context
                    ig_renderer.destroy(&gl);
                }
                event => {
                    winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
                }
            }
        })
        .expect("EventLoop error");
}
