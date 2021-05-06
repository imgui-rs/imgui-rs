use tiny_skia::Pixmap;
use imgui::{im_str, FontConfig, FontSource};

fn main() {
    // Size of our software "display"
    let width = 1000;
    let height = 500;

    // Create imgui Context as per usual
    let mut imgui_ctx = imgui::Context::create();

    // Don't save window layout etc
    imgui_ctx.set_ini_filename(None);

    // Tell imgui to draw a cursor, and set the cursor position
    imgui_ctx.io_mut().mouse_draw_cursor = true;
    imgui_ctx.io_mut().mouse_pos = [200.0, 50.0];

    // Register the default font
    imgui_ctx.fonts().add_font(&[FontSource::DefaultFontData {
        config: Some(FontConfig {
            size_pixels: 13.0,
            ..FontConfig::default()
        }),
    }]);

    // Generate font atlas texture
    // FIXME: Belongs as helper in lib
    let font_pixmap = {
        let mut font_atlas = imgui_ctx.fonts();
        let font_atlas_tex = font_atlas.build_rgba32_texture();

        let mut font_pixmap = Pixmap::new(font_atlas_tex.width, font_atlas_tex.height).unwrap();

        {
            let data = font_pixmap.pixels_mut();
            for (i, src) in font_atlas_tex.data.chunks(4).enumerate() {
                data[i] =
                    tiny_skia::ColorU8::from_rgba(src[0], src[1], src[2], src[3]).premultiply();
            }
        }

        font_pixmap
    };

    // Set display size
    // FIXME: Belongs as helper in lib
    imgui_ctx.io_mut().display_size = [width as f32, height as f32];
    imgui_ctx.io_mut().display_framebuffer_scale = [1.0, 1.0];

    for frame in 0..10 {
        println!("Frame {}", frame);
        imgui_ctx
            .io_mut()
            .update_delta_time(std::time::Duration::from_millis(20));

        let draw_data: &imgui::DrawData = {
            // New frame
            let ui = imgui_ctx.frame();

            // Create an example window
            imgui::Window::new(im_str!("Example"))
                .size([250.0, 100.0], imgui::Condition::FirstUseEver)
                .position([10.0, 200.0], imgui::Condition::FirstUseEver)
                .build(&ui, || {
                    // Some basic widgets
                    ui.button(imgui::im_str!("Hi"));
                    ui.text("Ok");
                    let mut thing = 0.4;
                    ui.input_float(im_str!("##Test"), &mut thing).build();

                    // Use custom drawing API to draw useless purple box
                    ui.get_window_draw_list()
                        .add_rect([10.0, 10.0], [50.0, 50.0], [0.5, 0.0, 1.0])
                        .filled(true)
                        .rounding(6.0)
                        .build();
                });

            // Show built-in example windows
            ui.show_demo_window(&mut true);
            ui.show_metrics_window(&mut true);

            // Done, get draw list data
            ui.render()
        };

        // Create empty pixmap
        let mut px = Pixmap::new(width, height).unwrap();
        px.fill(tiny_skia::Color::from_rgba8(89, 89, 89, 255));

        // Render imgui data
        let r = imgui_software_renderer::Renderer::new();
        r.render(&mut px, draw_data, font_pixmap.as_ref());

        // Save output
        px.save_png(format!("test_{}.png", frame)).unwrap();
    }
}
