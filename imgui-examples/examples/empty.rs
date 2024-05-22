mod support;

fn main() {
    support::simple_init(file!(), move |_, _ui| {
        // nothing! don't actually do any imgui funtimes
    });
}
