extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate imgui;
extern crate imgui_gfx_renderer;

use imgui::*;

mod support_gfx;

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() { support_gfx::run("hello_gfx.rs".to_owned(), CLEAR_COLOR, hello_world); }

fn hello_world<'a>(ui: &Ui<'a>) -> bool {
    ui.window("Hello world")
        .size((300.0, 100.0), ImGuiCond::FirstUseEver)
        .build(|| {
            ui.text("Hello world!");
            ui.text("This...is...imgui-rs!");
            ui.separator();
            let mouse_pos = ui.imgui().mouse_pos();
            ui.text(&format!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos.0,
                mouse_pos.1
            ));
        });

    true
}
