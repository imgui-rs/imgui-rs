//! This crate provides a SDL 2 based backend platform for imgui-rs.
//!
//! A backend platform handles window/input device events and manages their
//! state.
//!
//! # Using the library
//!
//! There are three things you need to do to use this library correctly:
//!
//! 1. Initialize a `SdlPlatform` instance
//! 2. Pass events to the platform (every frame)
//! 3. Call frame preparation callback (every frame)
//!
//! For a complete example, take a look at the imgui-rs' GitHub repository.

use std::time::Instant;

use imgui::{BackendFlags, ConfigFlags, Context, Io, Key, MouseCursor};
use sdl2::{
    event::Event,
    keyboard::{Mod, Scancode},
    mouse::{Cursor, MouseState, SystemCursor},
    video::Window,
    EventPump,
};

/// Handle changes in the key modifier states.
fn handle_key_modifier(io: &mut Io, keymod: &Mod) {
    io.key_shift = keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD);
    io.key_ctrl = keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD);
    io.key_alt = keymod.intersects(Mod::LALTMOD | Mod::RALTMOD);
    io.key_super = keymod.intersects(Mod::LGUIMOD | Mod::RGUIMOD);
}

/// Map an imgui::MouseCursor to an equivalent sdl2::mouse::SystemCursor.
fn to_sdl_cursor(cursor: MouseCursor) -> SystemCursor {
    match cursor {
        MouseCursor::Arrow => SystemCursor::Arrow,
        MouseCursor::TextInput => SystemCursor::IBeam,
        MouseCursor::ResizeAll => SystemCursor::SizeAll,
        MouseCursor::ResizeNS => SystemCursor::SizeNS,
        MouseCursor::ResizeEW => SystemCursor::SizeWE,
        MouseCursor::ResizeNESW => SystemCursor::SizeNESW,
        MouseCursor::ResizeNWSE => SystemCursor::SizeNWSE,
        MouseCursor::Hand => SystemCursor::Hand,
        MouseCursor::NotAllowed => SystemCursor::No,
    }
}

/// Returns `true` if the provided event is associated with the provided window.
///
/// # Example
/// ```rust,no_run
/// # let mut event_pump: sdl2::EventPump = unimplemented!();
/// # let window: sdl2::video::Window = unimplemented!();
/// # let mut imgui = imgui::Context::create();
/// # let mut platform = SdlPlatform::init(&mut imgui);
/// use imgui_sdl2_support::{SdlPlatform, filter_event};
/// // Assuming there are multiple windows, we only want to provide the events
/// // of the window where we are rendering to imgui-rs
/// for event in event_pump.poll_iter().filter(|event| filter_event(&window, event)) {
///     platform.handle_event(&mut imgui, &event);
/// }
/// ```
pub fn filter_event(window: &Window, event: &Event) -> bool {
    Some(window.id()) == event.get_window_id()
}

/// SDL 2 backend platform state.
///
/// A backend platform handles window/input device events and manages their
/// state.
///
/// There are three things you need to do to use this library correctly:
///
/// 1. Initialize a `SdlPlatform` instance
/// 2. Pass events to the platform (every frame)
/// 3. Call frame preparation callback (every frame)
pub struct SdlPlatform {
    cursor_instance: Option<Cursor>, /* to avoid dropping cursor instances */
    last_frame: Instant,
}

impl SdlPlatform {
    /// Initializes a SDL platform instance and configures imgui.
    ///
    /// This function configures imgui-rs in the following ways:
    ///
    /// * backend flags are updated
    /// * keys are configured
    /// * platform name is set
    pub fn init(imgui: &mut Context) -> SdlPlatform {
        let io = imgui.io_mut();

        io.backend_flags.insert(BackendFlags::HAS_MOUSE_CURSORS);
        io.backend_flags.insert(BackendFlags::HAS_SET_MOUSE_POS);

        io[Key::Tab] = Scancode::Tab as _;
        io[Key::LeftArrow] = Scancode::Left as _;
        io[Key::RightArrow] = Scancode::Right as _;
        io[Key::UpArrow] = Scancode::Up as _;
        io[Key::DownArrow] = Scancode::Down as _;
        io[Key::PageUp] = Scancode::PageUp as _;
        io[Key::PageDown] = Scancode::PageDown as _;
        io[Key::Home] = Scancode::Home as _;
        io[Key::End] = Scancode::End as _;
        io[Key::Insert] = Scancode::Insert as _;
        io[Key::Delete] = Scancode::Delete as _;
        io[Key::Backspace] = Scancode::Backspace as _;
        io[Key::Space] = Scancode::Space as _;
        io[Key::Enter] = Scancode::Return as _;
        io[Key::Escape] = Scancode::Escape as _;
        io[Key::KeypadEnter] = Scancode::KpEnter as _;
        io[Key::A] = Scancode::A as _;
        io[Key::C] = Scancode::C as _;
        io[Key::V] = Scancode::V as _;
        io[Key::X] = Scancode::X as _;
        io[Key::Y] = Scancode::Y as _;
        io[Key::Z] = Scancode::Z as _;

        imgui.set_platform_name(Some(format!(
            "imgui-sdl2-support {}",
            env!("CARGO_PKG_VERSION")
        )));

        SdlPlatform {
            cursor_instance: None,
            last_frame: Instant::now(),
        }
    }

