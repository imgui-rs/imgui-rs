use imgui::*;

mod support;

fn main() {
    let mut state = State::default();
    let system = support::init(file!());
    system.main_loop(move |run, ui| {
        example_selector(run, ui, &mut state);
        match state.example {
            1 => example_1(ui, &mut state),
            2 => example_2(ui),
            3 => example_3(ui),
            _ => (),
        }
    });
}

fn example_selector(run: &mut bool, ui: &mut Ui, state: &mut State) {
    let w = Window::new(im_str!("Color button examples"))
        .opened(run)
        .position([20.0, 20.0], Condition::Appearing)
        .size([700.0, 100.0], Condition::Appearing)
        .resizable(false);
    w.build(ui, || {
        let ex1 = ui.radio_button(im_str!("Example 1: Basics"), &mut state.example, 1);
        let ex2 = ui.radio_button(im_str!("Example 2: Alpha component"), &mut state.example, 2);
        let ex3 = ui.radio_button(im_str!("Example 3: Input format"), &mut state.example, 3);
        if ex1 || ex2 || ex3 {
            state.reset();
        }
    });
}

fn example_1(ui: &Ui, state: &mut State) {
    let w = Window::new(im_str!("Example 1: Basics"))
        .size([700.0, 300.0], Condition::Appearing)
        .position([20.0, 140.0], Condition::Appearing);
    w.build(ui, || {
        ui.text_wrapped(im_str!(
            "Color button is a widget that displays a color value as a clickable rectangle. \
             It also supports a tooltip with detailed information about the color value. \
             Try hovering over and clicking these buttons!"
        ));
        ui.text(state.notify_text);

        ui.text("This button is black:");
        if ColorButton::new(im_str!("Black color"), [0.0, 0.0, 0.0, 1.0]).build(ui) {
            state.notify_text = "*** Black button was clicked";
        }

        ui.text("This button is red:");
        if ColorButton::new(im_str!("Red color"), [1.0, 0.0, 0.0, 1.0]).build(ui) {
            state.notify_text = "*** Red button was clicked";
        }

        ui.text("This button is BIG because it has a custom size:");
        if ColorButton::new(im_str!("Green color"), [0.0, 1.0, 0.0, 1.0])
            .size([100.0, 50.0])
            .build(ui)
        {
            state.notify_text = "*** BIG button was clicked";
        }

        ui.text("This button doesn't use the tooltip at all:");
        if ColorButton::new(im_str!("No tooltip"), [0.0, 0.0, 1.0, 1.0])
            .tooltip(false)
            .build(ui)
        {
            state.notify_text = "*** No tooltip button was clicked";
        }
    });
}

fn example_2(ui: &Ui) {
    let w = Window::new(im_str!("Example 2: Alpha component"))
        .size([700.0, 320.0], Condition::Appearing)
        .position([20.0, 140.0], Condition::Appearing);
    w.build(ui, || {
        ui.text_wrapped(im_str!(
            "The displayed color is passed to the button as four float values between \
             0.0 - 1.0 (RGBA). If you don't care about the alpha component, it can be \
             disabled and it won't show up in the tooltip"
        ));

        ui.text("This button ignores the alpha component:");
        ColorButton::new(im_str!("Red color"), [1.0, 0.0, 0.0, 0.5])
            .alpha(false)
            .build(ui);

        ui.spacing();
        ui.spacing();
        ui.spacing();
        ui.text_wrapped(im_str!(
            "If you *do* care about the alpha component, you can choose how it's \
             displayed in the button and the tooltip"
        ));

        ui.separator();
        ui.text_wrapped(im_str!(
            "ColorPreview::Opaque (default) doesn't show the alpha component at all"
        ));
        ColorButton::new(im_str!("Red + ColorPreview::Opaque"), [1.0, 0.0, 0.0, 0.5])
            .preview(ColorPreview::Opaque)
            .build(ui);

        ui.separator();
        ui.text_wrapped(im_str!(
            "ColorPreview::HalfAlpha divides the color area into two halves and uses a \
             checkerboard pattern in one half to illustrate the alpha component"
        ));
        ColorButton::new(
            im_str!("Red + ColorPreview::HalfAlpha"),
            [1.0, 0.0, 0.0, 0.5],
        )
        .preview(ColorPreview::HalfAlpha)
        .build(ui);

        ui.separator();
        ui.text_wrapped(im_str!(
            "ColorPreview::Alpha uses a checkerboard pattern in the entire color area to \
             illustrate the alpha component"
        ));
        ColorButton::new(im_str!("Red + ColorPreview::Alpha"), [1.0, 0.0, 0.0, 0.5])
            .preview(ColorPreview::Alpha)
            .build(ui);
    });
}

fn example_3(ui: &Ui) {
    let w = Window::new(im_str!("Example 3: Input format"))
        .size([700.0, 320.0], Condition::Appearing)
        .position([20.0, 140.0], Condition::Appearing);
    w.build(ui, || {
        ui.text("This button interprets the input value [1.0, 0.0, 0.0, 1.0] as RGB(A) (default):");
        ColorButton::new(im_str!("RGBA red"), [1.0, 0.0, 0.0, 1.0]).build(ui);

        ui.separator();
        ui.text("This button interprets the input value [1.0, 0.0, 0.0, 1.0] as HSV(A):");
        ColorButton::new(im_str!("HSVA black"), [1.0, 0.0, 0.0, 1.0])
            .input_mode(ColorEditInputMode::HSV)
            .build(ui);
    });
}

#[derive(Default)]
struct State {
    example: u32,
    notify_text: &'static str,
}

impl State {
    fn reset(&mut self) {
        self.notify_text = "";
    }
}
