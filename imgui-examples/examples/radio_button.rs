use imgui::*;

mod support;

fn main() {
    let mut state = State::default();
    let system = support::init(file!());
    system.main_loop(move |run, ui| {
        example_selector(run, ui, &mut state);
        match state.example {
            1 => example_1(ui, &mut state),
            2 => example_2(ui, &mut state),
            _ => (),
        }
    });
}

fn example_selector(run: &mut bool, ui: &mut Ui, state: &mut State) {
    let w = Window::new("Radio button examples")
        .opened(run)
        .position([20.0, 20.0], Condition::Appearing)
        .size([700.0, 80.0], Condition::Appearing)
        .resizable(false);
    w.build(ui, || {
        let mut clicked = false;
        clicked |= ui.radio_button("Example 1: Boolean radio buttons", &mut state.example, 1);
        clicked |= ui.radio_button("Example 2: Radio buttons", &mut state.example, 2);
        if clicked {
            state.reset();
        }
    });
}

fn example_1(ui: &Ui, state: &mut State) {
    let w = Window::new("Example 1: Boolean radio buttons")
        .size([700.0, 200.0], Condition::Appearing)
        .position([20.0, 120.0], Condition::Appearing);
    w.build(ui, || {
        ui.text_wrapped(
            "Boolean radio buttons accept a boolean active state, which is passed as a value and \
             not as a mutable reference. This means that it's not updated automatically, so you \
             can implement any click behaviour you want. The return value is true if the button \
             was clicked.",
        );
        ui.text(state.notify_text);

        if ui.radio_button_bool("I'm permanently active", true) {
            state.notify_text = "*** Permanently active radio button was clicked";
        }
        if ui.radio_button_bool("I'm permanently inactive", false) {
            state.notify_text = "*** Permanently inactive radio button was clicked";
        }
        if ui.radio_button_bool("I toggle my state on click", state.simple_bool) {
            state.simple_bool = !state.simple_bool; // flip state on click
            state.notify_text = "*** Toggling radio button was clicked";
        }
    });
}

fn example_2(ui: &Ui, state: &mut State) {
    let w = Window::new("Example 2: Radio buttons")
        .size([700.0, 300.0], Condition::Appearing)
        .position([20.0, 120.0], Condition::Appearing);
    w.build(ui, || {
        ui.text_wrapped(
            "Normal radio buttons accept a mutable reference to state, and the value \
             corresponding to this button. They are very flexible, because the value can be any \
             type that is both Copy and PartialEq. This is especially useful with Rust enums",
        );
        ui.text(state.notify_text);

        ui.separator();
        if ui.radio_button("I'm number 1", &mut state.number, 1) {
            state.notify_text = "*** Number 1 was clicked";
        }
        if ui.radio_button("I'm number 2", &mut state.number, 2) {
            state.notify_text = "*** Number 2 was clicked";
        }
        if ui.radio_button("I'm number 3", &mut state.number, 3) {
            state.notify_text = "*** Number 3 was clicked";
        }

        ui.separator();
        if ui.radio_button("I'm choice A", &mut state.choice, Some(Choice::A)) {
            state.notify_text = "*** Choice A was clicked";
        }
        if ui.radio_button("I'm choice B", &mut state.choice, Some(Choice::B)) {
            state.notify_text = "*** Choice B was clicked";
        }
        if ui.radio_button("I'm choice C", &mut state.choice, Some(Choice::C)) {
            state.notify_text = "*** Choice C was clicked";
        }
    });
}

#[derive(Default)]
struct State {
    example: u32,
    notify_text: &'static str,
    simple_bool: bool,
    number: u8,
    // We use Option here because we don't want any initial value.
    // Another choice could be to choose one of the Choice enum values to be the default.
    choice: Option<Choice>,
}

#[derive(Copy, Clone, PartialEq)]
enum Choice {
    A,
    B,
    C,
}

impl State {
    fn reset(&mut self) {
        self.notify_text = "";
    }
}
