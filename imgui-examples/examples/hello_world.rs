use imgui::*;

mod support;

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() {
    support::run("hello_world.rs".to_owned(), CLEAR_COLOR, |ui, _, _| {
        hello_world(ui)
    });
}

fn hello_world<'a>(ui: &Ui<'a>) -> bool {
    ui.window(im_str!("Hello world"))
        .size((300.0, 100.0), Condition::FirstUseEver)
        .build(|| {
            ui.text(im_str!("Hello world!"));
            ui.text(im_str!("こんにちは世界！"));
            ui.text(im_str!("This...is...imgui-rs!"));
            ui.separator();
            let mouse_pos = ui.io().mouse_pos;
            ui.text(im_str!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos[0],
                mouse_pos[1]
            ));
        });

    true
}
