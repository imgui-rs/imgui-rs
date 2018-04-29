extern crate glium;
extern crate imgui;
extern crate imgui_glium_renderer;

mod support;

const CLEAR_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

fn main() {
    support::run(
        "test_drawing_channels_split".to_owned(),
        CLEAR_COLOR,
        |ui| {
            let draw_list = ui.get_window_draw_list();
            // Will draw channel 0 first, then channel 1, whatever the order of
            // the calls in the code.
            //
            // Here, we draw a red line on channel 1 then a white circle on
            // channel 0. As a result, the red line will always appear on top of
            // the white circle.
            draw_list.channels_split(2, |channels| {
                const RADIUS: f32 = 100.0;
                let canvas_pos = ui.get_cursor_screen_pos();
                channels.set_current(1);
                draw_list
                    .add_line(
                        canvas_pos,
                        [canvas_pos.0 + RADIUS, canvas_pos.1 + RADIUS],
                        RED,
                    )
                    .thickness(5.0)
                    .build();

                channels.set_current(0);
                let center = (canvas_pos.0 + RADIUS, canvas_pos.1 + RADIUS);
                draw_list
                    .add_circle(center, RADIUS, WHITE)
                    .thickness(10.0)
                    .num_segments(50)
                    .build();
            });
            true
        },
    );
}
