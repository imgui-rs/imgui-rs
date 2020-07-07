use gfx::Device;
use glutin::{Event, WindowEvent};
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource, Ui};
use imgui_gfx_renderer::{Renderer, Shaders};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;

type ColorFormat = gfx::format::Rgba8;

pub struct System {
    pub events_loop: glutin::EventsLoop,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub render_sys: RenderSystem,
    pub font_size: f32,
}

pub fn init(title: &str) -> System {
    let title = match title.rfind('/') {
        Some(idx) => title.split_at(idx + 1).1,
        None => title,
    };
    let events_loop = glutin::EventsLoop::new();
    let builder = glutin::WindowBuilder::new()
        .with_title(title.to_owned())
        .with_dimensions(glutin::dpi::LogicalSize::new(1024f64, 768f64));

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let mut platform = WinitPlatform::init(&mut imgui);

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

    let render_sys = RenderSystem::init(&mut imgui, builder, &events_loop);
    platform.attach_window(imgui.io_mut(), render_sys.window(), HiDpiMode::Rounded);
    System {
        events_loop,
        imgui,
        platform,
        render_sys,
        font_size,
    }
}

impl System {
    pub fn main_loop<F: FnMut(&mut bool, &mut Ui)>(self, mut run_ui: F) {
        let System {
            mut events_loop,
            mut imgui,
            mut platform,
            mut render_sys,
            ..
        } = self;
        let mut encoder: gfx::Encoder<_, _> = render_sys.factory.create_command_buffer().into();

        let mut last_frame = Instant::now();
        let mut run = true;

        while run {
            events_loop.poll_events(|event| {
                platform.handle_event(imgui.io_mut(), render_sys.window(), &event);

                if let Event::WindowEvent { event, .. } = event {
                    match event {
                        WindowEvent::Resized(size) => render_sys.update_views(size),
                        WindowEvent::CloseRequested => run = false,
                        _ => (),
                    }
                }
            });

            let io = imgui.io_mut();
            platform
                .prepare_frame(io, render_sys.window())
                .expect("Failed to start frame");
            let now = Instant::now();
            io.update_delta_time(now - last_frame);
            last_frame = now;
            let mut ui = imgui.frame();
            run_ui(&mut run, &mut ui);

            if let Some(main_color) = render_sys.main_color.as_mut() {
                encoder.clear(main_color, [1.0, 1.0, 1.0, 1.0]);
            }
            platform.prepare_render(&ui, render_sys.window());
            let draw_data = ui.render();
            if let Some(main_color) = render_sys.main_color.as_mut() {
                render_sys
                    .renderer
                    .render(&mut render_sys.factory, &mut encoder, main_color, draw_data)
                    .expect("Rendering failed");
            }
            encoder.flush(&mut render_sys.device);
            render_sys.swap_buffers();
            render_sys.device.cleanup();
        }
    }
}

#[cfg(feature = "opengl")]
mod types {
    pub type Device = gfx_device_gl::Device;
    pub type Factory = gfx_device_gl::Factory;
    pub type Resources = gfx_device_gl::Resources;
}

#[cfg(feature = "opengl")]
pub struct RenderSystem {
    pub renderer: Renderer<ColorFormat, types::Resources>,
    pub windowed_context: glutin::WindowedContext<glutin::PossiblyCurrent>,
    pub device: types::Device,
    pub factory: types::Factory,
    pub main_color: Option<gfx::handle::RenderTargetView<types::Resources, ColorFormat>>,
    pub main_depth: gfx::handle::DepthStencilView<types::Resources, gfx::format::DepthStencil>,
}

#[cfg(feature = "opengl")]
impl RenderSystem {
    pub fn init(
        imgui: &mut Context,
        builder: glutin::WindowBuilder,
        events_loop: &glutin::EventsLoop,
    ) -> RenderSystem {
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

        let context = glutin::ContextBuilder::new().with_vsync(true);
        let (windowed_context, device, mut factory, main_color, main_depth) =
            gfx_window_glutin::init(builder, context, &events_loop)
                .expect("Failed to initialize graphics");
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
        let renderer =
            Renderer::init(imgui, &mut factory, shaders).expect("Failed to initialize renderer");
        RenderSystem {
            renderer,
            windowed_context,
            device,
            factory,
            main_color: Some(main_color),
            main_depth,
        }
    }
    pub fn window(&self) -> &glutin::Window {
        self.windowed_context.window()
    }
    pub fn update_views(&mut self, _: glutin::dpi::LogicalSize) {
        if let Some(main_color) = self.main_color.as_mut() {
            gfx_window_glutin::update_views(
                &self.windowed_context,
                main_color,
                &mut self.main_depth,
            );
        }
    }
    pub fn swap_buffers(&mut self) {
        self.windowed_context.swap_buffers().unwrap();
    }
}

#[cfg(feature = "directx")]
mod types {
    pub type Device = gfx_device_dx11::Device;
    pub type Factory = gfx_device_dx11::Factory;
    pub type Resources = gfx_device_dx11::Resources;
}

#[cfg(feature = "directx")]
pub struct RenderSystem {
    pub renderer: Renderer<ColorFormat, types::Resources>,
    pub window: gfx_window_dxgi::Window,
    pub device: types::Device,
    pub factory: types::Factory,
    pub main_color: Option<gfx::handle::RenderTargetView<types::Resources, ColorFormat>>,
}

#[cfg(feature = "directx")]
impl RenderSystem {
    pub fn init(
        imgui: &mut Context,
        builder: glutin::WindowBuilder,
        events_loop: &glutin::EventsLoop,
    ) -> RenderSystem {
        let (window, device, mut factory, main_color) =
            gfx_window_dxgi::init(builder, &events_loop).expect("Failed to initialize graphics");
        let renderer = Renderer::init(imgui, &mut factory, Shaders::HlslSm40)
            .expect("Failed to initialize renderer");
        RenderSystem {
            renderer,
            window,
            device,
            factory,
            main_color: Some(main_color),
        }
    }
    pub fn window(&self) -> &glutin::Window {
        &self.window.inner
    }
    pub fn update_views(&mut self, size: glutin::dpi::LogicalSize) {
        let physical = size.to_physical(self.window().get_hidpi_factor());
        let (width, height): (u32, u32) = physical.into();
        let _ = self.main_color.take(); // we need to drop main_color before calling update_views
        self.main_color = Some(
            gfx_window_dxgi::update_views(
                &mut self.window,
                &mut self.factory,
                &mut self.device,
                width as u16,
                height as u16,
            )
            .expect("Failed to update resize"),
        );
    }
    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers(1);
    }
}
