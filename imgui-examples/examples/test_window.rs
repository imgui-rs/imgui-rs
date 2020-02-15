mod support;

fn main() {
    let system = support::init(file!());
    system.main_loop(move |run, ui| ui.show_demo_window(run));
}
