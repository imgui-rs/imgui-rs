use gfx::{self, Device};
use gfx_window_glutin;
use glutin::{self, Event, WindowEvent};
use imgui::{Context, Ui};
use imgui_gfx_renderer::{GfxRenderer, Shaders};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;

fn run_ui(ui: &Ui) -> bool {
    let mut opened = true;
    ui.show_about_window(&mut opened);
    opened
}

type ColorFormat = gfx::format::Srgba8;
type DepthFormat = gfx::format::Depth;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let builder = glutin::WindowBuilder::new()
        .with_title(file!().to_owned())
        .with_dimensions(glutin::dpi::LogicalSize::new(1024f64, 768f64));
    let (window, mut device, mut factory, mut main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, context, &events_loop)
            .expect("Failed to initalize graphics");
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let shaders = {
        let version = device.get_info().shading_language;
        if version.is_embedded {
            if version.major >= 3 {
                Shaders::GlSlEs300
            } else {
                Shaders::GlSlEs100
            }
        } else if version.major >= 4 {
            Shaders::GlSl400
        } else if version.major >= 3 {
            if version.minor >= 2 {
                Shaders::GlSl150
            } else {
                Shaders::GlSl130
            }
        } else {
            Shaders::GlSl110
        }
    };

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let mut platform = WinitPlatform::init(&mut imgui);
    platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);

    let mut renderer = GfxRenderer::init(&mut imgui, &mut factory, shaders)
        .expect("Failed to initialize renderer");

    let mut last_frame = Instant::now();
    let mut run = true;

    while run {
        events_loop.poll_events(|event| {
            platform.handle_event(imgui.io_mut(), &window, &event);

            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::Resized(_) => {
                        gfx_window_glutin::update_views(&window, &mut main_color, &mut main_depth)
                    }
                    WindowEvent::CloseRequested => run = false,
                    _ => (),
                }
            }
        });

        let io = imgui.io_mut();
        platform
            .prepare_frame(io, &window)
            .expect("Failed to start frame");
        last_frame = io.update_delta_time(last_frame);
        let ui = imgui.frame();

        if !run_ui(&ui) {
            break;
        }

        encoder.clear(&main_color, [1.0, 1.0, 1.0, 1.0]);
        platform.prepare_render(&ui, &window);
        renderer
            .render(&mut factory, &mut encoder, &mut main_color, ui)
            .expect("Rendering failed");
        encoder.flush(&mut device);
        window.swap_buffers().expect("Failed to swap buffers");
        device.cleanup();
    }
}
