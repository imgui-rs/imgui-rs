use glium::{DisplayBuild, Surface};
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin;
use glium::glutin::{ElementState, Event, MouseButton, VirtualKeyCode};
use imgui::{ImGui, Frame};
use imgui::glium_renderer::Renderer;
use time::SteadyTime;

pub struct Support {
    display: GlutinFacade,
    imgui: ImGui,
    renderer: Renderer,
    last_frame: SteadyTime,
    mouse_pos: (i32, i32),
    mouse_pressed: (bool, bool, bool)
}

impl Support {
    pub fn init() -> Support {
        let display = glutin::WindowBuilder::new()
            .build_glium()
            .unwrap();

        let mut imgui = ImGui::init();
        let renderer = Renderer::init(&mut imgui, &display).unwrap();

        Support {
            display: display,
            imgui: imgui,
            renderer: renderer,
            last_frame: SteadyTime::now(),
            mouse_pos: (0, 0),
            mouse_pressed: (false, false, false)
        }
    }

    pub fn update_mouse(&mut self) {
        self.imgui.set_mouse_pos(self.mouse_pos.0 as f32, self.mouse_pos.1 as f32);
        self.imgui.set_mouse_down(&[self.mouse_pressed.0, self.mouse_pressed.1, self.mouse_pressed.2, false, false]);
    }

    pub fn render<'fr, 'a: 'fr , F: FnMut(&Frame<'fr>) -> bool>(
            &'a mut self, clear_color: (f32, f32, f32, f32), mut f: F) -> bool {
        let mut result;
        let now = SteadyTime::now();
        let delta = now - self.last_frame;
        let delta_f = delta.num_nanoseconds().unwrap() as f32 / 1_000_000_000.0;
        self.last_frame = now;

        self.update_mouse();

        let mut target = self.display.draw();
        target.clear_color(clear_color.0, clear_color.1,
                           clear_color.2, clear_color.3);

        let (width, height) = target.get_dimensions();
        let frame = self.imgui.frame(width, height, delta_f);
        result = f(&frame);
        self.renderer.render(&mut target, frame).unwrap();

        target.finish().unwrap();

        for event in self.display.poll_events() {
            match event {
                Event::Closed |
                    Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape))
                    => result = false,
                    Event::MouseMoved(pos) => self.mouse_pos = pos,
                    Event::MouseInput(state, MouseButton::Left) =>
                        self.mouse_pressed.0 = state == ElementState::Pressed,
                    Event::MouseInput(state, MouseButton::Right) =>
                        self.mouse_pressed.1 = state == ElementState::Pressed,
                    Event::MouseInput(state, MouseButton::Middle) =>
                        self.mouse_pressed.2 = state == ElementState::Pressed,
                    _ => ()
            }
        }
        result
    }
}
