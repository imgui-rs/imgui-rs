use imgui::*;

mod support;

// rect is [x, y, w, h]
fn draw_text_centered(
    ui: &Ui,
    draw_list: &DrawListMut,
    rect: [f32; 4],
    text: &str,
    color: [f32; 3],
) {
    let text_size = ui.calc_text_size(text);
    let cx = (rect[2] - text_size[0]) / 2.0;
    let cy = (rect[3] - text_size[1]) / 2.0;
    draw_list.add_text([rect[0] + cx, rect[1] + cy], color, text);
}

fn main() {
    let system = support::init(file!());
    system.main_loop(move |_, ui| {
        // Get access to draw FG and BG draw lists.
        let bg_draw_list = ui.get_background_draw_list();
        let fg_draw_list = ui.get_foreground_draw_list();

        // Note we cannot access two instances of the same draw list
        // at once. That is to say, the following line would panic if
        // uncommented:
        //let bg_draw_list_2 = ui.get_background_draw_list(); // panic!

        {
            bg_draw_list
                .add_circle([150.0, 150.0], 150.0, [1.0, 0.0, 0.0])
                .thickness(4.0)
                .build();
            draw_text_centered(
                ui,
                &bg_draw_list,
                [0.0, 0.0, 300.0, 300.0],
                "background draw list",
                [0.0, 0.0, 0.0],
            );
        }

        {
            let [w, h] = ui.io().display_size;
            fg_draw_list
                .add_circle([w - 150.0, h - 150.0], 150.0, [1.0, 0.0, 0.0])
                .thickness(4.0)
                .build();
            draw_text_centered(
                ui,
                &fg_draw_list,
                [w - 300.0, h - 300.0, 300.0, 300.0],
                "foreground draw list",
                [1.0, 0.0, 0.0],
            );
        }

        ui.window("Draw list")
            .size([300.0, 110.0], Condition::FirstUseEver)
            .scroll_bar(false)
            .build(|| {
                ui.button("random button");
                let draw_list = ui.get_window_draw_list();
                let o = ui.cursor_screen_pos();
                let ws = ui.content_region_avail();
                draw_list
                    .add_circle([o[0] + 10.0, o[1] + 10.0], 5.0, [1.0, 0.0, 0.0])
                    .thickness(4.0)
                    .build();
                draw_list
                    .add_circle([o[0] + ws[0] - 10.0, o[1] + 10.0], 5.0, [0.0, 1.0, 0.0])
                    .thickness(4.0)
                    .build();
                draw_list
                    .add_circle(
                        [o[0] + ws[0] - 10.0, o[1] + ws[1] - 10.0],
                        5.0,
                        [0.0, 0.0, 1.0],
                    )
                    .thickness(4.0)
                    .build();
                draw_list
                    .add_circle([o[0] + 10.0, o[1] + ws[1] - 10.0], 5.0, [1.0, 1.0, 0.0])
                    .thickness(4.0)
                    .build();
                draw_text_centered(
                    ui,
                    &draw_list,
                    [o[0], o[1], ws[0], ws[1]],
                    "window draw list",
                    [1.0, 1.0, 1.0],
                );
            });

        ui.window("Polygons")
            .size([300.0, 150.0], Condition::FirstUseEver)
            .position([400.0, 110.0], Condition::FirstUseEver)
            .scroll_bar(false)
            .build(|| {
                let draw_list = ui.get_window_draw_list();

                // Origin
                let o = ui.cursor_screen_pos();

                draw_list
                    .add_polyline(
                        vec![
                            [o[0] + 0.0, o[1] + 0.0],
                            [o[0] + 100.0, o[1] + 25.0],
                            [o[0] + 50.0, o[1] + 50.0],
                            [o[0] + 100.0, o[1] + 75.0],
                            [o[0] + 0.0, o[1] + 100.0],
                            [o[0] + 0.0, o[1] + 0.0],
                        ],
                        [1.0, 0.0, 1.0],
                    )
                    .build();

                draw_list
                    .add_polyline(
                        vec![
                            [o[0] + 120.0 + 0.0, o[1] + 0.0],
                            [o[0] + 120.0 + 100.0, o[1] + 25.0],
                            [o[0] + 120.0 + 50.0, o[1] + 50.0],
                            [o[0] + 120.0 + 100.0, o[1] + 75.0],
                            [o[0] + 120.0 + 0.0, o[1] + 100.0],
                            [o[0] + 120.0 + 0.0, o[1] + 0.0],
                        ],
                        [0.0, 1.0, 1.0],
                    )
                    .filled(true)
                    .build();
            });
    });
}
