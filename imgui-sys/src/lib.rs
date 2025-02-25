#![no_std]

cfg_if::cfg_if! {
    if #[cfg(feature = "wasm")] {
        cfg_if::cfg_if! {
            if #[cfg(feature = "docking")] {
                cfg_if::cfg_if! {
                    if #[cfg(feature = "freetype")] {
                        mod wasm_docking_freetype_bindings;
                        pub use crate::wasm_docking_freetype_bindings::*;
                    } else {
                        mod wasm_docking_bindings;
                        pub use crate::wasm_docking_bindings::*;
                    }
                }
            } else {
                cfg_if::cfg_if! {
                    if #[cfg(feature = "freetype")] {
                        mod wasm_freetype_bindings;
                        pub use crate::wasm_freetype_bindings::*;
                    } else {
                        mod wasm_bindings;
                        pub use crate::wasm_bindings::*;
                    }
                }
            }
        }
    } else {
        cfg_if::cfg_if! {
            if #[cfg(feature = "docking")] {
                cfg_if::cfg_if! {
                    if #[cfg(feature = "freetype")] {
                        mod docking_freetype_bindings;
                        pub use crate::docking_freetype_bindings::*;
                    } else {
                        mod docking_bindings;
                        pub use crate::docking_bindings::*;
                    }
                }
            } else {
                cfg_if::cfg_if! {
                    if #[cfg(feature = "freetype")] {
                        mod freetype_bindings;
                        pub use crate::freetype_bindings::*;
                    } else {
                        mod bindings;
                        pub use crate::bindings::*;
                    }
                }
            }
        }
    }
}

impl ImVec2 {
    #[inline]
    pub const fn new(x: f32, y: f32) -> ImVec2 {
        ImVec2 { x, y }
    }
    #[inline]
    pub const fn zero() -> ImVec2 {
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

impl From<ImVec2> for [f32; 2] {
    #[inline]
    fn from(v: ImVec2) -> [f32; 2] {
        [v.x, v.y]
    }
}

impl From<ImVec2> for (f32, f32) {
    #[inline]
    fn from(v: ImVec2) -> (f32, f32) {
        (v.x, v.y)
    }
}

impl From<mint::Vector2<f32>> for ImVec2 {
    fn from(o: mint::Vector2<f32>) -> Self {
        Self { x: o.x, y: o.y }
    }
}

impl From<mint::Vector4<f32>> for ImVec4 {
    fn from(o: mint::Vector4<f32>) -> Self {
        Self {
            x: o.x,
            y: o.y,
            z: o.z,
            w: o.w,
        }
    }
}

impl ImVec4 {
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> ImVec4 {
        ImVec4 { x, y, z, w }
    }
    #[inline]
    pub const fn zero() -> ImVec4 {
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

impl From<ImVec4> for [f32; 4] {
    #[inline]
    fn from(v: ImVec4) -> [f32; 4] {
        [v.x, v.y, v.z, v.w]
    }
}

impl From<ImVec4> for (f32, f32, f32, f32) {
    #[inline]
    fn from(v: ImVec4) -> (f32, f32, f32, f32) {
        (v.x, v.y, v.z, v.w)
    }
}

#[test]
fn test_imvec2_memory_layout() {
    use core::mem;
    assert_eq!(mem::size_of::<ImVec2>(), mem::size_of::<[f32; 2]>());
    assert_eq!(mem::align_of::<ImVec2>(), mem::align_of::<[f32; 2]>());
    let test = ImVec2::new(1.0, 2.0);
    let ref_a: &ImVec2 = &test;
    let ref_b: &[f32; 2] = unsafe { &*(&test as *const _ as *const [f32; 2]) };
    assert_eq!(&ref_a.x as *const _, &ref_b[0] as *const _);
    assert_eq!(&ref_a.y as *const _, &ref_b[1] as *const _);
}

#[test]
fn test_imvec4_memory_layout() {
    use core::mem;
    assert_eq!(mem::size_of::<ImVec4>(), mem::size_of::<[f32; 4]>());
    assert_eq!(mem::align_of::<ImVec4>(), mem::align_of::<[f32; 4]>());
    let test = ImVec4::new(1.0, 2.0, 3.0, 4.0);
    let ref_a: &ImVec4 = &test;
    let ref_b: &[f32; 4] = unsafe { &*(&test as *const _ as *const [f32; 4]) };
    assert_eq!(&ref_a.x as *const _, &ref_b[0] as *const _);
    assert_eq!(&ref_a.y as *const _, &ref_b[1] as *const _);
    assert_eq!(&ref_a.z as *const _, &ref_b[2] as *const _);
    assert_eq!(&ref_a.w as *const _, &ref_b[3] as *const _);
}
