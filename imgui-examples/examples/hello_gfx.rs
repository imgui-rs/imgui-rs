extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
#[macro_use]
extern crate imgui;
extern crate imgui_gfx_renderer;
extern crate imgui_sys;

use imgui::*;

mod support_gfx;

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() { support_gfx::run("hello_gfx.rs".to_owned(), CLEAR_COLOR, hello_world); }

fn hello_world<'a>(ui: &Ui<'a>) -> bool {
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

    true
}
