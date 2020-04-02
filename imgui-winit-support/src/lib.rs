//! This crate provides a winit-based backend platform for imgui-rs.
//!
//! A backend platform handles window/input device events and manages their state.
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

#[cfg(feature = "winit-19")]
use winit_19 as winit;

#[cfg(feature = "winit-20")]
use winit_20 as winit;

#[cfg(feature = "winit-22")]
use winit_22 as winit;

use imgui::{self, BackendFlags, ConfigFlags, Context, ImString, Io, Key, Ui};
use std::cmp::Ordering;
use winit::dpi::{LogicalPosition, LogicalSize};

#[cfg(feature = "winit-19")]
use winit::{
    DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, MouseCursor, MouseScrollDelta,
    TouchPhase, VirtualKeyCode, Window, WindowEvent,
};

#[cfg(any(feature = "winit-20", feature ="winit-22"))]
use winit::{
    error::ExternalError,
    event::{
        DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, MouseScrollDelta, TouchPhase,
        VirtualKeyCode, WindowEvent,
    },
    window::{CursorIcon as MouseCursor, Window},
};

/// winit backend platform state
#[derive(Debug)]
pub struct WinitPlatform {
    hidpi_mode: ActiveHiDpiMode,
    hidpi_factor: f64,
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

impl WinitPlatform {
    /// Initializes a winit platform instance and configures imgui.
    ///
    /// This function configures imgui-rs in the following ways:
    ///
    /// * backend flags are updated
    /// * keys are configured
    /// * platform name is set
    pub fn init(imgui: &mut Context) -> WinitPlatform {
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
        imgui.set_platform_name(Some(ImString::from(format!(
            "imgui-winit-support {}",
            env!("CARGO_PKG_VERSION")
        ))));
        WinitPlatform {
            hidpi_mode: ActiveHiDpiMode::Default,
            hidpi_factor: 1.0,
        }
    }
    /// Attaches the platform instance to a winit window.
    ///
    /// This function configures imgui-rs in the following ways:
    ///
    /// * framebuffer scale (= DPI factor) is set
    /// * display size is set
    #[cfg(feature = "winit-19")]
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
    #[cfg(any(feature = "winit-20", feature ="winit-22"))]
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
    #[cfg(feature = "winit-19")]
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
    #[cfg(any(feature = "winit-20", feature ="winit-22"))]
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
    #[cfg(feature = "winit-19")]
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
    #[cfg(any(feature = "winit-20", feature ="winit-22"))]
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
    #[cfg(feature = "winit-19")]
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
    #[cfg(any(feature = "winit-20", feature ="winit-22"))]
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
    #[cfg(feature = "winit-19")]
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
    #[cfg(feature = "winit-20")]
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
    #[cfg(any(feature = "winit-22"))]
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
    #[cfg(feature = "winit-19")]
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
                    MouseButton::Left => io.mouse_down[0] = pressed,
                    MouseButton::Right => io.mouse_down[1] = pressed,
                    MouseButton::Middle => io.mouse_down[2] = pressed,
                    MouseButton::Other(idx @ 0..=4) => io.mouse_down[idx as usize] = pressed,
                    _ => (),
                }
            }
            _ => (),
        }
    }
    #[cfg(any(feature = "winit-20", feature = "winit-22"))]
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
                    MouseButton::Left => io.mouse_down[0] = pressed,
                    MouseButton::Right => io.mouse_down[1] = pressed,
                    MouseButton::Middle => io.mouse_down[2] = pressed,
                    MouseButton::Other(idx @ 0..=4) => io.mouse_down[idx as usize] = pressed,
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
    #[cfg(feature = "winit-19")]
    pub fn prepare_frame(&self, io: &mut Io, window: &Window) -> Result<(), String> {
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
    #[cfg(any(feature = "winit-20", feature ="winit-22"))]
    pub fn prepare_frame(&self, io: &mut Io, window: &Window) -> Result<(), ExternalError> {
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
    /// Render preparation callback.
    ///
    /// Call this before calling the imgui-rs UI `render_with`/`render` function.
    /// This function performs the following actions:
    ///
    /// * mouse cursor is changed and/or hidden (if requested by imgui-rs)
    #[cfg(feature = "winit-19")]
    pub fn prepare_render(&self, ui: &Ui, window: &Window) {
        let io = ui.io();
        if !io
            .config_flags
            .contains(ConfigFlags::NO_MOUSE_CURSOR_CHANGE)
        {
            match ui.mouse_cursor() {
                Some(mouse_cursor) if !io.mouse_draw_cursor => {
                    window.hide_cursor(false);
                    window.set_cursor(match mouse_cursor {
                        imgui::MouseCursor::Arrow => MouseCursor::Arrow,
                        imgui::MouseCursor::TextInput => MouseCursor::Text,
                        imgui::MouseCursor::ResizeAll => MouseCursor::Move,
                        imgui::MouseCursor::ResizeNS => MouseCursor::NsResize,
                        imgui::MouseCursor::ResizeEW => MouseCursor::EwResize,
                        imgui::MouseCursor::ResizeNESW => MouseCursor::NeswResize,
                        imgui::MouseCursor::ResizeNWSE => MouseCursor::NwseResize,
                        imgui::MouseCursor::Hand => MouseCursor::Hand,
                        imgui::MouseCursor::NotAllowed => MouseCursor::NotAllowed,
                    });
                }
                _ => window.hide_cursor(true),
            }
        }
    }
    /// Render preparation callback.
    ///
    /// Call this before calling the imgui-rs UI `render_with`/`render` function.
    /// This function performs the following actions:
    ///
    /// * mouse cursor is changed and/or hidden (if requested by imgui-rs)
    #[cfg(any(feature = "winit-20", feature ="winit-22"))]
    pub fn prepare_render(&self, ui: &Ui, window: &Window) {
        let io = ui.io();
        if !io
            .config_flags
            .contains(ConfigFlags::NO_MOUSE_CURSOR_CHANGE)
        {
            match ui.mouse_cursor() {
                Some(mouse_cursor) if !io.mouse_draw_cursor => {
                    window.set_cursor_visible(true);
                    window.set_cursor_icon(match mouse_cursor {
                        imgui::MouseCursor::Arrow => MouseCursor::Arrow,
                        imgui::MouseCursor::TextInput => MouseCursor::Text,
                        imgui::MouseCursor::ResizeAll => MouseCursor::Move,
                        imgui::MouseCursor::ResizeNS => MouseCursor::NsResize,
                        imgui::MouseCursor::ResizeEW => MouseCursor::EwResize,
                        imgui::MouseCursor::ResizeNESW => MouseCursor::NeswResize,
                        imgui::MouseCursor::ResizeNWSE => MouseCursor::NwseResize,
                        imgui::MouseCursor::Hand => MouseCursor::Hand,
                        imgui::MouseCursor::NotAllowed => MouseCursor::NotAllowed,
                    });
                }
                _ => window.set_cursor_visible(false),
            }
        }
    }
}
