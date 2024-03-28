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
//! ## Complete example (without a renderer)
//!
//! ```no_run
//! use imgui::Context;
//! use imgui_winit_support::{HiDpiMode, WinitPlatform};
//! use std::time::Instant;
//! use winit::event::{Event, WindowEvent};
//! use winit::event_loop::{ControlFlow, EventLoop};
//! use winit::window::Window;
//!
//! let mut event_loop = EventLoop::new().expect("Failed to create EventLoop");
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
//! event_loop.run(move |event, window_target| {
//!     match event {
//!         Event::NewEvents(_) => {
//!             // other application-specific logic
//!             let now = Instant::now();
//!             imgui.io_mut().update_delta_time(now - last_frame);
//!             last_frame = now;
//!         },
//!         Event::AboutToWait => {
//!             // other application-specific logic
//!             platform.prepare_frame(imgui.io_mut(), &window) // step 4
//!                 .expect("Failed to prepare frame");
//!             window.request_redraw();
//!         }
//!         Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
//!             let ui = imgui.frame();
//!             // application-specific rendering *under the UI*
//!
//!             // construct the UI
//!
//!             platform.prepare_render(&ui, &window); // step 5
//!             // render the UI with a renderer
//!             let draw_data = imgui.render();
//!             // renderer.render(..., draw_data).expect("UI rendering failed");
//!
//!             // application-specific rendering *over the UI*
//!         },
//!         Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
//!             window_target.exit();
//!         }
//!         // other application-specific event handling
//!         event => {
//!             platform.handle_event(imgui.io_mut(), &window, &event); // step 3
//!             // other application-specific event handling
//!         }
//!     }
//! }).expect("EventLoop error");
//! ```

use imgui::{self, BackendFlags, ConfigFlags, Context, Io, Key, Ui};
use std::cmp::Ordering;

// Re-export winit to make it easier for users to use the correct version.
pub use winit;
use winit::{
    dpi::{LogicalPosition, LogicalSize},
    keyboard::{Key as WinitKey, KeyLocation, NamedKey},
    platform::modifier_supplement::KeyEventExtModifierSupplement,
};

use winit::{
    error::ExternalError,
    event::{ElementState, Event, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent},
    window::{CursorIcon as MouseCursor, Window},
};

/// winit backend platform state
#[derive(Debug)]
pub struct WinitPlatform {
    hidpi_mode: ActiveHiDpiMode,
    hidpi_factor: f64,
    cursor_cache: Option<CursorSettings>,
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
        imgui::MouseCursor::Hand => MouseCursor::Grab,
        imgui::MouseCursor::NotAllowed => MouseCursor::NotAllowed,
    }
}

