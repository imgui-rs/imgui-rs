use gfx::texture::{FilterMethod, SamplerInfo, WrapMode};
use image::ImageFormat;
use imgui::*;
use std::error::Error;

mod support;

#[derive(Default)]
struct CustomTexturesApp {
    my_texture_id: Option<TextureId>,
    lenna: Option<Lenna>,
}

struct Lenna {
    texture_id: TextureId,
    size: [f32; 2],
}

impl CustomTexturesApp {
    fn register_textures<R, F>(
        &mut self,
        factory: &mut F,
        textures: &mut Textures<imgui_gfx_renderer::Texture<R>>,
    ) -> Result<(), Box<dyn Error>>
    where
        R: gfx::Resources,
        F: gfx::Factory<R>,
    {
        const WIDTH: usize = 128;
        const HEIGHT: usize = 128;

        if self.my_texture_id.is_none() {
            // Generate dummy texture
            let mut data = Vec::with_capacity(WIDTH * HEIGHT * 4);
            for i in 0..WIDTH {
                for j in 0..HEIGHT {
                    // Insert RGBA values
                    data.push(i as u8);
                    data.push(j as u8);
                    data.push((i + j) as u8);
                    data.push(255);
                }
            }

            let (_, texture_view) = factory.create_texture_immutable_u8::<gfx::format::Srgba8>(
                gfx::texture::Kind::D2(WIDTH as u16, HEIGHT as u16, gfx::texture::AaMode::Single),
                gfx::texture::Mipmap::Provided,
                &[data.as_slice()],
            )?;
            let sampler =
                factory.create_sampler(SamplerInfo::new(FilterMethod::Bilinear, WrapMode::Clamp));
            let texture_id = textures.insert((texture_view, sampler));

            self.my_texture_id = Some(texture_id);
        }

        if self.lenna.is_none() {
            self.lenna = Some(Lenna::new(factory, textures)?);
        }

        Ok(())
    }

    fn show_textures(&self, ui: &Ui) {
        Window::new(im_str!("Hello textures"))
            .size([400.0, 600.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Hello textures!"));
                if let Some(my_texture_id) = self.my_texture_id {
                    ui.text("Some generated texture");
                    Image::new(my_texture_id, [100.0, 100.0]).build(ui);
                }

                if let Some(lenna) = &self.lenna {
                    ui.text("Say hello to Lenna.jpg");
                    lenna.show(ui);
                }
            });
    }
}

impl Lenna {
    fn new<R, F>(
        factory: &mut F,
        textures: &mut Textures<imgui_gfx_renderer::Texture<R>>,
    ) -> Result<Self, Box<dyn Error>>
    where
        R: gfx::Resources,
        F: gfx::Factory<R>,
    {
        let lenna_bytes = include_bytes!("../../resources/Lenna.jpg");

        let image = image::load_from_memory_with_format(lenna_bytes, ImageFormat::Jpeg)?;
        // gfx doesn't support easily RGB8 without alpha, so we need to convert
        let image = image.to_rgba8();
        let (width, height) = image.dimensions();
        let raw_data = image.into_raw();

        let (_, texture_view) = factory.create_texture_immutable_u8::<gfx::format::Srgba8>(
            gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single),
            gfx::texture::Mipmap::Provided,
            &[raw_data.as_slice()],
        )?;
        let sampler =
            factory.create_sampler(SamplerInfo::new(FilterMethod::Bilinear, WrapMode::Clamp));
        let texture_id = textures.insert((texture_view, sampler));
        Ok(Lenna {
            texture_id,
            size: [width as f32, height as f32],
        })
    }

    fn show(&self, ui: &Ui) {
        Image::new(self.texture_id, self.size).build(ui);
    }
}

fn main() {
    let mut my_app = CustomTexturesApp::default();
    let mut system = support::init(file!());
    my_app
        .register_textures(
            &mut system.render_sys.factory,
            system.render_sys.renderer.textures(),
        )
        .expect("Failed to register textures");
    system.main_loop(|_, ui| my_app.show_textures(ui));
}
