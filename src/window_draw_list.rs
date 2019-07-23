use std::marker::PhantomData;
use std::slice;
use sys;

use crate::internal::RawWrapper;
use crate::legacy::ImDrawCornerFlags;
use crate::render::draw_data::{DrawCmdIterator, DrawIdx, DrawVert};
use crate::Ui;

#[macro_use]
macro_rules! draw_list_def {
    ($type_name:ident, $static_name:ident, $ui_fn_name:literal, $sys_fn_name:path) => {
        #[doc = "Object implementing the custom draw API.\n"]
        #[doc = "\n"]
        #[doc = "Called from [`"]
        #[doc = $ui_fn_name]
        #[doc ="`]. No more than one instance of this structure can live in a program at the same time.\n"]
        #[doc = "The program will panic on creating a second instance."]
        pub struct $type_name<'ui> {
            draw_list: DrawList<'ui>,
        }

        impl<'ui> std::ops::Deref for $type_name<'ui> {
            type Target = DrawList<'ui>;
            fn deref(&self) -> &Self::Target {
                &self.draw_list
            }
        }

        impl<'ui> std::ops::DerefMut for $type_name<'ui> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.draw_list
            }
        }

        static $static_name: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

        impl<'ui> Drop for $type_name<'ui> {
            fn drop(&mut self) {
                $static_name.store(false, std::sync::atomic::Ordering::SeqCst);
            }
        }

        impl<'ui> $type_name<'ui> {
            pub(crate) fn new(_: &Ui<'ui>) -> Self {
                if $static_name.load(std::sync::atomic::Ordering::SeqCst) {
                    panic!(concat!(
                        stringify!($type_name),
                        " is already loaded! You can only load one instance of it!"
                    ))
                }
                $static_name.store(true, std::sync::atomic::Ordering::SeqCst);

                Self {
                    draw_list: DrawList::new(unsafe { $sys_fn_name() }),
                }
            }
        }
    };
}

draw_list_def!(
    WindowDrawList,
    WINDOW_DRAW_LIST_LOADED,
    "Ui::get_window_draw_list",
    sys::igGetWindowDrawList
);
draw_list_def!(
    ForegroundDrawList,
    FOREGROUND_DRAW_LIST_LOADED,
    "Ui::get_foreground_draw_list",
    sys::igGetForegroundDrawList
);
draw_list_def!(
    BackgroundDrawList,
    BACKGROUND_DRAW_LIST_LOADED,
    "Ui::get_background_draw_list",
    sys::igGetBackgroundDrawList
);

/// Wrap `ImU32` (a type typically used by ImGui to store packed colors)
/// This type is used to represent the color of drawing primitives in ImGui's
/// custom drawing API.
///
/// The type implements `From<ImU32>`, `From<ImVec4>`, `From<[f32; 4]>`,
/// `From<[f32; 3]>`, `From<(f32, f32, f32, f32)>` and `From<(f32, f32, f32)>`
/// for convenience. If alpha is not provided, it is assumed to be 1.0 (255).
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ImColor(sys::ImU32);

impl From<ImColor> for sys::ImU32 {
    fn from(color: ImColor) -> Self {
        color.0
    }
}

impl From<sys::ImU32> for ImColor {
    fn from(color: sys::ImU32) -> Self {
        ImColor(color)
    }
}

impl From<[f32; 4]> for ImColor {
    fn from(v: [f32; 4]) -> Self {
        ImColor(unsafe { sys::igColorConvertFloat4ToU32(v.into()) })
    }
}

impl From<(f32, f32, f32, f32)> for ImColor {
    fn from(v: (f32, f32, f32, f32)) -> Self {
        ImColor(unsafe { sys::igColorConvertFloat4ToU32(v.into()) })
    }
}

impl From<[f32; 3]> for ImColor {
    fn from(v: [f32; 3]) -> Self {
        [v[0], v[1], v[2], 1.0].into()
    }
}

impl From<(f32, f32, f32)> for ImColor {
    fn from(v: (f32, f32, f32)) -> Self {
        [v.0, v.1, v.2, 1.0].into()
    }
}

