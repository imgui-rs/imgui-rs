use imgui::*;

mod support;

fn main() {
    let mut system = support::init(file!());
    let dokdo = system.imgui.fonts().add_font(&[FontSource::TtfData {
        data: include_bytes!("../../resources/Dokdo-Regular.ttf"),
        size_pixels: system.font_size,
        config: None,
    }]);
    let roboto = system.imgui.fonts().add_font(&[FontSource::TtfData {
        data: include_bytes!("../../resources/Roboto-Regular.ttf"),
        size_pixels: system.font_size,
        config: None,
    }]);
    system.main_loop(|run, ui| {
        let w = Window::new(im_str!("Hello world")).opened(run);
        w.build(&ui, || {
            ui.text("Hello, I'm the default font!");
            ui.with_font(roboto, || {
                ui.text("Hello, I'm Roboto Regular!");
                ui.with_font(dokdo, || {
                    ui.text("Hello, I'm Dokdo Regular!");
                });
                ui.text("Hello, I'm Roboto Regular again!");
            });
            ui.text("Hello, I'm the default font again!");
        });
    });
}
