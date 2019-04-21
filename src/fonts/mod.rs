use crate::fonts::atlas::FontId;
use crate::fonts::font::Font;
use crate::internal::RawCast;
use crate::Ui;

pub mod atlas;
pub mod font;
pub mod glyph;
pub mod glyph_ranges;

impl<'ui> Ui<'ui> {
    pub fn with_font<T, F>(&self, id: FontId, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let fonts = self.fonts();
        let font = fonts
            .get_font(id)
            .expect("Font atlas did not contain the given font");
        unsafe { sys::igPushFont(font.raw() as *const _ as *mut _) };
        let result = f();
        unsafe { sys::igPopFont() };
        result
    }
    /// Returns the current font
    pub fn get_font(&self) -> &Font {
        unsafe { Font::from_raw(&*sys::igGetFont()) }
    }
    /// Returns the current font size (= height in pixels) with font scale applied
    pub fn get_font_size(&self) -> f32 {
        unsafe { sys::igGetFontSize() }
    }
    /// Returns the UV coordinate for a white pixel.
    ///
    /// Useful for drawing custom shapes with the draw list API.
    pub fn get_font_tex_uv_white_pixel(&self) -> [f32; 2] {
        unsafe { sys::igGetFontTexUvWhitePixel_nonUDT2().into() }
    }
    /// Set the font scale of the current window
    pub fn set_window_font_scale(&self, scale: f32) {
        unsafe { sys::igSetWindowFontScale(scale) }
    }
}