/// Draw command list
#[repr(transparent)]
pub struct DrawList<'ui> {
    inner: *mut sys::ImDrawList,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl RawWrapper for DrawList<'_> {
    type Raw = sys::ImDrawList;
    unsafe fn raw(&self) -> &Self::Raw {
        &*self.inner
    }
    unsafe fn raw_mut(&mut self) -> &mut Self::Raw {
        &mut *self.inner
    }
}

impl<'ui> DrawList<'ui> {
    pub(crate) unsafe fn cmd_buffer(&self) -> &[sys::ImDrawCmd] {
        slice::from_raw_parts(
            self.raw().CmdBuffer.Data as *const sys::ImDrawCmd,
            self.raw().CmdBuffer.Size as usize,
        )
    }
    pub fn idx_buffer(&self) -> &[DrawIdx] {
        unsafe {
            slice::from_raw_parts(
                self.raw().IdxBuffer.Data as *const DrawIdx,
                self.raw().IdxBuffer.Size as usize,
            )
        }
    }
    pub fn vtx_buffer(&self) -> &[DrawVert] {
        unsafe {
            slice::from_raw_parts(
                self.raw().VtxBuffer.Data as *const DrawVert,
                self.raw().VtxBuffer.Size as usize,
            )
        }
    }
    pub fn commands(&self) -> DrawCmdIterator {
        unsafe {
            DrawCmdIterator {
                iter: self.cmd_buffer().iter(),
            }
        }
    }
}

impl<'ui> DrawList<'ui> {
    pub(crate) fn new(draw_list: *mut sys::ImDrawList) -> Self {
        Self {
            inner: draw_list,
            _phantom: PhantomData,
        }
    }

    /// Split into *channels_count* drawing channels.
    /// At the end of the closure, the channels are merged. The objects
    /// are then drawn in the increasing order of their channel number, and not
    /// in the order they were called.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use imgui::*;
    /// fn custom_drawing(ui: &Ui) {
    ///     let mut draw_list = ui.get_window_draw_list();
    ///     draw_list.channels_split(2, |channels| {
    ///         channels.set_current(1);
    ///         // ... Draw channel 1
    ///         channels.set_current(0);
    ///         // ... Draw channel 0
    ///     });
    /// }
    /// ```
    pub fn channels_split<F: FnOnce(&mut ChannelsSplit)>(&mut self, channels_count: u32, f: F) {
        unsafe { sys::ImDrawList_ChannelsSplit(self.raw_mut(), channels_count as i32) };
        f(&mut ChannelsSplit {
            draw_list: self,
            channels_count,
        });
        unsafe { sys::ImDrawList_ChannelsMerge(self.raw_mut()) };
    }
}

/// Represent the drawing interface within a call to [`channels_split`].
///
/// [`channels_split`]: DrawList::channels_split
pub struct ChannelsSplit<'ui, 'a> {
    draw_list: &'a mut DrawList<'ui>,
    channels_count: u32,
}

impl<'ui, 'a> ChannelsSplit<'ui, 'a> {
    /// Change current channel.
    ///
    /// Panic if channel_index overflows the number of channels.
    pub fn set_current(&mut self, channel_index: u32) {
        assert!(
            channel_index < self.channels_count,
            "Channel cannot be set! Provided channel index ({}) is higher than channel count ({}).",
            channel_index,
            self.channels_count
        );
        unsafe {
            sys::ImDrawList_ChannelsSetCurrent(self.draw_list.raw_mut(), channel_index as i32)
        };
    }
}

