use imgui::*;

mod support;

fn main() {
    let mut state = State {
        render_closable: true,
    };
    let system = support::init(file!());
    system.main_loop(move |run, ui| {
        let w = Window::new(im_str!("Collapsing header"))
            .opened(run)
            .position([20.0, 20.0], Condition::Appearing)
            .size([700.0, 500.0], Condition::Appearing);
        w.build(ui, || {
            if CollapsingHeader::new(im_str!("I'm a collapsing header. Click me!")).build(ui) {
                ui.text(
                    "A collapsing header can be used to toggle rendering of a group of widgets",
                );
            }

            ui.spacing();
            if CollapsingHeader::new(im_str!("I'm open by default"))
                .default_open(true)
                .build(ui)
            {
                ui.text("You can still close me with a click!");
            }

            ui.spacing();
            if CollapsingHeader::new(im_str!("I only open with double-click"))
                .open_on_double_click(true)
                .build(ui)
            {
                ui.text("Double the clicks, double the fun!");
            }

            ui.spacing();
            if CollapsingHeader::new(im_str!("I don't have an arrow"))
                .bullet(true)
                .build(ui)
            {
                ui.text("Collapsing headers can use a bullet instead of an arrow");
            }

            ui.spacing();
            if CollapsingHeader::new(im_str!("I only open if you click the arrow"))
                .open_on_arrow(true)
                .build(ui)
            {
                ui.text("You clicked the arrow");
            }

            ui.spacing();
            ui.checkbox(
                im_str!("Toggle rendering of the next example"),
                &mut state.render_closable,
            );
            if CollapsingHeader::new(im_str!("I've got a separate close button"))
                .build_with_close_button(ui, &mut state.render_closable)
            {
                ui.text("I've got contents just like any other collapsing header");
            }
        });
    });
}

struct State {
    render_closable: bool,
}
