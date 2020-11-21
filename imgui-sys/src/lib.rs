mod all_bindings;

pub use all_bindings::*;

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