/// Drawing functions
impl<'ui> DrawList<'ui> {
    /// Returns a line from point `p1` to `p2` with color `c`.
    pub fn add_line<'a, C>(&'a mut self, p1: [f32; 2], p2: [f32; 2], c: C) -> Line<'ui, 'a>
    where
        C: Into<ImColor>,
    {
        Line::new(self, p1, p2, c)
    }

    /// Returns a rectangle whose upper-left corner is at point `p1`
    /// and lower-right corner is at point `p2`, with color `c`.
    pub fn add_rect<'a, C>(&'a mut self, p1: [f32; 2], p2: [f32; 2], c: C) -> Rect<'ui, 'a>
    where
        C: Into<ImColor>,
    {
        Rect::new(self, p1, p2, c)
    }

    /// Draw a rectangle whose upper-left corner is at point `p1`
    /// and lower-right corner is at point `p2`.
    /// The remains parameters are the respective color of the corners
    /// in the counter-clockwise starting from the upper-left corner
    /// first.
    pub fn add_rect_filled_multicolor<C1, C2, C3, C4>(
        &mut self,
        p1: [f32; 2],
        p2: [f32; 2],
        col_upr_left: C1,
        col_upr_right: C2,
        col_bot_right: C3,
        col_bot_left: C4,
    ) where
        C1: Into<ImColor>,
        C2: Into<ImColor>,
        C3: Into<ImColor>,
        C4: Into<ImColor>,
    {
        unsafe {
            sys::ImDrawList_AddRectFilledMultiColor(
                self.raw_mut(),
                p1.into(),
                p2.into(),
                col_upr_left.into().into(),
                col_upr_right.into().into(),
                col_bot_right.into().into(),
                col_bot_left.into().into(),
            );
        }
    }

    /// Returns a triangle with the given 3 vertices `p1`, `p2` and `p3`
    /// and color `c`.
    pub fn add_triangle<'a, C>(
        &'a mut self,
        p1: [f32; 2],
        p2: [f32; 2],
        p3: [f32; 2],
        c: C,
    ) -> Triangle<'ui, 'a>
    where
        C: Into<ImColor>,
    {
        Triangle::new(self, p1, p2, p3, c)
    }

    /// Returns a circle with the given `center`, `radius` and `color`.
    pub fn add_circle<'a, C>(
        &'a mut self,
        center: [f32; 2],
        radius: f32,
        color: C,
    ) -> Circle<'ui, 'a>
    where
        C: Into<ImColor>,
    {
        Circle::new(self, center, radius, color)
    }

    /// Draw a text whose upper-left corner is at point `pos`.
    pub fn add_text<C, T>(&mut self, pos: [f32; 2], col: C, text: T)
    where
        C: Into<ImColor>,
        T: AsRef<str>,
    {
        use std::os::raw::c_char;

        let text = text.as_ref();
        unsafe {
            let start = text.as_ptr() as *const c_char;
            let end = (start as usize + text.len()) as *const c_char;
            sys::ImDrawList_AddText(self.raw_mut(), pos.into(), col.into().into(), start, end)
        }
    }

    /// Returns a Bezier curve stretching from `pos0` to `pos1`, whose
    /// curvature is defined by `cp0` and `cp1`.
    pub fn add_bezier_curve<'a, C>(
        &'a mut self,
        pos0: [f32; 2],
        cp0: [f32; 2],
        cp1: [f32; 2],
        pos1: [f32; 2],
        color: C,
    ) -> BezierCurve<'ui, 'a>
    where
        C: Into<ImColor>,
    {
        BezierCurve::new(self, pos0, cp0, cp1, pos1, color)
    }

    /// Push a clipping rectangle on the stack, run `f` and pop it.
    ///
    /// Clip all drawings done within the closure `f` in the given
    /// rectangle.
    pub fn with_clip_rect<F>(&mut self, min: [f32; 2], max: [f32; 2], f: F)
    where
        F: FnOnce(),
    {
        unsafe { sys::ImDrawList_PushClipRect(self.raw_mut(), min.into(), max.into(), false) }
        f();
        unsafe { sys::ImDrawList_PopClipRect(self.raw_mut()) }
    }

    /// Push a clipping rectangle on the stack, run `f` and pop it.
    ///
    /// Clip all drawings done within the closure `f` in the given
    /// rectangle. Intersect with all clipping rectangle previously on
    /// the stack.
    pub fn with_clip_rect_intersect<F>(&mut self, min: [f32; 2], max: [f32; 2], f: F)
    where
        F: FnOnce(),
    {
        unsafe { sys::ImDrawList_PushClipRect(self.raw_mut(), min.into(), max.into(), true) }
        f();
        unsafe { sys::ImDrawList_PopClipRect(self.raw_mut()) }
    }
}

/// Represents a line about to be drawn
#[must_use = "should call .build() to draw the object"]
pub struct Line<'ui, 'a> {
    p1: [f32; 2],
    p2: [f32; 2],
    color: ImColor,
    thickness: f32,
    draw_list: &'a mut DrawList<'ui>,
}

impl<'ui, 'a> Line<'ui, 'a> {
    fn new<C>(draw_list: &'a mut DrawList<'ui>, p1: [f32; 2], p2: [f32; 2], c: C) -> Self
    where
        C: Into<ImColor>,
    {
        Self {
            p1,
            p2,
            color: c.into(),
            thickness: 1.0,
            draw_list,
        }
    }

