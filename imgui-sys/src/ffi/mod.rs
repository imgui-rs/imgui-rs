#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_float;

impl<T> ImVector<T> {
    pub unsafe fn as_slice(&self) -> &[T] {
        ::std::slice::from_raw_parts(self.Data, self.Size as usize)
    }
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
    fn from(array: [f32; 2]) -> ImVec2 { ImVec2::new(array[0], array[1]) }
}

impl From<(f32, f32)> for ImVec2 {
    fn from((x, y): (f32, f32)) -> ImVec2 { ImVec2::new(x, y) }
}

impl Into<[f32; 2]> for ImVec2 {
    fn into(self) -> [f32; 2] { [self.x, self.y] }
}

impl Into<(f32, f32)> for ImVec2 {
    fn into(self) -> (f32, f32) { (self.x, self.y) }
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
    fn from(array: [f32; 4]) -> ImVec4 { ImVec4::new(array[0], array[1], array[2], array[3]) }
}

impl From<(f32, f32, f32, f32)> for ImVec4 {
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> ImVec4 { ImVec4::new(x, y, z, w) }
}

impl Into<[f32; 4]> for ImVec4 {
    fn into(self) -> [f32; 4] { [self.x, self.y, self.z, self.w] }
}

impl Into<(f32, f32, f32, f32)> for ImVec4 {
    fn into(self) -> (f32, f32, f32, f32) { (self.x, self.y, self.z, self.w) }
}

impl ImGuiCond_ {
    pub const None: ImGuiCond_ = ImGuiCond(0);
}

include!(concat!(env!("OUT_DIR"), "/imgui_ffi.rs"));

//pub mod debug_ffi;
//pub use debug_ffi::*;
