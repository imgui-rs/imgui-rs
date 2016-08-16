#[macro_use]
extern crate glium;
#[macro_use]
extern crate imgui;

use imgui::*;

use self::support::Support;

mod support;

const CLEAR_COLOR: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 1.0);

fn main() {
    let mut support = Support::init();

    loop {
        support.render(CLEAR_COLOR, hello_world);
        let active = support.update_events();
        if !active { break }
    }
}

fn hello_world<'a>(ui: &Ui<'a>) {
    ui.window("Hello world")
        .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
            ui.text("Hello world!");
            ui.text("This...is...imgui-rs!");
            ui.separator();
            let mouse_pos = ui.imgui().mouse_pos();
            ui.text(&format!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
        })
}
