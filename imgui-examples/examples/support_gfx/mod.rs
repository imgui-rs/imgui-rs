use imgui::{ImGui, Ui};
use imgui_gfx_renderer::{Renderer, Shaders};
use std::time::Instant;

use gfx::{GraphicsPoolExt, CommandQueue, WindowExt, FrameSync, Adapter, Surface, Swapchain, SwapchainExt};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (i32, i32),
    pressed: (bool, bool, bool),
    wheel: f32,
}

pub fn run<F: FnMut(&Ui) -> bool>(title: String, clear_color: [f32; 4], mut run_ui: F) {
    use gfx::{self, Device};
    use gfx_window_glutin;
    use glutin;

    type ColorFormat = gfx::format::Rgba8;
    type DepthFormat = gfx::format::DepthStencil;

    let mut events_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new()
        .with_title(title)
        .with_dimensions(1024, 768);
    let gl_builder = glutin::ContextBuilder::new().with_vsync(true);
    let gl_window = glutin::GlWindow::new(wb, gl_builder, &events_loop).unwrap();
    let mut glutin_window = gfx_window_glutin::Window::new(gl_window);

    // Acquire surface and adapters
    let (mut surface, adapters) = glutin_window.get_surface_and_adapters();

    // Open gpu (device and queues)
    let gfx::Gpu { mut device, mut graphics_queues, .. } =
        adapters[0].open_with(|family, ty| {
            ((ty.supports_graphics() && surface.supports_queue(&family)) as u32, gfx::QueueType::Graphics)
        });
    let mut graphics_queue = graphics_queues.pop().expect("Unable to find a graphics queue.");

    // Create swapchain
    let config = gfx::SwapchainConfig::new()
        .with_color::<ColorFormat>()
        .with_depth_stencil::<DepthFormat>();
    let mut swap_chain = surface.build_swapchain(config, &graphics_queue);
    let views = swap_chain.create_color_views(&mut device);

    let mut graphics_pool = graphics_queue.create_graphics_pool(1);
    let frame_semaphore = device.create_semaphore();
    let draw_semaphore = device.create_semaphore();
    let frame_fence = device.create_fence(false);

    let shaders = {
        let version = device.get_info().shading_language;
        if version.is_embedded {
            if version.major >= 3 {
                Shaders::GlSlEs300
            } else {
                Shaders::GlSlEs100
            }
        } else {
            if version.major >= 4 {
                Shaders::GlSl400
            } else if version.major >= 3 {
                Shaders::GlSl130
            } else {
                Shaders::GlSl110
            }
        }
    };

    let mut imgui = ImGui::init();
    let mut renderer = Renderer::init(&mut imgui, &mut device, shaders, views[0].clone())
        .expect("Failed to initialize renderer");

    configure_keys(&mut imgui);

    let mut last_frame = Instant::now();
    let mut mouse_state = MouseState::default();
    let mut quit = false;

    loop {
        events_loop.poll_events(|event| {
            use glutin::WindowEvent::*;
            use glutin::ElementState::Pressed;
            use glutin::{Event, MouseButton, MouseScrollDelta, TouchPhase};

            if let Event::WindowEvent { event, .. } = event {
                match event {
                    Closed => quit = true,
                    KeyboardInput { input, .. } => {
                        use glutin::VirtualKeyCode as Key;

                        let pressed = input.state == Pressed;
                        match input.virtual_keycode {
                            Some(Key::Tab) => imgui.set_key(0, pressed),
                            Some(Key::Left) => imgui.set_key(1, pressed),
                            Some(Key::Right) => imgui.set_key(2, pressed),
                            Some(Key::Up) => imgui.set_key(3, pressed),
                            Some(Key::Down) => imgui.set_key(4, pressed),
                            Some(Key::PageUp) => imgui.set_key(5, pressed),
                            Some(Key::PageDown) => imgui.set_key(6, pressed),
                            Some(Key::Home) => imgui.set_key(7, pressed),
                            Some(Key::End) => imgui.set_key(8, pressed),
                            Some(Key::Delete) => imgui.set_key(9, pressed),
                            Some(Key::Back) => imgui.set_key(10, pressed),
                            Some(Key::Return) => imgui.set_key(11, pressed),
                            Some(Key::Escape) => imgui.set_key(12, pressed),
                            Some(Key::A) => imgui.set_key(13, pressed),
                            Some(Key::C) => imgui.set_key(14, pressed),
                            Some(Key::V) => imgui.set_key(15, pressed),
                            Some(Key::X) => imgui.set_key(16, pressed),
                            Some(Key::Y) => imgui.set_key(17, pressed),
                            Some(Key::Z) => imgui.set_key(18, pressed),
                            Some(Key::LControl) |
                            Some(Key::RControl) => imgui.set_key_ctrl(pressed),
                            Some(Key::LShift) |
                            Some(Key::RShift) => imgui.set_key_shift(pressed),
                            Some(Key::LAlt) | Some(Key::RAlt) => imgui.set_key_alt(pressed),
                            Some(Key::LWin) | Some(Key::RWin) => imgui.set_key_super(pressed),
                            _ => {}
                        }
                    }
                    MouseMoved { position: (x, y), .. } => mouse_state.pos = (x as i32, y as i32),
                    MouseInput { state, button, .. } => {
                        match button {
                            MouseButton::Left => mouse_state.pressed.0 = state == Pressed,
                            MouseButton::Right => mouse_state.pressed.1 = state == Pressed,
                            MouseButton::Middle => mouse_state.pressed.2 = state == Pressed,
                            _ => {}
                        }
                    }
                    MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_, y),
                        phase: TouchPhase::Moved,
                        ..
                    } |
                    MouseWheel {
                        delta: MouseScrollDelta::PixelDelta(_, y),
                        phase: TouchPhase::Moved,
                        ..
                    } => mouse_state.wheel = y,
                    ReceivedCharacter(c) => imgui.add_input_character(c),
                    _ => (),
                }
            }
        });

        if quit {
            break;
        }

        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;

        update_mouse(&mut imgui, &mut mouse_state);

        let size_points = glutin_window.raw().get_inner_size_points().unwrap();
        let size_pixels = glutin_window.raw().get_inner_size_pixels().unwrap();

        let ui = imgui.frame(size_points, size_pixels, delta_s);
        if !run_ui(&ui) {
            break;
        }

        // Get next frame
        let frame = swap_chain.acquire_frame(FrameSync::Semaphore(&frame_semaphore));
        renderer.update_render_target(views[frame.id()].clone());

        // draw a frame
        // wait for frame -> draw -> signal -> present
        {
            let mut encoder = graphics_pool.acquire_graphics_encoder();
            encoder.clear(&views[0].clone(), clear_color);

            renderer.render(ui, &mut device, &mut encoder).expect(
                "Rendering failed",
            );

            encoder.synced_flush(&mut graphics_queue, &[&frame_semaphore], &[&draw_semaphore], Some(&frame_fence))
                .expect("Could not flush encoder");
        }

        swap_chain.present(&mut graphics_queue, &[&draw_semaphore]);
        device.wait_for_fences(&[&frame_fence], gfx::WaitFor::All, 1_000_000);
        graphics_queue.cleanup();
        graphics_pool.reset();
    }
}

