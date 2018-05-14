use std::marker::PhantomData;
use std::mem;
use std::os::raw::{c_int, c_void};
use std::ptr;
use sys;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum FontGlyphRangeData {
    Chinese, Cyrillic, Default, Japanese, Korean, Thai, Custom(*const sys::ImWchar),
}

/// A set of 16-bit Unicode codepoints
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct FontGlyphRange(FontGlyphRangeData);
impl FontGlyphRange {
    /// The default set of glyph ranges used by imgui.
    pub fn default() -> FontGlyphRange {
        FontGlyphRange(FontGlyphRangeData::Default)
    }

    /// A set of glyph ranges appropriate for use with Chinese text.
    pub fn chinese() -> FontGlyphRange {
        FontGlyphRange(FontGlyphRangeData::Chinese)
    }
    /// A set of glyph ranges appropriate for use with Cyrillic text.
    pub fn cyrillic() -> FontGlyphRange {
        FontGlyphRange(FontGlyphRangeData::Cyrillic)
    }
    /// A set of glyph ranges appropriate for use with Japanese text.
    pub fn japanese() -> FontGlyphRange {
        FontGlyphRange(FontGlyphRangeData::Japanese)
    }
    /// A set of glyph ranges appropriate for use with Korean text.
    pub fn korean() -> FontGlyphRange {
        FontGlyphRange(FontGlyphRangeData::Korean)
    }
    /// A set of glyph ranges appropriate for use with Thai text.
    pub fn thai() -> FontGlyphRange {
        FontGlyphRange(FontGlyphRangeData::Thai)
    }

    /// Creates a glyph range from a static slice. The expected format is a series of pairs of
    /// non-zero shorts, each representing an inclusive range of codepoints, followed by a single
    /// zero terminating the range. The ranges must not overlap.
    ///
    /// As the slice is expected to last as long as a font is used, and is written into global
    /// state, it must be `'static`.
    ///
    /// Panics
    /// ======
    ///
    /// This function will panic if the given slice is not a valid font range.
    pub fn from_slice(slice: &'static [sys::ImWchar]) -> FontGlyphRange {
        assert_eq!(slice.len() % 2, 1, "The length of a glyph range must be odd.");
        assert_eq!(slice.last(), Some(&0), "A glyph range must be zero-terminated.");

        for i in 0..slice.len()-1 {
            assert_ne!(slice[i], 0, "A glyph in a range cannot be zero. \
                                     (Glyph is zero at index {})", i)
        }

        let mut ranges = Vec::new();
        for i in 0..slice.len()/2 {
            let (start, end) = (slice[i * 2], slice[i * 2 + 1]);
            assert!(start <= end, "The start of a range cannot be larger than its end. \
                                   (At index {}, {} > {})", i * 2, start, end);
            ranges.push((start, end));
        }
        ranges.sort_unstable_by_key(|x| x.0);
        for i in 0..ranges.len()-1 {
            let (range_a, range_b) = (ranges[i], ranges[i + 1]);
            if range_a.1 >= range_b.0 {
                panic!("The glyph ranges {:?} and {:?} overlap between {:?}.",
                       range_a, range_b, (range_a.1, range_b.0));
            }
        }

        unsafe { FontGlyphRange::from_slice_unchecked(slice) }
    }

    /// Creates a glyph range from a static slice without checking its validity.
    ///
    /// See [`FontRangeGlyph::from_slice`] for more information.
    pub unsafe fn from_slice_unchecked(slice: &'static [sys::ImWchar]) -> FontGlyphRange {
        FontGlyphRange::from_ptr(slice.as_ptr())
    }

    /// Creates a glyph range from a pointer, without checking its validity or enforcing its
    /// lifetime. The memory the pointer points to must be valid for as long as the font is
    /// in use.
    pub unsafe fn from_ptr(ptr: *const sys::ImWchar) -> FontGlyphRange {
        FontGlyphRange(FontGlyphRangeData::Custom(ptr))
    }

