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
        // Show the C++ style API
        ui.window("Hello long world")
            .size([100.0, 500.0], Condition::FirstUseEver)
            .position([10.0, 10.0], crate::Condition::Always)
            .build(|| {
                let mut clipper = imgui::ListClipper::new(lots_of_words.len() as i32)
                    .items_height(ui.current_font_size())
                    .begin(ui);
                while clipper.step() {
                    for row_num in clipper.display_start()..clipper.display_end() {
                        ui.text(&lots_of_words[row_num as usize]);
                    }
                }
            });

        // Show the more Rust'y iterator
        ui.window("Hello long world (iterator API)")
            .size([100.0, 500.0], Condition::FirstUseEver)
            .position([150.0, 10.0], crate::Condition::Always)
            .build(|| {
                let clipper = imgui::ListClipper::new(lots_of_words.len() as i32)
                    .items_height(ui.current_font_size())
                    .begin(ui);
                for row_num in clipper.iter() {
                    ui.text(&lots_of_words[row_num as usize]);
                }
            });
    });
}
