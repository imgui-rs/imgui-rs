use gfx::Device;
use glutin::{Event, WindowEvent};
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource, Ui};
use imgui_gfx_renderer::{GfxRenderer, Shaders};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;

type ColorFormat = gfx::format::Rgba8;

#[cfg(feature = "opengl")]
pub fn run<F: FnMut(&Ui) -> bool>(title: String, clear_color: [f32; 4], mut run_ui: F) {
    type DepthFormat = gfx::format::DepthStencil;

    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let builder = glutin::WindowBuilder::new()
        .with_title(title.to_owned())
        .with_dimensions(glutin::dpi::LogicalSize::new(1024f64, 768f64));
    let (windowed_context, mut device, mut factory, mut main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, context, &events_loop)
            .expect("Failed to initialize graphics");
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
    {
        // Fix incorrect colors with sRGB framebuffer
        fn imgui_gamma_to_linear(col: [f32; 4]) -> [f32; 4] {
            let x = col[0].powf(2.2);
            let y = col[1].powf(2.2);
            let z = col[2].powf(2.2);
            let w = 1.0 - (1.0 - col[3]).powf(2.2);
            [x, y, z, w]
        }

        let style = imgui.style_mut();
        for col in 0..style.colors.len() {
            style.colors[col] = imgui_gamma_to_linear(style.colors[col]);
        }
    }
    imgui.set_ini_filename(None);

    let mut platform = WinitPlatform::init(&mut imgui);
    platform.attach_window(
        imgui.io_mut(),
        &windowed_context.window(),
        HiDpiMode::Rounded,
    );

    let hidpi_factor = platform.hidpi_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.fonts().add_font(&[
        FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: font_size,
                ..FontConfig::default()
            }),
        },
        FontSource::TtfData {
            data: include_bytes!("../../../resources/mplus-1p-regular.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                rasterizer_multiply: 1.75,
                glyph_ranges: FontGlyphRanges::japanese(),
                ..FontConfig::default()
            }),
        },
    ]);

    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    let mut renderer = GfxRenderer::init(&mut imgui, &mut factory, shaders)
        .expect("Failed to initialize renderer");

    let mut last_frame = Instant::now();
    let mut quit = false;

    loop {
        events_loop.poll_events(|event| {
            platform.handle_event(imgui.io_mut(), &windowed_context.window(), &event);

            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::Resized(_) => gfx_window_glutin::update_views(
                        &windowed_context,
                        &mut main_color,
                        &mut main_depth,
                    ),
                    WindowEvent::CloseRequested => quit = true,
                    _ => (),
                }
            }
        });
        if quit {
            break;
        }

        let io = imgui.io_mut();
        platform
            .prepare_frame(io, &windowed_context.window())
            .expect("Failed to start frame");
        last_frame = io.update_delta_time(last_frame);

        let ui = imgui.frame();
        if !run_ui(&ui) {
            break;
        }

        encoder.clear(&main_color, clear_color);
        platform.prepare_render(&ui, &windowed_context.window());
        let draw_data = ui.render();
        renderer
            .render(&mut factory, &mut encoder, &mut main_color, draw_data)
            .expect("Rendering failed");
        encoder.flush(&mut device);
        windowed_context.swap_buffers().unwrap();
        device.cleanup();
    }
}

#[cfg(feature = "directx")]
pub fn run<F: FnMut(&Ui) -> bool>(title: String, clear_color: [f32; 4], mut run_ui: F) {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title(title)
        .with_dimensions(glutin::dpi::LogicalSize::new(1024f64, 768f64));
    let (mut window, mut device, mut factory, mut main_color) =
        gfx_window_dxgi::init::<ColorFormat>(window, &events_loop)
            .expect("Failed to initialize graphics");
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let mut platform = WinitPlatform::init(&mut imgui);
    platform.attach_window(imgui.io_mut(), &window.inner, HiDpiMode::Rounded);

    let hidpi_factor = platform.hidpi_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.fonts().add_font(&[
        FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: font_size,
                ..FontConfig::default()
            }),
        },
        FontSource::TtfData {
            data: include_bytes!("../../../resources/mplus-1p-regular.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                rasterizer_multiply: 1.75,
                glyph_ranges: FontGlyphRanges::japanese(),
                ..FontConfig::default()
            }),
        },
    ]);

    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    let mut renderer = GfxRenderer::init(&mut imgui, &mut factory, Shaders::HlslSm40)
        .expect("Failed to initialize renderer");

    let mut last_frame = Instant::now();
    let mut quit = false;

    loop {
        let mut new_size = None;
        events_loop.poll_events(|event| {
            platform.handle_event(imgui.io_mut(), &window.inner, &event);

            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::Resized(size) => {
                        let physical = size.to_physical(window.inner.get_hidpi_factor());
                        let (width, height): (u32, u32) = physical.into();
                        new_size = Some((width as u16, height as u16));
                    }
                    WindowEvent::CloseRequested => quit = true,
                    _ => (),
                }
            }
        });
        if quit {
            break;
        }
        if let Some((width, height)) = new_size {
            drop(main_color);
            main_color = gfx_window_dxgi::update_views(
                &mut window,
                &mut factory,
                &mut device,
                width,
                height,
            )
            .expect("Failed to update resize");
        }

        let io = imgui.io_mut();
        platform
            .prepare_frame(io, &window.inner)
            .expect("Failed to start frame");
        last_frame = io.update_delta_time(last_frame);

        let ui = imgui.frame();
        if !run_ui(&ui) {
            break;
        }

        encoder.clear(&main_color, clear_color);
        platform.prepare_render(&ui, &window.inner);
        let draw_data = ui.render();
        renderer
            .render(&mut factory, &mut encoder, &mut main_color, draw_data)
            .expect("Rendering failed");
        encoder.flush(&mut device);
        window.swap_buffers(1);
        device.cleanup();
    }
}
