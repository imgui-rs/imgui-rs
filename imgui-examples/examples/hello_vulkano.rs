#[macro_use] extern crate vulkano;
#[macro_use] extern crate vulkano_shader_derive;
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

    use std::mem;
    use std::sync::Arc;
    use std::path::Path;

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

    use imgui::{FontGlyphRange, FrameSize, ImFontConfig, ImGui, ImGuiMouseCursor, Ui};

    const WIN_W: u32 = 1024;
    const WIN_H: u32 =  768;
    const CLEAR_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

    pub fn main() {
        let mut window = support::vulkano_window::Window::new(WIN_W, WIN_H, "Conrod with vulkano");

        let subpass = vulkano::framebuffer::Subpass::from(window.render_pass.clone(), 0).expect("Couldn't create subpass for gui!");
        let queue = window.queue.clone();

        let mut render_helper = RenderHelper::new(&window);

        // IMGUI //
        let mut imgui = ImGui::init();
        imgui.set_ini_filename(None);

        let font_size = 13.0 as f32;

        imgui.fonts().add_font_with_config(
            include_bytes!("mplus-1p-regular.ttf"),
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

        'main: loop {
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
                                    hidpi_factor: 1.0,
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
                Err(e) => {
                    previous_frame_end = Box::new(now(window.device.clone())) as Box<_>;
                }
            }


            let mut should_quit = false;

            let winit_window_handle = window.surface.clone();
            let winit_window_handle = winit_window_handle.window();

            window.events_loop.poll_events(|event| {
                //let dpi_factor = dpi_factor as conrod::Scalar;

                // Convert winit event to conrod event, requires conrod to be built with the `winit` feature
                //if let Some(event) = conrod::backend::winit::convert_event(event.clone(), winit_window_handle) {
                //
                //}

                // Close window if the escape key or the exit button is pressed
                match event {
                    winit::Event::WindowEvent { event: winit::WindowEvent::KeyboardInput { input: winit::KeyboardInput { virtual_keycode: Some(winit::VirtualKeyCode::Escape), .. }, .. }, .. } |
                    winit::Event::WindowEvent { event: winit::WindowEvent::CloseRequested, .. } =>
                        should_quit = true,
                    _ => {}
                }
            });
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
            mem::replace(&mut self.frame_buffers, new_framebuffers);
        }
    }
}