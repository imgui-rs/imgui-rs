use imgui::*;

mod support;

fn main() {
    support::init(file!()).main_loop(|run, ui| {
        let red = (1.0, 0.0, 0.0).into();
        let green = (0.0, 1.0, 0.0).into();
        let blue = (0.5, 0.5, 1.0).into();

        Window::new(im_str!("Hello draw list"))
            .opened(run)
            .size([250.0, 250.0], Condition::Once)
            .build(&ui, || {
                ui.text("Hello world!");

                let window_draw_list = ui.get_window_draw_list();

                // in screen space
                window_draw_list.add_circle((250.0, 250.0).into(), 50.0, red, 25, 1.0);
                window_draw_list.add_text((160.0, 280.0).into(), red, "window screen space");

                // in window space
                let p1 = ui.get_cursor_screen_pos();
                window_draw_list.add_line(p1, sys::ImVec2::new(p1.x + 50.0, p1.y + 50.0), red, 1.0);
                window_draw_list.add_text(sys::ImVec2::new(p1.x + 50.0, p1.y + 50.0), red, "window local");
            });

        let bg_draw_list = ui.get_background_draw_list();
        bg_draw_list.add_circle((250.0, 250.0).into(), 100.0, blue, 25, 1.0);
        bg_draw_list.add_text((100.0, 160.0).into(), blue, "background");

        let fg_draw_list = ui.get_foreground_draw_list();
        fg_draw_list.add_circle((250.0, 250.0).into(), 75.0, green, 25, 1.0);
        fg_draw_list.add_text((100.0, 200.0).into(), green, "foreground");
    });
}
