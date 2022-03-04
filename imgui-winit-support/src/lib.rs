//! This crate provides a winit-based backend platform for imgui-rs.
//!
//! A backend platform handles window/input device events and manages their
//! state.
//!
//! # Using the library
//!
//! There are five things you need to do to use this library correctly:
//!
//! 1. Initialize a `WinitPlatform` instance
//! 2. Attach it to a winit `Window`
//! 3. Pass events to the platform (every frame)
//! 4. Call frame preparation callback (every frame)
//! 5. Call render preparation callback (every frame)
//!
//! ## Complete example for winit 0.20+ (without a renderer)
//!
//! ```rust,no_run,ignore
//! # // TODO: Remove ignore when only one winit version is used
//! use imgui::Context;
//! use imgui_winit_support::{HiDpiMode, WinitPlatform};
//! use std::time::Instant;
//! use winit::event::{Event, WindowEvent};
//! use winit::event_loop::{ControlFlow, EventLoop};
//! use winit::window::{Window};
//!
//! let mut event_loop = EventLoop::new();
//! let mut window = Window::new(&event_loop).unwrap();
//!
//! let mut imgui = Context::create();
//! // configure imgui-rs Context if necessary
//!
//! let mut platform = WinitPlatform::init(&mut imgui); // step 1
//! platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Default); // step 2
//!
//! let mut last_frame = Instant::now();
//! let mut run = true;
//! event_loop.run(move |event, _, control_flow| {
//!     match event {
//!         Event::NewEvents(_) => {
//!             // other application-specific logic
//!             last_frame = imgui.io_mut().update_delta_time(last_frame);
//!         },
//!         Event::MainEventsCleared => {
//!             // other application-specific logic
//!             platform.prepare_frame(imgui.io_mut(), &window) // step 4
//!                 .expect("Failed to prepare frame");
//!             window.request_redraw();
//!         }
//!         Event::RedrawRequested(_) => {
//!             let ui = imgui.frame();
//!             // application-specific rendering *under the UI*
//!
//!             // construct the UI
//!
//!             platform.prepare_render(&ui, &window); // step 5
//!             // render the UI with a renderer
//!             let draw_data = ui.render();
//!             // renderer.render(..., draw_data).expect("UI rendering failed");
//!
//!             // application-specific rendering *over the UI*
//!         },
//!         Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
//!             *control_flow = ControlFlow::Exit;
//!         }
//!         // other application-specific event handling
//!         event => {
//!             platform.handle_event(imgui.io_mut(), &window, &event); // step 3
//!             // other application-specific event handling
//!         }
//!     }
//! })
//! ```
//!
//! ## `winit` versions and features.
//!
//! This crate has several features which control the version of winit which is
//! used.
//!
//! The following versions are supported, controlled by the listed feature.
//!
//! - The `winit-26` feature uses winit versions compatible with `0.26`. This is
//!   on by default, so to use any other version you need to disable this crates
//!   default features.
//! - The `winit-25` feature supports winit versions `0.25`.
//! - The `winit-24` feature supports winit versions `0.24`.
//! - The `winit-23` feature uses winit versions compatible with `0.23`.
//! - The `winit-22` feature uses winit versions compatible with `0.22`.
//! - The `winit-20` feature should support winit either `0.20` or winit `0.21`.
//! - The `winit-19` feature should support winits older than `0.19` (possibly
//!   back to winit 0.16.*, but this isn't regularly tested and may not work).
//!
//! If multiple `winit-*` features are enabled, and it is a debug build (as
//! determined by `debug_assertions`), we will log a warning to stderr during
//! init. This can be disabled by either turning on the `no-warn-on-multiple`
//! feature, fixing the configuration, or disabling `debug_assertions`.
//!
//! Conversely, if no `winit-*` features are enabled, we will fail to compile.
//! This is not an issue generally, as by default we turn on `winit-26`.
//!
//! All of this is in attempt to preserve the additive nature of features (while
//! still helping users notice project configuration issues), however it's done
//! fairly weakly as our this crate's API isn't 100% identical across winit
//! versions.
//!
//! ### Using an older `winit` version
//!
//! To use an older version, you must configure `default-features = false` in
//! your `Cargo.toml`:
//!
//! ```toml
//! [dependencies.imgui-winit-support]
//! version = "0.6"
//! features = ["winit-$YOUR_VERSION_HERE"]
//! default-features = false
//! ```
//!
//! ### Old `winit` compatibility
//!
//! No guarantee is made on how long this crate will support legacy versions of
//! `winit`, but we'll try to follow these rules:
//!
//! - Versions which are still in widespread use in the ecosystem will be
//!   supported while that is true (for example, 0.19 at the time of writing is
//!   quite old, but used by the most recent version of several popular crates).
//!
//! - Versions which are not a significant maintenance burden will be supported
//!   (for example, supporting versions older than winit 0.19 given that we
//!   support 0.19).
//!
//! - Explicitly removing support for a feature-indicated version will be
//!   considered a breaking change.
//!
//! - Changing the default feature to the new latest `winit` version is *not* a
//!   breaking change.

#[cfg(feature = "winit-26")]
use winit_26 as winit;

#[cfg(all(not(any(feature = "winit-26")), feature = "winit-25"))]
use winit_25 as winit;

#[cfg(all(
    not(any(feature = "winit-26", feature = "winit-25")),
    feature = "winit-24"
))]
use winit_24 as winit;

#[cfg(all(
    not(any(feature = "winit-26", feature = "winit-25", feature = "winit-24")),
    feature = "winit-23"
))]
use winit_23 as winit;

#[cfg(all(
    not(any(
        feature = "winit-26",
        feature = "winit-25",
        feature = "winit-24",
        feature = "winit-23"
    )),
    feature = "winit-22",
))]
use winit_22 as winit;

#[cfg(all(
    not(any(
        feature = "winit-26",
        feature = "winit-25",
        feature = "winit-24",
        feature = "winit-23",
        feature = "winit-22"
    )),
    feature = "winit-20",
))]
use winit_20 as winit;

#[cfg(all(
    not(any(
        feature = "winit-26",
        feature = "winit-25",
        feature = "winit-24",
        feature = "winit-23",
        feature = "winit-22",
        feature = "winit-20"
    )),
    feature = "winit-19",
))]
use winit_19 as winit;

use imgui::{self, BackendFlags, ConfigFlags, Context, Io, Key, Ui};
use std::cell::Cell;
use std::cmp::Ordering;
use winit::dpi::{LogicalPosition, LogicalSize};

#[cfg(all(
    not(any(
        feature = "winit-26",
        feature = "winit-25",
        feature = "winit-24",
        feature = "winit-23",
        feature = "winit-22",
        feature = "winit-20"
    )),
    feature = "winit-19",
))]
use winit::{
    DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, MouseCursor, MouseScrollDelta,
    TouchPhase, VirtualKeyCode, Window, WindowEvent,
};

