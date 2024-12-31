mod copypaste;
pub mod drawing;

use tiny_skia::{Pixmap, PixmapRef};

use imgui::{FontConfig, FontSource};

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }
    pub fn render(&self, mut px: &mut Pixmap, draw_data: &imgui::DrawData, font_pixmap: PixmapRef) {
        crate::drawing::rasterize(&mut px, draw_data, font_pixmap);
    }
}

pub struct TestHelper {
    context: imgui::Context,
    renderer: Renderer,
    buffer: tiny_skia::Pixmap,
    font_atlas_px: tiny_skia::Pixmap,
}

impl TestHelper {
    pub fn setup(size: [f32; 2]) -> Self {
        let mut imgui_ctx = imgui::Context::create();

        // Disable settings save/restore
        imgui_ctx.set_ini_filename(None);

        // Set display size
        imgui_ctx.io_mut().display_size = size;

        // Register font
        imgui_ctx.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: 13.0,
                ..FontConfig::default()
            }),
        }]);

        // Create pixmap for font atlas
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

        Self {
            renderer: Renderer::new(),
            context: imgui_ctx,
            buffer: tiny_skia::Pixmap::new(size[0] as u32, size[1] as u32).unwrap(),
            font_atlas_px: font_pixmap,
        }
    }

    pub fn process<F>(&mut self, interface: F)
    where
        F: FnOnce(&imgui::Ui),
    {
        self.buffer
            .fill(tiny_skia::Color::from_rgba(0.18, 0.18, 0.18, 1.0).unwrap());
        let ui = self.context.frame();
        interface(&ui);
        let draw_data = ui.render();
        self.renderer
            .render(&mut self.buffer, &draw_data, self.font_atlas_px.as_ref());
    }

    pub fn save_snapshot(&self, path: std::path::PathBuf) {
        self.buffer.save_png(path).unwrap();
    }
}
