use sys;
use sys::ImVec2;

use std::marker::PhantomData;
use std::os::raw::{c_float, c_int};

use Ui;

#[must_use]
pub struct WindowDrawList<'ui> {
    pub(super) window_draw_list: *mut sys::ImDrawList,
    pub(super) _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui> WindowDrawList<'ui> {

    #[inline]
    pub fn add_line(
        &mut self,
        a: ImVec2,
        b: ImVec2, 
        col: u32,
        thickness: f32) {

        unsafe {
            sys::ImDrawList_AddLine(
                self.window_draw_list,
                a,
                b,
                col as sys::ImU32,
                thickness as c_float);
        }
    }

    #[inline]
    pub fn add_rect(
        &mut self,
        a: ImVec2,
        b: ImVec2, 
        col: u32,
        rounding: f32,
        rounding_corners_flags: i32,
        thickness: f32) {

        unsafe {
            sys::ImDrawList_AddRect(
                self.window_draw_list,
                a,
                b,
                col as sys::ImU32,
                rounding as c_float,
                rounding_corners_flags as c_int,
                thickness as c_float);
        }
    }

} 