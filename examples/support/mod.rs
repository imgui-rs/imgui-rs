use glium::{DisplayBuild, Surface};
use glium::glutin;
use glium::glutin::{ElementState, Event, MouseButton, VirtualKeyCode};
use imgui::{ImGui, Frame};
use imgui::glium_renderer::Renderer;
use time::SteadyTime;

pub fn main_with_frame<'a, F: Fn(&Frame<'a>)>(f: F) {
    let display = glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();

    let mut imgui = ImGui::init();
    let mut renderer = Renderer::init(&mut imgui, &display).unwrap();

    let mut last_frame = SteadyTime::now();
    let mut mouse_pos = (0, 0);
    let mut mouse_pressed = (false, false, false);

    'main: loop {
        let now = SteadyTime::now();
        let delta = now - last_frame;
        let delta_f = delta.num_nanoseconds().unwrap() as f32 / 1_000_000_000.0;
        last_frame = now;

        imgui.set_mouse_pos(mouse_pos.0 as f32, mouse_pos.1 as f32);
        imgui.set_mouse_down(&[mouse_pressed.0, mouse_pressed.1, mouse_pressed.2, false, false]);

        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);

        let (width, height) = target.get_dimensions();
        let frame = imgui.frame(width, height, delta_f);
        f(&frame);
        renderer.render(&mut target, frame).unwrap();

        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                Event::Closed |
                    Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape))
                        => break 'main,
                Event::MouseMoved(pos) => mouse_pos = pos,
                Event::MouseInput(state, MouseButton::Left) =>
                    mouse_pressed.0 = state == ElementState::Pressed,
                Event::MouseInput(state, MouseButton::Right) =>
                    mouse_pressed.1 = state == ElementState::Pressed,
                Event::MouseInput(state, MouseButton::Middle) =>
                    mouse_pressed.2 = state == ElementState::Pressed,
                _ => ()
            }
        }
    }
}
