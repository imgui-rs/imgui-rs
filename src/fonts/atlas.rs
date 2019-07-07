use bitflags::bitflags;
use std::cell;
use std::f32;
use std::ops::{Deref, DerefMut};
use std::os::raw::{c_int, c_uchar, c_void};
use std::ptr;
use std::slice;

use crate::fonts::font::Font;
use crate::fonts::glyph_ranges::FontGlyphRanges;
use crate::internal::{ImVector, RawCast};
use crate::sys;
use crate::TextureId;

bitflags! {
    /// Font atlas configuration flags
    #[repr(transparent)]
    pub struct FontAtlasFlags: u32 {
        const NO_POWER_OF_TWO_HEIGHT = sys::ImFontAtlasFlags_NoPowerOfTwoHeight;
        const NO_MOUSE_CURSORS = sys::ImFontAtlasFlags_NoMouseCursors;
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FontId(pub(crate) *const Font);

/// A font atlas that builds a single texture
#[repr(C)]
pub struct FontAtlas {
    locked: bool,
    /// Configuration flags
    pub flags: FontAtlasFlags,
    /// Texture identifier
    pub tex_id: TextureId,
    /// Texture width desired by user before building the atlas.
    ///
    /// Must be a power-of-two. If you have many glyphs and your graphics API has texture size
    /// restrictions, you may want to increase texture width to decrease the height.
    pub tex_desired_width: i32,
    /// Padding between glyphs within texture in pixels.
    ///
    /// Defaults to 1. If your rendering method doesn't rely on bilinear filtering, you may set
    /// this to 0.
    pub tex_glyph_padding: i32,

    tex_pixels_alpha8: *mut u8,
    tex_pixels_rgba32: *mut u32,
    tex_width: i32,
    tex_height: i32,
    tex_uv_scale: [f32; 2],
    tex_uv_white_pixel: [f32; 2],
    fonts: ImVector<*mut Font>,
    custom_rects: sys::ImVector_CustomRect,
    config_data: sys::ImVector_ImFontConfig,
    custom_rect_ids: [i32; 1],
}

unsafe impl RawCast<sys::ImFontAtlas> for FontAtlas {}

impl FontAtlas {
    pub fn add_font(&mut self, font_sources: &[FontSource]) -> FontId {
        let (head, tail) = font_sources.split_first().unwrap();
        let font_id = self.add_font_internal(head, false);
        for font in tail {
            self.add_font_internal(font, true);
        }
        font_id
    }
    fn add_font_internal(&mut self, font_source: &FontSource, merge_mode: bool) -> FontId {
        let mut raw_config = sys_font_config_default();
        raw_config.MergeMode = merge_mode;
        let raw_font = match font_source {
            FontSource::DefaultFontData { config } => unsafe {
                if let Some(config) = config {
                    config.apply_to_raw_config(&mut raw_config, self.raw_mut());
                }
                sys::ImFontAtlas_AddFontDefault(self.raw_mut(), &raw_config)
            },
            FontSource::TtfData {
                data,
                size_pixels,
                config,
            } => {
                if let Some(config) = config {
                    unsafe {
                        config.apply_to_raw_config(&mut raw_config, self.raw_mut());
                    }
                }
                // We can't guarantee `data` is alive when the font atlas is built, so
                // make a copy and move ownership of the data to the atlas
                let data_copy = unsafe {
                    let ptr = sys::igMemAlloc(data.len()) as *mut u8;
                    assert!(!ptr.is_null());
                    slice::from_raw_parts_mut(ptr, data.len())
                };
                data_copy.copy_from_slice(data);
                raw_config.FontData = data_copy.as_mut_ptr() as *mut c_void;
                raw_config.FontDataSize = data_copy.len() as i32;
                raw_config.FontDataOwnedByAtlas = true;
                raw_config.SizePixels = *size_pixels;
                unsafe { sys::ImFontAtlas_AddFont(self.raw_mut(), &raw_config) }
            }
        };
        FontId(raw_font as *const _)
    }
    pub fn fonts(&self) -> Vec<FontId> {
        let mut result = Vec::new();
        unsafe {
            for &font in self.fonts.as_slice() {
                result.push((*font).id());
            }
        }
        result
    }
    pub fn get_font(&self, id: FontId) -> Option<&Font> {
        unsafe {
            for &font in self.fonts.as_slice() {
                if id == FontId(font) {
                    return Some(&*(font as *const Font));
                }
            }
        }
        None
    }
    /// Returns true if the font atlas has been built
    pub fn is_built(&self) -> bool {
        unsafe { sys::ImFontAtlas_IsBuilt(self.raw() as *const sys::ImFontAtlas as *mut _) }
    }
    /// Builds a 1 byte per-pixel font atlas texture
    pub fn build_alpha8_texture(&mut self) -> FontAtlasTexture {
        let mut pixels: *mut c_uchar = ptr::null_mut();
        let mut width: c_int = 0;
        let mut height: c_int = 0;
        let mut bytes_per_pixel: c_int = 0;
        unsafe {
            sys::ImFontAtlas_GetTexDataAsAlpha8(
                self.raw_mut(),
                &mut pixels,
                &mut width,
                &mut height,
                &mut bytes_per_pixel,
            );
            assert!(width >= 0, "font texture width must be positive");
            assert!(height >= 0, "font texture height must be positive");
            assert!(
                bytes_per_pixel >= 0,
                "font texture bytes per pixel must be positive"
            );
            let height = height as usize;
            // Check multiplication to avoid constructing an invalid slice in case of overflow
            let pitch = width
                .checked_mul(bytes_per_pixel)
                .expect("Overflow in font texture pitch calculation")
                as usize;
            FontAtlasTexture {
                width: width as u32,
                height: height as u32,
                data: slice::from_raw_parts(pixels, pitch * height),
            }
        }
    }
    /// Builds a 4 byte per-pixel font atlas texture
    pub fn build_rgba32_texture(&mut self) -> FontAtlasTexture {
        let mut pixels: *mut c_uchar = ptr::null_mut();
        let mut width: c_int = 0;
        let mut height: c_int = 0;
        let mut bytes_per_pixel: c_int = 0;
        unsafe {
            sys::ImFontAtlas_GetTexDataAsRGBA32(
                self.raw_mut(),
                &mut pixels,
                &mut width,
                &mut height,
                &mut bytes_per_pixel,
            );
            assert!(width >= 0, "font texture width must be positive");
            assert!(height >= 0, "font texture height must be positive");
            assert!(
                bytes_per_pixel >= 0,
                "font texture bytes per pixel must be positive"
            );
            let height = height as usize;
            // Check multiplication to avoid constructing an invalid slice in case of overflow
            let pitch = width
                .checked_mul(bytes_per_pixel)
                .expect("Overflow in font texture pitch calculation")
                as usize;
            FontAtlasTexture {
                width: width as u32,
                height: height as u32,
                data: slice::from_raw_parts(pixels, pitch * height),
            }
        }
    }
    /// Clears the font atlas completely (both input and output data)
    pub fn clear(&mut self) {
        unsafe {
            sys::ImFontAtlas_Clear(self.raw_mut());
        }
    }
    /// Clears output font data (glyph storage, UV coordinates)
    pub fn clear_fonts(&mut self) {
        unsafe {
            sys::ImFontAtlas_ClearFonts(self.raw_mut());
        }
    }
    /// Clears output texture data.
    ///
    /// Can be used to save RAM once the texture has been transferred to the GPU.
    pub fn clear_tex_data(&mut self) {
        unsafe {
            sys::ImFontAtlas_ClearTexData(self.raw_mut());
        }
    }
    /// Clears all the data used to build the textures and fonts
    pub fn clear_input_data(&mut self) {
        unsafe {
            sys::ImFontAtlas_ClearInputData(self.raw_mut());
        }
    }
}

#[test]
fn test_font_atlas_memory_layout() {
    use std::mem;
    assert_eq!(
        mem::size_of::<FontAtlas>(),
        mem::size_of::<sys::ImFontAtlas>()
    );
    assert_eq!(
        mem::align_of::<FontAtlas>(),
        mem::align_of::<sys::ImFontAtlas>()
    );
    use memoffset::offset_of;
    use sys::ImFontAtlas;
    macro_rules! assert_field_offset {
        ($l:ident, $r:ident) => {
            assert_eq!(offset_of!(FontAtlas, $l), offset_of!(ImFontAtlas, $r));
        };
    };
    assert_field_offset!(locked, Locked);
    assert_field_offset!(flags, Flags);
    assert_field_offset!(tex_id, TexID);
    assert_field_offset!(tex_desired_width, TexDesiredWidth);
    assert_field_offset!(tex_glyph_padding, TexGlyphPadding);
    assert_field_offset!(tex_pixels_alpha8, TexPixelsAlpha8);
    assert_field_offset!(tex_pixels_rgba32, TexPixelsRGBA32);
    assert_field_offset!(tex_width, TexWidth);
    assert_field_offset!(tex_height, TexHeight);
    assert_field_offset!(tex_uv_scale, TexUvScale);
    assert_field_offset!(tex_uv_white_pixel, TexUvWhitePixel);
    assert_field_offset!(fonts, Fonts);
    assert_field_offset!(custom_rects, CustomRects);
    assert_field_offset!(config_data, ConfigData);
    assert_field_offset!(custom_rect_ids, CustomRectIds);
}

#[derive(Clone, Debug)]
pub enum FontSource<'a> {
    DefaultFontData {
        config: Option<FontConfig>,
    },
    TtfData {
        data: &'a [u8],
        size_pixels: f32,
        config: Option<FontConfig>,
    },
}

#[derive(Clone, Debug)]
pub struct FontConfig {
    pub size_pixels: f32,
    pub oversample_h: i32,
    pub oversample_v: i32,
    pub pixel_snap_h: bool,
    pub glyph_extra_spacing: [f32; 2],
    pub glyph_offset: [f32; 2],
    pub glyph_ranges: FontGlyphRanges,
    pub glyph_min_advance_x: f32,
    pub glyph_max_advance_x: f32,
    pub rasterizer_flags: u32,
    pub rasterizer_multiply: f32,
}

impl Default for FontConfig {
    fn default() -> FontConfig {
        FontConfig {
            size_pixels: 0.0,
            oversample_h: 3,
            oversample_v: 1,
            pixel_snap_h: false,
            glyph_extra_spacing: [0.0, 0.0],
            glyph_offset: [0.0, 0.0],
            glyph_ranges: FontGlyphRanges::default(),
            glyph_min_advance_x: 0.0,
            glyph_max_advance_x: f32::MAX,
            rasterizer_flags: 0,
            rasterizer_multiply: 1.0,
        }
    }
}

impl FontConfig {
    fn apply_to_raw_config(&self, raw: &mut sys::ImFontConfig, atlas: *mut sys::ImFontAtlas) {
        raw.SizePixels = self.size_pixels;
        raw.OversampleH = self.oversample_h;
        raw.OversampleV = self.oversample_v;
        raw.PixelSnapH = self.pixel_snap_h;
        raw.GlyphExtraSpacing = self.glyph_extra_spacing.into();
        raw.GlyphOffset = self.glyph_offset.into();
        raw.GlyphRanges = unsafe { self.glyph_ranges.to_ptr(atlas) };
        raw.GlyphMinAdvanceX = self.glyph_min_advance_x;
        raw.GlyphMaxAdvanceX = self.glyph_max_advance_x;
        raw.RasterizerFlags = self.rasterizer_flags;
        raw.RasterizerMultiply = self.rasterizer_multiply;
    }
}

fn sys_font_config_default() -> sys::ImFontConfig {
    unsafe {
        let heap_allocated = sys::ImFontConfig_ImFontConfig();
        let copy = *heap_allocated;
        sys::ImFontConfig_destroy(heap_allocated);
        copy
    }
}

#[test]
fn test_font_config_default() {
    let sys_font_config = sys_font_config_default();
    let font_config = FontConfig::default();
    assert_eq!(font_config.size_pixels, sys_font_config.SizePixels);
    assert_eq!(font_config.oversample_h, sys_font_config.OversampleH);
    assert_eq!(font_config.oversample_v, sys_font_config.OversampleV);
    assert_eq!(font_config.pixel_snap_h, sys_font_config.PixelSnapH);
    assert_eq!(
        font_config.glyph_extra_spacing[0],
        sys_font_config.GlyphExtraSpacing.x
    );
    assert_eq!(
        font_config.glyph_extra_spacing[1],
        sys_font_config.GlyphExtraSpacing.y
    );
    assert_eq!(font_config.glyph_offset[0], sys_font_config.GlyphOffset.x);
    assert_eq!(font_config.glyph_offset[1], sys_font_config.GlyphOffset.y);
    assert_eq!(
        font_config.glyph_min_advance_x,
        sys_font_config.GlyphMinAdvanceX
    );
    assert_eq!(
        font_config.glyph_max_advance_x,
        sys_font_config.GlyphMaxAdvanceX
    );
    assert_eq!(
        font_config.rasterizer_flags,
        sys_font_config.RasterizerFlags
    );
    assert_eq!(
        font_config.rasterizer_multiply,
        sys_font_config.RasterizerMultiply
    );
}

/// Handle to a font atlas texture
#[derive(Clone, Debug)]
pub struct FontAtlasTexture<'a> {
    pub width: u32,
    pub height: u32,
    pub data: &'a [u8],
}