    unsafe fn to_ptr(&self, atlas: *mut sys::ImFontAtlas) -> *const sys::ImWchar {
        match &self.0 {
            &FontGlyphRangeData::Chinese  => sys::ImFontAtlas_GetGlyphRangesChinese(atlas),
            &FontGlyphRangeData::Cyrillic => sys::ImFontAtlas_GetGlyphRangesCyrillic(atlas),
            &FontGlyphRangeData::Default  => sys::ImFontAtlas_GetGlyphRangesDefault(atlas),
            &FontGlyphRangeData::Japanese => sys::ImFontAtlas_GetGlyphRangesJapanese(atlas),
            &FontGlyphRangeData::Korean   => sys::ImFontAtlas_GetGlyphRangesKorean(atlas),
            &FontGlyphRangeData::Thai     => sys::ImFontAtlas_GetGlyphRangesThai(atlas),

            &FontGlyphRangeData::Custom(ptr) => ptr,
        }
    }
}

/// A builder for the configuration for a font.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ImFontConfig {
    size_pixels: f32, oversample_h: u32, oversample_v: u32, pixel_snap_h: bool,
    glyph_extra_spacing: sys::ImVec2, glyph_offset: sys::ImVec2, merge_mode: bool,
    rasterizer_multiply: f32,
}
impl ImFontConfig {
    pub fn new() -> ImFontConfig {
        ImFontConfig {
            size_pixels: 0.0, oversample_h: 3, oversample_v: 1, pixel_snap_h: false,
            glyph_extra_spacing: sys::ImVec2::zero(), glyph_offset: sys::ImVec2::zero(),
            merge_mode: false, rasterizer_multiply: 1.0,
        }
    }

    pub fn size_pixels(mut self, size_pixels: f32) -> ImFontConfig {
        self.size_pixels = size_pixels;
        self
    }
    pub fn oversample_h(mut self, oversample_h: u32) -> ImFontConfig {
        self.oversample_h = oversample_h;
        self
    }
    pub fn oversample_v(mut self, oversample_v: u32) -> ImFontConfig {
        self.oversample_v = oversample_v;
        self
    }
    pub fn pixel_snap_h(mut self, pixel_snap_h: bool) -> ImFontConfig {
        self.pixel_snap_h = pixel_snap_h;
        self
    }
    pub fn glyph_extra_spacing<I: Into<sys::ImVec2>>(mut self, extra_spacing: I) -> ImFontConfig {
        self.glyph_extra_spacing = extra_spacing.into();
        self
    }
    pub fn glyph_offset<I: Into<sys::ImVec2>>(mut self, glyph_offset: I) -> ImFontConfig {
        self.glyph_offset = glyph_offset.into();
        self
    }
    pub fn merge_mode(mut self, merge_mode: bool) -> ImFontConfig {
        self.merge_mode = merge_mode;
        self
    }
    pub fn rasterizer_multiply(mut self, rasterizer_multiply: f32) -> ImFontConfig {
        self.rasterizer_multiply = rasterizer_multiply;
        self
    }

    fn make_config(self) -> sys::ImFontConfig {
        let mut config = unsafe {
            let mut config = mem::uninitialized();
            sys::ImFontConfig_DefaultConstructor(&mut config);
            config
        };
        config.size_pixels = self.size_pixels;
        config.oversample_h = self.oversample_h as c_int;
        config.oversample_v = self.oversample_v as c_int;
        config.pixel_snap_h = self.pixel_snap_h;
        config.glyph_extra_spacing = self.glyph_extra_spacing;
        config.glyph_offset = self.glyph_offset;
        config.merge_mode = self.merge_mode;
        config.rasterizer_multiply = self.rasterizer_multiply;
        config
    }

    /// Adds a custom font to the font set with the given configuration. A font size must be set
    /// in the configuration.
    ///
    /// Panics
    /// ======
    ///
    /// If no font size is set for the configuration.
    pub fn add_font<'a>(self, atlas: &'a mut ImFontAtlas<'a>, data: &[u8],
                        range: &FontGlyphRange) -> ImFont<'a> {
        atlas.add_font_with_config(data, self, range)
    }

    /// Adds the default font to a given atlas using this configuration.
    pub fn add_default_font<'a>(self, atlas: &'a mut ImFontAtlas<'a>) -> ImFont<'a> {
        atlas.add_default_font_with_config(self)
    }
}
impl Default for ImFontConfig {
    fn default() -> Self {
        ImFontConfig::new()
    }
}

