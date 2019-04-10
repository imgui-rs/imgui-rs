use std::borrow::Cow;
use std::error::Error;
use std::io::Cursor;

use glium::{
    backend::Facade,
    texture::{ClientFormat, RawImage2d},
    Texture2d,
};
use image::{jpeg::JPEGDecoder, ImageDecoder};
use imgui::*;

mod support;
use self::support::Textures;

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

#[derive(Default)]
struct CustomTexturesApp {
    my_texture_id: Option<ImTexture>,
    lenna: Option<Lenna>,
}

struct Lenna {
    texture_id: ImTexture,
    size: (f32, f32),
}

impl CustomTexturesApp {
    fn register_textures<F>(
        &mut self,
        gl_ctx: &F,
        textures: &mut Textures,
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
            let texture_id = textures.insert(gl_texture);

            self.my_texture_id = Some(texture_id);
        }

        if self.lenna.is_none() {
            self.lenna = Some(Lenna::new(gl_ctx, textures)?);
        }

        Ok(())
    }

    fn show_textures(&self, ui: &Ui) {
        ui.window(im_str!("Hello textures"))
            .size((400.0, 600.0), ImGuiCond::FirstUseEver)
            .build(|| {
                ui.text(im_str!("Hello textures!"));
                if let Some(my_texture_id) = self.my_texture_id {
                    ui.text("Some generated texture");
                    ui.image(my_texture_id, (100.0, 100.0)).build();
                }

                if let Some(lenna) = &self.lenna {
                    ui.text("Say hello to Lenna.jpg");
                    lenna.show(ui);
                }
            });
    }
}

impl Lenna {
    fn new<F>(gl_ctx: &F, textures: &mut Textures) -> Result<Self, Box<dyn Error>>
    where
        F: Facade,
    {
        let lenna_bytes = include_bytes!("../../resources/Lenna.jpg");
        let byte_stream = Cursor::new(lenna_bytes.as_ref());
        let decoder = JPEGDecoder::new(byte_stream)?;

        let (width, height) = decoder.dimensions();
        let image = decoder.read_image()?;
        let raw = RawImage2d {
            data: Cow::Owned(image),
            width: width as u32,
            height: height as u32,
            format: ClientFormat::U8U8U8,
        };
        let gl_texture = Texture2d::new(gl_ctx, raw)?;
        let texture_id = textures.insert(gl_texture);
        Ok(Lenna {
            texture_id,
            size: (width as f32, height as f32),
        })
    }

    fn show(&self, ui: &Ui) {
        ui.image(self.texture_id, self.size).build();
    }
}

fn main() {
    let mut my_app = CustomTexturesApp::default();

    support::run(
        "custom_textures.rs".to_owned(),
        CLEAR_COLOR,
        |ui, gl_ctx, textures| {
            if let Err(e) = my_app.register_textures(gl_ctx, textures) {
                panic!("Failed to register textures! {}", e);
            }
            my_app.show_textures(ui);

            true
        },
    );
}
