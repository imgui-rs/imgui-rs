use std::cell;
use std::ops::{Deref, DerefMut};
use std::os::raw::{c_int, c_uchar, c_void};
use std::ptr;
use std::slice;

use crate::internal::RawWrapper;
use crate::sys;
use crate::TextureId;

#[repr(transparent)]
#[derive(Debug)]
pub struct FontAtlas(sys::ImFontAtlas);

impl RawWrapper for FontAtlas {
    type Raw = sys::ImFontAtlas;
    unsafe fn raw(&self) -> &sys::ImFontAtlas {
        &self.0
    }
    unsafe fn raw_mut(&mut self) -> &mut sys::ImFontAtlas {
        &mut self.0
    }
}

impl FontAtlas {
    pub fn build_alpha8_texture(&mut self) -> FontAtlasTexture {
        let mut pixels: *mut c_uchar = ptr::null_mut();
        let mut width: c_int = 0;
        let mut height: c_int = 0;
        let mut bytes_per_pixel: c_int = 0;
        unsafe {
            sys::ImFontAtlas_GetTexDataAsAlpha8(
                &mut self.0,
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
    pub fn build_rgba32_texture(&mut self) -> FontAtlasTexture {
        let mut pixels: *mut c_uchar = ptr::null_mut();
        let mut width: c_int = 0;
        let mut height: c_int = 0;
        let mut bytes_per_pixel: c_int = 0;
        unsafe {
            sys::ImFontAtlas_GetTexDataAsRGBA32(
                &mut self.0,
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
    pub fn texture_id(&self) -> TextureId {
        TextureId::from(self.0.TexID as usize)
    }
    pub fn set_texture_id(&mut self, value: TextureId) {
        self.0.TexID = value.id() as *mut c_void;
    }
    pub fn clear(&mut self) {
        unsafe {
            sys::ImFontAtlas_Clear(&mut self.0);
        }
    }
    pub fn clear_fonts(&mut self) {
        unsafe {
            sys::ImFontAtlas_ClearFonts(&mut self.0);
        }
    }
    pub fn clear_tex_data(&mut self) {
        unsafe {
            sys::ImFontAtlas_ClearTexData(&mut self.0);
        }
    }
    pub fn clear_input_data(&mut self) {
        unsafe {
            sys::ImFontAtlas_ClearInputData(&mut self.0);
        }
    }
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
        unsafe {
            sys::ImFontAtlas_destroy(self.0);
        }
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
#[derive(Debug)]
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
}
