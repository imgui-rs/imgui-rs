use imgui::*;

mod support;

fn main() {
    let system = support::init(file!());
    system.main_loop(|_, ui| {
        let red = [1.0, 0.0, 0.0];
        let green = [0.0, 1.0, 0.0];
        let blue = [0.5, 0.5, 1.0];

        Window::new(im_str!("Hello world"))
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text("Hello world!");

                let mut window_draw_list = ui.get_window_draw_list();

                // in screen space
                window_draw_list
                    .add_circle([250.0, 250.0], 50.0, red)
                    .build();
                window_draw_list.add_text([160.0, 280.0], red, "window screen space");

                // in window space
                let p1 = ui.cursor_screen_pos();
                window_draw_list
                    .add_line(p1, [p1[0] + 50.0, p1[1] + 50.0], red)
                    .build();
                window_draw_list.add_text([p1[0] + 50.0, p1[1] + 50.0], red, "window local");
            });

        let mut bg_draw_list = ui.get_background_draw_list();
        bg_draw_list.add_circle([250.0, 250.0], 100.0, blue).build();
        bg_draw_list.add_text([100.0, 160.0], blue, "background");

        let mut fg_draw_list = ui.get_foreground_draw_list();
        fg_draw_list.add_circle([250.0, 250.0], 75.0, green).build();
        fg_draw_list.add_text([100.0, 200.0], green, "foreground");
    });
}
