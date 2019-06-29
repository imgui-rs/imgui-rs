mod support;

fn main() {
    let system = support::init(file!());
    system.main_loop(|run, ui| ui.show_demo_window(run));
}
