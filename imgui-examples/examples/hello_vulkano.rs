#[macro_use] extern crate vulkano;
extern crate vulkano_shader_derive;
extern crate winit;
extern crate vulkano_win;

mod support_vulkano;
use support_vulkano as support;

#[macro_use]
extern crate imgui;
extern crate imgui_vulkano_renderer;


fn main() {
    feature::main();
}

mod feature {
    use super::*;
    extern crate image;

    use std::sync::Arc;

    use support::MouseState;

    use winit;
    use vulkano;
    use vulkano::{
        format,
        command_buffer::AutoCommandBufferBuilder,
        format::D16Unorm,
        framebuffer::{
            Framebuffer,
            RenderPassAbstract,
        },
        image::{
            AttachmentImage,
            SwapchainImage,
        },
        swapchain,
        swapchain::AcquireError,
        sync::{
            GpuFuture,
            now,
            FlushError
        },
    };

    use winit::VirtualKeyCode as Key;
    use imgui::{FontGlyphRange, FrameSize, ImFontConfig, ImGui, };

    const WIN_W: u32 = 1024;
    const WIN_H: u32 =  768;
    const CLEAR_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

    pub fn main() {
        let mut window = support::vulkano_window::Window::new(WIN_W, WIN_H, "Conrod with vulkano");

        let subpass = vulkano::framebuffer::Subpass::from(window.render_pass.clone(), 0).expect("Couldn't create subpass for gui!");
        let mut render_helper = RenderHelper::new(&window);

        // IMGUI //
        let mut imgui = ImGui::init();
        imgui.set_ini_filename(None);

        support::configure_keys(&mut imgui);

        let font_size = 13.0 as f32;

        imgui.fonts().add_font_with_config(
            include_bytes!("../../resources/mplus-1p-regular.ttf"),
            ImFontConfig::new()
                .oversample_h(1)
                .pixel_snap_h(true)
                .size_pixels(font_size)
                .rasterizer_multiply(1.75),
            &FontGlyphRange::japanese(),
        );

        imgui.fonts().add_default_font_with_config(
            ImFontConfig::new()
                .merge_mode(true)
                .oversample_h(1)
                .pixel_snap_h(true)
                .size_pixels(font_size),
        );

        imgui.set_font_global_scale(1.0);

        let (mut renderer, futures) = imgui_vulkano_renderer::Renderer::init(&mut imgui,
                                                                             window.device.clone(),
                                                                             subpass,
                                                                             window.queue.clone(),
                                                                             WIN_W,  WIN_H,
                                                                             1.0).unwrap();

        let mut previous_frame_end = futures;
        let mut mouse_state = MouseState::default();

        let window_dpi = window.get_hidpi_factor();
        'main: loop {
            let mut should_quit = false;
            window.events_loop.poll_events(|event| {
                use winit::WindowEvent::*;

                if let winit::Event::WindowEvent { event, .. } = event {
                    match event {
                        CloseRequested => should_quit = true,
                        KeyboardInput { input, .. } => {
                            let pressed = input.state == winit::ElementState::Pressed;
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
                                Some(Key::LControl) | Some(Key::RControl) => {
                                    imgui.set_key_ctrl(pressed)
                                }
                                Some(Key::LShift) | Some(Key::RShift) => imgui.set_key_shift(pressed),
                                Some(Key::LAlt) | Some(Key::RAlt) => imgui.set_key_alt(pressed),
                                Some(Key::LWin) | Some(Key::RWin) => imgui.set_key_super(pressed),
                                _ => {}
                            }
                        },
                        CursorMoved { position: pos, .. } => {
                            // Rescale position from glutin logical coordinates to our logical
                            // coordinates
                            mouse_state.pos = pos
                                .to_physical(window_dpi)
                                .to_logical(1.0)
                                .into();
                        }
                        MouseInput { state, button, .. } => match button {
                            winit::MouseButton::Left => mouse_state.pressed.0 = state == winit::ElementState::Pressed,
                            winit::MouseButton::Right => mouse_state.pressed.1 = state == winit::ElementState::Pressed,
                            winit::MouseButton::Middle => mouse_state.pressed.2 = state == winit::ElementState::Pressed,
                            _ => { }
                        },
                        MouseWheel {
                            delta: winit::MouseScrollDelta::LineDelta(_, y),
                            phase: winit::TouchPhase::Moved,
                            ..
                        } => mouse_state.wheel = y,
                        MouseWheel {
                            delta: winit::MouseScrollDelta::PixelDelta(pos),
                            phase: winit::TouchPhase::Moved,
                            ..
                        } => {
                            // Rescale pixel delta from glutin logical coordinates to our logical
                            // coordinates
                            mouse_state.wheel = pos
                                .to_physical(window_dpi)
                                .to_logical(1.0)
                                .y as f32;
                        }
                        ReceivedCharacter(c) => imgui.add_input_character(c),
                        _ => (),
                    }
                }
            });

            support::update_mouse(&mut imgui, &mut mouse_state);

