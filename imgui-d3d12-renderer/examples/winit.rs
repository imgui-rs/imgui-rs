use imgui::{im_str, FontConfig, FontSource};
use imgui_winit_support::{HiDpiMode, WinitPlatform};

use log::{error, info, trace, warn};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    platform::windows::WindowExtWindows,
    window::WindowBuilder,
};

use rusty_d3d12::*;

use std::{
    ffi::{c_void, CStr},
    os::raw::c_char,
    rc::Rc,
    slice,
    time::Instant,
};

#[no_mangle]
pub static D3D12SDKVersion: u32 = 4;

#[no_mangle]
pub static D3D12SDKPath: &[u8; 9] = b".\\D3D12\\\0";

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn debug_callback(
    category: i32,
    severity: i32,
    id: i32,
    description: *const c_char,
    _context: *mut c_void,
) {
    let severity: MessageSeverity = unsafe { std::mem::transmute(severity) };
    let category: MessageCategory = unsafe { std::mem::transmute(category) };
    let description = unsafe { CStr::from_ptr(description) };

    match severity {
        MessageSeverity::Message | MessageSeverity::Info => {
            info!(
                "[D3D12 Message][{}][{}][{:#x}] {}",
                severity,
                category,
                id as i32,
                description
                    .to_str()
                    .expect("Cannot make Rust string from D3D12 layer message")
            );
        }
        MessageSeverity::Warning => {
            warn!(
                "[D3D12 Message][{}][{}][{:#x}] {}",
                severity,
                category,
                id as i32,
                description
                    .to_str()
                    .expect("Cannot make Rust string from D3D12 layer message")
            );
        }
        _ => {
            error!(
                "[D3D12 Message][{}][{}][{:#x}] {}",
                severity,
                category,
                id as i32,
                description
                    .to_str()
                    .expect("Cannot make Rust string from D3D12 layer message")
            );
        }
    }
}

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

const FRAMES_IN_FLIGHT: usize = 3;

fn get_hardware_adapter(factory: &Factory) -> Adapter {
    let mut adapters = factory
        .enum_adapters_by_gpu_preference(GpuPreference::HighPerformance)
        .expect("Cannot enumerate adapters");

    for adapter in &adapters {
        let desc = adapter.get_desc().expect("Cannot get adapter desc");
        info!("found adapter: {}", desc);
    }
    adapters.remove(0)
}

fn create_device(factory: &Factory) -> Device {
    let adapter;
    adapter = get_hardware_adapter(factory);

    let adapter_desc = adapter.get_desc().expect("Cannot get adapter desc");

    info!("Enumerated adapter: \n\t{}", adapter_desc,);
    Device::new(&adapter)
        .unwrap_or_else(|_| panic!("Cannot create device on adapter {}", adapter_desc))
}

fn create_swapchain(
    factory: Factory,
    command_queue: &CommandQueue,
    hwnd: *mut std::ffi::c_void,
) -> Swapchain {
    let swapchain_desc = SwapchainDesc::default()
        .set_width(WINDOW_WIDTH)
        .set_height(WINDOW_HEIGHT)
        .set_buffer_count(FRAMES_IN_FLIGHT as u32);
    let swapchain = factory
        .create_swapchain(command_queue, hwnd as *mut HWND__, &swapchain_desc)
        .expect("Cannot create swapchain");
    factory
        .make_window_association(hwnd, MakeWindowAssociationFlags::NoAltEnter)
        .expect("Cannot make window association");
    swapchain
}

fn create_descriptor_heaps(device: &Device) -> (DescriptorHeap, DescriptorHeap) {
    let rtv_heap = device
        .create_descriptor_heap(
            &DescriptorHeapDesc::default()
                .set_heap_type(DescriptorHeapType::Rtv)
                .set_num_descriptors(FRAMES_IN_FLIGHT as u32),
        )
        .expect("Cannot create RTV heap");
    rtv_heap
        .set_name("RTV heap")
        .expect("Cannot set RTV heap name");

    let srv_uav_heap = device
        .create_descriptor_heap(
            &DescriptorHeapDesc::default()
                .set_heap_type(DescriptorHeapType::CbvSrvUav)
                .set_num_descriptors(1)
                .set_flags(DescriptorHeapFlags::ShaderVisible),
        )
        .expect("Cannot create srv_uav_heap");
    srv_uav_heap
        .set_name("CBV_SRV heap")
        .expect("Cannot set srv_uav_heap name");

    (rtv_heap, srv_uav_heap)
}