/// A handle to an imgui font.
pub struct ImFont<'a> {
    font: *mut sys::ImFont, _phantom: PhantomData<&'a mut sys::ImFont>,
}
impl <'a> ImFont<'a> {
    unsafe fn from_ptr(font: *mut sys::ImFont) -> ImFont<'a> {
        ImFont { font, _phantom: PhantomData }
    }

    fn chain(&mut self) -> ImFont {
        ImFont { font: self.font, _phantom: PhantomData }
    }

    pub fn font_size(&self) -> f32 {
        unsafe { sys::ImFont_GetFontSize(self.font) }
    }
    pub fn set_font_size(&mut self, size: f32) -> ImFont {
        unsafe { sys::ImFont_SetFontSize(self.font, size) }
        self.chain()
    }

    pub fn scale(&self) -> f32 {
        unsafe { sys::ImFont_GetScale(self.font) }
    }
    pub fn set_scale(&mut self, size: f32) -> ImFont {
        unsafe { sys::ImFont_SetScale(self.font, size) }
        self.chain()
    }

    pub fn display_offset(&self) -> (f32, f32) {
        let mut display_offset = unsafe { mem::uninitialized() };
        unsafe { sys::ImFont_GetDisplayOffset(self.font, &mut display_offset) }
        display_offset.into()
    }
}

/// A handle to imgui's font manager.
#[repr(C)]
pub struct ImFontAtlas<'a> {
    atlas: *mut sys::ImFontAtlas, _phantom: PhantomData<&'a mut sys::ImFontAtlas>,
}
impl <'a> ImFontAtlas<'a> {
    pub(crate) unsafe fn from_ptr(atlas: *mut sys::ImFontAtlas) -> ImFontAtlas<'a> {
        ImFontAtlas { atlas, _phantom: PhantomData }
    }

    /// Adds the default font to the font set.
    pub fn add_default_font(&mut self) -> ImFont {
        unsafe { ImFont::from_ptr(sys::ImFontAtlas_AddFontDefault(self.atlas, ptr::null_mut())) }
    }

    /// Adds the default fnt to the font set with the given configuration.
    pub fn add_default_font_with_config(&mut self, config: ImFontConfig) -> ImFont {
        let config = config.make_config();
        unsafe { ImFont::from_ptr(sys::ImFontAtlas_AddFontDefault(self.atlas, &config)) }
    }

    fn raw_add_font(&mut self, data: &[u8], config: ImFontConfig,
                    range: &FontGlyphRange) -> ImFont {
        assert!((data.len() as u64) < (c_int::max_value() as u64), "Font data is too long.");
        unsafe {
            let mut config = config.make_config();
            assert!(config.size_pixels > 0.0, "Font size cannot be zero.");
            config.font_data = data.as_ptr() as *mut c_void;
            config.font_data_size = data.len() as c_int;
            config.glyph_ranges = range.to_ptr(self.atlas);
            config.font_data_owned_by_atlas = false;

            ImFont::from_ptr(sys::ImFontAtlas_AddFont(self.atlas, &config))
        }
    }

    /// Adds a custom font to the font set.
    pub fn add_font(&mut self, data: &[u8], size: f32, range: &FontGlyphRange) -> ImFont {
        self.raw_add_font(data, ImFontConfig::new().size_pixels(size), range)
    }

    /// Adds a custom font to the font set with the given configuration. A font size must be set
    /// in the configuration.
    ///
    /// Panics
    /// ======
    ///
    /// If no font size is set for the configuration.
    pub fn add_font_with_config(&mut self, data: &[u8], config: ImFontConfig,
                                range: &FontGlyphRange) -> ImFont {
        self.raw_add_font(data, config, range)
    }

    /// The number of fonts currently registered in the atlas.
    pub fn font_count(&self) -> usize {
        unsafe { sys::ImFontAtlas_Fonts_size(self.atlas) as usize }
    }

    /// Gets a font from the atlas.
    ///
    /// Panics
    /// ======
    ///
    /// Panics if the index is out of range.
    pub fn index_font(&mut self, index: usize) -> ImFont {
        assert!(index < self.font_count(), "Font index is out of range.");
        unsafe { ImFont::from_ptr(sys::ImFontAtlas_Fonts_index(self.atlas, index as c_int)) }
    }

    /// Clears all fonts associated with this texture atlas.
    pub fn clear(&mut self) {
        unsafe { sys::ImFontAtlas_Clear(self.atlas) }
    }

    pub fn texture_id(&self) -> usize {
        unsafe { (*self.atlas).tex_id as usize }
    }
    pub fn set_texture_id(&mut self, value: usize) {
        unsafe { (*self.atlas).tex_id = value as *mut c_void; }
    }
}