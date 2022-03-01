use imgui::*;

mod support;

fn main() {
    let mut system = support::init(file!());
    system.imgui.io_mut().config_flags.insert(ConfigFlags::VIEWPORTS_ENABLE);

    let mut value = 0;
    let choices = ["test test this is 1", "test test this is 2"];
    let mut open = true;

    system.main_loop(move |_, ui| {
        ui.window("Hello world")
            .opened(&mut open)
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(|| {
                ui.text_wrapped("Hello world!");
                ui.text_wrapped("こんにちは世界！");
                if ui.button(choices[value]) {
                    value += 1;
                    value %= 2;
                }

                ui.button("This...is...imgui-rs!");
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
    });
}
