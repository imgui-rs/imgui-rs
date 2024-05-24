use imgui::*;

mod support;

/// NOTE on this example:
/// Most of this complexity is because of how we initialize our support
/// (which primarily is a result of `winit`'s requirements for making a window).
/// In reality, most of these two functions can be made directly after each other --
/// to run the initialization code (which loads a font), all you need is imgui + a renderer.
fn main() {
    let dokdo = std::rc::Rc::new(std::cell::RefCell::new(None));
    let roboto = std::rc::Rc::new(std::cell::RefCell::new(None));

    let dokdo_init = dokdo.clone();
    let roboto_init = roboto.clone();

    support::init_with_startup(
        file!(),
        move |ctx, renderer, _| {
            let mut dokdo_handle = dokdo_init.borrow_mut();
            let mut roboto_handle = roboto_init.borrow_mut();

            // this function runs right after the window is created.
            // In your own code, this can be done whenever you have a renderer
            // and a context.
            *dokdo_handle = Some(ctx.fonts().add_font(&[FontSource::TtfData {
                data: include_bytes!("../../resources/Dokdo-Regular.ttf"),
                size_pixels: support::FONT_SIZE,
                config: None,
            }]));
            *roboto_handle = Some(ctx.fonts().add_font(&[FontSource::TtfData {
                data: include_bytes!("../../resources/Roboto-Regular.ttf"),
                size_pixels: support::FONT_SIZE,
                config: None,
            }]));

            renderer
                .reload_font_texture(ctx)
                .expect("Failed to reload fonts");
        },
        move |run, ui| {
            let dokdo = dokdo.borrow().unwrap();
            let roboto = roboto.borrow().unwrap();

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
        },
    );
}
