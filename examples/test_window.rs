#[macro_use]
extern crate glium;
#[macro_use]
extern crate imgui;
extern crate time;

use self::support::Support;

mod support;

const CLEAR_COLOR: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 1.0);

fn main() {
    let mut support = Support::init();

     loop {
        let mut open = true;
        let active = support.render(CLEAR_COLOR, |ui| {
            ui.show_test_window(&mut open)
        });
        if !active || !open { break }
    }
}
