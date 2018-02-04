use sys;
use sys::ImVec2;

use std::marker::PhantomData;
use std::os::raw::{c_char, c_float, c_int};

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

    #[inline]
    pub fn add_rect_filled(
        &mut self,
        a: ImVec2,
        b: ImVec2,
        col: u32,
        rounding: f32,
        rounding_corners_flags: i32) {

        unsafe {
            sys::ImDrawList_AddRectFilled(
                self.window_draw_list,
                a,
                b,
                col as sys::ImU32,
                rounding as c_float,
                rounding_corners_flags as c_int);
        }
    }

    #[inline]
    pub fn add_rect_filled_multi_color(
        &mut self,
        a: ImVec2,
        b: ImVec2,
        col_upr_left: u32,
        col_upr_right: u32,
        col_bot_right: u32,
        col_bot_left: u32) {

        unsafe {
            sys::ImDrawList_AddRectFilledMultiColor(
                self.window_draw_list,
                a,
                b,
                col_upr_left as sys::ImU32,
                col_upr_right as sys::ImU32,
                col_bot_right as sys::ImU32,
                col_bot_left as sys::ImU32);
        }
    }

    #[inline]
    pub fn add_quad(
        &mut self,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        d: ImVec2,
        col: u32,
        thickness: f32) {

        unsafe {
            sys::ImDrawList_AddQuad(
                self.window_draw_list,
                a,
                b,
                c,
                d,
                col as sys::ImU32,
                thickness as c_float);
        }
    }

    #[inline]
    pub fn add_quad_filled(
        &mut self,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        d: ImVec2,
        col: u32) {

        unsafe {
            sys::ImDrawList_AddQuadFilled(
                self.window_draw_list,
                a,
                b,
                c,
                d,
                col as sys::ImU32);
        }
    }

    #[inline]
    pub fn add_triangle(
        &mut self,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        col: u32,
        thickness: f32) {

        unsafe {
            sys::ImDrawList_AddTriangle(
                self.window_draw_list,
                a,
                b,
                c,
                col as sys::ImU32,
                thickness as c_float);
        }
    }

    #[inline]
    pub fn add_triangle_filled(
        &mut self,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        col: u32) {

        unsafe {
            sys::ImDrawList_AddTriangleFilled(
                self.window_draw_list,
                a,
                b,
                c,
                col as sys::ImU32);
        }
    }

    #[inline]
    pub fn add_circle(
        &mut self,
        centre: ImVec2,
        radius: f32,
        col: u32,
        num_segments: i32) {

        unsafe {
            sys::ImDrawList_AddCircle(
                self.window_draw_list,
                centre,
                radius as c_float,
                col as sys::ImU32,
                num_segments as c_int);
        }
    }

    #[inline]
    pub fn add_circle_filled(
        &mut self,
        centre: ImVec2,
        radius: f32,
        col: u32,
        num_segments: i32) {

        unsafe {
            sys::ImDrawList_AddCircleFilled(
                self.window_draw_list,
                centre,
                radius as c_float,
                col as sys::ImU32,
                num_segments as c_int);
        }
    }

    #[inline]
    pub fn add_text<'p, T: AsRef<str>>(
        &mut self,
        pos: ImVec2,
        col: u32,
        text: T) {

        let s = text.as_ref();

        unsafe {

            let start = s.as_ptr();
            let end = start.offset(s.len() as isize);

            sys::ImDrawList_AddText(
                self.window_draw_list,
                pos,
                col as sys::ImU32,
                start as *const c_char,
                end as *const c_char);
        }
    }

    #[inline]
    pub fn add_poly_line(
        &mut self,
        points: &[ImVec2],
        col: u32,
        closed: bool,
        thickness: f32,
        anti_aliased: bool) {

        unsafe {
            sys::ImDrawList_AddPolyLine(
                self.window_draw_list,
                points.as_ptr(),
                points.len() as i32,
                col as sys::ImU32,
                closed,
                thickness as c_float,
                anti_aliased);
        }
    }

    #[inline]
    pub fn add_convex_poly_filled(
        &mut self,
        points: &[ImVec2],
        col: u32,
        anti_aliased: bool) {

        unsafe {
            sys::ImDrawList_AddConvexPolyFilled(
                self.window_draw_list,
                points.as_ptr(),
                points.len() as i32,
                col as sys::ImU32,
                anti_aliased);
        }
    }

    #[inline]
    pub fn add_bezier_curve(
        &mut self,
        pos0: ImVec2,
        cp0: ImVec2,
        cp1: ImVec2,
        pos1: ImVec2,
        col: u32,
        thickness: f32,
        num_segments: i32) {

        unsafe {
            sys::ImDrawList_AddBezierCurve(
                self.window_draw_list,
                pos0,
                cp0,
                cp1,
                pos1,
                col as sys::ImU32,
                thickness as c_float,
                num_segments as i32);
        }
    }

} 