/// A font atlas that can be shared between contexts
#[derive(Debug)]
pub struct SharedFontAtlas(*mut sys::ImFontAtlas);

impl SharedFontAtlas {
    pub fn create() -> SharedFontAtlas {
        SharedFontAtlas(unsafe { sys::ImFontAtlas_ImFontAtlas() })
    }
}

impl Drop for SharedFontAtlas {
    fn drop(&mut self) {
        unsafe { sys::ImFontAtlas_destroy(self.0) };
    }
}

impl Deref for SharedFontAtlas {
    type Target = FontAtlas;
    fn deref(&self) -> &FontAtlas {
        unsafe { &*(self.0 as *const FontAtlas) }
    }
}

impl DerefMut for SharedFontAtlas {
    fn deref_mut(&mut self) -> &mut FontAtlas {
        unsafe { &mut *(self.0 as *mut FontAtlas) }
    }
}

/// An immutably borrowed reference to a (possibly shared) font atlas
pub enum FontAtlasRef<'a> {
    Owned(&'a FontAtlas),
    Shared(&'a cell::RefMut<'a, SharedFontAtlas>),
}

impl<'a> Deref for FontAtlasRef<'a> {
    type Target = FontAtlas;
    fn deref(&self) -> &FontAtlas {
        use self::FontAtlasRef::*;
        match self {
            Owned(atlas) => atlas,
            Shared(cell) => cell,
        }
    }
}

/// A mutably borrowed reference to a (possibly shared) font atlas
pub enum FontAtlasRefMut<'a> {
    Owned(&'a mut FontAtlas),
    Shared(cell::RefMut<'a, SharedFontAtlas>),
}

impl<'a> Deref for FontAtlasRefMut<'a> {
    type Target = FontAtlas;
    fn deref(&self) -> &FontAtlas {
        use self::FontAtlasRefMut::*;
        match self {
            Owned(atlas) => atlas,
            Shared(cell) => cell,
        }
    }
}

impl<'a> DerefMut for FontAtlasRefMut<'a> {
    fn deref_mut(&mut self) -> &mut FontAtlas {
        use self::FontAtlasRefMut::*;
        match self {
            Owned(atlas) => atlas,
            Shared(cell) => cell,
        }
    }
}
