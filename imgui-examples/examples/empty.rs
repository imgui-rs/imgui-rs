mod support;

fn main() {
    let system = support::init(file!());

    system.main_loop(move |_, _ui| {
        // nothing! don't actually do any imgui funtimes
    });
}
