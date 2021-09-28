use std::{cell::Cell, cmp::Ordering};

use imgui::{BackendFlags, ConfigFlags, Context, Io, Key, MouseCursor, Ui};
use sdl2::{
    event::Event,
    keyboard::{Mod, Scancode},
    mouse::{Cursor, MouseButton, MouseState, SystemCursor},
    video::Window,
    EventPump,
};

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

pub struct SdlPlatform {
    previous_cursor: Option<Cursor>,
    mouse_buttons: [Button; 5],
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
        io[Key::KeyPadEnter] = Scancode::KpEnter as _;
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
            previous_cursor: None,
            mouse_buttons: [Button::INIT; 5],
        }
    }

    /// Handles a SDL event.
    ///
    /// This function performs the following actions (depends on the event):
    ///
    /// * keyboard state is updated
    /// * mouse state is updated
    pub fn handle_event(&mut self, io: &mut Io, window: &Window, event: &Event) {
        /* ignore all events not affecting the provided window */
        if let Some(id) = event.get_window_id() {
            if window.id() != id {
                return;
            }
        }

        match *event {
            Event::MouseWheel { x, y, .. } => {
                match x.partial_cmp(&0) {
                    Some(Ordering::Greater) => io.mouse_wheel_h += 1.0,
                    Some(Ordering::Less) => io.mouse_wheel_h -= 1.0,
                    _ => {}
                }

                match y.partial_cmp(&0) {
                    Some(Ordering::Greater) => io.mouse_wheel += 1.0,
                    Some(Ordering::Less) => io.mouse_wheel -= 1.0,
                    _ => {}
                }
            }

            Event::MouseButtonDown { mouse_btn, .. } => match mouse_btn {
                MouseButton::Left => self.mouse_buttons[0].set(true),
                MouseButton::Right => self.mouse_buttons[1].set(true),
                MouseButton::Middle => self.mouse_buttons[2].set(true),
                MouseButton::X1 => self.mouse_buttons[3].set(true),
                MouseButton::X2 => self.mouse_buttons[4].set(true),

                _ => {}
            },

            Event::MouseButtonUp { mouse_btn, .. } => match mouse_btn {
                MouseButton::Left => self.mouse_buttons[0].set(false),
                MouseButton::Right => self.mouse_buttons[1].set(false),
                MouseButton::Middle => self.mouse_buttons[2].set(false),
                MouseButton::X1 => self.mouse_buttons[3].set(false),
                MouseButton::X2 => self.mouse_buttons[4].set(false),

                _ => {}
            },

            Event::TextInput { ref text, .. } => {
                text.chars().for_each(|c| io.add_input_character(c));
            }

            Event::KeyDown {
                scancode: Some(key),
                keymod,
                ..
            } => {
                io.keys_down[key as usize] = true;

                io.key_shift = keymod.contains(Mod::LSHIFTMOD | Mod::RSHIFTMOD);
                io.key_ctrl = keymod.contains(Mod::LCTRLMOD | Mod::RCTRLMOD);
                io.key_alt = keymod.contains(Mod::LALTMOD | Mod::RALTMOD);
                io.key_super = keymod.contains(Mod::LGUIMOD | Mod::RGUIMOD);
            }

            Event::KeyUp {
                scancode: Some(key),
                keymod,
                ..
            } => {
                io.keys_down[key as usize] = false;

                io.key_shift = keymod.contains(Mod::LSHIFTMOD | Mod::RSHIFTMOD);
                io.key_ctrl = keymod.contains(Mod::LCTRLMOD | Mod::RCTRLMOD);
                io.key_alt = keymod.contains(Mod::LALTMOD | Mod::RALTMOD);
                io.key_super = keymod.contains(Mod::LGUIMOD | Mod::RGUIMOD);
            }

            _ => {}
        }
    }

    pub fn prepare_frame(&self, io: &mut Io, window: &Window, event_pump: &EventPump) {
        let window_size = window.size();
        let drawable_size = window.drawable_size();

        io.display_size = [window_size.0 as f32, window_size.1 as f32];
        io.display_framebuffer_scale = [
            (drawable_size.0 as f32) / (window_size.0 as f32),
            (drawable_size.1 as f32) / (window_size.1 as f32),
        ];

        for (io_down, button) in io.mouse_down.iter_mut().zip(&self.mouse_buttons) {
            *io_down = button.get();
        }

        let mouse_state = MouseState::new(event_pump);
        io.mouse_pos = [mouse_state.x() as f32, mouse_state.y() as f32];
    }

    pub fn prepare_render(&mut self, ui: &Ui, window: &Window) {
        let io = ui.io();

        if !io
            .config_flags
            .contains(ConfigFlags::NO_MOUSE_CURSOR_CHANGE)
        {
            let mouse_util = window.subsystem().sdl().mouse();

            match ui.mouse_cursor() {
                Some(mouse_cursor) if !io.mouse_draw_cursor => {
                    let cursor = Cursor::from_system(to_sdl_cursor(mouse_cursor)).unwrap();
                    cursor.set();

                    mouse_util.show_cursor(true);
                    self.previous_cursor = Some(cursor);
                }

                _ => {
                    mouse_util.show_cursor(false);
                    self.previous_cursor = None;
                }
            }
        }
    }
}
