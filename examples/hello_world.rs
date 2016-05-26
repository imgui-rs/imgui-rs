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
        support.render(CLEAR_COLOR, hello_world);
        let active = support.update_events();
        if !active { break }
    }
}

fn hello_world<'a>(ui: &Ui<'a>) {
    ui.window(im_str!("Hello world"))
        .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
            ui.text(im_str!("Hello world!"));
            ui.text(im_str!("This...is...imgui-rs!"));
           ui.combo3(im_str!("Sample combo"), &mut 1, |idx|{
               println!("{} index closure", idx);
                match idx {
                    0 => "zero\0",
                    1 => "one\0",
                    2 => "two\0",
                    _ => "three\0",
                }
            }, 3,2);
            ui.separator();
            let mouse_pos = ui.imgui().mouse_pos();
            ui.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
        })
}
