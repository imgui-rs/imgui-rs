#[macro_use]
extern crate glium;
#[macro_use]
extern crate imgui;
extern crate time;

use imgui::*;

use self::support::Support;

mod support;

const CLEAR_COLOR: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 1.0);

fn main() {
    let mut support = Support::init();

    loop {
        let active = support.render(CLEAR_COLOR, |frame| {
            hello_world(frame)
        });
        if !active { break }
    }
}

fn hello_world<'a>(frame: &Frame<'a>) {
    frame.window()
        .name(im_str!("Hello world"))
        .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
            frame.text(im_str!("Hello world!"));
            frame.text(im_str!("This...is...imgui-rs!"));
            frame.separator();
            let mouse_pos = frame.imgui().mouse_pos();
            frame.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
        })
}