impl CursorSettings {
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

fn to_imgui_mouse_button(button: MouseButton) -> Option<imgui::MouseButton> {
    match button {
        MouseButton::Left | MouseButton::Other(0) => Some(imgui::MouseButton::Left),
        MouseButton::Right | MouseButton::Other(1) => Some(imgui::MouseButton::Right),
        MouseButton::Middle | MouseButton::Other(2) => Some(imgui::MouseButton::Middle),
        MouseButton::Other(3) => Some(imgui::MouseButton::Extra1),
        MouseButton::Other(4) => Some(imgui::MouseButton::Extra2),
        _ => None,
    }
}

fn to_imgui_key(key: winit::keyboard::Key, location: KeyLocation) -> Option<Key> {
    match (key.as_ref(), location) {
        (WinitKey::Named(NamedKey::Tab), _) => Some(Key::Tab),
        (WinitKey::Named(NamedKey::ArrowLeft), _) => Some(Key::LeftArrow),
        (WinitKey::Named(NamedKey::ArrowRight), _) => Some(Key::RightArrow),
        (WinitKey::Named(NamedKey::ArrowUp), _) => Some(Key::UpArrow),
        (WinitKey::Named(NamedKey::ArrowDown), _) => Some(Key::DownArrow),
        (WinitKey::Named(NamedKey::PageUp), _) => Some(Key::PageUp),
        (WinitKey::Named(NamedKey::PageDown), _) => Some(Key::PageDown),
        (WinitKey::Named(NamedKey::Home), _) => Some(Key::Home),
        (WinitKey::Named(NamedKey::End), _) => Some(Key::End),
        (WinitKey::Named(NamedKey::Insert), _) => Some(Key::Insert),
        (WinitKey::Named(NamedKey::Delete), _) => Some(Key::Delete),
        (WinitKey::Named(NamedKey::Backspace), _) => Some(Key::Backspace),
        (WinitKey::Named(NamedKey::Space), _) => Some(Key::Space),
        (WinitKey::Named(NamedKey::Enter), KeyLocation::Standard) => Some(Key::Enter),
        (WinitKey::Named(NamedKey::Enter), KeyLocation::Numpad) => Some(Key::KeypadEnter),
        (WinitKey::Named(NamedKey::Escape), _) => Some(Key::Escape),
        (WinitKey::Named(NamedKey::Control), KeyLocation::Left) => Some(Key::LeftCtrl),
        (WinitKey::Named(NamedKey::Control), KeyLocation::Right) => Some(Key::RightCtrl),
        (WinitKey::Named(NamedKey::Shift), KeyLocation::Left) => Some(Key::LeftShift),
        (WinitKey::Named(NamedKey::Shift), KeyLocation::Right) => Some(Key::RightShift),
        (WinitKey::Named(NamedKey::Alt), KeyLocation::Left) => Some(Key::LeftAlt),
        (WinitKey::Named(NamedKey::Alt), KeyLocation::Right) => Some(Key::RightAlt),
        (WinitKey::Named(NamedKey::Super), KeyLocation::Left) => Some(Key::LeftSuper),
        (WinitKey::Named(NamedKey::Super), KeyLocation::Right) => Some(Key::RightSuper),
        (WinitKey::Named(NamedKey::ContextMenu), _) => Some(Key::Menu),
        (WinitKey::Named(NamedKey::F1), _) => Some(Key::F1),
        (WinitKey::Named(NamedKey::F2), _) => Some(Key::F2),
        (WinitKey::Named(NamedKey::F3), _) => Some(Key::F3),
        (WinitKey::Named(NamedKey::F4), _) => Some(Key::F4),
        (WinitKey::Named(NamedKey::F5), _) => Some(Key::F5),
        (WinitKey::Named(NamedKey::F6), _) => Some(Key::F6),
        (WinitKey::Named(NamedKey::F7), _) => Some(Key::F7),
        (WinitKey::Named(NamedKey::F8), _) => Some(Key::F8),
        (WinitKey::Named(NamedKey::F9), _) => Some(Key::F9),
        (WinitKey::Named(NamedKey::F10), _) => Some(Key::F10),
        (WinitKey::Named(NamedKey::F11), _) => Some(Key::F11),
        (WinitKey::Named(NamedKey::F12), _) => Some(Key::F12),
        (WinitKey::Named(NamedKey::CapsLock), _) => Some(Key::CapsLock),
        (WinitKey::Named(NamedKey::ScrollLock), _) => Some(Key::ScrollLock),
        (WinitKey::Named(NamedKey::NumLock), _) => Some(Key::NumLock),
        (WinitKey::Named(NamedKey::PrintScreen), _) => Some(Key::PrintScreen),
        (WinitKey::Named(NamedKey::Pause), _) => Some(Key::Pause),
        (WinitKey::Character("0"), KeyLocation::Standard) => Some(Key::Alpha0),
        (WinitKey::Character("1"), KeyLocation::Standard) => Some(Key::Alpha1),
        (WinitKey::Character("2"), KeyLocation::Standard) => Some(Key::Alpha2),
        (WinitKey::Character("3"), KeyLocation::Standard) => Some(Key::Alpha3),
        (WinitKey::Character("4"), KeyLocation::Standard) => Some(Key::Alpha4),
        (WinitKey::Character("5"), KeyLocation::Standard) => Some(Key::Alpha5),
        (WinitKey::Character("6"), KeyLocation::Standard) => Some(Key::Alpha6),
        (WinitKey::Character("7"), KeyLocation::Standard) => Some(Key::Alpha7),
        (WinitKey::Character("8"), KeyLocation::Standard) => Some(Key::Alpha8),
        (WinitKey::Character("9"), KeyLocation::Standard) => Some(Key::Alpha9),
        (WinitKey::Character("0"), KeyLocation::Numpad) => Some(Key::Keypad0),
        (WinitKey::Character("1"), KeyLocation::Numpad) => Some(Key::Keypad1),
        (WinitKey::Character("2"), KeyLocation::Numpad) => Some(Key::Keypad2),
        (WinitKey::Character("3"), KeyLocation::Numpad) => Some(Key::Keypad3),
        (WinitKey::Character("4"), KeyLocation::Numpad) => Some(Key::Keypad4),
        (WinitKey::Character("5"), KeyLocation::Numpad) => Some(Key::Keypad5),
        (WinitKey::Character("6"), KeyLocation::Numpad) => Some(Key::Keypad6),
        (WinitKey::Character("7"), KeyLocation::Numpad) => Some(Key::Keypad7),
        (WinitKey::Character("8"), KeyLocation::Numpad) => Some(Key::Keypad8),
        (WinitKey::Character("9"), KeyLocation::Numpad) => Some(Key::Keypad9),
        (WinitKey::Character("a"), _) => Some(Key::A),
        (WinitKey::Character("b"), _) => Some(Key::B),
        (WinitKey::Character("c"), _) => Some(Key::C),
        (WinitKey::Character("d"), _) => Some(Key::D),
        (WinitKey::Character("e"), _) => Some(Key::E),
        (WinitKey::Character("f"), _) => Some(Key::F),
        (WinitKey::Character("g"), _) => Some(Key::G),
        (WinitKey::Character("h"), _) => Some(Key::H),
        (WinitKey::Character("i"), _) => Some(Key::I),
        (WinitKey::Character("j"), _) => Some(Key::J),
        (WinitKey::Character("k"), _) => Some(Key::K),
        (WinitKey::Character("l"), _) => Some(Key::L),
        (WinitKey::Character("m"), _) => Some(Key::M),
        (WinitKey::Character("n"), _) => Some(Key::N),
        (WinitKey::Character("o"), _) => Some(Key::O),
        (WinitKey::Character("p"), _) => Some(Key::P),
        (WinitKey::Character("q"), _) => Some(Key::Q),
        (WinitKey::Character("r"), _) => Some(Key::R),
        (WinitKey::Character("s"), _) => Some(Key::S),
        (WinitKey::Character("t"), _) => Some(Key::T),
        (WinitKey::Character("u"), _) => Some(Key::U),
        (WinitKey::Character("v"), _) => Some(Key::V),
        (WinitKey::Character("w"), _) => Some(Key::W),
        (WinitKey::Character("x"), _) => Some(Key::X),
        (WinitKey::Character("y"), _) => Some(Key::Y),
        (WinitKey::Character("z"), _) => Some(Key::Z),
        (WinitKey::Character("'"), _) => Some(Key::Apostrophe),
        (WinitKey::Character(","), KeyLocation::Standard) => Some(Key::Comma),
        (WinitKey::Character("-"), KeyLocation::Standard) => Some(Key::Minus),
        (WinitKey::Character("-"), KeyLocation::Numpad) => Some(Key::KeypadSubtract),
        (WinitKey::Character("."), KeyLocation::Standard) => Some(Key::Period),
        (WinitKey::Character("."), KeyLocation::Numpad) => Some(Key::KeypadDecimal),
        (WinitKey::Character("/"), KeyLocation::Standard) => Some(Key::Slash),
        (WinitKey::Character("/"), KeyLocation::Numpad) => Some(Key::KeypadDivide),
        (WinitKey::Character(";"), _) => Some(Key::Semicolon),
        (WinitKey::Character("="), KeyLocation::Standard) => Some(Key::Equal),
        (WinitKey::Character("="), KeyLocation::Numpad) => Some(Key::KeypadEqual),
        (WinitKey::Character("["), _) => Some(Key::LeftBracket),
        (WinitKey::Character("\\"), _) => Some(Key::Backslash),
        (WinitKey::Character("]"), _) => Some(Key::RightBracket),
        (WinitKey::Character("`"), _) => Some(Key::GraveAccent),
        (WinitKey::Character("*"), KeyLocation::Numpad) => Some(Key::KeypadMultiply),
        (WinitKey::Character("+"), KeyLocation::Numpad) => Some(Key::KeypadAdd),
        _ => None,
    }
}

fn handle_key_modifier(io: &mut Io, key: &WinitKey, down: bool) {
    match key {
        WinitKey::Named(NamedKey::Shift) => io.add_key_event(imgui::Key::ModShift, down),
        WinitKey::Named(NamedKey::Control) => io.add_key_event(imgui::Key::ModCtrl, down),
        WinitKey::Named(NamedKey::Alt) => io.add_key_event(imgui::Key::ModAlt, down),
        WinitKey::Named(NamedKey::Super) => io.add_key_event(imgui::Key::ModSuper, down),
        _ => {}
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
        imgui.set_platform_name(Some(format!(
            "imgui-winit-support {}",
            env!("CARGO_PKG_VERSION")
        )));
        WinitPlatform {
            hidpi_mode: ActiveHiDpiMode::Default,
            hidpi_factor: 1.0,
            cursor_cache: None,
        }
    }
    /// Attaches the platform instance to a winit window.
    ///
    /// This function configures imgui-rs in the following ways:
    ///
    /// * framebuffer scale (= DPI factor) is set
    /// * display size is set
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
            // Event::DeviceEvent {
            //     event:
            //         DeviceEvent::Key(RawKeyEvent {
            //             physical_key,
            //             state: ElementState::Released,
            //         }),
            //     ..
            // } => {
            //     if let Some(key) = to_imgui_key(key) {
            //         io.add_key_event(key, false);
            //     }
            // }
            _ => (),
        }
    }
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
            WindowEvent::ModifiersChanged(modifiers) => {
                let state = modifiers.state();
                // We need to track modifiers separately because some system like macOS, will
                // not reliably send modifier states during certain events like ScreenCapture.
                // Gotta let the people show off their pretty imgui widgets!
                io.add_key_event(Key::ModShift, state.shift_key());
                io.add_key_event(Key::ModCtrl, state.control_key());
                io.add_key_event(Key::ModAlt, state.alt_key());
                io.add_key_event(Key::ModSuper, state.super_key());
            }
            WindowEvent::KeyboardInput { ref event, .. } => {
                if let Some(txt) = &event.text {
                    for ch in txt.chars() {
                        if ch != '\u{7f}' {
                            io.add_input_character(ch)
                        }
                    }
                }

                let key = event.key_without_modifiers();
                let pressed = event.state == ElementState::Pressed;

                // We map both left and right ctrl to `ModCtrl`, etc.
                // imgui is told both "left control is pressed" and
                // "consider the control key is pressed". Allows
                // applications to use either general "ctrl" or a
                // specific key. Same applies to other modifiers.
                // https://github.com/ocornut/imgui/issues/5047
                handle_key_modifier(io, &key, pressed);

                // Add main key event
                if let Some(key) = to_imgui_key(key, event.location) {
                    io.add_key_event(key, pressed);
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let position = position.to_logical(window.scale_factor());
                let position = self.scale_pos_from_winit(window, position);
                io.add_mouse_pos_event([position.x as f32, position.y as f32]);
            }
            WindowEvent::MouseWheel {
                delta,
                phase: TouchPhase::Moved,
                ..
            } => {
                let (h, v) = match delta {
                    MouseScrollDelta::LineDelta(h, v) => (h, v),
                    MouseScrollDelta::PixelDelta(pos) => {
                        let pos = pos.to_logical::<f64>(self.hidpi_factor);
                        let h = match pos.x.partial_cmp(&0.0) {
                            Some(Ordering::Greater) => 1.0,
                            Some(Ordering::Less) => -1.0,
                            _ => 0.0,
                        };
                        let v = match pos.y.partial_cmp(&0.0) {
                            Some(Ordering::Greater) => 1.0,
                            Some(Ordering::Less) => -1.0,
                            _ => 0.0,
                        };
                        (h, v)
                    }
                };
                io.add_mouse_wheel_event([h, v]);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if let Some(mb) = to_imgui_mouse_button(button) {
                    let pressed = state == ElementState::Pressed;
                    io.add_mouse_button_event(mb, pressed);
                }
            }
            WindowEvent::Focused(newly_focused) => {
                if !newly_focused {
                    // Set focus-lost to avoid stuck keys (like 'alt'
                    // when alt-tabbing)
                    io.app_focus_lost = true;
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
