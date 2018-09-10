#[macro_use]
extern crate glium;
#[macro_use]
extern crate imgui;
extern crate imgui_glium_textured_renderer;

use glium::backend::Context;
use glium::Texture2d;
use imgui::*;
use std::rc::Rc;
mod support_texture;

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() {
    let mut init = false;
    let mut t = 0.0;

    support_texture::run("hello_world.rs".to_owned(), CLEAR_COLOR, |ui, gl_ctx| {
        if !init {
            let mut image_data: Vec<Vec<(f32, f32, f32, f32)>> = Vec::new();
            for i in 0..100 {
                let mut row: Vec<(f32, f32, f32, f32)> = Vec::new();
                for j in 0..100 {
                    row.push((i as f32 / 100.0, j as f32 / 100.0, 0.0, 1.0));
                }
                image_data.push(row);
            }
            let texture = Texture2d::new(gl_ctx, image_data).unwrap();

            ui.make_texture("#custom".into(), Rc::new(texture));
            ui.make_texture(
                "#changing".into(),
                generate_changing_texture(&mut t, gl_ctx),
            );
            init = true;
        }

        ui.replace_texture(
            "#changing".into(),
            generate_changing_texture(&mut t, gl_ctx),
        ).expect("should be able to replace textures");

        ui.window(im_str!("Hello world"))
            .size((300.0, 100.0), ImGuiCond::FirstUseEver)
            .build(|| {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("こんにちは世界！"));
                ui.text(im_str!("This...is...imgui-rs!"));
                ui.separator();
                let mouse_pos = ui.imgui().mouse_pos();
                ui.text(im_str!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos.0,
                    mouse_pos.1
                ));
            });

        ui.window(im_str!("Custom Texture"))
            .size((300.0, 100.0), ImGuiCond::FirstUseEver)
            .build(|| {
                ui.image("#custom", (100.0, 100.0)).build();
            });

        // Changing texture (re-defined and swap texture for each frame)
        ui.window(im_str!("Changing Texture"))
            .size((300.0, 100.0), ImGuiCond::FirstUseEver)
            .build(|| {
                ui.text(im_str!("Variable texture - {:?}", t));
                ui.image("#changing", (100.0, 100.0)).build();
            });

        true
    });
}

fn generate_changing_texture(t: &mut f32, gl_ctx: &Rc<Context>) -> Rc<Texture2d> {
    let mut image_data: Vec<Vec<(f32, f32, f32, f32)>> = Vec::new();

    for i in 0..100 {
        let mut row: Vec<(f32, f32, f32, f32)> = Vec::new();
        for j in 0..100 {
            row.push((i as f32 / 100.0, j as f32 / 100.0, *t, 1.0));
        }
        image_data.push(row);
    }

    *t += 0.01;
    if *t > 1.0 {
        *t = 0.0;
    }
    Rc::new(Texture2d::new(gl_ctx, image_data).unwrap())
}
