use glium::{DisplayBuild, Surface};
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin;
use glium::glutin::{ElementState, Event, MouseButton, MouseScrollDelta, VirtualKeyCode};
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

        // TODO: How can we get the virtual key -> scancode mapping from glium?
        imgui.set_imgui_key(ImGuiKey::Tab, 15);
        imgui.set_imgui_key(ImGuiKey::LeftArrow, 75);
        imgui.set_imgui_key(ImGuiKey::RightArrow, 77);
        imgui.set_imgui_key(ImGuiKey::UpArrow, 72);
        imgui.set_imgui_key(ImGuiKey::DownArrow, 80);
        imgui.set_imgui_key(ImGuiKey::PageUp, 73);
        imgui.set_imgui_key(ImGuiKey::PageDown, 81);
        imgui.set_imgui_key(ImGuiKey::Home, 71);
        imgui.set_imgui_key(ImGuiKey::End, 79);
        imgui.set_imgui_key(ImGuiKey::Delete, 83);
        imgui.set_imgui_key(ImGuiKey::Backspace, 14);
        imgui.set_imgui_key(ImGuiKey::Enter, 28);
        imgui.set_imgui_key(ImGuiKey::Escape, 1);
        imgui.set_imgui_key(ImGuiKey::A, 30);
        imgui.set_imgui_key(ImGuiKey::C, 46);
        imgui.set_imgui_key(ImGuiKey::V, 47);
        imgui.set_imgui_key(ImGuiKey::X, 45);
        imgui.set_imgui_key(ImGuiKey::Y, 21);
        imgui.set_imgui_key(ImGuiKey::Z, 44);

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

    pub fn update_mouse(&mut self) {
        self.imgui.set_mouse_pos(self.mouse_pos.0 as f32, self.mouse_pos.1 as f32);
        self.imgui.set_mouse_down(&[self.mouse_pressed.0, self.mouse_pressed.1, self.mouse_pressed.2, false, false]);
        self.imgui.set_mouse_wheel(self.mouse_wheel);
    }

    pub fn render<'ui, 'a: 'ui , F: FnMut(&Ui<'ui>)>(
            &'a mut self, clear_color: (f32, f32, f32, f32), mut f: F) {
        let now = SteadyTime::now();
        let delta = now - self.last_frame;
        let delta_f = delta.num_nanoseconds().unwrap() as f32 / 1_000_000_000.0;
        self.last_frame = now;

        self.update_mouse();
        self.mouse_wheel = 0.0;

        let mut target = self.display.draw();
        target.clear_color(clear_color.0, clear_color.1,
                           clear_color.2, clear_color.3);

        let (width, height) = target.get_dimensions();
        let ui = self.imgui.frame(width, height, delta_f);
        f(&ui);
        self.renderer.render(&mut target, ui).unwrap();

        target.finish().unwrap();
    }

    pub fn update_events(&mut self) -> bool {
        for event in self.display.poll_events() {
            match event {
                Event::Closed => return false,
                // TODO: Why does glutin not give us scancodes for left/right control?
                Event::KeyboardInput(state, _, Some(code))
                    if code == VirtualKeyCode::RControl || code == VirtualKeyCode::LControl
                        => self.imgui.set_key_ctrl(state == ElementState::Pressed),
                Event::KeyboardInput(state, scancode, code) => {
                    println!("KeyCode {:?} scancode {} pressed? {}", code, scancode,
                             state == ElementState::Pressed);
                    self.imgui.set_key(scancode, state == ElementState::Pressed);
                    // TODO: This hack to do Ctrl should not be needed!
                    if scancode == 29 {
                        println!("Bad! Hacking in ctrl key press");
                        self.imgui.set_key_ctrl(state == ElementState::Pressed);
                    }
                },
                Event::MouseMoved(pos) => self.mouse_pos = pos,
                Event::MouseInput(state, MouseButton::Left) =>
                    self.mouse_pressed.0 = state == ElementState::Pressed,
                Event::MouseInput(state, MouseButton::Right) =>
                    self.mouse_pressed.1 = state == ElementState::Pressed,
                Event::MouseInput(state, MouseButton::Middle) =>
                    self.mouse_pressed.2 = state == ElementState::Pressed,
                Event::MouseWheel(MouseScrollDelta::LineDelta(_, y)) => self.mouse_wheel = y,
                Event::ReceivedCharacter(c) => {
                    println!("Got character {}", c);
                    self.imgui.add_input_character(c);
                },
                _ => ()
            }
        }
        true
    }
}
