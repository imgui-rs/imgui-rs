use glium::{DisplayBuild, Surface};
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin;
use glium::glutin::{ElementState, Event, MouseButton, MouseScrollDelta, VirtualKeyCode, TouchPhase};
use imgui::{ImGui, Ui, ImGuiKey};
use imgui::glium_renderer::Renderer;
use time::SteadyTime;

pub struct Support {
    display: GlutinFacade,
    imgui: ImGui,
    renderer: Renderer,
    last_frame: SteadyTime,
    mouse_pos: (i32, i32),
    mouse_pressed: (bool, bool, bool),
    mouse_wheel: f32,
}

impl Support {
    pub fn init() -> Support {
        let display = glutin::WindowBuilder::new()
            .build_glium()
            .unwrap();

        let mut imgui = ImGui::init();
        let renderer = Renderer::init(&mut imgui, &display).unwrap();

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

        Support {
            display: display,
            imgui: imgui,
            renderer: renderer,
            last_frame: SteadyTime::now(),
            mouse_pos: (0, 0),
            mouse_pressed: (false, false, false),
            mouse_wheel: 0.0
        }
    }

    pub fn update_mouse(&mut self, hidpi_factor: f32) {
        self.imgui.set_mouse_pos(self.mouse_pos.0 as f32 / hidpi_factor, self.mouse_pos.1 as f32 / hidpi_factor);
        self.imgui.set_mouse_down(&[self.mouse_pressed.0, self.mouse_pressed.1, self.mouse_pressed.2, false, false]);
        self.imgui.set_mouse_wheel(self.mouse_wheel / hidpi_factor);
    }

    pub fn render<'ui, 'a: 'ui , F: FnMut(&Ui<'ui>)>(
            &'a mut self, clear_color: (f32, f32, f32, f32), mut f: F) {
        let now = SteadyTime::now();
        let delta = now - self.last_frame;
        let delta_f = delta.num_nanoseconds().unwrap() as f32 / 1_000_000_000.0;
        self.last_frame = now;

        let hidpi_factor =
            self.display.get_window()
                .expect("Failed to get window")
                .hidpi_factor();

        self.update_mouse(hidpi_factor);
        self.mouse_wheel = 0.0;

        let mut target = self.display.draw();
        target.clear_color(clear_color.0, clear_color.1,
                           clear_color.2, clear_color.3);

        let (width, height) = target.get_dimensions();
        let ui = self.imgui.frame(width, height, hidpi_factor, delta_f);
        f(&ui);

        self.renderer.render(&mut target, ui).unwrap();

        target.finish().unwrap();
    }

    pub fn update_events(&mut self) -> bool {
        for event in self.display.poll_events() {
            match event {
                Event::Closed => return false,
                Event::KeyboardInput(state, _, code) => {
                    let pressed = state == ElementState::Pressed;
                    match code {
                        Some(VirtualKeyCode::Tab) => self.imgui.set_key(0, pressed),
                        Some(VirtualKeyCode::Left) => self.imgui.set_key(1, pressed),
                        Some(VirtualKeyCode::Right) => self.imgui.set_key(2, pressed),
                        Some(VirtualKeyCode::Up) => self.imgui.set_key(3, pressed),
                        Some(VirtualKeyCode::Down) => self.imgui.set_key(4, pressed),
                        Some(VirtualKeyCode::PageUp) => self.imgui.set_key(5, pressed),
                        Some(VirtualKeyCode::PageDown) => self.imgui.set_key(6, pressed),
                        Some(VirtualKeyCode::Home) => self.imgui.set_key(7, pressed),
                        Some(VirtualKeyCode::End) => self.imgui.set_key(8, pressed),
                        Some(VirtualKeyCode::Delete) => self.imgui.set_key(9, pressed),
                        Some(VirtualKeyCode::Back) => self.imgui.set_key(10, pressed),
                        Some(VirtualKeyCode::Return) => self.imgui.set_key(11, pressed),
                        Some(VirtualKeyCode::Escape) => self.imgui.set_key(12, pressed),
                        Some(VirtualKeyCode::A) => self.imgui.set_key(13, pressed),
                        Some(VirtualKeyCode::C) => self.imgui.set_key(14, pressed),
                        Some(VirtualKeyCode::V) => self.imgui.set_key(15, pressed),
                        Some(VirtualKeyCode::X) => self.imgui.set_key(16, pressed),
                        Some(VirtualKeyCode::Y) => self.imgui.set_key(17, pressed),
                        Some(VirtualKeyCode::Z) => self.imgui.set_key(18, pressed),
                        Some(VirtualKeyCode::LControl) | Some(VirtualKeyCode::RControl) =>
                            self.imgui.set_key_ctrl(pressed),
                        Some(VirtualKeyCode::LShift) | Some(VirtualKeyCode::RShift) =>
                            self.imgui.set_key_shift(pressed),
                        Some(VirtualKeyCode::LAlt) | Some(VirtualKeyCode::RAlt) =>
                            self.imgui.set_key_alt(pressed),
                        Some(VirtualKeyCode::LWin) | Some(VirtualKeyCode::RWin) =>
                            self.imgui.set_key_super(pressed),
                        _ => {},
                    }
                },
                Event::MouseMoved(x, y) => self.mouse_pos = (x, y),
                Event::MouseInput(state, MouseButton::Left) =>
                    self.mouse_pressed.0 = state == ElementState::Pressed,
                Event::MouseInput(state, MouseButton::Right) =>
                    self.mouse_pressed.1 = state == ElementState::Pressed,
                Event::MouseInput(state, MouseButton::Middle) =>
                    self.mouse_pressed.2 = state == ElementState::Pressed,
                Event::MouseWheel(MouseScrollDelta::LineDelta(_, y), TouchPhase::Moved) =>
                    self.mouse_wheel = y,
                Event::MouseWheel(MouseScrollDelta::PixelDelta(_, y), TouchPhase::Moved) =>
                    self.mouse_wheel = y,
                Event::ReceivedCharacter(c) => self.imgui.add_input_character(c),
                _ => ()
            }
        }
        true
    }
}
