use crate::internal::RawCast;
use crate::sys;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct FontGlyph {
    pub codepoint: u16,
    pub advance_x: f32,
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
    pub u0: f32,
    pub v0: f32,
    pub u1: f32,
    pub v1: f32,
}

unsafe impl RawCast<sys::ImFontGlyph> for FontGlyph {}

#[test]
fn test_font_glyph_memory_layout() {
    use std::mem;
    assert_eq!(
        mem::size_of::<FontGlyph>(),
        mem::size_of::<sys::ImFontGlyph>()
    );
    assert_eq!(
        mem::align_of::<FontGlyph>(),
        mem::align_of::<sys::ImFontGlyph>()
    );
    use memoffset::offset_of;
    use sys::ImFontGlyph;
    macro_rules! assert_field_offset {
        ($l:ident, $r:ident) => {
            assert_eq!(offset_of!(FontGlyph, $l), offset_of!(ImFontGlyph, $r));
        };
    };
    assert_field_offset!(codepoint, Codepoint);
    assert_field_offset!(advance_x, AdvanceX);
    assert_field_offset!(x0, X0);
    assert_field_offset!(y0, Y0);
    assert_field_offset!(x1, X1);
    assert_field_offset!(y1, Y1);
    assert_field_offset!(u0, U0);
    assert_field_offset!(v0, V0);
    assert_field_offset!(u1, U1);
    assert_field_offset!(v1, V1);
}
