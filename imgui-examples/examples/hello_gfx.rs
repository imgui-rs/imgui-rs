extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

#[macro_use]
extern crate imgui;
extern crate imgui_gfx_renderer;
extern crate imgui_sys;

use gfx::Device;

use imgui::*;
use imgui_gfx_renderer::Renderer;

mod support_gfx;

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub fn main() {
    let mut support = support_gfx::Support::init();
    let builder = glutin::WindowBuilder::new()
        .with_title("Hello World (GFX)".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();
    let (window, mut device, mut factory, mut main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let mut renderer = Renderer::init(&mut support.imgui, &mut factory, main_color.clone());

    'main: loop {
        for event in window.poll_events() {
            support.update_event(&event);
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,
                glutin::Event::Resized(_width, _height) => {
                    gfx_window_glutin::update_views(&window, &mut main_color, &mut main_depth);
                }
                _ => (),
            }
        }

        support.update_mouse();

        let size_points = window.get_inner_size_points().unwrap();
        let size_pixels = window.get_inner_size_pixels().unwrap();
        let ui = support.imgui.frame(size_points, size_pixels, 1.0 / 16.0);
        hello_world(&ui);

        encoder.clear(&mut main_color, CLEAR_COLOR);

        renderer.render(ui, &mut factory, &mut encoder)
            .expect("Rendering failed");
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}

fn hello_world<'a>(ui: &Ui<'a>) {
    ui.window(im_str!("Hello world"))
        .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
            ui.text(im_str!("Hello world!"));
            ui.text(im_str!("This...is...imgui-rs!"));
            ui.separator();
            let mouse_pos = ui.imgui().mouse_pos();
            ui.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
        })
}