    /// Set line's thickness (default to 1.0 pixel)
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Draw the line on the window
    pub fn build(self) {
        unsafe {
            sys::ImDrawList_AddLine(
                self.draw_list.raw_mut(),
                self.p1.into(),
                self.p2.into(),
                self.color.into(),
                self.thickness,
            )
        }
    }
}

/// Represents a rectangle about to be drawn
#[must_use = "should call .build() to draw the object"]
pub struct Rect<'ui, 'a> {
    p1: [f32; 2],
    p2: [f32; 2],
    color: ImColor,
    rounding: f32,
    flags: ImDrawCornerFlags,
    thickness: f32,
    filled: bool,
    draw_list: &'a mut DrawList<'ui>,
}

impl<'ui, 'a> Rect<'ui, 'a> {
    fn new<C>(draw_list: &'a mut DrawList<'ui>, p1: [f32; 2], p2: [f32; 2], c: C) -> Self
    where
        C: Into<ImColor>,
    {
        Self {
            p1,
            p2,
            color: c.into(),
            rounding: 0.0,
            flags: ImDrawCornerFlags::All,
            thickness: 1.0,
            filled: false,
            draw_list,
        }
    }

    /// Set rectangle's corner rounding (default to 0.0: no rounding).
    /// By default all corners are rounded if this value is set.
    pub fn rounding(mut self, rounding: f32) -> Self {
        self.rounding = rounding;
        self
    }

    /// Set flag to indicate if rectangle's top-left corner will be rounded.
    pub fn round_top_left(mut self, value: bool) -> Self {
        self.flags.set(ImDrawCornerFlags::TopLeft, value);
        self
    }

    /// Set flag to indicate if rectangle's top-right corner will be rounded.
    pub fn round_top_right(mut self, value: bool) -> Self {
        self.flags.set(ImDrawCornerFlags::TopRight, value);
        self
    }

    /// Set flag to indicate if rectangle's bottom-left corner will be rounded.
    pub fn round_bot_left(mut self, value: bool) -> Self {
        self.flags.set(ImDrawCornerFlags::BotLeft, value);
        self
    }

    /// Set flag to indicate if rectangle's bottom-right corner will be rounded.
    pub fn round_bot_right(mut self, value: bool) -> Self {
        self.flags.set(ImDrawCornerFlags::BotRight, value);
        self
    }

    /// Set rectangle's thickness (default to 1.0 pixel).
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set to `true` to make a filled rectangle (default to `false`).
    pub fn filled(mut self, filled: bool) -> Self {
        self.filled = filled;
        self
    }

    /// Draw the rectangle on the window.
    pub fn build(self) {
        if self.filled {
            unsafe {
                sys::ImDrawList_AddRectFilled(
                    self.draw_list.raw_mut(),
                    self.p1.into(),
                    self.p2.into(),
                    self.color.into(),
                    self.rounding,
                    self.flags.bits(),
                );
            }
        } else {
            unsafe {
                sys::ImDrawList_AddRect(
                    self.draw_list.raw_mut(),
                    self.p1.into(),
                    self.p2.into(),
                    self.color.into(),
                    self.rounding,
                    self.flags.bits(),
                    self.thickness,
                );
            }
        }
    }
}

/// Represents a triangle about to be drawn on the window
#[must_use = "should call .build() to draw the object"]
pub struct Triangle<'ui, 'a> {
    p1: [f32; 2],
    p2: [f32; 2],
    p3: [f32; 2],
    color: ImColor,
    thickness: f32,
    filled: bool,
    draw_list: &'a mut DrawList<'ui>,
}

impl<'ui, 'a> Triangle<'ui, 'a> {
    fn new<C>(
        draw_list: &'a mut DrawList<'ui>,
        p1: [f32; 2],
        p2: [f32; 2],
        p3: [f32; 2],
        c: C,
    ) -> Self
    where
        C: Into<ImColor>,
    {
        Self {
            p1,
            p2,
            p3,
            color: c.into(),
            thickness: 1.0,
            filled: false,
            draw_list,
        }
    }

