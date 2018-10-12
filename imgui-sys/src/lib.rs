#![allow(non_upper_case_globals)]

#[macro_use]
extern crate bitflags;

extern crate libc;

#[cfg(feature = "gfx")]
#[macro_use]
extern crate gfx;

#[cfg(feature = "glium")]
extern crate glium;

use std::convert::From;
use std::os::raw::{c_char, c_float, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};

pub use self::enums::*;
pub use self::flags::*;
pub use self::structs::*;

mod enums;
mod flags;
mod structs;

#[cfg(feature = "gfx")]
mod gfx_support;

#[cfg(feature = "glium")]
mod glium_support;

/// Vertex index
pub type ImDrawIdx = c_ushort;

/// ImGui context (opaque)
pub enum ImGuiContext {}

/// Unique ID used by widgets (typically hashed from a stack of string)
pub type ImGuiID = ImU32;

/// User data to identify a texture
pub type ImTextureID = *mut c_void;

/// 32-bit unsigned integer (typically used to store packed colors)
pub type ImU32 = c_uint;

/// Character for keyboard input/display
pub type ImWchar = c_ushort;

/// Draw callback for advanced use
pub type ImDrawCallback =
    Option<extern "C" fn(parent_list: *const ImDrawList, cmd: *const ImDrawCmd)>;

/// Input text callback for advanced use
pub type ImGuiInputTextCallback =
    Option<extern "C" fn(data: *mut ImGuiInputTextCallbackData) -> c_int>;

/// Size constraint callback for advanced use
pub type ImGuiSizeCallback = Option<extern "C" fn(data: *mut ImGuiSizeCallbackData)>;

/// A tuple of 2 floating-point values
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ImVec2 {
    pub x: c_float,
    pub y: c_float,
}

impl ImVec2 {
    pub fn new(x: f32, y: f32) -> ImVec2 {
        ImVec2 {
            x: x as c_float,
            y: y as c_float,
        }
    }
    pub fn zero() -> ImVec2 {
        ImVec2 {
            x: 0.0 as c_float,
            y: 0.0 as c_float,
        }
    }
}

impl From<[f32; 2]> for ImVec2 {
    fn from(array: [f32; 2]) -> ImVec2 {
        ImVec2::new(array[0], array[1])
    }
}

impl From<(f32, f32)> for ImVec2 {
    fn from((x, y): (f32, f32)) -> ImVec2 {
        ImVec2::new(x, y)
    }
}

impl Into<[f32; 2]> for ImVec2 {
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl Into<(f32, f32)> for ImVec2 {
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}

/// A tuple of 4 floating-point values
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ImVec4 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
    pub w: c_float,
}

impl ImVec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> ImVec4 {
        ImVec4 {
            x: x as c_float,
            y: y as c_float,
            z: z as c_float,
            w: w as c_float,
        }
    }
    pub fn zero() -> ImVec4 {
        ImVec4 {
            x: 0.0 as c_float,
            y: 0.0 as c_float,
            z: 0.0 as c_float,
            w: 0.0 as c_float,
        }
    }
}

impl From<[f32; 4]> for ImVec4 {
    fn from(array: [f32; 4]) -> ImVec4 {
        ImVec4::new(array[0], array[1], array[2], array[3])
    }
}

impl From<(f32, f32, f32, f32)> for ImVec4 {
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> ImVec4 {
        ImVec4::new(x, y, z, w)
    }
}

