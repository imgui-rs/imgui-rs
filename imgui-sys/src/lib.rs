#![allow(non_upper_case_globals)]

use std::convert::From;

pub use self::enums::*;
pub use self::flags::*;
pub use self::structs::*;

mod bindings;
mod enums;
mod flags;
mod legacy;
mod structs;

pub use bindings::{
    igGetIO, CustomRect, ImDrawCallback, ImDrawChannel, ImDrawCmd, ImDrawData, ImDrawData_Clear,
    ImDrawData_DeIndexAllBuffers, ImDrawData_ScaleClipRects, ImDrawIdx, ImDrawList,
    ImDrawListSharedData, ImDrawListSplitter, ImDrawList_AddBezierCurve, ImDrawList_AddCallback,
    ImDrawList_AddCircle, ImDrawList_AddCircleFilled, ImDrawList_AddConvexPolyFilled,
    ImDrawList_AddDrawCmd, ImDrawList_AddImage, ImDrawList_AddImageQuad,
    ImDrawList_AddImageRounded, ImDrawList_AddLine, ImDrawList_AddPolyline, ImDrawList_AddQuad,
    ImDrawList_AddQuadFilled, ImDrawList_AddRect, ImDrawList_AddRectFilled,
    ImDrawList_AddRectFilledMultiColor, ImDrawList_AddText, ImDrawList_AddTextFontPtr,
    ImDrawList_AddTriangle, ImDrawList_AddTriangleFilled, ImDrawList_ChannelsMerge,
    ImDrawList_ChannelsSetCurrent, ImDrawList_ChannelsSplit, ImDrawList_Clear,
    ImDrawList_ClearFreeMemory, ImDrawList_CloneOutput, ImDrawList_ImDrawList,
    ImDrawList_PathArcTo, ImDrawList_PathArcToFast, ImDrawList_PathBezierCurveTo,
    ImDrawList_PathClear, ImDrawList_PathFillConvex, ImDrawList_PathLineTo,
    ImDrawList_PathLineToMergeDuplicate, ImDrawList_PathRect, ImDrawList_PathStroke,
    ImDrawList_PopClipRect, ImDrawList_PopTextureID, ImDrawList_PrimQuadUV, ImDrawList_PrimRect,
    ImDrawList_PrimRectUV, ImDrawList_PrimReserve, ImDrawList_PrimVtx, ImDrawList_PrimWriteIdx,
    ImDrawList_PrimWriteVtx, ImDrawList_PushClipRect, ImDrawList_PushClipRectFullScreen,
    ImDrawList_PushTextureID, ImDrawList_UpdateClipRect, ImDrawList_UpdateTextureID,
    ImDrawList_destroy, ImDrawVert, ImFont, ImFontAtlas, ImFontAtlas_AddFont,
    ImFontAtlas_AddFontDefault, ImFontAtlas_Clear, ImFontAtlas_GetGlyphRangesChineseFull,
    ImFontAtlas_GetGlyphRangesChineseSimplifiedCommon, ImFontAtlas_GetGlyphRangesCyrillic,
    ImFontAtlas_GetGlyphRangesDefault, ImFontAtlas_GetGlyphRangesJapanese,
    ImFontAtlas_GetGlyphRangesKorean, ImFontAtlas_GetGlyphRangesThai,
    ImFontAtlas_GetGlyphRangesVietnamese, ImFontAtlas_GetTexDataAsRGBA32, ImFontConfig,
    ImFontGlyphRangesBuilder, ImGuiContext, ImGuiID, ImGuiIO, ImGuiIO_AddInputCharacter,
    ImGuiIO_AddInputCharactersUTF8, ImGuiIO_ClearInputCharacters, ImGuiInputTextCallback,
    ImGuiInputTextCallbackData, ImGuiInputTextCallbackData_DeleteChars,
    ImGuiInputTextCallbackData_HasSelection, ImGuiInputTextCallbackData_ImGuiInputTextCallbackData,
    ImGuiInputTextCallbackData_InsertChars, ImGuiInputTextCallbackData_destroy, ImGuiListClipper,
    ImGuiSizeCallback, ImGuiStorage, ImTextureID, ImU32, ImVec2, ImVec2_Simple, ImVec4,
    ImVec4_Simple, ImVector_ImFontPtr, ImVector_ImWchar, ImVector_char, ImWchar,
};
pub use legacy::*;

#[cfg(feature = "gfx")]
mod gfx_support;

#[cfg(feature = "glium")]
mod glium_support;

impl ImVec2 {
    #[inline]
    pub fn new(x: f32, y: f32) -> ImVec2 {
        ImVec2 { x, y }
    }
    #[inline]
    pub fn zero() -> ImVec2 {
        ImVec2 { x: 0.0, y: 0.0 }
    }
}

impl From<[f32; 2]> for ImVec2 {
    #[inline]
    fn from(array: [f32; 2]) -> ImVec2 {
        ImVec2::new(array[0], array[1])
    }
}

impl From<(f32, f32)> for ImVec2 {
    #[inline]
    fn from((x, y): (f32, f32)) -> ImVec2 {
        ImVec2::new(x, y)
    }
}

