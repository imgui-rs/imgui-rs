//! A basic example showing imgui rendering on top of a simple custom scene.

use std::{cell::RefCell, num::NonZeroU32, rc::Rc, time::Instant};

mod utils;

use glow::HasContext;
use glutin::surface::GlSurface;
use utils::Triangler;

struct UserData {
    gl: Rc<glow::Context>,
    fbo: glow::NativeFramebuffer,
    _rbo: glow::NativeRenderbuffer,
}

const FBO_SIZE: i32 = 128;

fn main() {
    let (event_loop, window, surface, context) = utils::create_window("Hello, FBO!", None);
    let (mut winit_platform, mut imgui_context) = utils::imgui_init(&window);
    let gl = utils::glow_context(&context);

    let mut ig_renderer = imgui_glow_renderer::AutoRenderer::initialize(gl, &mut imgui_context)
        .expect("failed to create renderer");
    let tri_renderer = Triangler::new(ig_renderer.gl_context(), "#version 330");

    let fbo;
    let rbo;
    unsafe {
        let gl = ig_renderer.gl_context();
        fbo = gl.create_framebuffer().unwrap();
        rbo = gl.create_renderbuffer().unwrap();

        gl.bind_framebuffer(glow::DRAW_FRAMEBUFFER, Some(fbo));
        gl.bind_renderbuffer(glow::RENDERBUFFER, Some(rbo));
        gl.renderbuffer_storage(glow::RENDERBUFFER, glow::RGBA8, FBO_SIZE, FBO_SIZE);
        gl.framebuffer_renderbuffer(
            glow::DRAW_FRAMEBUFFER,
            glow::COLOR_ATTACHMENT0,
            glow::RENDERBUFFER,
            Some(rbo),
        );
        gl.bind_renderbuffer(glow::RENDERBUFFER, None);

        gl.viewport(0, 0, FBO_SIZE, FBO_SIZE);
        tri_renderer.render(gl);

        gl.bind_framebuffer(glow::DRAW_FRAMEBUFFER, None);
    }

    let data = Rc::new(RefCell::new(UserData {
        gl: Rc::clone(ig_renderer.gl_context()),
        fbo,
        _rbo: rbo,
    }));

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
                    unsafe {
                        ig_renderer.gl_context().clear(glow::COLOR_BUFFER_BIT);
                    }

                    let ui = imgui_context.frame();
                    ui.show_demo_window(&mut true);
                    ui.window("FBO").resizable(false).build(|| {
                        let pos = ui.cursor_screen_pos();
                        ui.set_cursor_screen_pos([
                            pos[0] + FBO_SIZE as f32,
                            pos[1] + FBO_SIZE as f32,
                        ]);

                        let draws = ui.get_window_draw_list();
                        let scale = ui.io().display_framebuffer_scale;
                        let dsp_size = ui.io().display_size;
                        draws
                            .add_callback({
                                let data = Rc::clone(&data);
                                move || {
                                    let data = data.borrow();
                                    let gl = &*data.gl;
                                    unsafe {
                                        let x = pos[0] * scale[0];
                                        let y = (dsp_size[1] - pos[1]) * scale[1];
                                        let dst_x0 = x as i32;
                                        let dst_y0 = (y - FBO_SIZE as f32 * scale[1]) as i32;
                                        let dst_x1 = (x + FBO_SIZE as f32 * scale[0]) as i32;
                                        let dst_y1 = y as i32;
                                        gl.scissor(
                                            dst_x0,
                                            dst_y0,
                                            dst_x1 - dst_x0,
                                            dst_y1 - dst_y0,
                                        );
                                        gl.enable(glow::SCISSOR_TEST);
                                        gl.bind_framebuffer(glow::READ_FRAMEBUFFER, Some(data.fbo));
                                        gl.blit_framebuffer(
                                            0,
                                            0,
                                            FBO_SIZE,
                                            FBO_SIZE,
                                            dst_x0,
                                            dst_y0,
                                            dst_x1,
                                            dst_y1,
                                            glow::COLOR_BUFFER_BIT,
                                            glow::NEAREST,
                                        );
                                        gl.bind_framebuffer(glow::READ_FRAMEBUFFER, None);
                                    }
                                }
                            })
                            .build();
                    });

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
