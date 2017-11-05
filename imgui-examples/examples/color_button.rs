extern crate glium;
#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;

use imgui::*;

mod support;

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

struct State {
    example: i32,
    notify_text: &'static str,
}

impl State {
    fn reset(&mut self) { self.notify_text = ""; }
}

impl Default for State {
    fn default() -> State {
        State {
            example: 0,
            notify_text: "",
        }
    }
}

fn main() {
    let mut state = State::default();
    support::run("color_button.rs".to_owned(), CLEAR_COLOR, |ui| {
        example_selector(&mut state, ui);
        match state.example {
            1 => example_1(&mut state, ui),
            2 => example_2(ui),
            _ => (),
        }
        true
    });
}

fn example_selector(state: &mut State, ui: &Ui) {
    ui.window(im_str!("Color button examples"))
        .position((20.0, 20.0), ImGuiCond::Appearing)
        .size((700.0, 80.0), ImGuiCond::Appearing)
        .resizable(false)
        .build(|| {
            let ex1 = ui.radio_button(im_str!("Example 1: Basics"), &mut state.example, 1);
            let ex2 = ui.radio_button(im_str!("Example 2: Alpha component"), &mut state.example, 2);
            if ex1 || ex2 {
                state.reset();
            }
        });
}

fn example_1(state: &mut State, ui: &Ui) {
    ui.window(im_str!("Example 1: Basics"))
        .size((700.0, 300.0), ImGuiCond::Appearing)
        .position((20.0, 120.0), ImGuiCond::Appearing)
        .build(|| {
            ui.text_wrapped(im_str!(
                "Color button is a widget that displays a color value as a clickable rectangle. \
                It also supports a tooltip with detailed information about the color value. \
                Try hovering over and clicking these buttons!"
            ));
            ui.text(state.notify_text);

            ui.text("This button is black:");
            if ui.color_button(im_str!("Black color"), (0.0, 0.0, 0.0, 1.0))
                .build()
            {
                state.notify_text = "*** Black button was clicked";
            }

            ui.text("This button is red:");
            if ui.color_button(im_str!("Red color"), (1.0, 0.0, 0.0, 1.0))
                .build()
            {
                state.notify_text = "*** Red button was clicked";
            }

            ui.text("This button is BIG because it has a custom size:");
            if ui.color_button(im_str!("Green color"), (0.0, 1.0, 0.0, 1.0))
                .size((100.0, 50.0))
                .build()
            {
                state.notify_text = "*** BIG button was clicked";
            }

            ui.text("This button doesn't use the tooltip at all:");
            if ui.color_button(im_str!("No tooltip"), (0.0, 0.0, 1.0, 1.0))
                .tooltip(false)
                .build()
            {
                state.notify_text = "*** No tooltip button was clicked";
            }
        });
}

fn example_2(ui: &Ui) {
    ui.window(im_str!("Example 2: Alpha component"))
        .size((700.0, 320.0), ImGuiCond::Appearing)
        .position((20.0, 140.0), ImGuiCond::Appearing)
        .build(|| {
            ui.text_wrapped(im_str!(
                "The displayed color is passed to the button as four float values between \
                    0.0 - 1.0 (RGBA). If you don't care about the alpha component, it can be \
                    disabled and it won't show up in the tooltip"
            ));

            ui.text("This button ignores the alpha component:");
            ui.color_button(im_str!("Red color"), (1.0, 0.0, 0.0, 0.5))
                .alpha(false)
                .build();

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
            ui.color_button(im_str!("Red + ColorPreview::Opaque"), (1.0, 0.0, 0.0, 0.5))
                .preview(ColorPreview::Opaque)
                .build();

            ui.separator();
            ui.text_wrapped(im_str!(
                "ColorPreview::HalfAlpha divides the color area into two halves and uses a \
                    checkerboard pattern in one half to illustrate the alpha component"
            ));
            ui.color_button(
                im_str!("Red + ColorPreview::HalfAlpha"),
                (1.0, 0.0, 0.0, 0.5),
            ).preview(ColorPreview::HalfAlpha)
                .build();

            ui.separator();
            ui.text_wrapped(im_str!(
                "ColorPreview::Alpha uses a checkerboard pattern in the entire color area to \
                    illustrate the alpha component"
            ));
            ui.color_button(im_str!("Red + ColorPreview::Alpha"), (1.0, 0.0, 0.0, 0.5))
                .preview(ColorPreview::Alpha)
                .build();
        });
}
