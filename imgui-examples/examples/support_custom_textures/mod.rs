use std::cell::Ref;
use std::rc::Rc;
use std::time::Instant;

use glium::backend::glutin::DisplayCreationError;
use glium::backend::{Context, Facade};
use glium::{glutin, Display, Surface, SwapBuffersError};

use imgui::{ImGui, ImGuiMouseCursor, ImString, Ui};
use imgui_glium_renderer::{Renderer, RendererError};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (i32, i32),
    pressed: (bool, bool, bool),
    wheel: f32,
}

pub struct AppContext {
    renderer: Renderer,
    display: Display,
    events_loop: glutin::EventsLoop,
    imgui: ImGui,
    quit: bool,
    mouse_state: MouseState,
    last_frame: Instant,
    clear_color: [f32; 4],
}

#[derive(Debug)]
pub enum ContextError {
    Glutin(DisplayCreationError),
    Render(RendererError),
    SwapBuffers(SwapBuffersError),
    Message(String),
}

impl From<DisplayCreationError> for ContextError {
    fn from(e: DisplayCreationError) -> Self { ContextError::Glutin(e) }
}

impl From<RendererError> for ContextError {
    fn from(e: RendererError) -> Self { ContextError::Render(e) }
}

impl From<SwapBuffersError> for ContextError {
    fn from(e: SwapBuffersError) -> Self { ContextError::SwapBuffers(e) }
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub clear_color: [f32; 4],
    pub ini_filename: Option<ImString>,
    pub log_filename: Option<ImString>,
    pub window_width: u32,
    pub window_height: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            clear_color: [1.0, 1.0, 1.0, 1.0],
            ini_filename: None,
            log_filename: None,
            window_width: 1024,
            window_height: 768,
        }
    }
}

impl Facade for AppContext {
    fn get_context(&self) -> &Rc<Context> { self.display.get_context() }
}

impl AppContext {
    pub fn init(title: String, config: AppConfig) -> Result<Self, ContextError> {
        let events_loop = glutin::EventsLoop::new();
        let context = glutin::ContextBuilder::new().with_vsync(true);
        let window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(config.window_width, config.window_height);
        let display = Display::new(window, context, &events_loop)?;
        let mut imgui = ImGui::init();
        imgui.set_ini_filename(config.ini_filename);
        imgui.set_log_filename(config.log_filename);

        let renderer = Renderer::init(&mut imgui, &display)?;

        configure_keys(&mut imgui);

        Ok(AppContext {
            renderer,
            display,
            events_loop,
            imgui,
            quit: false,
            mouse_state: Default::default(),
            last_frame: Instant::now(),
            clear_color: config.clear_color,
        })
    }

    pub fn run<F: FnMut(&Ui) -> bool>(&mut self, mut run_ui: F) -> Result<(), ContextError> {
        loop {
            self.poll_events();
            let now = Instant::now();
            let delta = now - self.last_frame;
            let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
            update_mouse(&mut self.imgui, &mut self.mouse_state);
            let gl_window = self.display.gl_window();

            update_os_cursor(&self.imgui, &gl_window);

            let size_pixels = gl_window
                .get_inner_size()
                .ok_or_else(|| ContextError::Message("Window no longer exists!".to_owned()))?;
            let hdipi = gl_window.hidpi_factor();
            let size_points = (
                (size_pixels.0 as f32 / hdipi) as u32,
                (size_pixels.1 as f32 / hdipi) as u32,
            );

            let ui = self.imgui.frame(size_points, size_pixels, delta_s);
            if !run_ui(&ui) {
                break;
            }
            let mut target = self.display.draw();
            target.clear_color(
                self.clear_color[0],
                self.clear_color[1],
                self.clear_color[2],
                self.clear_color[3],
            );
            self.renderer.render(&mut target, ui)?;
            target.finish()?;

            if self.quit {
                break;
            }
        }
        Ok(())
    }

    fn poll_events(&mut self) {
        let quit = &mut self.quit;
        let imgui = &mut self.imgui;
        let events_loop = &mut self.events_loop;
        let mouse_state = &mut self.mouse_state;
        events_loop.poll_events(|event| {
            use glium::glutin::ElementState::Pressed;
            use glium::glutin::WindowEvent::*;
            use glium::glutin::{Event, MouseButton, MouseScrollDelta, TouchPhase};

            if let Event::WindowEvent { event, .. } = event {
                match event {
                    Closed => *quit = true,
                    KeyboardInput { input, .. } => {
                        use glium::glutin::VirtualKeyCode as Key;

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
                            Some(Key::LControl) | Some(Key::RControl) => {
                                imgui.set_key_ctrl(pressed)
                            }
                            Some(Key::LShift) | Some(Key::RShift) => imgui.set_key_shift(pressed),
                            Some(Key::LAlt) | Some(Key::RAlt) => imgui.set_key_alt(pressed),
                            Some(Key::LWin) | Some(Key::RWin) => imgui.set_key_super(pressed),
                            _ => {}
                        }
                    }
                    CursorMoved {
                        position: (x, y), ..
                    } => mouse_state.pos = (x as i32, y as i32),
                    MouseInput { state, button, .. } => match button {
                        MouseButton::Left => mouse_state.pressed.0 = state == Pressed,
                        MouseButton::Right => mouse_state.pressed.1 = state == Pressed,
                        MouseButton::Middle => mouse_state.pressed.2 = state == Pressed,
                        _ => {}
                    },
                    MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_, y),
                        phase: TouchPhase::Moved,
                        ..
                    }
                    | MouseWheel {
                        delta: MouseScrollDelta::PixelDelta(_, y),
                        phase: TouchPhase::Moved,
                        ..
                    } => mouse_state.wheel = y,
                    ReceivedCharacter(c) => imgui.add_input_character(c),
                    _ => (),
                }
            }
        });
    }

    pub fn imgui(&self) -> &ImGui { &self.imgui }

    pub fn imgui_mut(&mut self) -> &mut ImGui { &mut self.imgui }
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
    imgui.set_mouse_down(&[
        mouse_state.pressed.0,
        mouse_state.pressed.1,
        mouse_state.pressed.2,
        false,
        false,
    ]);
    imgui.set_mouse_wheel(mouse_state.wheel / scale.1);
    mouse_state.wheel = 0.0;
}

fn update_os_cursor(imgui: &ImGui, gl_window: &Ref<glutin::GlWindow>) {
    let mouse_cursor = imgui.mouse_cursor();
    if imgui.mouse_draw_cursor() || mouse_cursor == ImGuiMouseCursor::None {
        // Hide OS cursor
        gl_window
            .set_cursor_state(glutin::CursorState::Hide)
            .unwrap();
    } else {
        // Set OS cursor
        gl_window
            .set_cursor_state(glutin::CursorState::Normal)
            .unwrap();
        gl_window.set_cursor(match mouse_cursor {
            ImGuiMouseCursor::None => unreachable!("mouse_cursor was None!"),
            ImGuiMouseCursor::Arrow => glutin::MouseCursor::Arrow,
            ImGuiMouseCursor::TextInput => glutin::MouseCursor::Text,
            ImGuiMouseCursor::Move => glutin::MouseCursor::Move,
            ImGuiMouseCursor::ResizeNS => glutin::MouseCursor::NsResize,
            ImGuiMouseCursor::ResizeEW => glutin::MouseCursor::EwResize,
            ImGuiMouseCursor::ResizeNESW => glutin::MouseCursor::NeswResize,
            ImGuiMouseCursor::ResizeNWSE => glutin::MouseCursor::NwseResize,
        });
    }
}
