//! Example showing the same functionality as
//! `imgui-examples/examples/custom_textures.rs`
//!
//! Not that the texture uses the internal format `glow::SRGB`, so that
//! OpenGL automatically converts colors to linear space before the shaders.
//! The renderer assumes you set this internal format correctly like this.

use std::{io::Cursor, num::NonZeroU32, time::Instant};

use glow::HasContext;
use glutin::surface::GlSurface;
use image::{jpeg::JpegDecoder, ImageDecoder};
use imgui::Condition;

use imgui_glow_renderer::Renderer;
use winit::event_loop::ControlFlow;

#[allow(dead_code)]
mod utils;

const LENNA_JPEG: &[u8] = include_bytes!("../../resources/Lenna.jpg");

fn main() {
    let (event_loop, window, surface, context) = utils::create_window("Custom textures", None);
    let (mut winit_platform, mut imgui_context) = utils::imgui_init(&window);
    let gl = utils::glow_context(&context);
    // This time, we tell OpenGL this is an sRGB framebuffer and OpenGL will
    // do the conversion to sSGB space for us after the fragment shader.
    unsafe { gl.enable(glow::FRAMEBUFFER_SRGB) };

    let mut textures = imgui::Textures::<glow::Texture>::default();
    // Note that `output_srgb` is `false`. This is because we set
    // `glow::FRAMEBUFFER_SRGB` so we don't have to manually do the conversion
    // in the shader.
    let mut ig_renderer = Renderer::initialize(&gl, &mut imgui_context, &mut textures, false)
        .expect("failed to create renderer");
    let textures_ui = TexturesUi::new(&gl, &mut textures);

    let mut last_frame = Instant::now();
    event_loop
        .run(move |event, window_target| {
            // Note we can potentially make the loop more efficient by
            // changing the `Poll` (default) value to `ControlFlow::Wait`
            // but be careful to test on all target platforms!
            window_target.set_control_flow(ControlFlow::Poll);

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
                    unsafe { gl.clear(glow::COLOR_BUFFER_BIT) };

                    let ui = imgui_context.frame();
                    textures_ui.show(ui);

                    winit_platform.prepare_render(ui, &window);
                    let draw_data = imgui_context.render();
                    ig_renderer
                        .render(&gl, &textures, draw_data)
                        .expect("error rendering imgui");

                    surface
                        .swap_buffers(&context)
                        .expect("Failed to swap buffers");
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
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    window_target.exit();
                }
                winit::event::Event::LoopExiting => {
                    ig_renderer.destroy(&gl);
                }
                event => {
                    winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
                }
            }
        })
        .expect("EventLoop error");
}

struct TexturesUi {
    generated_texture: imgui::TextureId,
    lenna: Lenna,
}

impl TexturesUi {
    fn new(gl: &glow::Context, textures: &mut imgui::Textures<glow::Texture>) -> Self {
        Self {
            generated_texture: Self::generate(gl, textures),
            lenna: Lenna::load(gl, textures),
        }
    }

