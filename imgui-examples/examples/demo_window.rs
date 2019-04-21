mod support;

fn main() {
    support::init(file!()).main_loop(|run, ui| ui.show_demo_window(run));
}