fn create_render_targets(
    device: &Device,
    rtv_heap: &DescriptorHeap,
    swapchain: &Swapchain,
) -> Vec<Resource> {
    let clear_value = ClearValue::default()
        .set_format(Format::R8G8B8A8_UNorm)
        .set_color([0.3, 0.3, 0.8, 1.]);

    let render_target_desc = ResourceDesc::default()
        .set_dimension(ResourceDimension::Texture2D)
        .set_format(Format::R8G8B8A8_UNorm)
        .set_width(WINDOW_WIDTH.into())
        .set_height(WINDOW_HEIGHT)
        .set_flags(ResourceFlags::AllowRenderTarget);

    let mut render_targets = vec![];

    for frame_idx in 0..FRAMES_IN_FLIGHT {
        render_targets.push(
            swapchain
                .get_buffer(frame_idx as u32)
                .expect("Cannot get buffer from swapchain"),
        );
    }

    let mut rtv_handle = rtv_heap.get_cpu_descriptor_handle_for_heap_start();
    for frame_idx in 0..FRAMES_IN_FLIGHT {
        device.create_render_target_view(&render_targets[frame_idx as usize], rtv_handle);

        rtv_handle = rtv_handle.advance(1);
    }

    trace!("created command allocators");

    render_targets
}

struct WinitSample {
    device: Device,
    debug_device: Option<DebugDevice>,
    info_queue: Option<Rc<InfoQueue>>,
    command_queue: CommandQueue,
    command_allocators: Vec<CommandAllocator>,
    command_list: CommandList,
    swapchain: Swapchain,
    render_targets: Vec<Resource>,
    frame_index: usize,
    rtv_heap: DescriptorHeap,
    srv_uav_heap: DescriptorHeap,

    frame_fence: Fence,
    frame_fence_value: u64,
    frame_fence_event: Win32Event,
}

impl WinitSample {
    fn new(hwnd: *mut c_void, use_debug: bool) -> Self {
        let mut factory_flags = CreateFactoryFlags::None;
        if use_debug {
            let debug_controller = Debug::new().expect("Cannot create debug controller");
            debug_controller.enable_debug_layer();
            debug_controller.enable_gpu_based_validation();
            debug_controller.enable_object_auto_name();
            factory_flags = CreateFactoryFlags::Debug;
        }

        let factory = Factory::new(factory_flags).expect("Cannot create factory");
        let device = create_device(&factory);

        let debug_device;
        if use_debug {
            debug_device = Some(DebugDevice::new(&device).expect("Cannot create debug device"));
        } else {
            debug_device = None;
        }

        let info_queue;
        if use_debug {
            let temp_info_queue = Rc::from(
                InfoQueue::new(
                    &device,
                    // Some(&[
                    //     MessageSeverity::Corruption,
                    //     MessageSeverity::Error,
                    //     MessageSeverity::Warning,
                    // ]),
                    None,
                )
                .expect("Cannot create debug info queue"),
            );

            temp_info_queue
                .register_callback(debug_callback, MessageCallbackFlags::FlagNone)
                .expect("Cannot set debug callback on info queue");

            info_queue = Some(temp_info_queue);
        } else {
            info_queue = None;
        }

        let mut command_allocators = vec![];
        for _ in 0..FRAMES_IN_FLIGHT {
            command_allocators.push(
                device
                    .create_command_allocator(CommandListType::Direct)
                    .expect("Cannot create command allocator"),
            );
        }

        let command_list = device
            .create_command_list(
                CommandListType::Direct,
                &command_allocators[0],
                None, // None,
            )
            .expect("Cannot create command list");
        command_list.close().expect("Cannot close command list");

        let command_queue = device
            .create_command_queue(
                &CommandQueueDesc::default().set_queue_type(CommandListType::Direct),
            )
            .expect("Cannot create direct command queue");

        let swapchain = create_swapchain(factory, &command_queue, hwnd);
        let frame_index = swapchain.get_current_back_buffer_index() as usize;
        trace!("Swapchain returned frame index {}", frame_index);

        let (rtv_heap, srv_uav_heap) = create_descriptor_heaps(&device);

        let render_targets = create_render_targets(&device, &rtv_heap, &swapchain);

        let frame_fence = device
            .create_fence(0, FenceFlags::None)
            .expect("Cannot create frame_fence");
        frame_fence
            .set_name("frame fence")
            .expect("Cannot set name on fence");

        let frame_fence_event = Win32Event::default();

        Self {
            device,
            debug_device,
            info_queue,
            command_queue,
            command_allocators,
            command_list,
            swapchain,
            render_targets,
            frame_index,
            rtv_heap,
            srv_uav_heap,

            frame_fence,
            frame_fence_value: 0,

            frame_fence_event,
        }
    }

    fn record_commands(&mut self) {
        trace!("Rendering frame, idx {}", self.frame_index);

        self.command_list
            .reset(&self.command_allocators[self.frame_index], None)
            .expect("Cannot reset command list");

        self.command_list
            .resource_barrier(slice::from_ref(&ResourceBarrier::new_transition(
                &ResourceTransitionBarrier::default()
                    .set_resource(&self.render_targets[self.frame_index])
                    .set_state_before(ResourceStates::Present)
                    .set_state_after(ResourceStates::RenderTarget),
            )));

        self.command_list
            .set_descriptor_heaps(slice::from_ref(&self.srv_uav_heap));

        self.command_list.clear_render_target_view(
            self.rtv_heap
                .get_cpu_descriptor_handle_for_heap_start()
                .advance(self.frame_index as u32),
            [1., 0.3, 0.3, 1.],
            &[],
        );

        let rtv_handles = [self
            .rtv_heap
            .get_cpu_descriptor_handle_for_heap_start()
            .advance(self.frame_index as u32)];
        self.command_list
            .set_render_targets(&rtv_handles, false, None);
    }

