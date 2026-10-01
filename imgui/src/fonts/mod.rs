use crate::fonts::font::Font;
use crate::internal::RawCast;
use crate::Ui;

pub mod atlas;
pub mod font;
pub mod glyph;
pub mod glyph_ranges;

/// # Fonts
impl Ui {
    /// Returns the current font
    #[doc(alias = "GetFont")]
    pub fn current_font(&self) -> &Font {
        unsafe { Font::from_raw(&*sys::igGetFont()) }
    }
    /// Returns the current font size (= height in pixels) with font scale applied
    #[doc(alias = "GetFontSize")]
    pub fn current_font_size(&self) -> f32 {
        unsafe { sys::igGetFontSize() }
    }
    /// Returns the UV coordinate for a white pixel.
    ///
    /// Useful for drawing custom shapes with the draw list API.
    #[doc(alias = "FontTexUvWhitePixel")]
    pub fn font_tex_uv_white_pixel(&self) -> [f32; 2] {
        let out = unsafe { sys::igGetFontTexUvWhitePixel() };
        out.into()
    }
    // `set_window_font_scale` was removed here: Dear ImGui 1.92 obsoleted
    // `ImGui::SetWindowFontScale()` and it is no longer compiled in under
    // IMGUI_DISABLE_OBSOLETE_FUNCTIONS.
    //
    // There is no drop-in replacement. Upstream suggests `PushFont(NULL, style.FontSizeBase *
    // factor)` for a scoped change, which is push/pop rather than a setter, or
    // `style.FontScaleMain` to scale every window at once. Picking between those is an API
    // decision rather than a mechanical port, so it is left for maintainers.
}