#[cfg(any(
    feature = "winit-26",
    feature = "winit-25",
    feature = "winit-24",
    feature = "winit-23",
    feature = "winit-22",
    feature = "winit-20"
))]
use winit::{
    error::ExternalError,
    event::{
        DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, MouseScrollDelta, TouchPhase,
        VirtualKeyCode, WindowEvent,
    },
    window::{CursorIcon as MouseCursor, Window},
};

// Ensure at least one is enabled
#[cfg(not(any(
    feature = "winit-19",
    feature = "winit-20",
    feature = "winit-22",
    feature = "winit-23",
    feature = "winit-24",
    feature = "winit-25",
    feature = "winit-26",
)))]
compile_error!("Please enable at least one version of `winit` (see documentation for details).");

// FIXME(thom): make updading winit and adding a new feature less of a hassle here.
fn check_multiple_winits() {
    use std::io::Write as _;
    use std::sync::atomic::{AtomicBool, Ordering};
    // bail out for release builds or if we've been explicitly disabled.
    if cfg!(any(not(debug_assertions), feature = "no-warn-on-multiple")) {
        return;
    }
    let winits_enabled = cfg!(feature = "winit-26") as usize
        + cfg!(feature = "winit-25") as usize
        + cfg!(feature = "winit-24") as usize
        + cfg!(feature = "winit-23") as usize
        + cfg!(feature = "winit-22") as usize
        + cfg!(feature = "winit-20") as usize
        + cfg!(feature = "winit-19") as usize;

    // Only complain once even if we're called multiple times.
    static COMPLAINED: AtomicBool = AtomicBool::new(false);
    // Note that the `Ordering` basically doesn't matter here, but even if it
    // did, `Relaxed` is still correct because we're only interested in the
    // effects on a single atomic variable.
    if winits_enabled <= 1
        || COMPLAINED
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
    {
        return;
    }
    let mut err = Vec::with_capacity(512);

    // Log the complaint into a buffer first — in practice this is enough to
    // ensure atomicity.
    let _ = writeln!(
        err,
        "Warning (imgui-winit-support): More than one `winit-*` version feature is enabled \
        (this likely indicates misconfiguration, see documentation for details)."
    );
    let feats = [
        ("winit-26", cfg!(feature = "winit-26"), " (default)"),
        ("winit-25", cfg!(feature = "winit-25"), ""),
        ("winit-24", cfg!(feature = "winit-24"), ""),
        ("winit-23", cfg!(feature = "winit-23"), ""),
        ("winit-22", cfg!(feature = "winit-22"), ""),
        ("winit-20", cfg!(feature = "winit-20"), ""),
        ("winit-19", cfg!(feature = "winit-19"), ""),
    ];
    for &(name, enabled, extra) in &feats {
        if enabled {
            let _ = writeln!(err, "    `feature = {:?}` is enabled{}", name, extra);
        }
    }
    if cfg!(feature = "winit-25") && winits_enabled == 2 {
        let _ = writeln!(
            err,
            "    Perhaps you are missing a `default-features = false`?",
        );
    }
    let _ = writeln!(
        err,
        "    (Note: This warning is only present in debug builds, and \
        can be disabled using the \"no-warn-on-multiple\" feature)"
    );
    let _ = std::io::stderr().write_all(&err);
}

/// State of a single mouse button. Used so that we can detect cases where mouse
/// press and release occur on the same frame (seems surprisingly frequent on
/// macOS now...)
#[derive(Debug, Clone, Default)]
struct Button {
    pressed_this_frame: Cell<bool>,
    state: Cell<bool>,
}

impl Button {
    // we can use this in an array initializer, unlike `Default::default()` or a
    // `const fn new()`.
    #[allow(clippy::declare_interior_mutable_const)]
    const INIT: Button = Self {
        pressed_this_frame: Cell::new(false),
        state: Cell::new(false),
    };
    fn set(&self, pressed: bool) {
        self.state.set(pressed);
        if pressed {
            self.pressed_this_frame.set(true);
        }
    }
    fn get(&self) -> bool {
        // If we got a press this frame, record it even if we got a release
        // too — this way we don't drop mouse clicks where the release comes
        // in on the same frame as the press. (This mirrors what Dear ImGUI
        // seems to do in the `imgui_impl_*`)
        self.pressed_this_frame.replace(false) || self.state.get()
    }
}

