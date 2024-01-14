use std::{ffi::CString, num::NonZeroU32, time::Instant};

use glow::{Context, HasContext};
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentGlContext},
    display::GetGlDisplay,
    prelude::GlDisplay,
    surface::{GlSurface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use imgui::ConfigFlags;
use imgui_winit_glow_renderer_viewports::Renderer;
use raw_window_handle::HasRawWindowHandle;
use winit::{
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create EventLoop");

    let window_builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(800.0, 600.0))
        .with_visible(true)
        .with_resizable(true)
        .with_title("Viewports example");

    let template_builder = ConfigTemplateBuilder::new();
    let (window, gl_config) = DisplayBuilder::new()
        .with_window_builder(Some(window_builder))
        .build(&event_loop, template_builder, |mut configs| {
            configs.next().unwrap()
        })
        .expect("Failed to create main window");

    let window = window.unwrap();

    let context_attribs = ContextAttributesBuilder::new().build(Some(window.raw_window_handle()));
    let context = unsafe {
        gl_config
            .display()
            .create_context(&gl_config, &context_attribs)
            .expect("Failed to create main context")
    };

    let size = window.inner_size();
    let surface_attribs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        window.raw_window_handle(),
        NonZeroU32::new(size.width).unwrap(),
        NonZeroU32::new(size.height).unwrap(),
    );
    let surface = unsafe {
        gl_config
            .display()
            .create_window_surface(&gl_config, &surface_attribs)
            .expect("Failed to create main surface")
    };

    let context = context
        .make_current(&surface)
        .expect("Failed to make current");

    let glow = unsafe {
        Context::from_loader_function(|name| {
            let name = CString::new(name).unwrap();
            context.display().get_proc_address(&name)
        })
    };

    let mut imgui = imgui::Context::create();
    imgui
        .io_mut()
        .config_flags
        .insert(ConfigFlags::DOCKING_ENABLE);
    imgui
        .io_mut()
        .config_flags
        .insert(ConfigFlags::VIEWPORTS_ENABLE);
    imgui.set_ini_filename(None);

    let mut renderer = Renderer::new(&mut imgui, &window, &glow).expect("Failed to init Renderer");

    let mut last_frame = Instant::now();

    event_loop
        .run(move |event, window_target| {
            window_target.set_control_flow(ControlFlow::Poll);

            renderer.handle_event(&mut imgui, &window, &event);

            match event {
                winit::event::Event::NewEvents(_) => {
                    let now = Instant::now();
                    imgui.io_mut().update_delta_time(now - last_frame);
                    last_frame = now;
                }
                winit::event::Event::WindowEvent {
                    window_id,
                    event: WindowEvent::CloseRequested,
                } if window_id == window.id() => {
                    window_target.exit();
                }
                winit::event::Event::WindowEvent {
                    window_id,
                    event: WindowEvent::Resized(new_size),
                } if window_id == window.id() => {
                    surface.resize(
                        &context,
                        NonZeroU32::new(new_size.width).unwrap(),
                        NonZeroU32::new(new_size.height).unwrap(),
                    );
                }
                winit::event::Event::AboutToWait => {
                    window.request_redraw();
                }
                winit::event::Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    let ui = imgui.frame();

                    ui.dockspace_over_main_viewport();

                    ui.show_demo_window(&mut true);
                    ui.window("Style Editor").build(|| {
                        ui.show_default_style_editor();
                    });

                    ui.end_frame_early();

                    renderer.prepare_render(&mut imgui, &window);

                    imgui.update_platform_windows();
                    renderer
                        .update_viewports(&mut imgui, window_target, &glow)
                        .expect("Failed to update viewports");

                    let draw_data = imgui.render();

                    if let Err(e) = context.make_current(&surface) {
                        // For some reason make_current randomly throws errors on windows.
                        // Until the reason for this is found, we just print it out instead of panicing.
                        eprintln!("Failed to make current: {e}");
                    }

                    unsafe {
                        glow.disable(glow::SCISSOR_TEST);
                        glow.clear(glow::COLOR_BUFFER_BIT);
                    }

                    renderer
                        .render(&window, &glow, draw_data)
                        .expect("Failed to render main viewport");

                    surface
                        .swap_buffers(&context)
                        .expect("Failed to swap buffers");

                    renderer
                        .render_viewports(&glow, &mut imgui)
                        .expect("Failed to render viewports");
                }
                _ => {}
            }
        })
        .expect("EventLoop error");
}
