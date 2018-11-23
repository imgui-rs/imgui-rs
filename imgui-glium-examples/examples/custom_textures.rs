extern crate glium;
extern crate imgui;
extern crate imgui_glium_renderer;
extern crate imgui_glutin_support;

use std::borrow::Cow;

use glium::{
    backend::Facade,
    texture::{ClientFormat, RawImage2d},
    Texture2d,
};
use imgui::*;

mod support;
use support::Textures;

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

#[derive(Default)]
struct CustomTexturesApp {
    my_texture_id: Option<ImTexture>,
}

impl CustomTexturesApp {
    fn register_textures<F>(&mut self, gl_ctx: &F, textures: &mut Textures)
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
                data: Cow::Borrowed(&data),
                width: WIDTH as u32,
                height: HEIGHT as u32,
                format: ClientFormat::U8U8U8,
            };
            let gl_texture = Texture2d::new(gl_ctx, raw).expect("Create texture");
            let texture_id = textures.insert(gl_texture);

            self.my_texture_id = Some(texture_id);
        }
    }

    fn show_textures(&self, ui: &Ui) {
        ui.window(im_str!("Hello textures"))
            .size((300.0, 400.0), ImGuiCond::FirstUseEver)
            .build(|| {
                ui.text(im_str!("Hello textures!"));
                if let Some(my_texture_id) = self.my_texture_id {
                    ui.image(my_texture_id, (100.0, 100.0)).build();
                }
            });
    }
}

fn main() {
    let mut my_app = CustomTexturesApp::default();

    support::run(
        "custom_textures.rs".to_owned(),
        CLEAR_COLOR,
        |ui, gl_ctx, textures| {
            my_app.register_textures(gl_ctx, textures);
            my_app.show_textures(ui);

            true
        },
    );
}