fn configure_keys(imgui: &mut ImGui) {
    use imgui::ImGuiKey;

    imgui.set_imgui_key(ImGuiKey::Tab, 0);
    imgui.set_imgui_key(ImGuiKey::LeftArrow, 1);
    imgui.set_imgui_key(ImGuiKey::RightArrow, 2);
    imgui.set_imgui_key(ImGuiKey::UpArrow, 3);
    imgui.set_imgui_key(ImGuiKey::DownArrow, 4);
    imgui.set_imgui_key(ImGuiKey::PageUp, 5);
    imgui.set_imgui_key(ImGuiKey::PageDown, 6);
    imgui.set_imgui_key(ImGuiKey::Home, 7);
    imgui.set_imgui_key(ImGuiKey::End, 8);
    imgui.set_imgui_key(ImGuiKey::Delete, 9);
    imgui.set_imgui_key(ImGuiKey::Backspace, 10);
    imgui.set_imgui_key(ImGuiKey::Enter, 11);
    imgui.set_imgui_key(ImGuiKey::Escape, 12);
    imgui.set_imgui_key(ImGuiKey::A, 13);
    imgui.set_imgui_key(ImGuiKey::C, 14);
    imgui.set_imgui_key(ImGuiKey::V, 15);
    imgui.set_imgui_key(ImGuiKey::X, 16);
    imgui.set_imgui_key(ImGuiKey::Y, 17);
    imgui.set_imgui_key(ImGuiKey::Z, 18);
}

fn update_mouse(imgui: &mut ImGui, mouse_state: &mut MouseState) {
    let scale = imgui.display_framebuffer_scale();
    imgui.set_mouse_pos(
        mouse_state.pos.0 as f32 / scale.0,
        mouse_state.pos.1 as f32 / scale.1,
    );
    imgui.set_mouse_down(
        &[
            mouse_state.pressed.0,
            mouse_state.pressed.1,
            mouse_state.pressed.2,
            false,
            false,
        ],
    );
    imgui.set_mouse_wheel(mouse_state.wheel / scale.1);
    mouse_state.wheel = 0.0;
}