impl Into<[f32; 4]> for ImVec4 {
    fn into(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl Into<(f32, f32, f32, f32)> for ImVec4 {
    fn into(self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
    }
}

extern "C" {
    pub fn igStyleColorsDark(dst: *mut ImGuiStyle);
    pub fn igStyleColorsClassic(dst: *mut ImGuiStyle);
    pub fn igStyleColorsLight(dst: *mut ImGuiStyle);
}

// Main
extern "C" {
    pub fn igGetIO() -> *mut ImGuiIO;
    pub fn igGetStyle() -> *mut ImGuiStyle;
    pub fn igGetDrawData() -> *mut ImDrawData;
    pub fn igNewFrame();
    pub fn igRender();
    pub fn igEndFrame();
    pub fn igShutdown();
}

// Demo/Debug/Info
extern "C" {
    pub fn igShowDemoWindow(opened: *mut bool);
    pub fn igShowMetricsWindow(opened: *mut bool);
    pub fn igShowStyleEditor(style: *mut ImGuiStyle);
    pub fn igShowStyleSelector(label: *const c_char);
    pub fn igShowFontSelector(label: *const c_char);
    pub fn igShowUserGuide();
}

// Window
extern "C" {
    pub fn igBegin(name: *const c_char, open: *mut bool, flags: ImGuiWindowFlags) -> bool;
    pub fn igEnd();
    pub fn igBeginChild(
        str_id: *const c_char,
        size: ImVec2,
        border: bool,
        extra_flags: ImGuiWindowFlags,
    ) -> bool;
    pub fn igBeginChildEx(
        id: ImGuiID,
        size: ImVec2,
        border: bool,
        extra_flags: ImGuiWindowFlags,
    ) -> bool;
    pub fn igEndChild();
    pub fn igGetContentRegionMax(out: *mut ImVec2);
    pub fn igGetContentRegionAvail(out: *mut ImVec2);
    pub fn igGetContentRegionAvailWidth() -> c_float;
    pub fn igGetWindowContentRegionMin(out: *mut ImVec2);
    pub fn igGetWindowContentRegionMax(out: *mut ImVec2);
    pub fn igGetWindowContentRegionWidth() -> c_float;
    pub fn igGetWindowDrawList() -> *mut ImDrawList;
    pub fn igGetWindowPos(out: *mut ImVec2);
    pub fn igGetWindowSize(out: *mut ImVec2);
    pub fn igGetWindowWidth() -> c_float;
    pub fn igGetWindowHeight() -> c_float;
    pub fn igIsWindowCollapsed() -> bool;
    pub fn igIsWindowAppearing() -> bool;
    pub fn igSetWindowFontScale(scale: c_float);

    pub fn igSetNextWindowPos(pos: ImVec2, cond: ImGuiCond, pivot: ImVec2);
    pub fn igSetNextWindowSize(size: ImVec2, cond: ImGuiCond);
    pub fn igSetNextWindowConstraints(
        size_min: ImVec2,
        size_max: ImVec2,
        custom_callback: ImGuiSizeCallback,
        custom_callback_data: *mut c_void,
    );
    pub fn igSetNextWindowContentSize(size: ImVec2);
    pub fn igSetNextWindowCollapsed(collapsed: bool, cond: ImGuiCond);
    pub fn igSetNextWindowFocus();
    pub fn igSetWindowPos(pos: ImVec2, cond: ImGuiCond);
    pub fn igSetWindowSize(size: ImVec2, cond: ImGuiCond);
    pub fn igSetWindowCollapsed(collapsed: bool, cond: ImGuiCond);
    pub fn igSetWindowFocus();
    pub fn igSetWindowPosByName(name: *const c_char, pos: ImVec2, cond: ImGuiCond);
    pub fn igSetWindowSize2(name: *const c_char, size: ImVec2, cond: ImGuiCond);
    pub fn igSetWindowCollapsed2(name: *const c_char, collapsed: bool, cond: ImGuiCond);
    pub fn igSetWindowFocus2(name: *const c_char);

    pub fn igGetScrollX() -> c_float;
    pub fn igGetScrollY() -> c_float;
    pub fn igGetScrollMaxX() -> c_float;
    pub fn igGetScrollMaxY() -> c_float;
    pub fn igSetScrollX(scroll_x: c_float);
    pub fn igSetScrollY(scroll_y: c_float);
    pub fn igSetScrollHere(center_y_ratio: c_float);
    pub fn igSetScrollFromPosY(pos_y: c_float, center_y_ratio: c_float);
    pub fn igSetStateStorage(tree: *mut ImGuiStorage);
    pub fn igGetStateStorage() -> *mut ImGuiStorage;
}

// Parameter stack (shared)
extern "C" {
    pub fn igPushFont(font: *mut ImFont);
    pub fn igPopFont();
    pub fn igPushStyleColorU32(idx: ImGuiCol, col: ImU32);
    pub fn igPushStyleColor(idx: ImGuiCol, col: ImVec4);
    pub fn igPopStyleColor(count: c_int);
    pub fn igPushStyleVar(idx: ImGuiStyleVar, val: c_float);
    pub fn igPushStyleVarVec(idx: ImGuiStyleVar, val: ImVec2);
    pub fn igPopStyleVar(count: c_int);
    pub fn igGetStyleColorVec4(out: *mut ImVec4, idx: ImGuiCol);
    pub fn igGetFont() -> *mut ImFont;
    pub fn igGetFontSize() -> c_float;
    pub fn igGetFontTexUvWhitePixel(out: *mut ImVec2);
    pub fn igGetColorU32(idx: ImGuiCol, alpha_mul: c_float) -> ImU32;
    pub fn igGetColorU32Vec(col: *const ImVec4) -> ImU32;
    pub fn igGetColorU32U32(col: ImU32) -> ImU32;
}

// Parameter stack (current window)
extern "C" {
    pub fn igPushItemWidth(item_width: c_float);
    pub fn igPopItemWidth();
    pub fn igCalcItemWidth() -> c_float;
    pub fn igPushTextWrapPos(wrap_pos_x: c_float);
    pub fn igPopTextWrapPos();
    pub fn igPushAllowKeyboardFocus(allow_keyboard_focus: bool);
    pub fn igPopAllowKeyboardFocus();
    pub fn igPushButtonRepeat(repeat: bool);
    pub fn igPopButtonRepeat();
}

// Cursor / Layout
extern "C" {
    pub fn igSeparator();
    pub fn igSameLine(pos_x: c_float, spacing_w: c_float);
    pub fn igNewLine();
    pub fn igSpacing();
    pub fn igDummy(size: *const ImVec2);
    pub fn igIndent(indent_w: c_float);
    pub fn igUnindent(indent_w: c_float);
    pub fn igBeginGroup();
    pub fn igEndGroup();
    pub fn igGetCursorPos(out: *mut ImVec2);
    pub fn igGetCursorPosX() -> c_float;
    pub fn igGetCursorPosY() -> c_float;
    pub fn igSetCursorPos(local_pos: ImVec2);
    pub fn igSetCursorPosX(x: c_float);
    pub fn igSetCursorPosY(y: c_float);
    pub fn igGetCursorStartPos(out: *mut ImVec2);
    pub fn igGetCursorScreenPos(out: *mut ImVec2);
    pub fn igSetCursorScreenPos(pos: ImVec2);
    pub fn igAlignTextToFramePadding();
    pub fn igGetTextLineHeight() -> c_float;
    pub fn igGetTextLineHeightWithSpacing() -> c_float;
    pub fn igGetFrameHeight() -> c_float;
    pub fn igGetFrameHeightWithSpacing() -> c_float;
}

// Columns
extern "C" {
    pub fn igColumns(count: c_int, id: *const c_char, border: bool);
    pub fn igNextColumn();
    pub fn igGetColumnIndex() -> c_int;
    pub fn igGetColumnWidth(column_index: c_int) -> c_float;
    pub fn igSetColumnWidth(column_index: c_int, width: c_float);
    pub fn igGetColumnOffset(column_index: c_int) -> c_float;
    pub fn igSetColumnOffset(column_index: c_int, offset_x: c_float);
    pub fn igGetColumnsCount() -> c_int;
}

// ID scopes
extern "C" {
    pub fn igPushIDStr(str_id: *const c_char);
    pub fn igPushIDStrRange(str_begin: *const c_char, str_end: *const c_char);
    pub fn igPushIDPtr(ptr_id: *const c_void);
    pub fn igPushIDInt(int_id: c_int);
    pub fn igPopID();
    pub fn igGetIDStr(str_id: *const c_char) -> ImGuiID;
    pub fn igGetIDStrRange(str_begin: *const c_char, str_end: *const c_char) -> ImGuiID;
    pub fn igGetIDPtr(ptr_id: *const c_void) -> ImGuiID;
}

// Widgets
extern "C" {
    pub fn igTextUnformatted(text: *const c_char, text_end: *const c_char);
    pub fn igText(fmt: *const c_char, ...);
    // pub fn igTextV(fmt: *const c_char, args: va_list);
    pub fn igTextColored(col: ImVec4, fmt: *const c_char, ...);
    // pub fn igTextColoredV(col: ImVec4, fmt: *const c_char, args: va_list);
    pub fn igTextDisabled(fmt: *const c_char, ...);
    // pub fn igTextDisabledV(fmt: *const c_char, args: va_list);
    pub fn igTextWrapped(fmt: *const c_char, ...);
    // pub fn igTextWrappedV(fmt: *const c_char, args: va_list);
    pub fn igLabelText(label: *const c_char, fmt: *const c_char, ...);
    // pub fn igLabelTextV(label: *const c_char, fmt: *const c_char, args: va_list);
    pub fn igBulletText(fmt: *const c_char, ...);
    // pub fn igBulletTextV(fmt: *const c_char, args: va_list);
    pub fn igBullet();
    pub fn igButton(label: *const c_char, size: ImVec2) -> bool;
    pub fn igSmallButton(label: *const c_char) -> bool;
    pub fn igInvisibleButton(str_id: *const c_char, size: ImVec2) -> bool;
    pub fn igImage(
        user_texture_id: ImTextureID,
        size: ImVec2,
        uv0: ImVec2,
        uv1: ImVec2,
        tint_col: ImVec4,
        border_col: ImVec4,
    );
    pub fn igImageButton(
        user_texture_id: ImTextureID,
        size: ImVec2,
        uv0: ImVec2,
        uv1: ImVec2,
        frame_padding: c_int,
        bg_col: ImVec4,
        tint_col: ImVec4,
    ) -> bool;
    pub fn igCheckbox(label: *const c_char, v: *mut bool) -> bool;
    pub fn igCheckboxFlags(label: *const c_char, flags: *mut c_uint, flags_value: c_uint) -> bool;
    pub fn igRadioButtonBool(label: *const c_char, active: bool) -> bool;
    pub fn igRadioButton(label: *const c_char, v: *mut c_int, v_button: c_int) -> bool;
    pub fn igPlotLines(
        label: *const c_char,
        values: *const c_float,
        values_count: c_int,
        values_offset: c_int,
        overlay_text: *const c_char,
        scale_min: c_float,
        scale_max: c_float,
        graph_size: ImVec2,
        stride: c_int,
    );
    pub fn igPlotLines2(
        label: *const c_char,
        values_getter: extern "C" fn(data: *mut c_void, idx: c_int) -> c_float,
        data: *mut c_void,
        values_count: c_int,
        values_offset: c_int,
        overlay_text: *const c_char,
        scale_min: c_float,
        scale_max: c_float,
        graph_size: ImVec2,
    );
    pub fn igPlotHistogram(
        label: *const c_char,
        values: *const c_float,
        values_count: c_int,
        values_offset: c_int,
        overlay_text: *const c_char,
        scale_min: c_float,
        scale_max: c_float,
        graph_size: ImVec2,
        stride: c_int,
    );
    pub fn igPlotHistogram2(
        label: *const c_char,
        values_getter: extern "C" fn(data: *mut c_void, idx: c_int) -> c_float,
        data: *mut c_void,
        values_count: c_int,
        values_offset: c_int,
        overlay_text: *const c_char,
        scale_min: c_float,
        scale_max: c_float,
        graph_size: ImVec2,
    );
    pub fn igProgressBar(fraction: c_float, size_arg: *const ImVec2, overlay: *const c_char);
}

// Combo
extern "C" {
    pub fn igBeginCombo(
        label: *const c_char,
        preview_value: *const c_char,
        flags: ImGuiComboFlags,
    ) -> bool;
    pub fn igEndCombo();
    pub fn igCombo(
        label: *const c_char,
        current_item: *mut c_int,
        items: *const *const c_char,
        items_count: c_int,
        height_in_items: c_int,
    ) -> bool;
    pub fn igCombo2(
        label: *const c_char,
        current_item: *mut c_int,
        items_separated_by_zeros: *const c_char,
        height_in_items: c_int,
    ) -> bool;
    pub fn igCombo3(
        label: *const c_char,
        current_item: *mut c_int,
        items_getter: extern "C" fn(data: *mut c_void, idx: c_int, out_text: *mut *const c_char)
            -> bool,
        data: *mut c_void,
        items_count: c_int,
        height_in_items: c_int,
    ) -> bool;
}

// Widgets: Color Editor/Picker
extern "C" {
    pub fn igColorEdit3(
        label: *const c_char,
        col: *mut c_float,
        flags: ImGuiColorEditFlags,
    ) -> bool;
    pub fn igColorEdit4(
        label: *const c_char,
        col: *mut c_float,
        flags: ImGuiColorEditFlags,
    ) -> bool;
    pub fn igColorPicker3(
        label: *const c_char,
        col: *mut c_float,
        flags: ImGuiColorEditFlags,
    ) -> bool;
    pub fn igColorPicker4(
        label: *const c_char,
        col: *mut c_float,
        flags: ImGuiColorEditFlags,
        ref_col: *const c_float,
    ) -> bool;
    pub fn igColorButton(
        desc_id: *const c_char,
        col: ImVec4,
        flags: ImGuiColorEditFlags,
        size: ImVec2,
    ) -> bool;
    pub fn igSetColorEditOptions(flags: ImGuiColorEditFlags);
}

// Widgets: Drags
extern "C" {
    pub fn igDragFloat(
        label: *const c_char,
        v: *mut c_float,
        v_speed: c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igDragFloat2(
        label: *const c_char,
        v: *mut c_float,
        v_speed: c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igDragFloat3(
        label: *const c_char,
        v: *mut c_float,
        v_speed: c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igDragFloat4(
        label: *const c_char,
        v: *mut c_float,
        v_speed: c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igDragFloatRange2(
        label: *const c_char,
        v_current_min: *mut c_float,
        v_current_max: *mut c_float,
        v_speed: c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: *const c_char,
        display_format_max: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igDragInt(
        label: *const c_char,
        v: *mut c_int,
        v_speed: c_float,
        v_min: c_int,
        v_max: c_int,
        display_format: *const c_char,
    ) -> bool;
    pub fn igDragInt2(
        label: *const c_char,
        v: *mut c_int,
        v_speed: c_float,
        v_min: c_int,
        v_max: c_int,
        display_format: *const c_char,
    ) -> bool;
    pub fn igDragInt3(
        label: *const c_char,
        v: *mut c_int,
        v_speed: c_float,
        v_min: c_int,
        v_max: c_int,
        display_format: *const c_char,
    ) -> bool;
    pub fn igDragInt4(
        label: *const c_char,
        v: *mut c_int,
        v_speed: c_float,
        v_min: c_int,
        v_max: c_int,
        display_format: *const c_char,
    ) -> bool;
    pub fn igDragIntRange2(
        label: *const c_char,
        v_current_min: *mut c_int,
        v_current_max: *mut c_int,
        v_speed: c_float,
        v_min: c_int,
        v_max: c_int,
        display_format: *const c_char,
        display_format_max: *const c_char,
    ) -> bool;
}

// Widgets: Input with Keyboard
extern "C" {
    pub fn igInputText(
        label: *const c_char,
        buf: *mut c_char,
        buf_size: usize,
        flags: ImGuiInputTextFlags,
        callback: ImGuiInputTextCallback,
        user_data: *mut c_void,
    ) -> bool;
    pub fn igInputTextMultiline(
        label: *const c_char,
        buf: *mut c_char,
        buf_size: usize,
        size: ImVec2,
        flags: ImGuiInputTextFlags,
        callback: ImGuiInputTextCallback,
        user_data: *mut c_void,
    ) -> bool;
    pub fn igInputFloat(
        label: *const c_char,
        v: *mut c_float,
        step: c_float,
        step_fast: c_float,
        decimal_precision: c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputFloat2(
        label: *const c_char,
        v: *mut c_float,
        decimal_precision: c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputFloat3(
        label: *const c_char,
        v: *mut c_float,
        decimal_precision: c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputFloat4(
        label: *const c_char,
        v: *mut c_float,
        decimal_precision: c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputInt(
        label: *const c_char,
        v: *mut c_int,
        step: c_int,
        step_fast: c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputInt2(
        label: *const c_char,
        v: *mut c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputInt3(
        label: *const c_char,
        v: *mut c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputInt4(
        label: *const c_char,
        v: *mut c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
}

// Widgets: Sliders
extern "C" {
    pub fn igSliderFloat(
        label: *const c_char,
        v: *mut c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igSliderFloat2(
        label: *const c_char,
        v: *mut c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igSliderFloat3(
        label: *const c_char,
        v: *mut c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igSliderFloat4(
        label: *const c_char,
        v: *mut c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igSliderAngle(
        label: *const c_char,
        v_rad: *mut c_float,
        v_degrees_min: c_float,
        v_degrees_max: c_float,
    ) -> bool;
    pub fn igSliderInt(
        label: *const c_char,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        display_format: *const c_char,
    ) -> bool;
    pub fn igSliderInt2(
        label: *const c_char,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        display_format: *const c_char,
    ) -> bool;
    pub fn igSliderInt3(
        label: *const c_char,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        display_format: *const c_char,
    ) -> bool;
    pub fn igSliderInt4(
        label: *const c_char,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        display_format: *const c_char,
    ) -> bool;
    pub fn igVSliderFloat(
        label: *const c_char,
        size: ImVec2,
        v: *mut c_float,
        v_min: c_float,
        v_max: c_float,
        display_format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igVSliderInt(
        label: *const c_char,
        size: ImVec2,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        display_format: *const c_char,
    ) -> bool;
}

// Widgets: Trees
extern "C" {
    pub fn igTreeNode(label: *const c_char) -> bool;
    pub fn igTreeNodeStr(str_id: *const c_char, fmt: *const c_char, ...) -> bool;
    pub fn igTreeNodePtr(ptr_id: *const c_void, fmt: *const c_char, ...) -> bool;
    // pub fn igTreeNodeStrV(str_id: *const c_char, fmt: *const c_char, args: va_list) -> bool;
    // pub fn igTreeNodePtrV(ptr_id: *const c_void, fmt: *const c_char, args: va_list) -> bool;
    pub fn igTreeNodeEx(label: *const c_char, flags: ImGuiTreeNodeFlags) -> bool;
    pub fn igTreeNodeExStr(
        str_id: *const c_char,
        flags: ImGuiTreeNodeFlags,
        fmt: *const c_char,
        ...
    ) -> bool;
    pub fn igTreeNodeExPtr(
        ptr_id: *const c_void,
        flags: ImGuiTreeNodeFlags,
        fmt: *const c_char,
        ...
    ) -> bool;
    // pub fn igTreeNodeExV(str_id: *const c_char, flags: ImGuiTreeNodeFlags,
    //                      fmt: *const c_char, args: va_list) -> bool;
    // pub fn igTreeNodeExVPtr(ptr_id: *const c_void, flags: ImGuiTreeNodeFlags,
    //                      fmt: *const c_char, args: va_list) -> bool;
    pub fn igTreePushStr(str_id: *const c_char);
    pub fn igTreePushPtr(ptr_id: *const c_void);
    pub fn igTreePop();
    pub fn igTreeAdvanceToLabelPos();
    pub fn igGetTreeNodeToLabelSpacing() -> c_float;
    pub fn igSetNextTreeNodeOpen(opened: bool, cond: ImGuiCond);
    pub fn igCollapsingHeader(label: *const c_char, flags: ImGuiTreeNodeFlags) -> bool;
    pub fn igCollapsingHeaderEx(
        label: *const c_char,
        open: *mut bool,
        flags: ImGuiTreeNodeFlags,
    ) -> bool;
}

// Widgets: Selectable / Lists
extern "C" {
    pub fn igSelectable(
        label: *const c_char,
        selected: bool,
        flags: ImGuiSelectableFlags,
        size: ImVec2,
    ) -> bool;
    pub fn igSelectableEx(
        label: *const c_char,
        p_selected: *mut bool,
        flags: ImGuiSelectableFlags,
        size: ImVec2,
    ) -> bool;
    pub fn igListBox(
        label: *const c_char,
        current_item: *mut c_int,
        items: *const *const c_char,
        items_count: c_int,
        height_in_items: c_int,
    ) -> bool;
    pub fn igListBox2(
        label: *const c_char,
        current_item: *mut c_int,
        items_getter: extern "C" fn(data: *mut c_void, idx: c_int, out_text: *mut *const c_char)
            -> bool,
        data: *mut c_void,
        items_count: c_int,
        height_in_items: c_int,
    ) -> bool;
    pub fn igListBoxHeader(label: *const c_char, size: ImVec2) -> bool;
    pub fn igListBoxHeader2(
        label: *const c_char,
        items_count: c_int,
        height_in_items: c_int,
    ) -> bool;
    pub fn igListBoxFooter();
}

// Widgets: Value() Helpers
extern "C" {
    pub fn igValueBool(prefix: *const c_char, b: bool);
    pub fn igValueInt(prefix: *const c_char, v: c_int);
    pub fn igValueUInt(prefix: *const c_char, v: c_uint);
    pub fn igValueFloat(prefix: *const c_char, v: c_float, float_format: *const c_char);
}

// Tooltip
extern "C" {
    pub fn igSetTooltip(fmt: *const c_char, ...);
    // pub fn igSetTooltipV(fmt: *const c_char, args: va_list);
    pub fn igBeginTooltip();
    pub fn igEndTooltip();
}

// Widgets: Menus
extern "C" {
    pub fn igBeginMainMenuBar() -> bool;
    pub fn igEndMainMenuBar();
    pub fn igBeginMenuBar() -> bool;
    pub fn igEndMenuBar();
    pub fn igBeginMenu(label: *const c_char, enabled: bool) -> bool;
    pub fn igEndMenu();
    pub fn igMenuItem(
        label: *const c_char,
        shortcut: *const c_char,
        selected: bool,
        enabled: bool,
    ) -> bool;
    pub fn igMenuItemPtr(
        label: *const c_char,
        shortcut: *const c_char,
        p_selected: *mut bool,
        enabled: bool,
    ) -> bool;
}

// Popup
extern "C" {
    pub fn igOpenPopup(str_id: *const c_char);
    pub fn igOpenPopupOnItemClick(str_id: *const c_char, mouse_button: c_int) -> bool;
    pub fn igBeginPopup(str_id: *const c_char) -> bool;
    pub fn igBeginPopupModal(
        name: *const c_char,
        open: *mut bool,
        extra_flags: ImGuiWindowFlags,
    ) -> bool;
    pub fn igBeginPopupContextItem(str_id: *const c_char, mouse_button: c_int) -> bool;
    pub fn igBeginPopupContextWindow(
        str_id: *const c_char,
        mouse_button: c_int,
        also_over_items: bool,
    ) -> bool;
    pub fn igBeginPopupContextVoid(str_id: *const c_char, mouse_button: c_int) -> bool;
    pub fn igEndPopup();
    pub fn igIsPopupOpen(str_id: *const c_char) -> bool;
    pub fn igCloseCurrentPopup();
}

// Logging
extern "C" {
    pub fn igLogToTTY(max_depth: c_int);
    pub fn igLogToFile(max_depth: c_int, filename: *const c_char);
    pub fn igLogToClipboard(max_depth: c_int);
    pub fn igLogFinish();
    pub fn igLogButtons();
    pub fn igLogText(fmt: *const c_char, ...);
}

// DragDrop
extern "C" {
    /// Call when current ID is active.
    ///
    /// When this returns true you need to:
    ///
    /// 1. call [`igSetDragDropPayload`] exactly once,
    /// 2. you may render the payload visual/description,
    /// 3. pcall [`igEndDragDropSource`]
    pub fn igBeginDragDropSource(flags: ImGuiDragDropFlags, mouse_button: c_int) -> bool;
    /// Use 'cond' to choose to submit payload on drag start or every frame
    pub fn igSetDragDropPayload(
        type_: *const c_char,
        data: *const c_void,
        size: libc::size_t,
        cond: ImGuiCond,
    ) -> bool;
    pub fn igEndDragDropSource();
    pub fn igBeginDragDropTarget() -> bool;
    pub fn igAcceptDragDropPayload(
        type_: *const c_char,
        flags: ImGuiDragDropFlags,
    ) -> *const ImGuiPayload;
    pub fn igEndDragDropTarget();
}

// Clipping
extern "C" {
    pub fn igPushClipRect(
        clip_rect_min: ImVec2,
        clip_rect_max: ImVec2,
        intersect_with_current_clip_rect: bool,
    );
    pub fn igPopClipRect();
}

// Focus
extern "C" {
    pub fn igSetItemDefaultFocus();
    pub fn igSetKeyboardFocusHere(offset: c_int);
}

// Utilities
extern "C" {
    pub fn igIsItemHovered(flags: ImGuiHoveredFlags) -> bool;
    pub fn igIsItemActive() -> bool;
    pub fn igIsItemClicked(mouse_button: c_int) -> bool;
    pub fn igIsItemVisible() -> bool;
    pub fn igIsAnyItemHovered() -> bool;
    pub fn igIsAnyItemActive() -> bool;
    pub fn igGetItemRectMin(out: *mut ImVec2);
    pub fn igGetItemRectMax(out: *mut ImVec2);
    pub fn igGetItemRectSize(out: *mut ImVec2);
    pub fn igSetItemAllowOverlap();
    pub fn igIsWindowFocused(flags: ImGuiFocusedFlags) -> bool;
    pub fn igIsWindowHovered(flags: ImGuiHoveredFlags) -> bool;
    pub fn igIsAnyWindowHovered() -> bool;
    pub fn igIsRectVisible(item_size: ImVec2) -> bool;
    pub fn igIsRectVisible2(rect_min: *const ImVec2, rect_max: *const ImVec2) -> bool;
    pub fn igGetTime() -> c_float;
    pub fn igGetFrameCount() -> c_int;
    pub fn igGetStyleColorName(idx: ImGuiCol) -> *const c_char;
    pub fn igCalcItemRectClosestPoint(
        out: *mut ImVec2,
        pos: ImVec2,
        on_edge: bool,
        outward: c_float,
    );
    pub fn igCalcTextSize(
        out: *mut ImVec2,
        text: *const c_char,
        text_end: *const c_char,
        hide_text_after_double_hash: bool,
        wrap_width: c_float,
    );
    pub fn igCalcListClipping(
        items_count: c_int,
        items_height: c_float,
        out_items_display_start: *mut c_int,
        out_items_display_end: *mut c_int,
    );

    pub fn igBeginChildFrame(id: ImGuiID, size: ImVec2, extra_flags: ImGuiWindowFlags) -> bool;
    pub fn igEndChildFrame();

    pub fn igColorConvertU32ToFloat4(out: *mut ImVec4, color: ImU32);
    pub fn igColorConvertFloat4ToU32(color: ImVec4) -> ImU32;
    pub fn igColorConvertRGBtoHSV(
        r: c_float,
        g: c_float,
        b: c_float,
        out_h: *mut c_float,
        out_s: *mut c_float,
        out_v: *mut c_float,
    );
    pub fn igColorConvertHSVtoRGB(
        h: c_float,
        s: c_float,
        v: c_float,
        out_r: *mut c_float,
        out_g: *mut c_float,
        out_b: *mut c_float,
    );
}

// DrawList
extern "C" {
    pub fn igGetOverlayDrawList() -> *mut ImDrawList;
    pub fn igGetDrawListSharedData() -> *mut ImDrawListSharedData;
}

// Inputs
extern "C" {
    pub fn igGetKeyIndex(imgui_key: ImGuiKey) -> c_int;
    pub fn igIsKeyDown(user_key_index: c_int) -> bool;
    pub fn igIsKeyPressed(user_key_index: c_int, repeat: bool) -> bool;
    pub fn igIsKeyReleased(user_key_index: c_int) -> bool;
    pub fn igGetKeyPressedAmount(key_index: c_int, repeat_delay: c_float, rate: c_float) -> c_int;
    pub fn igIsMouseDown(button: c_int) -> bool;
    pub fn igIsMouseClicked(button: c_int, repeat: bool) -> bool;
    pub fn igIsMouseDoubleClicked(button: c_int) -> bool;
    pub fn igIsMouseReleased(button: c_int) -> bool;
    pub fn igIsMouseDragging(button: c_int, lock_threshold: c_float) -> bool;
    pub fn igIsMouseHoveringRect(r_min: ImVec2, r_max: ImVec2, clip: bool) -> bool;
    pub fn igIsMousePosValid(mouse_pos: *const ImVec2) -> bool;
    pub fn igGetMousePos(out: *mut ImVec2);
    pub fn igGetMousePosOnOpeningCurrentPopup(out: *mut ImVec2);
    pub fn igGetMouseDragDelta(out: *mut ImVec2, button: c_int, lock_threshold: c_float);
    pub fn igResetMouseDragDelta(button: c_int);
    pub fn igGetMouseCursor() -> ImGuiMouseCursor;
    pub fn igSetMouseCursor(cursor: ImGuiMouseCursor);
    pub fn igCaptureKeyboardFromApp(capture: bool);
    pub fn igCaptureMouseFromApp(capture: bool);
}

// Helpers functions to access functions pointers in ImGui::GetIO()
extern "C" {
    pub fn igMemAlloc(sz: usize) -> *mut c_void;
    pub fn igMemFree(ptr: *mut c_void);
    pub fn igGetClipboardText() -> *const c_char;
    pub fn igSetClipboardText(text: *const c_char);
}

// Internal state access
extern "C" {
    pub fn igGetVersion() -> *const c_char;
    pub fn igCreateContext(
        malloc_fn: Option<extern "C" fn(size: usize) -> *mut c_void>,
        free_fn: Option<extern "C" fn(ptr: *mut c_void)>,
    ) -> *mut ImGuiContext;
    pub fn igDestroyContext(ctx: *mut ImGuiContext);
    pub fn igGetCurrentContext() -> *mut ImGuiContext;
    pub fn igSetCurrentContext(ctx: *mut ImGuiContext);
}

extern "C" {
    pub fn ImFontConfig_DefaultConstructor(config: *mut ImFontConfig);
}

// ImGuiIO
extern "C" {
    pub fn ImGuiIO_AddInputCharacter(c: c_ushort);
    pub fn ImGuiIO_AddInputCharactersUTF8(utf8_chars: *const c_char);
    pub fn ImGuiIO_ClearInputCharacters();
}

// ImGuiTextFilter
extern "C" {
    pub fn ImGuiTextFilter_Create(default_filter: *const c_char) -> *mut ImGuiTextFilter;
    pub fn ImGuiTextFilter_Destroy(filter: *mut ImGuiTextFilter);
    pub fn ImGuiTextFilter_Clear(filter: *mut ImGuiTextFilter);
    pub fn ImGuiTextFilter_Draw(
        filter: *mut ImGuiTextFilter,
        label: *const c_char,
        width: c_float,
    ) -> bool;
    pub fn ImGuiTextFilter_PassFilter(
        filter: *const ImGuiTextFilter,
        text: *const c_char,
        text_end: *const c_char,
    ) -> bool;
    pub fn ImGuiTextFilter_IsActive(filter: *const ImGuiTextFilter) -> bool;
    pub fn ImGuiTextFilter_Build(filter: *const ImGuiTextFilter);
    pub fn ImGuiTextFilter_GetInputBuf(filter: *mut ImGuiTextFilter) -> *const c_char;
}

// ImGuiTextBuffer
extern "C" {
    pub fn ImGuiTextBuffer_Create() -> *mut ImGuiTextBuffer;
    pub fn ImGuiTextBuffer_Destroy(buffer: *mut ImGuiTextBuffer);
    pub fn ImGuiTextBuffer_index(buffer: *mut ImGuiTextBuffer, i: c_int) -> c_char;
    pub fn ImGuiTextBuffer_begin(buffer: *const ImGuiTextBuffer) -> *const c_char;
    pub fn ImGuiTextBuffer_end(buffer: *const ImGuiTextBuffer) -> *const c_char;
    pub fn ImGuiTextBuffer_size(buffer: *const ImGuiTextBuffer) -> c_int;
    pub fn ImGuiTextBuffer_empty(buffer: *mut ImGuiTextBuffer) -> bool;
    pub fn ImGuiTextBuffer_clear(buffer: *mut ImGuiTextBuffer);
    pub fn ImGuiTextBuffer_c_str(buffer: *const ImGuiTextBuffer) -> *const c_char;
    pub fn ImGuiTextBuffer_appendf(buffer: *const ImGuiTextBuffer, fmt: *const c_char, ...);
// pub fn ImGuiTextBuffer_appendv(
//     buffer: *const ImGuiTextBuffer,
//     fmt: *const c_char,
//     args: va_list
// );
}

// ImGuiStorage
extern "C" {
    pub fn ImGuiStorage_Create() -> *mut ImGuiStorage;
    pub fn ImGuiStorage_Destroy(storage: *mut ImGuiStorage);
    pub fn ImGuiStorage_GetInt(
        storage: *mut ImGuiStorage,
        key: ImGuiID,
        default_val: c_int,
    ) -> c_int;
    pub fn ImGuiStorage_SetInt(storage: *mut ImGuiStorage, key: ImGuiID, val: c_int);
    pub fn ImGuiStorage_GetBool(
        storage: *mut ImGuiStorage,
        key: ImGuiID,
        default_val: bool,
    ) -> bool;
    pub fn ImGuiStorage_SetBool(storage: *mut ImGuiStorage, key: ImGuiID, val: bool);
    pub fn ImGuiStorage_GetFloat(
        storage: *mut ImGuiStorage,
        key: ImGuiID,
        default_val: c_float,
    ) -> c_float;
    pub fn ImGuiStorage_SetFloat(storage: *mut ImGuiStorage, key: ImGuiID, val: c_float);
    pub fn ImGuiStorage_GetVoidPtr(storage: *mut ImGuiStorage, key: ImGuiID);
    pub fn ImGuiStorage_SetVoidPtr(storage: *mut ImGuiStorage, key: ImGuiID, val: *mut c_void);
    pub fn ImGuiStorage_GetIntRef(
        storage: *mut ImGuiStorage,
        key: ImGuiID,
        default_val: c_int,
    ) -> *mut c_int;
    pub fn ImGuiStorage_GetBoolRef(
        storage: *mut ImGuiStorage,
        key: ImGuiID,
        default_val: bool,
    ) -> *mut bool;
    pub fn ImGuiStorage_GetFloatRef(
        storage: *mut ImGuiStorage,
        key: ImGuiID,
        default_val: c_float,
    ) -> *mut c_float;
    pub fn ImGuiStorage_GetVoidPtrRef(
        storage: *mut ImGuiStorage,
        key: ImGuiID,
        default_val: *mut c_void,
    ) -> *mut *mut c_void;
    pub fn ImGuiStorage_SetAllInt(storage: *mut ImGuiStorage, val: c_int);
}

// ImGuiTextEditCallbackData
extern "C" {
    pub fn ImGuiTextEditCallbackData_DeleteChars(
        data: *mut ImGuiInputTextCallbackData,
        pos: c_int,
        bytes_count: c_int,
    );
    pub fn ImGuiTextEditCallbackData_InsertChars(
        data: *mut ImGuiInputTextCallbackData,
        pos: c_int,
        text: *const c_char,
        text_end: *const c_char,
    );
    pub fn ImGuiTextEditCallbackData_HasSelection(data: *mut ImGuiInputTextCallbackData) -> bool;
}

// ImGuiListClipper
extern "C" {
    pub fn ImGuiListClipper_Step(clipper: *mut ImGuiListClipper) -> bool;
    pub fn ImGuiListClipper_Begin(
        clipper: *mut ImGuiListClipper,
        count: c_int,
        items_height: c_float,
    );
    pub fn ImGuiListClipper_End(clipper: *mut ImGuiListClipper);
    pub fn ImGuiListClipper_GetDisplayStart(clipper: *mut ImGuiListClipper) -> c_int;
    pub fn ImGuiListClipper_GetDisplayEnd(clipper: *mut ImGuiListClipper) -> c_int;
}

// ImDrawList
extern "C" {
    pub fn ImDrawList_GetVertexBufferSize(list: *mut ImDrawList) -> c_int;
    pub fn ImDrawList_GetVertexPtr(list: *mut ImDrawList, n: c_int) -> *mut ImDrawVert;
    pub fn ImDrawList_GetIndexBufferSize(list: *mut ImDrawList) -> c_int;
    pub fn ImDrawList_GetIndexPtr(list: *mut ImDrawList, n: c_int) -> *mut ImDrawIdx;
    pub fn ImDrawList_GetCmdSize(list: *mut ImDrawList) -> c_int;
    pub fn ImDrawList_GetCmdPtr(list: *mut ImDrawList, n: c_int) -> *mut ImDrawCmd;

    pub fn ImDrawList_Clear(list: *mut ImDrawList);
    pub fn ImDrawList_ClearFreeMemory(list: *mut ImDrawList);
    pub fn ImDrawList_PushClipRect(
        list: *mut ImDrawList,
        clip_rect_min: ImVec2,
        clip_rect_max: ImVec2,
        intersect_with_current_: bool,
    );
    pub fn ImDrawList_PushClipRectFullScreen(list: *mut ImDrawList);
    pub fn ImDrawList_PopClipRect(list: *mut ImDrawList);
    pub fn ImDrawList_PushTextureID(list: *mut ImDrawList, texture_id: ImTextureID);
    pub fn ImDrawList_PopTextureID(list: *mut ImDrawList);
    pub fn ImDrawList_GetClipRectMin(out: *mut ImVec2, list: *mut ImDrawList);
    pub fn ImDrawList_GetClipRectMax(out: *mut ImVec2, list: *mut ImDrawList);

    pub fn ImDrawList_AddLine(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        col: ImU32,
        thickness: c_float,
    );
    pub fn ImDrawList_AddRect(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        col: ImU32,
        rounding: c_float,
        rounding_corners_flags: ImDrawCornerFlags,
        thickness: c_float,
    );
    pub fn ImDrawList_AddRectFilled(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        col: ImU32,
        rounding: c_float,
        rounding_corners_flags: ImDrawCornerFlags,
    );
    pub fn ImDrawList_AddRectFilledMultiColor(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        col_upr_left: ImU32,
        col_upr_right: ImU32,
        col_bot_right: ImU32,
        col_bot_left: ImU32,
    );
    pub fn ImDrawList_AddQuad(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        d: ImVec2,
        col: ImU32,
        thickness: c_float,
    );
    pub fn ImDrawList_AddQuadFilled(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        d: ImVec2,
        col: ImU32,
    );
    pub fn ImDrawList_AddTriangle(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        col: ImU32,
        thickness: c_float,
    );
    pub fn ImDrawList_AddTriangleFilled(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        col: ImU32,
    );
    pub fn ImDrawList_AddCircle(
        list: *mut ImDrawList,
        centre: ImVec2,
        radius: c_float,
        col: ImU32,
        num_segments: c_int,
        thickness: c_float,
    );
    pub fn ImDrawList_AddCircleFilled(
        list: *mut ImDrawList,
        centre: ImVec2,
        radius: c_float,
        col: ImU32,
        num_segments: c_int,
    );
    pub fn ImDrawList_AddText(
        list: *mut ImDrawList,
        pos: ImVec2,
        col: ImU32,
        text_begin: *const c_char,
        text_end: *const c_char,
    );
    pub fn ImDrawList_AddTextExt(
        list: *mut ImDrawList,
        font: *const ImFont,
        font_size: c_float,
        pos: ImVec2,
        col: ImU32,
        text_begin: *const c_char,
        text_end: *const c_char,
        wrap_width: c_float,
        cpu_fine_clip_rect: *const ImVec4,
    );
    pub fn ImDrawList_AddImage(
        list: *mut ImDrawList,
        user_texture_id: ImTextureID,
        a: ImVec2,
        b: ImVec2,
        uv_a: ImVec2,
        uv_b: ImVec2,
        col: ImU32,
    );
    pub fn ImDrawList_AddImageQuad(
        list: *mut ImDrawList,
        user_texture_id: ImTextureID,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        d: ImVec2,
        uv_a: ImVec2,
        uv_b: ImVec2,
        uv_c: ImVec2,
        uv_d: ImVec2,
        col: ImU32,
    );
    pub fn ImDrawList_AddImageRounded(
        list: *mut ImDrawList,
        user_texture_id: ImTextureID,
        a: ImVec2,
        b: ImVec2,
        uv_a: ImVec2,
        uv_b: ImVec2,
        col: ImU32,
        rounding: c_float,
        rounding_corners: c_int,
    );
    pub fn ImDrawList_AddPolyLine(
        list: *mut ImDrawList,
        points: *const ImVec2,
        num_points: c_int,
        col: ImU32,
        closed: bool,
        thickness: c_float,
    );
    pub fn ImDrawList_AddConvexPolyFilled(
        list: *mut ImDrawList,
        points: *const ImVec2,
        num_points: c_int,
        col: ImU32,
    );
    pub fn ImDrawList_AddBezierCurve(
        list: *mut ImDrawList,
        pos0: ImVec2,
        cp0: ImVec2,
        cp1: ImVec2,
        pos1: ImVec2,
        col: ImU32,
        thickness: c_float,
        num_segments: c_int,
    );

    pub fn ImDrawList_PathClear(list: *mut ImDrawList);
    pub fn ImDrawList_PathLineTo(list: *mut ImDrawList, pos: ImVec2);
    pub fn ImDrawList_PathLineToMergeDuplicate(list: *mut ImDrawList, pos: ImVec2);
    pub fn ImDrawList_PathFillConvex(list: *mut ImDrawList, col: ImU32);
    pub fn ImDrawList_PathStroke(
        list: *mut ImDrawList,
        col: ImU32,
        closed: bool,
        thickness: c_float,
    );
    pub fn ImDrawList_PathArcTo(
        list: *mut ImDrawList,
        centre: ImVec2,
        radius: c_float,
        a_min: c_float,
        a_max: c_float,
        num_segments: c_int,
    );
    pub fn ImDrawList_PathArcToFast(
        list: *mut ImDrawList,
        centre: ImVec2,
        radius: c_float,
        a_min_of_12: c_int,
        a_max_of_12: c_int,
    );
    pub fn ImDrawList_PathBezierCurveTo(
        list: *mut ImDrawList,
        p1: ImVec2,
        p2: ImVec2,
        p3: ImVec2,
        num_segments: c_int,
    );
    pub fn ImDrawList_PathRect(
        list: *mut ImDrawList,
        rect_min: ImVec2,
        rect_max: ImVec2,
        rounding: c_float,
        rounding_corners_flags: c_int,
    );

    pub fn ImDrawList_ChannelsSplit(list: *mut ImDrawList, channels_count: c_int);
    pub fn ImDrawList_ChannelsMerge(list: *mut ImDrawList);
    pub fn ImDrawList_ChannelsSetCurrent(list: *mut ImDrawList, channel_index: c_int);

    pub fn ImDrawList_AddCallback(
        list: *mut ImDrawList,
        callback: ImDrawCallback,
        callback_data: *mut c_void,
    );
    pub fn ImDrawList_AddDrawCmd(list: *mut ImDrawList);

    pub fn ImDrawList_PrimReserve(list: *mut ImDrawList, idx_count: c_int, vtx_count: c_int);
    pub fn ImDrawList_PrimRect(list: *mut ImDrawList, a: ImVec2, b: ImVec2, col: ImU32);
    pub fn ImDrawList_PrimRectUV(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        uv_a: ImVec2,
        uv_b: ImVec2,
        col: ImU32,
    );
    pub fn ImDrawList_PrimQuadUV(
        list: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        d: ImVec2,
        uv_a: ImVec2,
        uv_b: ImVec2,
        uv_c: ImVec2,
        uv_d: ImVec2,
        col: ImU32,
    );
    pub fn ImDrawList_PrimWriteVtx(list: *mut ImDrawList, pos: ImVec2, uv: ImVec2, col: ImU32);
    pub fn ImDrawList_PrimWriteIdx(list: *mut ImDrawList, idx: ImDrawIdx);
    pub fn ImDrawList_PrimVtx(list: *mut ImDrawList, pos: ImVec2, uv: ImVec2, col: ImU32);
    pub fn ImDrawList_UpdateClipRect(list: *mut ImDrawList);
    pub fn ImDrawList_UpdateTextureID(list: *mut ImDrawList);
}

// ImDrawData
extern "C" {
    pub fn ImDrawData_DeIndexAllBuffers(drawData: *mut ImDrawData);
    pub fn ImDrawData_ScaleClipRects(drawData: *mut ImDrawData, sc: ImVec2);
}

extern "C" {
    pub fn ImFontAtlas_GetTexDataAsRGBA32(
        atlas: *mut ImFontAtlas,
        out_pixels: *mut *mut c_uchar,
        out_width: *mut c_int,
        out_height: *mut c_int,
        out_bytes_per_pixel: *mut c_int,
    );
    pub fn ImFontAtlas_GetTexDataAsAlpha8(
        atlas: *mut ImFontAtlas,
        out_pixels: *mut *mut c_uchar,
        out_width: *mut c_int,
        out_height: *mut c_int,
        out_bytes_per_pixel: *mut c_int,
    );
    pub fn ImFontAtlas_SetTexID(atlas: *mut ImFontAtlas, tex: ImTextureID);
    pub fn ImFontAtlas_AddFont(
        atlas: *mut ImFontAtlas,
        font_cfg: *const ImFontConfig,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontDefault(
        atlas: *mut ImFontAtlas,
        font_cfg: *const ImFontConfig,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontFromFileTTF(
        atlas: *mut ImFontAtlas,
        filename: *const c_char,
        size_pixels: c_float,
        font_cfg: *const ImFontConfig,
        glyph_ranges: *const ImWchar,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontFromMemoryTTF(
        atlas: *mut ImFontAtlas,
        font_data: *mut c_void,
        font_size: c_int,
        size_pixels: c_float,
        font_cfg: *const ImFontConfig,
        glyph_ranges: *const ImWchar,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontFromMemoryCompressedTTF(
        atlas: *mut ImFontAtlas,
        compressed_font_data: *const c_void,
        compressed_font_size: c_int,
        size_pixels: c_float,
        font_cfg: *const ImFontConfig,
        glyph_ranges: *const ImWchar,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontFromMemoryCompressedBase85TTF(
        atlas: *mut ImFontAtlas,
        compressed_font_data_base85: *const c_char,
        size_pixels: c_float,
        font_cfg: *const ImFontConfig,
        glyph_ranges: *const ImWchar,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_ClearTexData(atlas: *mut ImFontAtlas);
    pub fn ImFontAtlas_Clear(atlas: *mut ImFontAtlas);
    pub fn ImFontAtlas_GetGlyphRangesDefault(atlas: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesKorean(atlas: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesJapanese(atlas: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesChinese(atlas: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesCyrillic(atlas: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesThai(atlas: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetTexID(atlas: *mut ImFontAtlas) -> ImTextureID;
    pub fn ImFontAtlas_GetTexPixelsAlpha8(atlas: *mut ImFontAtlas) -> *mut c_uchar;
    pub fn ImFontAtlas_GetTexPixelsRGBA32(altas: *mut ImFontAtlas) -> *mut c_uint;
    pub fn ImFontAtlas_GetTexWidth(atlas: *mut ImFontAtlas) -> c_int;
    pub fn ImFontAtlas_GetTexHeight(atlas: *mut ImFontAtlas) -> c_int;
    pub fn ImFontAtlas_GetTexDesiredWidth(atlas: *mut ImFontAtlas) -> c_int;
    pub fn ImFontAtlas_SetTexDesiredWidth(atlas: *mut ImFontAtlas, TexDesiredWidth_: c_int);
    pub fn ImFontAtlas_GetTexGlyphPadding(atlas: *mut ImFontAtlas) -> c_int;
    pub fn ImFontAtlas_SetTexGlyphPadding(atlas: *mut ImFontAtlas, TexGlyphPadding_: c_int);
    pub fn ImFontAtlas_GetTexUvWhitePixel(atlas: *mut ImFontAtlas, out: *mut ImVec2);
}

// ImFontAtlas::Fonts
extern "C" {
    pub fn ImFontAtlas_Fonts_size(atlas: *mut ImFontAtlas) -> c_int;
    pub fn ImFontAtlas_Fonts_index(atlas: *mut ImFontAtlas, index: c_int) -> *mut ImFont;
}

// ImFont
extern "C" {
    pub fn ImFont_GetFontSize(font: *const ImFont) -> c_float;
    pub fn ImFont_SetFontSize(font: *mut ImFont, FontSize_: c_float);
    pub fn ImFont_GetScale(font: *const ImFont) -> c_float;
    pub fn ImFont_SetScale(font: *mut ImFont, Scale_: c_float);
    pub fn ImFont_GetDisplayOffset(font: *const ImFont, out: *mut ImVec2);
    pub fn ImFont_GetFallbackGlyph(font: *const ImFont) -> *const ImFontGlyph;
    pub fn ImFont_SetFallbackGlyph(font: *mut ImFont, FallbackGlyph: *const ImFontGlyph);
    pub fn ImFont_GetFallbackAdvanceX(font: *const ImFont) -> c_float;
    pub fn ImFont_GetFallbackChar(font: *const ImFont) -> ImWchar;
    pub fn ImFont_GetConfigDataCount(font: *const ImFont) -> c_short;
    pub fn ImFont_GetConfigData(font: *mut ImFont) -> *mut ImFontConfig;
    pub fn ImFont_GetContainerAtlas(font: *mut ImFont) -> *mut ImFontAtlas;
    pub fn ImFont_GetAscent(font: *const ImFont) -> c_float;
    pub fn ImFont_GetDescent(font: *const ImFont) -> c_float;
    pub fn ImFont_GetMetricsTotalSurface(font: *const ImFont) -> c_int;
    pub fn ImFont_ClearOutputData(font: *mut ImFont);
    pub fn ImFont_BuildLookupTable(font: *mut ImFont);
    pub fn ImFont_FindGlyph(font: *const ImFont, c: ImWchar) -> *const ImFontGlyph;
    pub fn ImFont_SetFallbackChar(font: *mut ImFont, c: ImWchar);
    pub fn ImFont_GetCharAdvance(font: *const ImFont, c: ImWchar) -> c_float;
    pub fn ImFont_IsLoaded(font: *const ImFont) -> bool;
    pub fn ImFont_GetDebugName(font: *const ImFont) -> *const c_char;
    pub fn ImFont_CalcTextSizeA(
        font: *const ImFont,
        out: *mut ImVec2,
        size: c_float,
        max_width: c_float,
        wrap_width: c_float,
        text_begin: *const c_char,
        text_end: *const c_char,
        remaining: *mut *const c_char,
    );
    pub fn ImFont_CalcWordWrapPositionA(
        font: *const ImFont,
        scale: c_float,
        text: *const c_char,
        text_end: *const c_char,
        wrap_width: c_float,
    ) -> *const c_char;
    pub fn ImFont_RenderChar(
        font: *const ImFont,
        draw_list: *mut ImDrawList,
        size: c_float,
        pos: ImVec2,
        col: ImU32,
        c: c_ushort,
    );
    pub fn ImFont_RenderText(
        font: *const ImFont,
        draw_list: *mut ImDrawList,
        size: c_float,
        pos: ImVec2,
        col: ImU32,
        clip_rect: *const ImVec4,
        text_begin: *const c_char,
        text_end: *const c_char,
        wrap_width: c_float,
        cpu_fine_clip: bool,
    );
}

// ImFont::Glyph
extern "C" {
    pub fn ImFont_Glyphs_size(font: *const ImFont) -> c_int;
    pub fn ImFont_Glyphs_index(font: *mut ImFont, index: c_int) -> *mut ImFontGlyph;
}

// ImFont::IndexXAdvance
extern "C" {
    pub fn ImFont_IndexXAdvance_size(font: *const ImFont) -> c_int;
    pub fn ImFont_IndexXAdvance_index(font: *const ImFont, index: c_int) -> c_float;
}

// ImFont::IndexLookup
extern "C" {
    pub fn ImFont_IndexLookup_size(ofnt: *const ImFont) -> c_int;
    pub fn ImFont_IndexLookup_index(font: *const ImFont, index: c_int) -> c_ushort;
}