    fn submit_commands(&mut self) {
        trace!("Submitting commands, frame idx {}", self.frame_index);

        self.command_list
            .resource_barrier(slice::from_ref(&ResourceBarrier::new_transition(
                &ResourceTransitionBarrier::default()
                    .set_resource(&self.render_targets[self.frame_index])
                    .set_state_before(ResourceStates::RenderTarget)
                    .set_state_after(ResourceStates::Present),
            )));

        self.command_list
            .close()
            .expect("Cannot close command list");

        self.command_queue
            .execute_command_lists(std::slice::from_ref(&self.command_list));

        self.frame_fence_value += 1;

        self.command_queue
            .signal(&self.frame_fence, self.frame_fence_value)
            .expect("Cannot signal fence on queue");

        if self.frame_fence.get_completed_value() < self.frame_fence_value {
            self.frame_fence
                .set_event_on_completion(self.frame_fence_value, &self.frame_fence_event)
                .expect("Cannot set frame fence event");

            trace!("waiting on fence event (value {})", self.frame_fence_value);
            self.frame_fence_event.wait(None);
        }
    }

    fn present(&mut self) {
        self.swapchain
            .present(0, PresentFlags::None)
            .unwrap_or_else(|err| {
                error!("{}", err);
                let real_error = self.device.get_device_removed_reason();
                error!("Device removed reason: {}", real_error);
            });

        trace!("moving to the next frame");

        self.frame_index = self.swapchain.get_current_back_buffer_index() as usize;
    }
}

fn main() {
    let app_title = "ImGUI D3D12 Example";
    let command_args = clap::App::new(app_title)
        .arg(
            clap::Arg::with_name("breakonerr")
                .short("b")
                .takes_value(false)
                .help("Break on validation errors"),
        )
        .arg(
            clap::Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Verbosity level"),
        )
        .arg(
            clap::Arg::with_name("debug")
                .short("d")
                .takes_value(false)
                .help("Use D3D debug layers"),
        )
        .get_matches();

    let log_level: log::Level;
    match command_args.occurrences_of("v") {
        0 => log_level = log::Level::Debug,
        1 | _ => log_level = log::Level::Trace,
    };
    simple_logger::init_with_level(log_level).unwrap();
    // simple_logger::init_with_level(log::Level::Trace).unwrap();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(app_title)
        .with_inner_size(LogicalSize {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
        })
        .build(&event_loop)
        .unwrap();

    let mut app = WinitSample::new(
        window.hwnd(),
        command_args.is_present("debug"), // true,
    );

    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);

    let mut platform = WinitPlatform::init(&mut imgui);
    platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);

    let hidpi_factor = platform.hidpi_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.fonts().add_font(&[FontSource::DefaultFontData {
        config: Some(FontConfig {
            size_pixels: font_size,
            ..FontConfig::default()
        }),
    }]);

    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    let mut renderer = imgui_d3d12_renderer::Renderer::new(
        &mut imgui,
        app.device.clone(),
        FRAMES_IN_FLIGHT,
        app.srv_uav_heap.get_cpu_descriptor_handle_for_heap_start(),
        app.srv_uav_heap.get_gpu_descriptor_handle_for_heap_start(),
    )
    .expect("Cannot create renderer");

    let mut last_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| match event {
        Event::NewEvents(_) => {
            let now = Instant::now();
            imgui.io_mut().update_delta_time(now - last_frame);
            last_frame = now;
        }
        Event::MainEventsCleared => {
            let io = imgui.io_mut();
            platform
                .prepare_frame(io, &window)
                .expect("Failed to start frame");
            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            app.record_commands();

            let ui = imgui.frame();
            imgui::Window::new(im_str!("Hello world"))
                .size([300.0, 100.0], imgui::Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text(im_str!("Hello world!"));
                    ui.text(im_str!("This...is...imgui-rs!"));
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(im_str!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0],
                        mouse_pos[1]
                    ));
                });
            ui.show_demo_window(&mut true);
            platform.prepare_render(&ui, &window);
            renderer.render(ui.render(), &app.command_list).unwrap();

            app.submit_commands();
            app.present();
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = winit::event_loop::ControlFlow::Exit,
        Event::WindowEvent {
            event: WindowEvent::Resized(winit::dpi::PhysicalSize { height, width }),
            ..
        } => unsafe {
            // std::ptr::drop_in_place(&mut main_rtv);
            // swapchain.ResizeBuffers(0, width, height, DXGI_FORMAT_UNKNOWN, 0);
            // std::ptr::write(&mut main_rtv, create_render_target(&swapchain, &device));
            platform.handle_event(imgui.io_mut(), &window, &event);
        },
        Event::LoopDestroyed => (),
        event => {
            platform.handle_event(imgui.io_mut(), &window, &event);
        }
    });
}
