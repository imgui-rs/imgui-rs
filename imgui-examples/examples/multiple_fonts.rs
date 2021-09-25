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
    system
        .renderer
        .reload_font_texture(&mut system.imgui)
        .expect("Failed to reload fonts");
    system.main_loop(move |run, ui| {
        ui.window("Hello world").opened(run).build(|| {
            ui.text("Hello, I'm the default font!");
            let _roboto = ui.push_font(roboto);
            ui.text("Hello, I'm Roboto Regular!");
            let _dokdo = ui.push_font(dokdo);
            ui.text("Hello, I'm Dokdo Regular!");
            _dokdo.pop();
            ui.text("Hello, I'm Roboto Regular again!");
            _roboto.pop();
            ui.text("Hello, I'm the default font again!");
        });
    });
}
