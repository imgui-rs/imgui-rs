use imgui::*;

mod support;

fn main() {
    support::run(file!(), hello_world);
}

fn hello_world(run: &mut bool, ui: &mut Ui) {
    let w = Window::new(im_str!("Hello world")).opened(run);
    w.build(&ui, || {
        ui.text("Hello world!");
        ui.text("こんにちは世界！");
        ui.text("This...is...imgui-rs!");
        ui.separator();
        let mouse_pos = ui.io().mouse_pos;
        ui.text(format!(
            "Mouse Position: ({:.1},{:.1})",
            mouse_pos[0], mouse_pos[1]
        ));
    });
}
