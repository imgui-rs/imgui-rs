use std::borrow::Cow;
use std::error::Error;
use std::io::Cursor;
use std::rc::Rc;

use glium::{
    backend::Facade,
    texture::{ClientFormat, RawImage2d},
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter, SamplerBehavior},
    Texture2d,
};
use image::{jpeg::JpegDecoder, ImageDecoder};
use imgui::*;
use imgui_glium_renderer::Texture;

mod support;

#[derive(Default)]
struct CustomTexturesApp {
    my_texture_id: Option<TextureId>,
    lenna: Option<Lenna>,
}

struct Lenna {
    texture_id: TextureId,
    size: [f32; 2],
}

impl CustomTexturesApp {
    fn register_textures<F>(
        &mut self,
        gl_ctx: &F,
        textures: &mut Textures<Texture>,
    ) -> Result<(), Box<dyn Error>>
    where
        F: Facade,
    {
        const WIDTH: usize = 100;
        const HEIGHT: usize = 100;

        if self.my_texture_id.is_none() {
            // Generate dummy texture
            let mut data = Vec::with_capacity(WIDTH * HEIGHT);
            for i in 0..WIDTH {
                for j in 0..HEIGHT {
                    // Insert RGB values
                    data.push(i as u8);
                    data.push(j as u8);
                    data.push((i + j) as u8);
                }
            }

            let raw = RawImage2d {
                data: Cow::Owned(data),
                width: WIDTH as u32,
                height: HEIGHT as u32,
                format: ClientFormat::U8U8U8,
            };
            let gl_texture = Texture2d::new(gl_ctx, raw)?;
            let texture = Texture {
                texture: Rc::new(gl_texture),
                sampler: SamplerBehavior {
                    magnify_filter: MagnifySamplerFilter::Linear,
                    minify_filter: MinifySamplerFilter::Linear,
                    ..Default::default()
                },
            };
            let texture_id = textures.insert(texture);

            self.my_texture_id = Some(texture_id);
        }

        if self.lenna.is_none() {
            self.lenna = Some(Lenna::new(gl_ctx, textures)?);
        }

        Ok(())
    }

    fn show_textures(&self, ui: &Ui) {
        Window::new(im_str!("Hello textures"))
            .size([400.0, 400.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Hello textures!"));
                if let Some(my_texture_id) = self.my_texture_id {
                    ui.text("Some generated texture");
                    Image::new(my_texture_id, [100.0, 100.0]).build(ui);
                }

                if let Some(lenna) = &self.lenna {
                    ui.text("Say hello to Lenna.jpg");
                    lenna.show(ui);
                }

                // Example of using custom textures on a button
                if let Some(lenna) = &self.lenna {
                    ui.text("The Lenna buttons");

                    {
                        ui.invisible_button(im_str!("Boring Button"), [100.0, 100.0]);
                        // See also `imgui::Ui::style_color`
                        let tint_none = [1.0, 1.0, 1.0, 1.0];
                        let tint_green = [0.5, 1.0, 0.5, 1.0];
                        let tint_red = [1.0, 0.5, 0.5, 1.0];

                        let tint = match (
                            ui.is_item_hovered(),
                            ui.is_mouse_down(imgui::MouseButton::Left),
                        ) {
                            (false, false) => tint_none,
                            (false, true) => tint_none,
                            (true, false) => tint_green,
                            (true, true) => tint_red,
                        };

                        let draw_list = ui.get_window_draw_list();
                        draw_list
                            .add_image(lenna.texture_id, ui.item_rect_min(), ui.item_rect_max())
                            .col(tint)
                            .build();
                    }

                    {
                        ui.same_line(0.0);

                        // Button using quad positioned image
                        ui.invisible_button(im_str!("Exciting Button"), [100.0, 100.0]);

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
                            .add_image_quad(lenna.texture_id, tl, tr, br, bl)
                            .build();
                    }

                    // Rounded image
                    {
                        ui.same_line(0.0);
                        ui.invisible_button(im_str!("Smooth Button"), [100.0, 100.0]);

                        let draw_list = ui.get_window_draw_list();
                        draw_list
                            .add_image_rounded(
                                lenna.texture_id,
                                ui.item_rect_min(),
                                ui.item_rect_max(),
                                16.0,
                            )
                            // Tint brighter for visiblity of corners
                            .col([2.0, 0.5, 0.5, 1.0])
                            // Rounding on each corner can be changed separately
                            .round_top_left((ui.frame_count() + 0) / 60 % 4 == 0)
                            .round_top_right((ui.frame_count() + 1) / 60 % 4 == 1)
                            .round_bot_right((ui.frame_count() + 3) / 60 % 4 == 2)
                            .round_bot_left((ui.frame_count() + 2) / 60 % 4 == 3)
                            .build();
                    }
                }
            });
    }
}

impl Lenna {
    fn new<F>(gl_ctx: &F, textures: &mut Textures<Texture>) -> Result<Self, Box<dyn Error>>
    where
        F: Facade,
    {
        let lenna_bytes = include_bytes!("../../resources/Lenna.jpg");
        let byte_stream = Cursor::new(lenna_bytes.as_ref());
        let decoder = JpegDecoder::new(byte_stream)?;

        let (width, height) = decoder.dimensions();
        let mut image = vec![0; decoder.total_bytes() as usize];
        decoder.read_image(&mut image)?;
        let raw = RawImage2d {
            data: Cow::Owned(image),
            width: width as u32,
            height: height as u32,
            format: ClientFormat::U8U8U8,
        };
        let gl_texture = Texture2d::new(gl_ctx, raw)?;
        let texture = Texture {
            texture: Rc::new(gl_texture),
            sampler: SamplerBehavior {
                magnify_filter: MagnifySamplerFilter::Linear,
                minify_filter: MinifySamplerFilter::Linear,
                ..Default::default()
            },
        };
        let texture_id = textures.insert(texture);
        Ok(Lenna {
            texture_id,
            size: [width as f32, height as f32],
        })
    }

    fn show(&self, ui: &Ui) {
        Image::new(self.texture_id, self.size).build(ui);
    }
}

fn main() {
    let mut my_app = CustomTexturesApp::default();

    let mut system = support::init(file!());
    my_app
        .register_textures(system.display.get_context(), system.renderer.textures())
        .expect("Failed to register textures");
    system.main_loop(move |_, ui| my_app.show_textures(ui));
}
