#[macro_use]
extern crate glium;
extern crate imgui;
extern crate time;

mod support;

fn main() {
    support::main_with_frame(|frame| {
        frame.show_test_window();
    });
}
