/// Shows the "Demo Window" provided by the Dear ImGui library. See
/// `test_window_impl` example for Rust implementation.

mod support;

fn main() {
    let system = support::init(file!());
    system.main_loop(move |run, ui| ui.show_demo_window(run));
}
