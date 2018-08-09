extern crate glium;
#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;

mod support_custom_textures;

use glium::backend::Facade;
use glium::{Surface, Texture2d};
use imgui::{FromImTexture, ImGuiCond, ImTexture};
use imgui_glium_renderer::Texture;

use support_custom_textures::AppContext;

fn main() {
    let mut app = AppContext::init("custom_texture.rs".to_owned(), Default::default()).unwrap();
    let font_id = app.imgui_mut().fonts().get_id();
    let font_size = app.imgui_mut().fonts().get_size();

    let gl_ctx = app.get_context().clone();
    let mut t = 0.0;

    app.run(|ui| {
        ui.window(im_str!("Custom texture"))
            .size((300.0, 400.0), ImGuiCond::FirstUseEver)
            .build(|| {
                // Font texture
                ui.text("Font texture");
                ui.image(&font_id, (font_size.0 as f32, font_size.1 as f32))
                    .build();

                // Constant texture (define once)
                ui.text("Constant texture");
                let constant_texture = ui.make_texture(im_str!("#Constant"), || {
                    let mut image_data: Vec<Vec<(f32, f32, f32, f32)>> = Vec::new();
                    for i in 0..100 {
                        let mut row: Vec<(f32, f32, f32, f32)> = Vec::new();
                        for j in 0..100 {
                            row.push((i as f32 / 100.0, j as f32 / 100.0, 0.0, 1.0));
                        }
                        image_data.push(row);
                    }
                    Texture2d::new(&gl_ctx, image_data).unwrap()
                });
                let size = constant_texture.get_size();
                ui.image(&constant_texture, (size.0 as f32, size.1 as f32))
                    .build();

                // Changing texture (re-defined and swap texture for each frame)
                ui.text("Variable texture");
                let changing_texture = ui.replace_texture(im_str!("#Changing"), {
                    let mut image_data: Vec<Vec<(f32, f32, f32, f32)>> = Vec::new();
                    for i in 0..100 {
                        let mut row: Vec<(f32, f32, f32, f32)> = Vec::new();
                        for j in 0..100 {
                            row.push((i as f32 / 100.0, j as f32 / 100.0, t, 1.0));
                        }
                        image_data.push(row);
                    }
                    t += 0.01;
                    if t > 1.0 {
                        t = 0.0;
                    }
                    Texture2d::new(&gl_ctx, image_data).unwrap()
                });
                let size = changing_texture.get_size();
                ui.image(&changing_texture, (size.0 as f32, size.1 as f32))
                    .build();

                // Texture defined only once, however, you can dynamically draw on it.
                ui.text("Draw on texture");
                let draw_texture = ui.make_texture(im_str!("#Draw"), || {
                    Texture2d::empty(&gl_ctx, 100, 100).unwrap()
                });
                // Get the texture as a surface. It must first be converted to a
                // `glium::Texture2d` object by using `Texture::from`.
                let mut surface = Texture::from_im_texture(&draw_texture).as_surface();
                surface.clear_color(1.0, 0.0, 0.0, 1.0);
                let size = draw_texture.get_size();
                ui.image(&draw_texture, (size.0 as f32, size.1 as f32))
                    .build();
            });
        true
    }).unwrap();
}
