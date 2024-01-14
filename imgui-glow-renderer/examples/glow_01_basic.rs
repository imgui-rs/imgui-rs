//! A basic self-contained example to get you from zero-to-demo-window as fast
//! as possible.

use std::{num::NonZeroU32, time::Instant};

use glow::HasContext;
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext},
    display::{GetGlDisplay, GlDisplay},
    surface::{GlSurface, Surface, SurfaceAttributesBuilder, WindowSurface},
};
use imgui_winit_support::{
    winit::{
        dpi::LogicalSize,
        event_loop::EventLoop,
        window::{Window, WindowBuilder},
    },
    WinitPlatform,
};
use raw_window_handle::HasRawWindowHandle;

const TITLE: &str = "Hello, imgui-rs!";

fn main() {
    // Common setup for creating a winit window and imgui context, not specifc
    // to this renderer at all except that glutin is used to create the window
    // since it will give us access to a GL context
    let (event_loop, window, surface, context) = create_window();
    let (mut winit_platform, mut imgui_context) = imgui_init(&window);

    // OpenGL context from glow
    let gl = glow_context(&context);

    // OpenGL renderer from this crate
    let mut ig_renderer = imgui_glow_renderer::AutoRenderer::initialize(gl, &mut imgui_context)
        .expect("failed to create renderer");

    let mut last_frame = Instant::now();

    // Standard winit event loop
    event_loop
        .run(move |event, window_target| {
            match event {
                winit::event::Event::NewEvents(_) => {
                    let now = Instant::now();
                    imgui_context
                        .io_mut()
                        .update_delta_time(now.duration_since(last_frame));
                    last_frame = now;
                }
                winit::event::Event::AboutToWait => {
                    winit_platform
                        .prepare_frame(imgui_context.io_mut(), &window)
                        .unwrap();
                    window.request_redraw();
                }
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::RedrawRequested,
                    ..
                } => {
                    // The renderer assumes you'll be clearing the buffer yourself
                    unsafe { ig_renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };

                    let ui = imgui_context.frame();
                    ui.show_demo_window(&mut true);

                    winit_platform.prepare_render(ui, &window);
                    let draw_data = imgui_context.render();

                    // This is the only extra render step to add
                    ig_renderer
                        .render(draw_data)
                        .expect("error rendering imgui");

                    surface
                        .swap_buffers(&context)
                        .expect("Failed to swap buffers");
                }
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    window_target.exit();
                }
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::Resized(new_size),
                    ..
                } => {
                    if new_size.width > 0 && new_size.height > 0 {
                        surface.resize(
                            &context,
                            NonZeroU32::new(new_size.width).unwrap(),
                            NonZeroU32::new(new_size.height).unwrap(),
                        );
                    }
                    winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
                }
                event => {
                    winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
                }
            }
        })
        .expect("EventLoop error");
}

fn create_window() -> (
    EventLoop<()>,
    Window,
    Surface<WindowSurface>,
    PossiblyCurrentContext,
) {
    let event_loop = EventLoop::new().unwrap();

    let window_builder = WindowBuilder::new()
        .with_title(TITLE)
        .with_inner_size(LogicalSize::new(1024, 768));
    let (window, cfg) = glutin_winit::DisplayBuilder::new()
        .with_window_builder(Some(window_builder))
        .build(&event_loop, ConfigTemplateBuilder::new(), |mut configs| {
            configs.next().unwrap()
        })
        .expect("Failed to create OpenGL window");

    let window = window.unwrap();

    let context_attribs = ContextAttributesBuilder::new().build(Some(window.raw_window_handle()));
    let context = unsafe {
        cfg.display()
            .create_context(&cfg, &context_attribs)
            .expect("Failed to create OpenGL context")
    };

    let surface_attribs = SurfaceAttributesBuilder::<WindowSurface>::new()
        .with_srgb(Some(true))
        .build(
            window.raw_window_handle(),
            NonZeroU32::new(1024).unwrap(),
            NonZeroU32::new(768).unwrap(),
        );
    let surface = unsafe {
        cfg.display()
            .create_window_surface(&cfg, &surface_attribs)
            .expect("Failed to create OpenGL surface")
    };

    let context = context
        .make_current(&surface)
        .expect("Failed to make OpenGL context current");

    (event_loop, window, surface, context)
}

fn glow_context(context: &PossiblyCurrentContext) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function_cstr(|s| context.display().get_proc_address(s).cast())
    }
}

fn imgui_init(window: &Window) -> (WinitPlatform, imgui::Context) {
    let mut imgui_context = imgui::Context::create();
    imgui_context.set_ini_filename(None);

    let mut winit_platform = WinitPlatform::init(&mut imgui_context);
    winit_platform.attach_window(
        imgui_context.io_mut(),
        window,
        imgui_winit_support::HiDpiMode::Rounded,
    );

    imgui_context
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    imgui_context.io_mut().font_global_scale = (1.0 / winit_platform.hidpi_factor()) as f32;

    (winit_platform, imgui_context)
}