    /// Set triangle's thickness (default to 1.0 pixel)
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set to `true` to make a filled triangle (default to `false`).
    pub fn filled(mut self, filled: bool) -> Self {
        self.filled = filled;
        self
    }

    /// Draw the triangle on the window.
    pub fn build(self) {
        if self.filled {
            unsafe {
                sys::ImDrawList_AddTriangleFilled(
                    self.draw_list.raw_mut(),
                    self.p1.into(),
                    self.p2.into(),
                    self.p3.into(),
                    self.color.into(),
                )
            }
        } else {
            unsafe {
                sys::ImDrawList_AddTriangle(
                    self.draw_list.raw_mut(),
                    self.p1.into(),
                    self.p2.into(),
                    self.p3.into(),
                    self.color.into(),
                    self.thickness,
                )
            }
        }
    }
}

/// Represents a circle about to be drawn
#[must_use = "should call .build() to draw the object"]
pub struct Circle<'ui, 'a> {
    center: [f32; 2],
    radius: f32,
    color: ImColor,
    num_segments: u32,
    thickness: f32,
    filled: bool,
    draw_list: &'a mut DrawList<'ui>,
}

impl<'ui, 'a> Circle<'ui, 'a> {
    pub fn new<C>(draw_list: &'a mut DrawList<'ui>, center: [f32; 2], radius: f32, color: C) -> Self
    where
        C: Into<ImColor>,
    {
        Self {
            center,
            radius,
            color: color.into(),
            num_segments: 12,
            thickness: 1.0,
            filled: false,
            draw_list,
        }
    }

    /// Set number of segment used to draw the circle, default to 12.
    /// Add more segments if you want a smoother circle.
    pub fn num_segments(mut self, num_segments: u32) -> Self {
        self.num_segments = num_segments;
        self
    }

    /// Set circle's thickness (default to 1.0 pixel)
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set to `true` to make a filled circle (default to `false`).
    pub fn filled(mut self, filled: bool) -> Self {
        self.filled = filled;
        self
    }

    /// Draw the circle on the window.
    pub fn build(self) {
        if self.filled {
            unsafe {
                sys::ImDrawList_AddCircleFilled(
                    self.draw_list.raw_mut(),
                    self.center.into(),
                    self.radius,
                    self.color.into(),
                    self.num_segments as i32,
                )
            }
        } else {
            unsafe {
                sys::ImDrawList_AddCircle(
                    self.draw_list.raw_mut(),
                    self.center.into(),
                    self.radius,
                    self.color.into(),
                    self.num_segments as i32,
                    self.thickness,
                )
            }
        }
    }
}

/// Represents a Bezier curve about to be drawn
#[must_use = "should call .build() to draw the object"]
pub struct BezierCurve<'ui, 'a> {
    pos0: [f32; 2],
    cp0: [f32; 2],
    pos1: [f32; 2],
    cp1: [f32; 2],
    color: ImColor,
    thickness: f32,
    /// If num_segments is not set, the bezier curve is auto-tessalated.
    num_segments: Option<u32>,
    draw_list: &'a mut DrawList<'ui>,
}

impl<'ui, 'a> BezierCurve<'ui, 'a> {
    fn new<C>(
        draw_list: &'a mut DrawList<'ui>,
        pos0: [f32; 2],
        cp0: [f32; 2],
        cp1: [f32; 2],
        pos1: [f32; 2],
        c: C,
    ) -> Self
    where
        C: Into<ImColor>,
    {
        Self {
            pos0,
            cp0,
            cp1,
            pos1,
            color: c.into(),
            thickness: 1.0,
            num_segments: None,
            draw_list,
        }
    }

    /// Set curve's thickness (default to 1.0 pixel)
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set number of segments used to draw the Bezier curve. If not set, the
    /// bezier curve is auto-tessalated.
    pub fn num_segments(mut self, num_segments: u32) -> Self {
        self.num_segments = Some(num_segments);
        self
    }

    /// Draw the curve on the window.
    pub fn build(self) {
        unsafe {
            sys::ImDrawList_AddBezierCurve(
                self.draw_list.raw_mut(),
                self.pos0.into(),
                self.cp0.into(),
                self.cp1.into(),
                self.pos1.into(),
                self.color.into(),
                self.thickness,
                self.num_segments.unwrap_or(0) as i32,
            )
        }
    }
}