impl Into<[f32; 2]> for ImVec2 {
    #[inline]
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl Into<(f32, f32)> for ImVec2 {
    #[inline]
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl ImVec2_Simple {
    #[inline]
    pub fn new(x: f32, y: f32) -> ImVec2_Simple {
        ImVec2_Simple { x, y }
    }
    #[inline]
    pub fn zero() -> ImVec2_Simple {
        ImVec2_Simple { x: 0.0, y: 0.0 }
    }
}

impl From<[f32; 2]> for ImVec2_Simple {
    #[inline]
    fn from(array: [f32; 2]) -> ImVec2_Simple {
        ImVec2_Simple::new(array[0], array[1])
    }
}

impl From<(f32, f32)> for ImVec2_Simple {
    #[inline]
    fn from((x, y): (f32, f32)) -> ImVec2_Simple {
        ImVec2_Simple::new(x, y)
    }
}

impl Into<[f32; 2]> for ImVec2_Simple {
    #[inline]
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl Into<(f32, f32)> for ImVec2_Simple {
    #[inline]
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl ImVec4 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> ImVec4 {
        ImVec4 { x, y, z, w }
    }
    #[inline]
    pub fn zero() -> ImVec4 {
        ImVec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}

impl From<[f32; 4]> for ImVec4 {
    #[inline]
    fn from(array: [f32; 4]) -> ImVec4 {
        ImVec4::new(array[0], array[1], array[2], array[3])
    }
}

impl From<(f32, f32, f32, f32)> for ImVec4 {
    #[inline]
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> ImVec4 {
        ImVec4::new(x, y, z, w)
    }
}

impl Into<[f32; 4]> for ImVec4 {
    #[inline]
    fn into(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl Into<(f32, f32, f32, f32)> for ImVec4 {
    #[inline]
    fn into(self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
    }
}

impl ImVec4_Simple {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> ImVec4_Simple {
        ImVec4_Simple { x, y, z, w }
    }
    #[inline]
    pub fn zero() -> ImVec4_Simple {
        ImVec4_Simple {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}

impl From<[f32; 4]> for ImVec4_Simple {
    #[inline]
    fn from(array: [f32; 4]) -> ImVec4_Simple {
        ImVec4_Simple::new(array[0], array[1], array[2], array[3])
    }
}

impl From<(f32, f32, f32, f32)> for ImVec4_Simple {
    #[inline]
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> ImVec4_Simple {
        ImVec4_Simple::new(x, y, z, w)
    }
}

impl Into<[f32; 4]> for ImVec4_Simple {
    #[inline]
    fn into(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl Into<(f32, f32, f32, f32)> for ImVec4_Simple {
    #[inline]
    fn into(self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
    }
}

#[test]
fn test_imvec2_memory_layout() {
    use std::mem;
    assert_eq!(mem::size_of::<ImVec2>(), mem::size_of::<[f32; 2]>());
    assert_eq!(mem::align_of::<ImVec2>(), mem::align_of::<[f32; 2]>());
    let test = ImVec2::new(1.0, 2.0);
    let ref_a: &ImVec2 = &test;
    let ref_b: &[f32; 2] = unsafe { mem::transmute(&test) };
    assert_eq!(&ref_a.x as *const _, &ref_b[0] as *const _);
    assert_eq!(&ref_a.y as *const _, &ref_b[1] as *const _);
}

#[test]
fn test_imvec2_simple_memory_layout() {
    use std::mem;
    assert_eq!(mem::size_of::<ImVec2_Simple>(), mem::size_of::<[f32; 2]>());
    assert_eq!(
        mem::align_of::<ImVec2_Simple>(),
        mem::align_of::<[f32; 2]>()
    );
    let test = ImVec2_Simple::new(1.0, 2.0);
    let ref_a: &ImVec2_Simple = &test;
    let ref_b: &[f32; 2] = unsafe { mem::transmute(&test) };
    assert_eq!(&ref_a.x as *const _, &ref_b[0] as *const _);
    assert_eq!(&ref_a.y as *const _, &ref_b[1] as *const _);
}

#[test]
fn test_imvec4_memory_layout() {
    use std::mem;
    assert_eq!(mem::size_of::<ImVec4>(), mem::size_of::<[f32; 4]>());
    assert_eq!(mem::align_of::<ImVec4>(), mem::align_of::<[f32; 4]>());
    let test = ImVec4::new(1.0, 2.0, 3.0, 4.0);
    let ref_a: &ImVec4 = &test;
    let ref_b: &[f32; 4] = unsafe { mem::transmute(&test) };
    assert_eq!(&ref_a.x as *const _, &ref_b[0] as *const _);
    assert_eq!(&ref_a.y as *const _, &ref_b[1] as *const _);
    assert_eq!(&ref_a.z as *const _, &ref_b[2] as *const _);
    assert_eq!(&ref_a.w as *const _, &ref_b[3] as *const _);
}

#[test]
fn test_imvec4_simple_memory_layout() {
    use std::mem;
    assert_eq!(mem::size_of::<ImVec4_Simple>(), mem::size_of::<[f32; 4]>());
    assert_eq!(
        mem::align_of::<ImVec4_Simple>(),
        mem::align_of::<[f32; 4]>()
    );
    let test = ImVec4_Simple::new(1.0, 2.0, 3.0, 4.0);
    let ref_a: &ImVec4_Simple = &test;
    let ref_b: &[f32; 4] = unsafe { mem::transmute(&test) };
    assert_eq!(&ref_a.x as *const _, &ref_b[0] as *const _);
    assert_eq!(&ref_a.y as *const _, &ref_b[1] as *const _);
    assert_eq!(&ref_a.z as *const _, &ref_b[2] as *const _);
    assert_eq!(&ref_a.w as *const _, &ref_b[3] as *const _);
}
