mod support;

fn main() {
    support::simple_init(file!(), move |run, ui| ui.show_demo_window(run));
}
