mod support;

fn main() {
    support::run(file!(), |run, ui| ui.show_demo_window(run));
}
