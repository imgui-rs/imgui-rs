use imgui::*;

mod support;

fn main() {
    let system = support::init(file!());
    system.main_loop(move |run, ui| {
        let w = Window::new(im_str!("Progress bar"))
            .opened(run)
            .position([20.0, 20.0], Condition::Appearing)
            .size([700.0, 200.0], Condition::Appearing);
        w.build(&ui, || {
            ui.text("This is a simple progress bar:");
            ProgressBar::new(0.5).build(ui);

            ui.separator();
            ui.text("This progress bar has a custom size:");
            ProgressBar::new(0.3).size([200.0, 50.0]).build(ui);

            ui.separator();
            ui.text("This progress bar uses overlay text:");
            ProgressBar::new(0.8)
                .overlay_text(im_str!("Lorem ipsum"))
                .build(ui);
        });
    });
}
