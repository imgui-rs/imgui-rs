use glow::HasContext;
use imgui::Context;
use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::SdlPlatform;
use sdl2::{event::Event, video::Window};

fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let window = video_subsystem
        .window("Hello imgui-rs!", 1280, 720)
        .allow_highdpi()
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();
    window.subsystem().gl_set_swap_interval(1).unwrap();

    let gl = glow_context(&window);

    let mut imgui = Context::create();

    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    let mut platform = SdlPlatform::init(&mut imgui);

    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    let mut renderer = AutoRenderer::initialize(gl, &mut imgui).unwrap();
    let mut last_frame = std::time::Instant::now();

    let mut event_pump = sdl.event_pump().unwrap();

    'main: loop {
        for event in event_pump.poll_iter() {
            platform.handle_event(imgui.io_mut(), &window, &event);

            match event {
                Event::Quit { .. } => break 'main,

                _ => {}
            }
        }
        platform.prepare_frame(imgui.io_mut(), &window, &event_pump);

        let now = std::time::Instant::now();

        imgui
            .io_mut()
            .update_delta_time(now.duration_since(last_frame));

        last_frame = now;

        let ui = imgui.frame();

        ui.show_demo_window(&mut true);

        platform.prepare_render(&ui, &window);
        let draw_data = ui.render();

        // This is the only extra render step to add
        unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
        renderer.render(draw_data).expect("error rendering imgui");

        window.gl_swap_window();
    }
}
