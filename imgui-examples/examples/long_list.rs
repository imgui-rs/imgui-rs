/// Demonstrates using the "list clipper" to efficiently display long
/// lists in a scrolling area.
///
/// You specify the height per item, and the `ListClipper` API will
/// provide which item index are visible. This avoids having to create
/// thousands of items only for them to not be made visible.
///
/// Note this requires a fixed (or easily computable) height per item.
use imgui::*;

mod support;

fn main() {
    let lots_of_words: Vec<String> = (0..10000).map(|x| format!("Line {}", x)).collect();

    let system = support::init(file!());
    system.main_loop(move |_, ui| {
        Window::new(im_str!("Hello long world"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || {
                let mut clipper = imgui::ListClipper::new(lots_of_words.len() as i32)
                    .items_height(ui.current_font_size())
                    .begin(ui);
                while clipper.step() {
                    for row_num in clipper.display_start()..clipper.display_end() {
                        ui.text(&lots_of_words[row_num as usize]);
                    }
                }
            });
    });
}