/// winit backend platform state
#[derive(Debug)]
pub struct WinitPlatform {
    hidpi_mode: ActiveHiDpiMode,
    hidpi_factor: f64,
    cursor_cache: Option<CursorSettings>,
    mouse_buttons: [Button; 5],
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct CursorSettings {
    cursor: Option<imgui::MouseCursor>,
    draw_cursor: bool,
}

fn to_winit_cursor(cursor: imgui::MouseCursor) -> MouseCursor {
    match cursor {
        imgui::MouseCursor::Arrow => MouseCursor::Default,
        imgui::MouseCursor::TextInput => MouseCursor::Text,
        imgui::MouseCursor::ResizeAll => MouseCursor::Move,
        imgui::MouseCursor::ResizeNS => MouseCursor::NsResize,
        imgui::MouseCursor::ResizeEW => MouseCursor::EwResize,
        imgui::MouseCursor::ResizeNESW => MouseCursor::NeswResize,
        imgui::MouseCursor::ResizeNWSE => MouseCursor::NwseResize,
        imgui::MouseCursor::Hand => MouseCursor::Hand,
        imgui::MouseCursor::NotAllowed => MouseCursor::NotAllowed,
    }
}

impl CursorSettings {
    #[cfg(all(
        not(any(
            feature = "winit-26",
            feature = "winit-25",
            feature = "winit-24",
            feature = "winit-23",
            feature = "winit-22",
            feature = "winit-20"
        )),
        feature = "winit-19",
    ))]
    fn apply(&self, window: &Window) {
        match self.cursor {
            Some(mouse_cursor) if !self.draw_cursor => {
                window.hide_cursor(false);
                window.set_cursor(to_winit_cursor(mouse_cursor));
            }
            _ => window.hide_cursor(true),
        }
    }
    #[cfg(any(
        feature = "winit-20",
        feature = "winit-22",
        feature = "winit-23",
        feature = "winit-24",
        feature = "winit-25",
        feature = "winit-26"
    ))]
    fn apply(&self, window: &Window) {
        match self.cursor {
            Some(mouse_cursor) if !self.draw_cursor => {
                window.set_cursor_visible(true);
                window.set_cursor_icon(to_winit_cursor(mouse_cursor));
            }
            _ => window.set_cursor_visible(false),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum ActiveHiDpiMode {
    Default,
    Rounded,
    Locked,
}

/// DPI factor handling mode.
///
/// Applications that use imgui-rs might want to customize the used DPI factor and not use
/// directly the value coming from winit.
///
/// **Note: if you use a mode other than default and the DPI factor is adjusted, winit and imgui-rs
/// will use different logical coordinates, so be careful if you pass around logical size or
/// position values.**
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HiDpiMode {
    /// The DPI factor from winit is used directly without adjustment
    Default,
    /// The DPI factor from winit is rounded to an integer value.
    ///
    /// This prevents the user interface from becoming blurry with non-integer scaling.
    Rounded,
    /// The DPI factor from winit is ignored, and the included value is used instead.
    ///
    /// This is useful if you want to force some DPI factor (e.g. 1.0) and not care about the value
    /// coming from winit.
    Locked(f64),
}

impl HiDpiMode {
    fn apply(&self, hidpi_factor: f64) -> (ActiveHiDpiMode, f64) {
        match *self {
            HiDpiMode::Default => (ActiveHiDpiMode::Default, hidpi_factor),
            HiDpiMode::Rounded => (ActiveHiDpiMode::Rounded, hidpi_factor.round()),
            HiDpiMode::Locked(value) => (ActiveHiDpiMode::Locked, value),
        }
    }
}

#[cfg(feature = "viewports")]
struct ViewportBackend {}

#[cfg(feature = "viewports")]
impl imgui::PlatformViewportBackend for ViewportBackend {
    fn create_window(&mut self, viewport: &mut imgui::Viewport) {
        viewport.platform_user_data = Box::into_raw(Box::new(ViewportState {
            create: true,
            set_show: false,
            set_pos: None,
            set_size: None,
            set_focus: false,
            set_title: None,
            pos: [0.0, 0.0],
            size: [0.0, 0.0],
            focus: false,
            minimized: false,
        })) as *mut _;
    }

    fn destroy_window(&mut self, viewport: &mut imgui::Viewport) {
        unsafe {
            Box::from_raw(viewport.platform_user_data as *mut ViewportState);
        }
        viewport.platform_user_data = std::ptr::null_mut();
    }

    fn show_window(&mut self, viewport: &mut imgui::Viewport) {
        let state = unsafe { &mut *(viewport.platform_user_data as *mut ViewportState) };
        state.set_show = true;
    }

    fn set_window_pos(&mut self, viewport: &mut imgui::Viewport, pos: [f32; 2]) {
        let state = unsafe { &mut *(viewport.platform_user_data as *mut ViewportState) };
        state.set_pos = Some(pos);
    }

    fn get_window_pos(&mut self, viewport: &mut imgui::Viewport) -> [f32; 2] {
        let state = unsafe { &mut *(viewport.platform_user_data as *mut ViewportState) };
        state.pos
    }

    fn set_window_size(&mut self, viewport: &mut imgui::Viewport, size: [f32; 2]) {
        let state = unsafe { &mut *(viewport.platform_user_data as *mut ViewportState) };
        state.set_size = Some(size);
    }

    fn get_window_size(&mut self, viewport: &mut imgui::Viewport) -> [f32; 2] {
        let state = unsafe { &mut *(viewport.platform_user_data as *mut ViewportState) };
        state.size
    }

    fn set_window_focus(&mut self, viewport: &mut imgui::Viewport) {
        let state = unsafe { &mut *(viewport.platform_user_data as *mut ViewportState) };
        state.set_focus = true;
    }

    fn get_window_focus(&mut self, viewport: &mut imgui::Viewport) -> bool {
        let state = unsafe { &mut *(viewport.platform_user_data as *mut ViewportState) };
        state.focus
    }

    fn get_window_minimized(&mut self, viewport: &mut imgui::Viewport) -> bool {
        let state = unsafe { &mut *(viewport.platform_user_data as *mut ViewportState) };
        state.minimized
    }

    fn set_window_title(&mut self, viewport: &mut imgui::Viewport, title: &str) {
        let state = unsafe { &mut *(viewport.platform_user_data as *mut ViewportState) };
        state.set_title = Some(title.to_string());
    }

    fn set_window_alpha(&mut self, _viewport: &mut imgui::Viewport, _alpha: f32) {}

    fn update_window(&mut self, _viewport: &mut imgui::Viewport) {}

    fn render_window(&mut self, _viewport: &mut imgui::Viewport) {}

    fn swap_buffers(&mut self, _viewport: &mut imgui::Viewport) {}

    fn create_vk_surface(
        &mut self,
        _viewport: &mut imgui::Viewport,
        _instance: u64,
        _out_surface: &mut u64,
    ) -> i32 {
        0
    }
}

#[cfg(feature = "viewports")]
struct ViewportState {
    create: bool,

    set_show: bool,
    set_pos: Option<[f32; 2]>,
    set_size: Option<[f32; 2]>,
    set_focus: bool,
    set_title: Option<String>,

    pos: [f32; 2],
    size: [f32; 2],
    focus: bool,
    minimized: bool,
}

#[cfg(feature = "viewports")]
pub trait WinitPlatformViewportStorage {
    fn create_window(&mut self, id: imgui::Id, flags: imgui::ViewportFlags);
    fn remove_windows(&mut self, filter: impl Fn(imgui::Id) -> bool);
    fn get_window(
        &mut self,
        id: winit::window::WindowId,
    ) -> Option<(imgui::Id, &winit::window::Window)>;

    fn for_each(&mut self, func: impl FnMut(imgui::Id, &winit::window::Window));
}

impl WinitPlatform {
    /// Initializes a winit platform instance and configures imgui.
    ///
    /// This function configures imgui-rs in the following ways:
    ///
    /// * backend flags are updated
    /// * keys are configured
    /// * platform name is set
    pub fn init(imgui: &mut Context) -> WinitPlatform {
        // noop in non-debug builds, if disabled, or if called a second time.
        check_multiple_winits();
        let io = imgui.io_mut();
        io.backend_flags.insert(BackendFlags::HAS_MOUSE_CURSORS);
        io.backend_flags.insert(BackendFlags::HAS_SET_MOUSE_POS);
        io[Key::Tab] = VirtualKeyCode::Tab as _;
        io[Key::LeftArrow] = VirtualKeyCode::Left as _;
        io[Key::RightArrow] = VirtualKeyCode::Right as _;
        io[Key::UpArrow] = VirtualKeyCode::Up as _;
        io[Key::DownArrow] = VirtualKeyCode::Down as _;
        io[Key::PageUp] = VirtualKeyCode::PageUp as _;
        io[Key::PageDown] = VirtualKeyCode::PageDown as _;
        io[Key::Home] = VirtualKeyCode::Home as _;
        io[Key::End] = VirtualKeyCode::End as _;
        io[Key::Insert] = VirtualKeyCode::Insert as _;
        io[Key::Delete] = VirtualKeyCode::Delete as _;
        io[Key::Backspace] = VirtualKeyCode::Back as _;
        io[Key::Space] = VirtualKeyCode::Space as _;
        io[Key::Enter] = VirtualKeyCode::Return as _;
        io[Key::Escape] = VirtualKeyCode::Escape as _;
        io[Key::KeyPadEnter] = VirtualKeyCode::NumpadEnter as _;
        io[Key::A] = VirtualKeyCode::A as _;
        io[Key::C] = VirtualKeyCode::C as _;
        io[Key::V] = VirtualKeyCode::V as _;
        io[Key::X] = VirtualKeyCode::X as _;
        io[Key::Y] = VirtualKeyCode::Y as _;
        io[Key::Z] = VirtualKeyCode::Z as _;
        imgui.set_platform_name(Some(format!(
            "imgui-winit-support {}",
            env!("CARGO_PKG_VERSION")
        )));
        WinitPlatform {
            hidpi_mode: ActiveHiDpiMode::Default,
            hidpi_factor: 1.0,
            cursor_cache: None,
            mouse_buttons: [Button::INIT; 5],
        }
    }

    #[cfg(feature = "viewports")]
    pub fn init_viewports<T>(
        imgui: &mut Context,
        main_window: &winit::window::Window,
        event_loop: &winit::event_loop::EventLoop<T>,
    ) {
        let io = imgui.io_mut();

        io.backend_flags
            .insert(BackendFlags::PLATFORM_HAS_VIEWPORTS);

        imgui.set_platform_backend(ViewportBackend {});

        let mut monitors = Vec::new();
        for monitor in event_loop.available_monitors() {
            monitors.push(imgui::PlatformMonitor {
                main_pos: [monitor.position().x as f32, monitor.position().y as f32],
                main_size: [monitor.size().width as f32, monitor.size().height as f32],
                work_pos: [monitor.position().x as f32, monitor.position().y as f32],
                work_size: [monitor.size().width as f32, monitor.size().height as f32],
                dpi_scale: 1.0,
            });
        }
        imgui
            .platform_io_mut()
            .monitors
            .replace_from_slice(&monitors);

        let pos = main_window.inner_position().unwrap();
        let pos = [pos.x as f32, pos.y as f32];
        let size = main_window.inner_size();
        let size = [size.width as f32, size.height as f32];

        let main_viewport = imgui.main_viewport_mut();
        main_viewport.platform_user_data = Box::into_raw(Box::new(ViewportState {
            create: false,
            set_show: false,
            set_pos: None,
            set_size: None,
            set_focus: false,
            set_title: None,
            pos,
            size,
            focus: true,
            minimized: false,
        })) as *mut _;
    }

    #[cfg(feature = "viewports")]
    pub fn update_viewports(
        &mut self,
        imgui: &mut Context,
        storage: &mut impl WinitPlatformViewportStorage,
    ) {
        // remove destroyed windows
        storage.remove_windows(|id| !imgui.viewports().any(|vp| vp.id == id));

        // handle new viewports
        for viewport in imgui.viewports_mut() {
            let state = unsafe { &mut *(viewport.platform_user_data as *mut ViewportState) };

            if state.create {
                storage.create_window(viewport.id, viewport.flags);
                state.create = false;
            }
        }

        // handle other viewport events
        storage.for_each(|id, wnd| {
            let viewport = imgui.viewport_by_id_mut(id).unwrap();
            let state = unsafe { &mut *(viewport.platform_user_data as *mut ViewportState) };

            if let Some(pos) = &state.set_pos {
                let wnd_pos = wnd.outer_position().unwrap();
                let inner_pos = wnd.inner_position().unwrap();
                let decoration_size = [inner_pos.x - wnd_pos.x, inner_pos.y - wnd_pos.y];

                wnd.set_outer_position(winit::dpi::LogicalPosition::new(
                    pos[0] - decoration_size[0] as f32,
                    pos[1] - decoration_size[1] as f32,
                ));
                state.set_pos = None;
            }
            if let Some(size) = &state.set_size {
                wnd.set_inner_size(winit::dpi::LogicalSize::new(size[0], size[1]));
                state.set_size = None;
            }
            if state.set_show {
                wnd.set_visible(true);
                state.set_show = false;
            }
            if state.set_focus {
                wnd.focus_window();
                state.set_focus = false;
            }
            if let Some(title) = &state.set_title {
                wnd.set_title(title);
                state.set_title = None;
            }
        });
    }

    /// Attaches the platform instance to a winit window.
    ///
    /// This function configures imgui-rs in the following ways:
    ///
    /// * framebuffer scale (= DPI factor) is set
    /// * display size is set
    #[cfg(all(
        not(any(
            feature = "winit-26",
            feature = "winit-25",
            feature = "winit-24",
            feature = "winit-23",
            feature = "winit-22",
            feature = "winit-20"
        )),
        feature = "winit-19",
    ))]
    pub fn attach_window(&mut self, io: &mut Io, window: &Window, hidpi_mode: HiDpiMode) {
        let (hidpi_mode, hidpi_factor) = hidpi_mode.apply(window.get_hidpi_factor());
        self.hidpi_mode = hidpi_mode;
        self.hidpi_factor = hidpi_factor;
        io.display_framebuffer_scale = [hidpi_factor as f32, hidpi_factor as f32];
        if let Some(logical_size) = window.get_inner_size() {
            let logical_size = self.scale_size_from_winit(window, logical_size);
            io.display_size = [logical_size.width as f32, logical_size.height as f32];
        }
    }
    /// Attaches the platform instance to a winit window.
    ///
    /// This function configures imgui-rs in the following ways:
    ///
    /// * framebuffer scale (= DPI factor) is set
    /// * display size is set
    #[cfg(any(
        feature = "winit-20",
        feature = "winit-22",
        feature = "winit-23",
        feature = "winit-24",
        feature = "winit-25",
        feature = "winit-26",
    ))]
    pub fn attach_window(&mut self, io: &mut Io, window: &Window, hidpi_mode: HiDpiMode) {
        let (hidpi_mode, hidpi_factor) = hidpi_mode.apply(window.scale_factor());
        self.hidpi_mode = hidpi_mode;
        self.hidpi_factor = hidpi_factor;
        io.display_framebuffer_scale = [hidpi_factor as f32, hidpi_factor as f32];
        let logical_size = window.inner_size().to_logical(hidpi_factor);
        let logical_size = self.scale_size_from_winit(window, logical_size);
        io.display_size = [logical_size.width as f32, logical_size.height as f32];
    }
    /// Returns the current DPI factor.
    ///
    /// The value might not be the same as the winit DPI factor (depends on the used DPI mode)
    pub fn hidpi_factor(&self) -> f64 {
        self.hidpi_factor
    }
    /// Scales a logical size coming from winit using the current DPI mode.
    ///
    /// This utility function is useful if you are using a DPI mode other than default, and want
    /// your application to use the same logical coordinates as imgui-rs.
    #[cfg(all(
        not(any(
            feature = "winit-26",
            feature = "winit-25",
            feature = "winit-24",
            feature = "winit-23",
            feature = "winit-22",
            feature = "winit-20"
        )),
        feature = "winit-19",
    ))]
    pub fn scale_size_from_winit(&self, window: &Window, logical_size: LogicalSize) -> LogicalSize {
        match self.hidpi_mode {
            ActiveHiDpiMode::Default => logical_size,
            _ => logical_size
                .to_physical(window.get_hidpi_factor())
                .to_logical(self.hidpi_factor),
        }
    }
    /// Scales a logical size coming from winit using the current DPI mode.
    ///
    /// This utility function is useful if you are using a DPI mode other than default, and want
    /// your application to use the same logical coordinates as imgui-rs.
    #[cfg(any(
        feature = "winit-20",
        feature = "winit-22",
        feature = "winit-23",
        feature = "winit-24",
        feature = "winit-25",
        feature = "winit-26"
    ))]
    pub fn scale_size_from_winit(
        &self,
        window: &Window,
        logical_size: LogicalSize<f64>,
    ) -> LogicalSize<f64> {
        match self.hidpi_mode {
            ActiveHiDpiMode::Default => logical_size,
            _ => logical_size
                .to_physical::<f64>(window.scale_factor())
                .to_logical(self.hidpi_factor),
        }
    }
    /// Scales a logical position coming from winit using the current DPI mode.
    ///
    /// This utility function is useful if you are using a DPI mode other than default, and want
    /// your application to use the same logical coordinates as imgui-rs.
    #[cfg(all(
        not(any(
            feature = "winit-26",
            feature = "winit-25",
            feature = "winit-24",
            feature = "winit-23",
            feature = "winit-22",
            feature = "winit-20"
        )),
        feature = "winit-19",
    ))]
    pub fn scale_pos_from_winit(
        &self,
        window: &Window,
        logical_pos: LogicalPosition,
    ) -> LogicalPosition {
        match self.hidpi_mode {
            ActiveHiDpiMode::Default => logical_pos,
            _ => logical_pos
                .to_physical(window.get_hidpi_factor())
                .to_logical(self.hidpi_factor),
        }
    }
    /// Scales a logical position coming from winit using the current DPI mode.
    ///
    /// This utility function is useful if you are using a DPI mode other than default, and want
    /// your application to use the same logical coordinates as imgui-rs.
    #[cfg(any(
        feature = "winit-20",
        feature = "winit-22",
        feature = "winit-23",
        feature = "winit-24",
        feature = "winit-25",
        feature = "winit-26"
    ))]
    pub fn scale_pos_from_winit(
        &self,
        window: &Window,
        logical_pos: LogicalPosition<f64>,
    ) -> LogicalPosition<f64> {
        match self.hidpi_mode {
            ActiveHiDpiMode::Default => logical_pos,
            _ => logical_pos
                .to_physical::<f64>(window.scale_factor())
                .to_logical(self.hidpi_factor),
        }
    }
    /// Scales a logical position for winit using the current DPI mode.
    ///
    /// This utility function is useful if you are using a DPI mode other than default, and want
    /// your application to use the same logical coordinates as imgui-rs.
    #[cfg(all(
        not(any(
            feature = "winit-26",
            feature = "winit-25",
            feature = "winit-24",
            feature = "winit-23",
            feature = "winit-22",
            feature = "winit-20"
        )),
        feature = "winit-19",
    ))]
    pub fn scale_pos_for_winit(
        &self,
        window: &Window,
        logical_pos: LogicalPosition,
    ) -> LogicalPosition {
        match self.hidpi_mode {
            ActiveHiDpiMode::Default => logical_pos,
            _ => logical_pos
                .to_physical(self.hidpi_factor)
                .to_logical(window.get_hidpi_factor()),
        }
    }
    /// Scales a logical position for winit using the current DPI mode.
    ///
    /// This utility function is useful if you are using a DPI mode other than default, and want
    /// your application to use the same logical coordinates as imgui-rs.
    #[cfg(any(
        feature = "winit-20",
        feature = "winit-22",
        feature = "winit-23",
        feature = "winit-24",
        feature = "winit-25",
        feature = "winit-26"
    ))]
    pub fn scale_pos_for_winit(
        &self,
        window: &Window,
        logical_pos: LogicalPosition<f64>,
    ) -> LogicalPosition<f64> {
        match self.hidpi_mode {
            ActiveHiDpiMode::Default => logical_pos,
            _ => logical_pos
                .to_physical::<f64>(self.hidpi_factor)
                .to_logical(window.scale_factor()),
        }
    }
    /// Handles a winit event.
    ///
    /// This function performs the following actions (depends on the event):
    ///
    /// * window size / dpi factor changes are applied
    /// * keyboard state is updated
    /// * mouse state is updated
    #[cfg(all(
        not(any(
            feature = "winit-26",
            feature = "winit-25",
            feature = "winit-24",
            feature = "winit-23",
            feature = "winit-22",
            feature = "winit-20"
        )),
        feature = "winit-19",
    ))]
    pub fn handle_event(&mut self, io: &mut Io, window: &Window, event: &Event) {
        match *event {
            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == window.id() => {
                self.handle_window_event(io, window, event);
            }
            // Track key release events outside our window. If we don't do this,
            // we might never see the release event if some other window gets focus.
            Event::DeviceEvent {
                event:
                    DeviceEvent::Key(KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(key),
                        ..
                    }),
                ..
            } => {
                io.keys_down[key as usize] = false;
                match key {
                    VirtualKeyCode::LShift | VirtualKeyCode::RShift => io.key_shift = false,
                    VirtualKeyCode::LControl | VirtualKeyCode::RControl => io.key_ctrl = false,
                    VirtualKeyCode::LAlt | VirtualKeyCode::RAlt => io.key_alt = false,
                    VirtualKeyCode::LWin | VirtualKeyCode::RWin => io.key_super = false,
                    _ => (),
                }
            }
            _ => (),
        }
    }
    /// Handles a winit event.
    ///
    /// This function performs the following actions (depends on the event):
    ///
    /// * window size / dpi factor changes are applied
    /// * keyboard state is updated
    /// * mouse state is updated
    #[cfg(all(
        not(any(
            feature = "winit-26",
            feature = "winit-25",
            feature = "winit-24",
            feature = "winit-23",
            feature = "winit-22"
        )),
        feature = "winit-20",
    ))]
    pub fn handle_event<T>(&mut self, io: &mut Io, window: &Window, event: &Event<T>) {
        match *event {
            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == window.id() => {
                self.handle_window_event(io, window, event);
            }
            // Track key release events outside our window. If we don't do this,
            // we might never see the release event if some other window gets focus.
            Event::DeviceEvent {
                event:
                    DeviceEvent::Key(KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(key),
                        ..
                    }),
                ..
            } => {
                io.keys_down[key as usize] = false;
            }

            // We need to track modifiers separately because some system like macOS, will
            // not reliably send modifier states during certain events like ScreenCapture.
            // Gotta let the people show off their pretty imgui widgets!
            Event::DeviceEvent {
                event: DeviceEvent::ModifiersChanged(modifiers),
                ..
            } => {
                io.key_shift = modifiers.shift();
                io.key_ctrl = modifiers.ctrl();
                io.key_alt = modifiers.alt();
                io.key_super = modifiers.logo();
            }
            _ => (),
        }
    }
    /// Handles a winit event.
    ///
    /// This function performs the following actions (depends on the event):
    ///
    /// * window size / dpi factor changes are applied
    /// * keyboard state is updated
    /// * mouse state is updated
    #[cfg(any(
        feature = "winit-22",
        feature = "winit-23",
        feature = "winit-24",
        feature = "winit-25",
        feature = "winit-26",
    ))]
    pub fn handle_event<T>(&mut self, io: &mut Io, window: &Window, event: &Event<T>) {
        match *event {
            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == window.id() => {
                // We need to track modifiers separately because some system like macOS, will
                // not reliably send modifier states during certain events like ScreenCapture.
                // Gotta let the people show off their pretty imgui widgets!
                if let WindowEvent::ModifiersChanged(modifiers) = event {
                    io.key_shift = modifiers.shift();
                    io.key_ctrl = modifiers.ctrl();
                    io.key_alt = modifiers.alt();
                    io.key_super = modifiers.logo();
                }

                self.handle_window_event(io, window, event);
            }
            // Track key release events outside our window. If we don't do this,
            // we might never see the release event if some other window gets focus.
            Event::DeviceEvent {
                event:
                    DeviceEvent::Key(KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(key),
                        ..
                    }),
                ..
            } => {
                io.keys_down[key as usize] = false;
            }
            _ => (),
        }
    }
    #[cfg(feature = "viewports")]
    pub fn handle_viewport_event<T>(
        &mut self,
        imgui: &mut imgui::Context,
        main_window: &Window,
        storage: &mut impl WinitPlatformViewportStorage,
        event: &Event<T>,
    ) {
        if !imgui
            .io()
            .backend_flags
            .contains(BackendFlags::PLATFORM_HAS_VIEWPORTS | BackendFlags::RENDERER_HAS_VIEWPORTS)
            || !imgui
                .io()
                .config_flags
                .contains(ConfigFlags::VIEWPORTS_ENABLE)
        {
            return;
        }

        if let Event::WindowEvent {
            window_id,
            ref event,
        } = *event
        {
            let (viewport, window) = if window_id == main_window.id() {
                (imgui.main_viewport_mut(), main_window)
            } else if let Some((viewport_id, window)) = storage.get_window(window_id) {
                if let Some(viewport) = imgui.viewport_by_id_mut(viewport_id) {
                    (viewport, window)
                } else {
                    return;
                }
            } else {
                return;
            };

            let state = unsafe { &mut *(viewport.platform_user_data as *mut ViewportState) };

            match *event {
                WindowEvent::Resized(new_size) => {
                    state.size = [new_size.width as f32, new_size.height as f32];
                    if new_size.width == 0 || new_size.height == 0 {
                        state.minimized = true;
                    } else {
                        state.minimized = false;
                    }
                }
                WindowEvent::Moved(_new_pos) => {
                    let pos = window.inner_position().unwrap();
                    state.pos = [pos.x as f32, pos.y as f32];
                }
                WindowEvent::CloseRequested => {
                    viewport.platform_request_close = true;
                }
                WindowEvent::Focused(focus) => {
                    state.focus = focus;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let wnd_pos = window.inner_position().unwrap();
                    let pos = [
                        wnd_pos.x as f32 + position.x as f32,
                        wnd_pos.y as f32 + position.y as f32,
                    ];
                    imgui.io_mut().mouse_pos = pos;
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(key),
                            state,
                            ..
                        },
                    ..
                } if window_id != main_window.id() => {
                    let io = imgui.io_mut();

                    let pressed = state == ElementState::Pressed;
                    io.keys_down[key as usize] = pressed;

                    // This is a bit redundant here, but we'll leave it in. The OS occasionally
                    // fails to send modifiers keys, but it doesn't seem to send false-positives,
                    // so double checking isn't terrible in case some system *doesn't* send
                    // device events sometimes.
                    match key {
                        VirtualKeyCode::LShift | VirtualKeyCode::RShift => io.key_shift = pressed,
                        VirtualKeyCode::LControl | VirtualKeyCode::RControl => {
                            io.key_ctrl = pressed
                        }
                        VirtualKeyCode::LAlt | VirtualKeyCode::RAlt => io.key_alt = pressed,
                        VirtualKeyCode::LWin | VirtualKeyCode::RWin => io.key_super = pressed,
                        _ => (),
                    }
                }
                WindowEvent::ReceivedCharacter(ch) if window_id != main_window.id() => {
                    let io = imgui.io_mut();

                    // Exclude the backspace key ('\u{7f}'). Otherwise we will insert this char and then
                    // delete it.
                    if ch != '\u{7f}' {
                        io.add_input_character(ch)
                    }
                }
                WindowEvent::MouseWheel {
                    delta,
                    phase: TouchPhase::Moved,
                    ..
                } if window_id != main_window.id() => match delta {
                    MouseScrollDelta::LineDelta(h, v) => {
                        let io = imgui.io_mut();
                        io.mouse_wheel_h = h;
                        io.mouse_wheel = v;
                    }
                    MouseScrollDelta::PixelDelta(pos) => {
                        let io = imgui.io_mut();
                        let pos = pos.to_logical::<f64>(self.hidpi_factor);
                        match pos.x.partial_cmp(&0.0) {
                            Some(Ordering::Greater) => io.mouse_wheel_h += 1.0,
                            Some(Ordering::Less) => io.mouse_wheel_h -= 1.0,
                            _ => (),
                        }
                        match pos.y.partial_cmp(&0.0) {
                            Some(Ordering::Greater) => io.mouse_wheel += 1.0,
                            Some(Ordering::Less) => io.mouse_wheel -= 1.0,
                            _ => (),
                        }
                    }
                },
                WindowEvent::MouseInput { state, button, .. } if window_id != main_window.id() => {
                    let pressed = state == ElementState::Pressed;
                    match button {
                        MouseButton::Left => self.mouse_buttons[0].set(pressed),
                        MouseButton::Right => self.mouse_buttons[1].set(pressed),
                        MouseButton::Middle => self.mouse_buttons[2].set(pressed),
                        MouseButton::Other(idx @ 0..=4) => {
                            self.mouse_buttons[idx as usize].set(pressed)
                        }
                        _ => (),
                    }
                }
                _ => {}
            }
        }
    }

    #[cfg(all(
        not(any(
            feature = "winit-26",
            feature = "winit-25",
            feature = "winit-24",
            feature = "winit-23",
            feature = "winit-22",
            feature = "winit-20"
        )),
        feature = "winit-19",
    ))]
    fn handle_window_event(&mut self, io: &mut Io, window: &Window, event: &WindowEvent) {
        match *event {
            WindowEvent::Resized(logical_size) => {
                let logical_size = self.scale_size_from_winit(window, logical_size);
                io.display_size = [logical_size.width as f32, logical_size.height as f32];
            }
            WindowEvent::HiDpiFactorChanged(scale) => {
                let hidpi_factor = match self.hidpi_mode {
                    ActiveHiDpiMode::Default => scale,
                    ActiveHiDpiMode::Rounded => scale.round(),
                    _ => return,
                };
                // Mouse position needs to be changed while we still have both the old and the new
                // values
                if io.mouse_pos[0].is_finite() && io.mouse_pos[1].is_finite() {
                    io.mouse_pos = [
                        io.mouse_pos[0] * (hidpi_factor / self.hidpi_factor) as f32,
                        io.mouse_pos[1] * (hidpi_factor / self.hidpi_factor) as f32,
                    ];
                }
                self.hidpi_factor = hidpi_factor;
                io.display_framebuffer_scale = [hidpi_factor as f32, hidpi_factor as f32];
                // Window size might change too if we are using DPI rounding
                if let Some(logical_size) = window.get_inner_size() {
                    let logical_size = self.scale_size_from_winit(window, logical_size);
                    io.display_size = [logical_size.width as f32, logical_size.height as f32];
                }
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => {
                io.keys_down[key as usize] = state == ElementState::Pressed;
            }
            WindowEvent::ReceivedCharacter(ch) => {
                // Exclude the backspace key ('\u{7f}'). Otherwise we will insert this char and then
                // delete it.
                if ch != '\u{7f}' {
                    io.add_input_character(ch)
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let position = self.scale_pos_from_winit(window, position);
                io.mouse_pos = [position.x as f32, position.y as f32];
            }
            WindowEvent::MouseWheel {
                delta,
                phase: TouchPhase::Moved,
                ..
            } => match delta {
                MouseScrollDelta::LineDelta(h, v) => {
                    io.mouse_wheel_h = h;
                    io.mouse_wheel = v;
                }
                MouseScrollDelta::PixelDelta(pos) => {
                    match pos.x.partial_cmp(&0.0) {
                        Some(Ordering::Greater) => io.mouse_wheel_h += 1.0,
                        Some(Ordering::Less) => io.mouse_wheel_h -= 1.0,
                        _ => (),
                    }
                    match pos.y.partial_cmp(&0.0) {
                        Some(Ordering::Greater) => io.mouse_wheel += 1.0,
                        Some(Ordering::Less) => io.mouse_wheel -= 1.0,
                        _ => (),
                    }
                }
            },
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = state == ElementState::Pressed;
                match button {
                    MouseButton::Left => self.mouse_buttons[0].set(pressed),
                    MouseButton::Right => self.mouse_buttons[1].set(pressed),
                    MouseButton::Middle => self.mouse_buttons[2].set(pressed),
                    MouseButton::Other(idx @ 0..=4) => {
                        self.mouse_buttons[idx as usize].set(pressed)
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
    #[cfg(all(
        not(any(
            feature = "winit-23",
            feature = "winit-24",
            feature = "winit-25",
            feature = "winit-26"
        )),
        any(feature = "winit-20", feature = "winit-22")
    ))]
    fn handle_window_event(&mut self, io: &mut Io, window: &Window, event: &WindowEvent) {
        match *event {
            WindowEvent::Resized(physical_size) => {
                let logical_size = physical_size.to_logical(window.scale_factor());
                let logical_size = self.scale_size_from_winit(window, logical_size);
                io.display_size = [logical_size.width as f32, logical_size.height as f32];
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                let hidpi_factor = match self.hidpi_mode {
                    ActiveHiDpiMode::Default => scale_factor,
                    ActiveHiDpiMode::Rounded => scale_factor.round(),
                    _ => return,
                };
                // Mouse position needs to be changed while we still have both the old and the new
                // values
                if io.mouse_pos[0].is_finite() && io.mouse_pos[1].is_finite() {
                    io.mouse_pos = [
                        io.mouse_pos[0] * (hidpi_factor / self.hidpi_factor) as f32,
                        io.mouse_pos[1] * (hidpi_factor / self.hidpi_factor) as f32,
                    ];
                }
                self.hidpi_factor = hidpi_factor;
                io.display_framebuffer_scale = [hidpi_factor as f32, hidpi_factor as f32];
                // Window size might change too if we are using DPI rounding
                let logical_size = window.inner_size().to_logical(scale_factor);
                let logical_size = self.scale_size_from_winit(window, logical_size);
                io.display_size = [logical_size.width as f32, logical_size.height as f32];
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => {
                let pressed = state == ElementState::Pressed;
                io.keys_down[key as usize] = pressed;

                // This is a bit redundant here, but we'll leave it in. The OS occasionally
                // fails to send modifiers keys, but it doesn't seem to send false-positives,
                // so double checking isn't terrible in case some system *doesn't* send
                // device events sometimes.
                match key {
                    VirtualKeyCode::LShift | VirtualKeyCode::RShift => io.key_shift = pressed,
                    VirtualKeyCode::LControl | VirtualKeyCode::RControl => io.key_ctrl = pressed,
                    VirtualKeyCode::LAlt | VirtualKeyCode::RAlt => io.key_alt = pressed,
                    VirtualKeyCode::LWin | VirtualKeyCode::RWin => io.key_super = pressed,
                    _ => (),
                }
            }
            WindowEvent::ReceivedCharacter(ch) => {
                // Exclude the backspace key ('\u{7f}'). Otherwise we will insert this char and then
                // delete it.
                if ch != '\u{7f}' {
                    io.add_input_character(ch)
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let position = position.to_logical(window.scale_factor());
                let position = self.scale_pos_from_winit(window, position);
                io.mouse_pos = [position.x as f32, position.y as f32];
            }
            WindowEvent::MouseWheel {
                delta,
                phase: TouchPhase::Moved,
                ..
            } => match delta {
                MouseScrollDelta::LineDelta(h, v) => {
                    io.mouse_wheel_h = h;
                    io.mouse_wheel = v;
                }
                MouseScrollDelta::PixelDelta(pos) => {
                    match pos.x.partial_cmp(&0.0) {
                        Some(Ordering::Greater) => io.mouse_wheel_h += 1.0,
                        Some(Ordering::Less) => io.mouse_wheel_h -= 1.0,
                        _ => (),
                    }
                    match pos.y.partial_cmp(&0.0) {
                        Some(Ordering::Greater) => io.mouse_wheel += 1.0,
                        Some(Ordering::Less) => io.mouse_wheel -= 1.0,
                        _ => (),
                    }
                }
            },
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = state == ElementState::Pressed;
                match button {
                    MouseButton::Left => self.mouse_buttons[0].set(pressed),
                    MouseButton::Right => self.mouse_buttons[1].set(pressed),
                    MouseButton::Middle => self.mouse_buttons[2].set(pressed),
                    MouseButton::Other(idx @ 0..=4) => {
                        self.mouse_buttons[idx as usize].set(pressed)
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    #[cfg(any(
        feature = "winit-23",
        feature = "winit-24",
        feature = "winit-25",
        feature = "winit-26"
    ))]
    fn handle_window_event(&mut self, io: &mut Io, window: &Window, event: &WindowEvent) {
        match *event {
            WindowEvent::Resized(physical_size) => {
                let logical_size = physical_size.to_logical(window.scale_factor());
                let logical_size = self.scale_size_from_winit(window, logical_size);
                io.display_size = [logical_size.width as f32, logical_size.height as f32];
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                let hidpi_factor = match self.hidpi_mode {
                    ActiveHiDpiMode::Default => scale_factor,
                    ActiveHiDpiMode::Rounded => scale_factor.round(),
                    _ => return,
                };
                // Mouse position needs to be changed while we still have both the old and the new
                // values
                if io.mouse_pos[0].is_finite() && io.mouse_pos[1].is_finite() {
                    io.mouse_pos = [
                        io.mouse_pos[0] * (hidpi_factor / self.hidpi_factor) as f32,
                        io.mouse_pos[1] * (hidpi_factor / self.hidpi_factor) as f32,
                    ];
                }
                self.hidpi_factor = hidpi_factor;
                io.display_framebuffer_scale = [hidpi_factor as f32, hidpi_factor as f32];
                // Window size might change too if we are using DPI rounding
                let logical_size = window.inner_size().to_logical(scale_factor);
                let logical_size = self.scale_size_from_winit(window, logical_size);
                io.display_size = [logical_size.width as f32, logical_size.height as f32];
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => {
                let pressed = state == ElementState::Pressed;
                io.keys_down[key as usize] = pressed;

                // This is a bit redundant here, but we'll leave it in. The OS occasionally
                // fails to send modifiers keys, but it doesn't seem to send false-positives,
                // so double checking isn't terrible in case some system *doesn't* send
                // device events sometimes.
                match key {
                    VirtualKeyCode::LShift | VirtualKeyCode::RShift => io.key_shift = pressed,
                    VirtualKeyCode::LControl | VirtualKeyCode::RControl => io.key_ctrl = pressed,
                    VirtualKeyCode::LAlt | VirtualKeyCode::RAlt => io.key_alt = pressed,
                    VirtualKeyCode::LWin | VirtualKeyCode::RWin => io.key_super = pressed,
                    _ => (),
                }
            }
            WindowEvent::ReceivedCharacter(ch) => {
                // Exclude the backspace key ('\u{7f}'). Otherwise we will insert this char and then
                // delete it.
                if ch != '\u{7f}' {
                    io.add_input_character(ch)
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let position = position.to_logical(window.scale_factor());
                let position = self.scale_pos_from_winit(window, position);
                io.mouse_pos = [position.x as f32, position.y as f32];
            }
            WindowEvent::MouseWheel {
                delta,
                phase: TouchPhase::Moved,
                ..
            } => match delta {
                MouseScrollDelta::LineDelta(h, v) => {
                    io.mouse_wheel_h = h;
                    io.mouse_wheel = v;
                }
                MouseScrollDelta::PixelDelta(pos) => {
                    let pos = pos.to_logical::<f64>(self.hidpi_factor);
                    match pos.x.partial_cmp(&0.0) {
                        Some(Ordering::Greater) => io.mouse_wheel_h += 1.0,
                        Some(Ordering::Less) => io.mouse_wheel_h -= 1.0,
                        _ => (),
                    }
                    match pos.y.partial_cmp(&0.0) {
                        Some(Ordering::Greater) => io.mouse_wheel += 1.0,
                        Some(Ordering::Less) => io.mouse_wheel -= 1.0,
                        _ => (),
                    }
                }
            },
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = state == ElementState::Pressed;
                match button {
                    MouseButton::Left => self.mouse_buttons[0].set(pressed),
                    MouseButton::Right => self.mouse_buttons[1].set(pressed),
                    MouseButton::Middle => self.mouse_buttons[2].set(pressed),
                    MouseButton::Other(idx @ 0..=4) => {
                        self.mouse_buttons[idx as usize].set(pressed)
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
    /// Frame preparation callback.
    ///
    /// Call this before calling the imgui-rs context `frame` function.
    /// This function performs the following actions:
    ///
    /// * mouse cursor is repositioned (if requested by imgui-rs)
    #[cfg(all(
        not(any(
            feature = "winit-26",
            feature = "winit-25",
            feature = "winit-24",
            feature = "winit-23",
            feature = "winit-22",
            feature = "winit-20"
        )),
        feature = "winit-19",
    ))]
    pub fn prepare_frame(&self, io: &mut Io, window: &Window) -> Result<(), String> {
        self.copy_mouse_to_io(&mut io.mouse_down);
        if io.want_set_mouse_pos {
            let logical_pos = self.scale_pos_for_winit(
                window,
                LogicalPosition::new(f64::from(io.mouse_pos[0]), f64::from(io.mouse_pos[1])),
            );
            window.set_cursor_position(logical_pos)
        } else {
            Ok(())
        }
    }
    /// Frame preparation callback.
    ///
    /// Call this before calling the imgui-rs context `frame` function.
    /// This function performs the following actions:
    ///
    /// * mouse cursor is repositioned (if requested by imgui-rs)
    #[cfg(any(
        feature = "winit-20",
        feature = "winit-22",
        feature = "winit-23",
        feature = "winit-24",
        feature = "winit-25",
        feature = "winit-26",
    ))]
    pub fn prepare_frame(&self, io: &mut Io, window: &Window) -> Result<(), ExternalError> {
        self.copy_mouse_to_io(&mut io.mouse_down);
        if io.want_set_mouse_pos {
            let logical_pos = self.scale_pos_for_winit(
                window,
                LogicalPosition::new(f64::from(io.mouse_pos[0]), f64::from(io.mouse_pos[1])),
            );
            window.set_cursor_position(logical_pos)
        } else {
            Ok(())
        }
    }

    fn copy_mouse_to_io(&self, io_mouse_down: &mut [bool]) {
        for (io_down, button) in io_mouse_down.iter_mut().zip(&self.mouse_buttons) {
            *io_down = button.get();
        }
    }

    /// Render preparation callback.
    ///
    /// Call this before calling the imgui-rs UI `render_with`/`render` function.
    /// This function performs the following actions:
    ///
    /// * mouse cursor is changed and/or hidden (if requested by imgui-rs)
    pub fn prepare_render(&mut self, ui: &Ui, window: &Window) {
        let io = ui.io();
        if !io
            .config_flags
            .contains(ConfigFlags::NO_MOUSE_CURSOR_CHANGE)
        {
            let cursor = CursorSettings {
                cursor: ui.mouse_cursor(),
                draw_cursor: io.mouse_draw_cursor,
            };
            if self.cursor_cache != Some(cursor) {
                cursor.apply(window);
                self.cursor_cache = Some(cursor);
            }
        }
    }
}