    /// Generate dummy texture
    fn generate(
        gl: &glow::Context,
        textures: &mut imgui::Textures<glow::Texture>,
    ) -> imgui::TextureId {
        const WIDTH: usize = 100;
        const HEIGHT: usize = 100;

        let mut data = Vec::with_capacity(WIDTH * HEIGHT);
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                // Insert RGB values
                data.push(i as u8);
                data.push(j as u8);
                data.push((i + j) as u8);
            }
        }

        let gl_texture = unsafe { gl.create_texture() }.expect("unable to create GL texture");

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
                glow::RGB as _, // When generating a texture like this, you're probably working in linear color space
                WIDTH as _,
                HEIGHT as _,
                0,
                glow::RGB,
                glow::UNSIGNED_BYTE,
                Some(&data),
            )
        }

        textures.insert(gl_texture)
    }

    fn show(&self, ui: &imgui::Ui) {
        ui.window("Hello textures")
            .size([400.0, 400.0], Condition::FirstUseEver)
            .build(|| {
                ui.text("Hello textures!");
                ui.text("Some generated texture");
                imgui::Image::new(self.generated_texture, [100.0, 100.0]).build(ui);

                ui.text("Say hello to Lenna.jpg");
                self.lenna.show(ui);

                // Example of using custom textures on a button
                ui.text("The Lenna buttons");
                {
                    ui.invisible_button("Boring Button", [100.0, 100.0]);
                    // See also `imgui::Ui::style_color`
                    let tint_none = [1.0, 1.0, 1.0, 1.0];
                    let tint_green = [0.5, 1.0, 0.5, 1.0];
                    let tint_red = [1.0, 0.5, 0.5, 1.0];

                    let tint = match (
                        ui.is_item_hovered(),
                        ui.is_mouse_down(imgui::MouseButton::Left),
                    ) {
                        (false, _) => tint_none,
                        (true, false) => tint_green,
                        (true, true) => tint_red,
                    };

                    let draw_list = ui.get_window_draw_list();
                    draw_list
                        .add_image(
                            self.lenna.texture_id,
                            ui.item_rect_min(),
                            ui.item_rect_max(),
                        )
                        .col(tint)
                        .build();
                }

                {
                    ui.same_line();

                    // Button using quad positioned image
                    ui.invisible_button("Exciting Button", [100.0, 100.0]);

                    // Button bounds
                    let min = ui.item_rect_min();
                    let max = ui.item_rect_max();

                    // get corner coordinates
                    let tl = [
                        min[0],
                        min[1] + (ui.frame_count() as f32 / 10.0).cos() * 10.0,
                    ];
                    let tr = [
                        max[0],
                        min[1] + (ui.frame_count() as f32 / 10.0).sin() * 10.0,
                    ];
                    let bl = [min[0], max[1]];
                    let br = max;

                    let draw_list = ui.get_window_draw_list();
                    draw_list
                        .add_image_quad(self.lenna.texture_id, tl, tr, br, bl)
                        .build();
                }

                // Rounded image
                {
                    ui.same_line();
                    ui.invisible_button("Smooth Button", [100.0, 100.0]);

                    let draw_list = ui.get_window_draw_list();
                    draw_list
                        .add_image_rounded(
                            self.lenna.texture_id,
                            ui.item_rect_min(),
                            ui.item_rect_max(),
                            16.0,
                        )
                        // Tint brighter for visiblity of corners
                        .col([2.0, 0.5, 0.5, 1.0])
                        // Rounding on each corner can be changed separately
                        .round_top_left(ui.frame_count() / 60 % 4 == 0)
                        .round_top_right((ui.frame_count() + 1) / 60 % 4 == 1)
                        .round_bot_right((ui.frame_count() + 3) / 60 % 4 == 2)
                        .round_bot_left((ui.frame_count() + 2) / 60 % 4 == 3)
                        .build();
                }
            });
    }
}

struct Lenna {
    texture_id: imgui::TextureId,
    size: [f32; 2],
}

impl Lenna {
    fn load(gl: &glow::Context, textures: &mut imgui::Textures<glow::Texture>) -> Self {
        let decoder = JpegDecoder::new(Cursor::new(LENNA_JPEG)).expect("could not create decoder");
        let (width, height) = decoder.dimensions();

        let lenna_image = {
            let mut bytes = vec![0; decoder.total_bytes() as usize];
            decoder
                .read_image(&mut bytes)
                .expect("unable to decode jpeg");
            bytes
        };

        let gl_texture = unsafe { gl.create_texture() }.expect("unable to create GL texture");

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
                glow::SRGB as _, // image file has sRGB encoded colors
                width as _,
                height as _,
                0,
                glow::RGB,
                glow::UNSIGNED_BYTE,
                Some(&lenna_image),
            )
        }

        Self {
            texture_id: textures.insert(gl_texture),
            size: [width as _, height as _],
        }
    }

    fn show(&self, ui: &imgui::Ui) {
        imgui::Image::new(self.texture_id, self.size).build(ui);
    }
}
