use glium::{
    backend::{Context, Facade},
    Texture2d,
};
use imgui::{self, FontGlyphRange, ImFontConfig, Ui};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::rc::Rc;
use std::time::Instant;

pub type Textures = imgui::Textures<Rc<Texture2d>>;

pub fn run<F>(title: String, clear_color: [f32; 4], mut run_ui: F)
where
    F: FnMut(&Ui, &Rc<Context>, &mut Textures) -> bool,
{
    use glium::glutin;
    use glium::{Display, Surface};
    use imgui_glium_renderer::GliumRenderer;

    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let builder = glutin::WindowBuilder::new()
        .with_title(title)
        .with_dimensions(glutin::dpi::LogicalSize::new(1024f64, 768f64));
    let display = Display::new(builder, context, &events_loop).unwrap();
    let gl_window = display.gl_window();
    let window = gl_window.window();

    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);

    let mut platform = WinitPlatform::init(&mut imgui);
    platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);

    let hidpi_factor = platform.hidpi_factor();
    let font_size = (13.0 * hidpi_factor) as f32;

    imgui.fonts().add_default_font_with_config(
        ImFontConfig::new()
            .oversample_h(1)
            .pixel_snap_h(true)
            .size_pixels(font_size),
    );

    imgui.fonts().add_font_with_config(
        include_bytes!("../../../resources/mplus-1p-regular.ttf"),
        ImFontConfig::new()
            .merge_mode(true)
            .oversample_h(1)
            .pixel_snap_h(true)
            .size_pixels(font_size)
            .rasterizer_multiply(1.75),
        &FontGlyphRange::japanese(),
    );

    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    let mut renderer =
        GliumRenderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    let mut last_frame = Instant::now();
    let mut quit = false;

    loop {
        events_loop.poll_events(|event| {
            use glium::glutin::{Event, WindowEvent::CloseRequested};

            platform.handle_event(imgui.io_mut(), &window, &event);

            if let Event::WindowEvent { event, .. } = event {
                match event {
                    CloseRequested => quit = true,
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
        if !run_ui(&ui, display.get_context(), renderer.textures()) {
            break;
        }

        let mut target = display.draw();
        target.clear_color(
            clear_color[0],
            clear_color[1],
            clear_color[2],
            clear_color[3],
        );
        platform.prepare_render(&ui, &window);
        renderer.render(&mut target, ui).expect("Rendering failed");
        target.finish().unwrap();

        if quit {
            break;
        }
    }
}
