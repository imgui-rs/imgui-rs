use glium::glutin::surface::WindowSurface;
use glium::Surface;
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::winit::dpi::LogicalSize;
use imgui_winit_support::winit::event::{Event, WindowEvent};
use imgui_winit_support::winit::event_loop::EventLoop;
use imgui_winit_support::winit::window::{Window, WindowBuilder};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::path::Path;
use std::time::Instant;

mod clipboard;

pub struct System {
    pub event_loop: EventLoop<()>,
    pub window: Window,
    pub display: glium::Display<WindowSurface>,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub font_size: f32,
}

pub fn init(title: &str) -> System {
    let title = match Path::new(&title).file_name() {
        Some(file_name) => file_name.to_str().unwrap(),
        None => title,
    };
    let event_loop = EventLoop::new().expect("Failed to create EventLoop");

    let builder = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(LogicalSize::new(1024, 768));
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(builder)
        .build(&event_loop);

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    if let Some(backend) = clipboard::init() {
        imgui.set_clipboard_backend(backend);
    } else {
        eprintln!("Failed to initialize clipboard");
    }

    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let dpi_mode = if let Ok(factor) = std::env::var("IMGUI_EXAMPLE_FORCE_DPI_FACTOR") {
            // Allow forcing of HiDPI factor for debugging purposes
            match factor.parse::<f64>() {
                Ok(f) => HiDpiMode::Locked(f),
                Err(e) => panic!("Invalid scaling factor: {}", e),
            }
        } else {
            HiDpiMode::Default
        };

        platform.attach_window(imgui.io_mut(), &window, dpi_mode);
    }

    // Fixed font size. Note imgui_winit_support uses "logical
    // pixels", which are physical pixels scaled by the devices
    // scaling factor. Meaning, 13.0 pixels should look the same size
    // on two different screens, and thus we do not need to scale this
    // value (as the scaling is handled by winit)
    let font_size = 13.0;

    imgui.fonts().add_font(&[
        FontSource::TtfData {
            data: include_bytes!("../../../resources/Roboto-Regular.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                // As imgui-glium-renderer isn't gamma-correct with
                // it's font rendering, we apply an arbitrary
                // multiplier to make the font a bit "heavier". With
                // default imgui-glow-renderer this is unnecessary.
                rasterizer_multiply: 1.5,
                // Oversampling font helps improve text rendering at
                // expense of larger font atlas texture.
                oversample_h: 4,
                oversample_v: 4,
                ..FontConfig::default()
            }),
        },
        FontSource::TtfData {
            data: include_bytes!("../../../resources/mplus-1p-regular.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                // Oversampling font helps improve text rendering at
                // expense of larger font atlas texture.
                oversample_h: 4,
                oversample_v: 4,
                // Range of glyphs to rasterize
                glyph_ranges: FontGlyphRanges::japanese(),
                ..FontConfig::default()
            }),
        },
    ]);

    let renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    System {
        event_loop,
        window,
        display,
        imgui,
        platform,
        renderer,
        font_size,
    }
}

impl System {
    pub fn main_loop<F: FnMut(&mut bool, &mut Ui) + 'static>(self, mut run_ui: F) {
        let System {
            event_loop,
            window,
            display,
            mut imgui,
            mut platform,
            mut renderer,
            ..
        } = self;
        let mut last_frame = Instant::now();

        event_loop
            .run(move |event, window_target| match event {
                Event::NewEvents(_) => {
                    let now = Instant::now();
                    imgui.io_mut().update_delta_time(now - last_frame);
                    last_frame = now;
                }
                Event::AboutToWait => {
                    platform
                        .prepare_frame(imgui.io_mut(), &window)
                        .expect("Failed to prepare frame");
                    window.request_redraw();
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    let ui = imgui.frame();

                    let mut run = true;
                    run_ui(&mut run, ui);
                    if !run {
                        window_target.exit();
                    }

                    let mut target = display.draw();
                    target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
                    platform.prepare_render(ui, &window);
                    let draw_data = imgui.render();
                    renderer
                        .render(&mut target, draw_data)
                        .expect("Rendering failed");
                    target.finish().expect("Failed to swap buffers");
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(new_size),
                    ..
                } => {
                    if new_size.width > 0 && new_size.height > 0 {
                        display.resize((new_size.width, new_size.height));
                    }
                    platform.handle_event(imgui.io_mut(), &window, &event);
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => window_target.exit(),
                event => {
                    platform.handle_event(imgui.io_mut(), &window, &event);
                }
            })
            .expect("EventLoop error");
    }
}
