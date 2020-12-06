use imgui::*;

mod support;

fn main() {
    let system = support::init(file!());

    let window_title = if cfg!(all(feature = "directx", windows)) {
        im_str!("Hello world (OpenGL)")
    } else {
        im_str!("Hello world (DirectX)")
    };

    system.main_loop(|_, ui| {
        Window::new(window_title)
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("こんにちは世界！"));
                ui.text(im_str!("This...is...imgui-rs!"));
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
    });
}