            // If the window is closed, this will be None for one tick, so to avoid panicking with
            // unwrap, instead break the loop
            let logical_size = match window.get_dimensions() {
                Some(s) => s,
                None => break 'main,
            };

            let (image_num, acquire_future) = match swapchain::acquire_next_image(window.swapchain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    render_helper.handle_resize(&mut window);
                    continue;
                },
                Err(err) => panic!("{:?}", err)
            };

            // We are tidy little fellows and cleanup our leftovers
            previous_frame_end.cleanup_finished();

            let mut command_buffer_builder = AutoCommandBufferBuilder::primary_one_time_submit(window.device.clone(), window.queue.family())
                .expect("Failed to create AutoCommandBufferBuilder");

            command_buffer_builder = command_buffer_builder.begin_render_pass(render_helper.frame_buffers[image_num].clone(), false,
                                                                                      vec![CLEAR_COLOR.into(), 1f32.into()]) // Info: We need to clear background AND depth buffer here!
                .expect("Failed to begin render pass!");

            let ui = imgui.frame(FrameSize {
                                    logical_size: (logical_size.width, logical_size.height),
                                    hidpi_factor: window.get_hidpi_factor(),
                                }, 1.0);

            ui.window(im_str!("Hello world"))
                .size((300.0, 100.0), imgui::ImGuiCond::FirstUseEver)
                .build(|| {
                            ui.text(im_str!("Hello world!"));
                            ui.text(im_str!("こんにちは世界！"));
                            ui.text(im_str!("This...is...imgui-rs!"));
                            ui.separator();
                            let mouse_pos = ui.imgui().mouse_pos();
                            ui.text(im_str!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos.0,
                        mouse_pos.1
                    ));
                });

            command_buffer_builder = renderer.render(ui, command_buffer_builder, window.device.clone(), [0.0, 0.0, logical_size.width as f32, logical_size.height as f32]).unwrap();

            let command_buffer = command_buffer_builder
                .end_render_pass().unwrap()
                .build().unwrap();

            let future = previous_frame_end.join(acquire_future)
                .then_execute(window.queue.clone(), command_buffer).expect("Failed to join previous frame with new one")
                .then_swapchain_present(window.queue.clone(), window.swapchain.clone(), image_num)
                .then_signal_fence_and_flush();

            match future {
                Ok(future) => previous_frame_end = Box::new(future) as Box<_>,
                Err(FlushError::OutOfDate) => previous_frame_end = Box::new(now(window.device.clone())) as Box<_>,
                Err(_) => {
                    previous_frame_end = Box::new(now(window.device.clone())) as Box<_>;
                }
            }

            //let winit_window_handle = window.surface.clone();
            //let winit_window_handle = winit_window_handle.window();

            if should_quit {
                break 'main;
            }

        }
    }


    pub struct RenderHelper {
        depth_buffer: Arc<AttachmentImage<D16Unorm>>,
        frame_buffers: Vec<Arc<Framebuffer<Arc<RenderPassAbstract + Send + Sync>, (((), Arc<SwapchainImage<winit::Window>>), Arc<AttachmentImage<D16Unorm>>)>>>,
        //previous_frame_end: Box<GpuFuture>,
        width: u32,
        height: u32,
    }

    impl RenderHelper {
        pub fn new(window: &support::vulkano_window::Window) -> Self {
            let logical_size = window.get_dimensions().expect("Couldn't get window dimensions");

            let depth_buffer = AttachmentImage::transient(window.device.clone(), [logical_size.width as u32, logical_size.height as u32], format::D16Unorm).unwrap();
            let frame_buffers: Vec<Arc<Framebuffer<Arc<RenderPassAbstract + Send + Sync>, (((), Arc<SwapchainImage<winit::Window>>), Arc<AttachmentImage<D16Unorm>>)>>> =
                window.images.iter().map(|image| {
                    Arc::new(Framebuffer::start(window.render_pass.clone())
                        .add(image.clone()).unwrap()
                        .add(depth_buffer.clone()).unwrap()
                        .build().unwrap())
                }).collect::<Vec<_>>();

            Self {
                depth_buffer,
                frame_buffers,
                width: logical_size.width as u32,
                height: logical_size.height as u32,
            }
        }

        pub fn handle_resize(&mut self, window: &mut support::vulkano_window::Window) -> () {
            window.handle_resize();

            let logical_size = window.get_dimensions().expect("Couldn't get window dimensions");
            if self.width != logical_size.width as u32 || self.height != logical_size.height as u32 {
                self.depth_buffer = AttachmentImage::transient(window.device.clone(), [logical_size.width as u32, logical_size.height as u32], format::D16Unorm).unwrap();
            }

            let new_framebuffers = window.images.iter().map(|image| {
                Arc::new(Framebuffer::start(window.render_pass.clone())
                    .add(image.clone()).unwrap()
                    .add(self.depth_buffer.clone()).unwrap()
                    .build().unwrap())
            }).collect::<Vec<_>>();
            std::mem::replace(&mut self.frame_buffers, new_framebuffers);
        }
    }
}