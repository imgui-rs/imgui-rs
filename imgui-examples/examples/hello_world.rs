extern crate glium;
extern crate imgui;
extern crate imgui_glium_renderer;

use imgui::*;

mod support;

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() { support::run("hellow_world.rs".to_owned(), CLEAR_COLOR, hello_world); }

fn hello_world<'a>(ui: &Ui<'a>) -> bool {
    ui.window("Hello world")
        .size((300.0, 100.0), ImGuiCond::FirstUseEver)
        .build(|| {
            ui.text("Hello world!");
            ui.text("This...is...imgui-rs!");
            ui.separator();
            let mouse_pos = ui.imgui().mouse_pos();
            ui.text(&format!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
        });

    true
}