    /// Handles a SDL event.
    ///
    /// This function performs the following actions (depends on the event):
    ///
    /// * keyboard state is updated
    /// * mouse state is updated
    pub fn handle_event(&mut self, context: &mut Context, event: &Event) -> bool {
        let io = context.io_mut();

        match *event {
            Event::MouseWheel { x, y, .. } => {
                io.add_mouse_wheel_event([x as f32, y as f32]);
                true
            }

            Event::MouseButtonDown { mouse_btn, .. } => {
                self.handle_mouse_button(io, &mouse_btn, true);
                true
            }

            Event::MouseButtonUp { mouse_btn, .. } => {
                self.handle_mouse_button(io, &mouse_btn, false);
                true
            }

            Event::TextInput { ref text, .. } => {
                text.chars().for_each(|c| io.add_input_character(c));
                true
            }

            Event::KeyDown {
                scancode: Some(key),
                keymod,
                ..
            } => {
                io.keys_down[key as usize] = true;
                handle_key_modifier(io, &keymod);
                true
            }

            Event::KeyUp {
                scancode: Some(key),
                keymod,
                ..
            } => {
                io.keys_down[key as usize] = false;
                handle_key_modifier(io, &keymod);
                true
            }

            _ => false,
        }
    }

    /// Frame preparation callback.
    ///
    /// Call this before calling the imgui-rs context `frame` function.
    /// This function performs the following actions:
    ///
    /// * display size and the framebuffer scale is set
    /// * mouse cursor is repositioned (if requested by imgui-rs)
    /// * current mouse cursor position is passed to imgui-rs
    /// * changes mouse cursor icon (if requested by imgui-rs)
    pub fn prepare_frame(
        &mut self,
        context: &mut Context,
        window: &Window,
        event_pump: &EventPump,
    ) {
        let mouse_cursor = context.mouse_cursor();
        let io = context.io_mut();

        // Update delta time
        let now = Instant::now();
        io.update_delta_time(now.duration_since(self.last_frame));
        self.last_frame = now;

        let mouse_state = MouseState::new(event_pump);
        let window_size = window.size();
        let window_drawable_size = window.drawable_size();

        // Set display size and scale here, since SDL 2 doesn't have
        // any easy way to get the scale factor, and changes in said
        // scale factor
        io.display_size = [window_size.0 as f32, window_size.1 as f32];
        io.display_framebuffer_scale = [
            (window_drawable_size.0 as f32) / (window_size.0 as f32),
            (window_drawable_size.1 as f32) / (window_size.1 as f32),
        ];

        // Set mouse position if requested by imgui-rs
        if io.want_set_mouse_pos {
            let mouse_util = window.subsystem().sdl().mouse();
            mouse_util.warp_mouse_in_window(window, io.mouse_pos[0] as i32, io.mouse_pos[1] as i32);
        }

        // Update mouse cursor position
        io.mouse_pos = [mouse_state.x() as f32, mouse_state.y() as f32];

        // Update mouse cursor icon if requested
        if !io
            .config_flags
            .contains(ConfigFlags::NO_MOUSE_CURSOR_CHANGE)
        {
            let mouse_util = window.subsystem().sdl().mouse();

            match mouse_cursor {
                Some(mouse_cursor) if !io.mouse_draw_cursor => {
                    let cursor = Cursor::from_system(to_sdl_cursor(mouse_cursor)).unwrap();
                    cursor.set();

                    mouse_util.show_cursor(true);
                    self.cursor_instance = Some(cursor);
                }

                _ => {
                    mouse_util.show_cursor(false);
                    self.cursor_instance = None;
                }
            }
        }
    }
}

impl SdlPlatform {
    fn handle_mouse_button(&mut self, io: &mut Io, button: &sdl2::mouse::MouseButton, pressed: bool) {
        match button {
            sdl2::mouse::MouseButton::Left => io.add_mouse_button_event(imgui::MouseButton::Left, pressed),
            sdl2::mouse::MouseButton::Right => io.add_mouse_button_event(imgui::MouseButton::Right, pressed),
            sdl2::mouse::MouseButton::Middle => io.add_mouse_button_event(imgui::MouseButton::Middle, pressed),
            sdl2::mouse::MouseButton::X1 => io.add_mouse_button_event(imgui::MouseButton::Extra1, pressed),
            sdl2::mouse::MouseButton::X2 => io.add_mouse_button_event(imgui::MouseButton::Extra2, pressed),
            _ => {}
        }
    }
}
