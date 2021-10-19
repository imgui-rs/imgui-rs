use imgui::*;

mod support;

fn main() {
    let system = support::init(file!());
    let mut stable_str = String::new();

    system.main_loop(move |_, ui| {
        if let Some(_window) = imgui::Window::new("Input text callbacks")
            .size([500.0, 300.0], Condition::FirstUseEver)
            .begin(ui)
        {
            if ui.input_text("input stable", &mut stable_str).build() {
                dbg!(&stable_str);
            }

            let mut per_frame_buf = String::new();
            ui.input_text("input per frame", &mut per_frame_buf).build();

            if ui.is_item_deactivated_after_edit() {
                dbg!(&per_frame_buf);
            }
        }
    });
}
