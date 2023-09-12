//! The draw list lets you create custom graphics within a window.
//!
//! Each dear imgui window contains its own draw list. You can use
//! [`Ui::get_window_draw_list`] to access the current window draw
//! list and draw custom primitives. You can interleave normal widget
//! calls and adding primitives to the current draw list.
//!
//! Interaction is mostly through the mtehods [`DrawListMut`] struct,
//! such as [`DrawListMut::add_line`], however you can also construct
//!  structs like [`Line`] directly, then call
//!  `Line::build` with a reference to your draw list
//!
//! There are examples such as `draw_list.rs` and `custom_textures.rs`
//! within the `imgui-examples` directory

use bitflags::bitflags;

use crate::{math::MintVec2, ImColor32};
use sys::{ImDrawCmd, ImDrawList};

use super::Ui;
use crate::render::renderer::TextureId;

use std::marker::PhantomData;

bitflags!(
    /// Options for some DrawList operations.
    #[repr(C)]
    pub struct DrawFlags: u32 {
        const CLOSED = sys::ImDrawFlags_Closed;
        const ROUND_CORNERS_TOP_LEFT = sys::ImDrawFlags_RoundCornersTopLeft;
        const ROUND_CORNERS_TOP_RIGHT = sys::ImDrawFlags_RoundCornersTopRight;
        const ROUND_CORNERS_BOT_LEFT = sys::ImDrawFlags_RoundCornersBottomLeft;
        const ROUND_CORNERS_BOT_RIGHT = sys::ImDrawFlags_RoundCornersBottomRight;
        const ROUND_CORNERS_TOP = sys::ImDrawFlags_RoundCornersTop;
        const ROUND_CORNERS_BOT = sys::ImDrawFlags_RoundCornersBottom;
        const ROUND_CORNERS_LEFT = sys::ImDrawFlags_RoundCornersLeft;
        const ROUND_CORNERS_RIGHT = sys::ImDrawFlags_RoundCornersRight;
        const ROUND_CORNERS_ALL = sys::ImDrawFlags_RoundCornersAll;
        const ROUND_CORNERS_NONE = sys::ImDrawFlags_RoundCornersNone;
    }
);

bitflags!(
    /// Draw list flags
    #[repr(C)]
    pub struct DrawListFlags: u32 {
        /// Enable anti-aliased lines/borders (*2 the number of triangles for 1.0f wide line or lines
        /// thin enough to be drawn using textures, otherwise *3 the number of triangles)
        const ANTI_ALIASED_LINES = sys::ImDrawListFlags_AntiAliasedLines;
        /// Enable anti-aliased lines/borders using textures when possible. Require backend to render
        /// with bilinear filtering.
        const ANTI_ALIASED_LINES_USE_TEX = sys::ImDrawListFlags_AntiAliasedLinesUseTex;
        /// Enable anti-aliased edge around filled shapes (rounded rectangles, circles).
        const ANTI_ALIASED_FILL = sys::ImDrawListFlags_AntiAliasedFill;
        /// Can emit 'VtxOffset > 0' to allow large meshes. Set when
        /// [`BackendFlags::RENDERER_HAS_VTX_OFFSET`] is enabled.
        const ALLOW_VTX_OFFSET = sys::ImDrawListFlags_AllowVtxOffset;
    }
);

enum DrawListType {
    Window,
    Background,
    Foreground,
}

/// Object implementing the custom draw API.
///
/// Called from [`Ui::get_window_draw_list`], [`Ui::get_background_draw_list`] or [`Ui::get_foreground_draw_list`].
/// No more than one instance of this structure can live in a program at the same time.
/// The program will panic on creating a second instance.
pub struct DrawListMut<'ui> {
    draw_list_type: DrawListType,
    draw_list: *mut ImDrawList,
    _phantom: PhantomData<&'ui Ui>,
}

// Lock for each variant of draw list. See https://github.com/imgui-rs/imgui-rs/issues/488
static DRAW_LIST_LOADED_WINDOW: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
static DRAW_LIST_LOADED_BACKGROUND: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
static DRAW_LIST_LOADED_FOREGROUND: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

impl<'ui> Drop for DrawListMut<'ui> {
    fn drop(&mut self) {
        match self.draw_list_type {
            DrawListType::Window => &DRAW_LIST_LOADED_WINDOW,
            DrawListType::Background => &DRAW_LIST_LOADED_BACKGROUND,
            DrawListType::Foreground => &DRAW_LIST_LOADED_FOREGROUND,
        }
        .store(false, std::sync::atomic::Ordering::Release);
    }
}

impl<'ui> DrawListMut<'ui> {
    fn lock_draw_list(t: DrawListType) {
        let lock = match t {
            DrawListType::Window => &DRAW_LIST_LOADED_WINDOW,
            DrawListType::Background => &DRAW_LIST_LOADED_BACKGROUND,
            DrawListType::Foreground => &DRAW_LIST_LOADED_FOREGROUND,
        };

        let already_loaded = lock
            .compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::Acquire,
                std::sync::atomic::Ordering::Relaxed,
            )
            .is_err();
        if already_loaded {
            let name = match t {
                DrawListType::Window => "window",
                DrawListType::Background => "background",
                DrawListType::Foreground => "foreground",
            };
            panic!("The DrawListMut instance for the {} draw list is already loaded! You can only load one instance of it!", name)
        }
    }

    #[doc(alias = "GetWindowDrawList")]
    pub(crate) fn window(_: &Ui) -> Self {
        Self::lock_draw_list(DrawListType::Window);

        Self {
            draw_list: unsafe { sys::igGetWindowDrawList() },
            draw_list_type: DrawListType::Window,
            _phantom: PhantomData,
        }
    }

    #[doc(alias = "GetBackgroundDrawList")]
    pub(crate) fn background(_: &Ui) -> Self {
        Self::lock_draw_list(DrawListType::Background);
        Self {
            draw_list: unsafe {
                cfg_if::cfg_if! {
                    if #[cfg(feature = "docking")] {
                        // Has extra overload in docking branch
                        sys::igGetBackgroundDrawList_Nil()
                    } else {
                        sys::igGetBackgroundDrawList()
                    }
                }
            },
            draw_list_type: DrawListType::Background,
            _phantom: PhantomData,
        }
    }

    #[doc(alias = "GetForegroundDrawList")]
    pub(crate) fn foreground(_: &Ui) -> Self {
        Self::lock_draw_list(DrawListType::Foreground);
        Self {
            draw_list: unsafe {
                cfg_if::cfg_if! {
                    if #[cfg(feature = "docking")] {
                        // Has extra overload in docking branch
                        sys::igGetForegroundDrawList_Nil()
                    } else {
                        sys::igGetForegroundDrawList()
                    }
                }
            },
            draw_list_type: DrawListType::Foreground,
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
    ///     let draw_list = ui.get_window_draw_list();
    ///     draw_list.channels_split(2, |channels| {
    ///         channels.set_current(1);
    ///         // ... Draw channel 1
    ///         channels.set_current(0);
    ///         // ... Draw channel 0
    ///     });
    /// }
    /// ```
    #[doc(alias = "ChannelsSplit")]
    pub fn channels_split<F: FnOnce(&ChannelsSplit<'_>)>(&self, channels_count: u32, f: F) {
        unsafe { sys::ImDrawList_ChannelsSplit(self.draw_list, channels_count as i32) };
        f(&ChannelsSplit {
            draw_list: self,
            channels_count,
        });
        unsafe { sys::ImDrawList_ChannelsMerge(self.draw_list) };
    }
}

/// Represent the drawing interface within a call to [`channels_split`].
///
/// [`channels_split`]: DrawListMut::channels_split
pub struct ChannelsSplit<'ui> {
    draw_list: &'ui DrawListMut<'ui>,
    channels_count: u32,
}

impl<'ui> ChannelsSplit<'ui> {
    /// Change current channel.
    ///
    /// Panic if channel_index overflows the number of channels.
    #[doc(alias = "ChannelsSetCurrent")]
    pub fn set_current(&self, channel_index: u32) {
        assert!(
            channel_index < self.channels_count,
            "Channel cannot be set! Provided channel index ({}) is higher than channel count ({}).",
            channel_index,
            self.channels_count
        );
        unsafe {
            sys::ImDrawList_ChannelsSetCurrent(self.draw_list.draw_list, channel_index as i32)
        };
    }
}

/// Drawing functions
impl<'ui> DrawListMut<'ui> {
    /// Returns a line from point `p1` to `p2` with color `c`.
    #[doc(alias = "AddLine")]
    pub fn add_line<C>(
        &'ui self,
        p1: impl Into<MintVec2>,
        p2: impl Into<MintVec2>,
        c: C,
    ) -> Line<'ui>
    where
        C: Into<ImColor32>,
    {
        Line::new(self, p1, p2, c)
    }

    /// Returns a polygonal line. If filled is rendered as a convex
    /// polygon, if not filled is drawn as a line specified by
    /// [`Polyline::thickness`] (default 1.0)
    #[doc(alias = "AddPolyline", alias = "AddConvexPolyFilled")]
    pub fn add_polyline<C, P>(&'ui self, points: Vec<P>, c: C) -> Polyline<'ui>
    where
        C: Into<ImColor32>,
        P: Into<MintVec2>,
    {
        Polyline::new(self, points, c)
    }

    /// Returns a rectangle whose upper-left corner is at point `p1`
    /// and lower-right corner is at point `p2`, with color `c`.
    #[doc(alias = "AddRectFilled", alias = "AddRect")]
    pub fn add_rect<C>(
        &'ui self,
        p1: impl Into<MintVec2>,
        p2: impl Into<MintVec2>,
        c: C,
    ) -> Rect<'ui>
    where
        C: Into<ImColor32>,
    {
        Rect::new(self, p1, p2, c)
    }

    /// Draw a rectangle whose upper-left corner is at point `p1`
    /// and lower-right corner is at point `p2`.
    /// The remains parameters are the respective color of the corners
    /// in the counter-clockwise starting from the upper-left corner
    /// first.
    #[doc(alias = "AddRectFilledMultiColor")]
    pub fn add_rect_filled_multicolor<C1, C2, C3, C4>(
        &self,
        p1: impl Into<MintVec2>,
        p2: impl Into<MintVec2>,
        col_upr_left: C1,
        col_upr_right: C2,
        col_bot_right: C3,
        col_bot_left: C4,
    ) where
        C1: Into<ImColor32>,
        C2: Into<ImColor32>,
        C3: Into<ImColor32>,
        C4: Into<ImColor32>,
    {
        unsafe {
            sys::ImDrawList_AddRectFilledMultiColor(
                self.draw_list,
                p1.into().into(),
                p2.into().into(),
                col_upr_left.into().into(),
                col_upr_right.into().into(),
                col_bot_right.into().into(),
                col_bot_left.into().into(),
            );
        }
    }

    /// Returns a triangle with the given 3 vertices `p1`, `p2` and `p3`
    /// and color `c`.
    #[doc(alias = "AddTriangleFilled", alias = "AddTriangle")]
    pub fn add_triangle<C>(
        &'ui self,
        p1: impl Into<MintVec2>,
        p2: impl Into<MintVec2>,
        p3: impl Into<MintVec2>,
        c: C,
    ) -> Triangle<'ui>
    where
        C: Into<ImColor32>,
    {
        Triangle::new(self, p1, p2, p3, c)
    }

    /// Returns a circle with the given `center`, `radius` and `color`.
    #[doc(alias = "AddCircleFilled", alias = "AddCircle")]
    pub fn add_circle<C>(
        &'ui self,
        center: impl Into<MintVec2>,
        radius: f32,
        color: C,
    ) -> Circle<'ui>
    where
        C: Into<ImColor32>,
    {
        Circle::new(self, center, radius, color)
    }

    /// Draw a text whose upper-left corner is at point `pos`.
    #[doc(alias = "AddText")]
    pub fn add_text(
        &self,
        pos: impl Into<MintVec2>,
        col: impl Into<ImColor32>,
        text: impl AsRef<str>,
    ) {
        use std::os::raw::c_char;

        let text = text.as_ref();
        unsafe {
            let start = text.as_ptr() as *const c_char;
            let end = (start as usize + text.len()) as *const c_char;
            sys::ImDrawList_AddText_Vec2(
                self.draw_list,
                pos.into().into(),
                col.into().into(),
                start,
                end,
            )
        }
    }

    /// Returns a Bezier curve stretching from `pos0` to `pos1`, whose
    /// curvature is defined by `cp0` and `cp1`.
    #[doc(alias = "AddBezier", alias = "AddBezierCubic")]
    pub fn add_bezier_curve(
        &'ui self,
        pos0: impl Into<MintVec2>,
        cp0: impl Into<MintVec2>,
        cp1: impl Into<MintVec2>,
        pos1: impl Into<MintVec2>,
        color: impl Into<ImColor32>,
    ) -> BezierCurve<'ui> {
        BezierCurve::new(self, pos0, cp0, cp1, pos1, color)
    }

    /// Push a clipping rectangle on the stack, run `f` and pop it.
    ///
    /// Clip all drawings done within the closure `f` in the given
    /// rectangle.
    #[doc(alias = "PushClipRect", alias = "PopClipRect")]
    pub fn with_clip_rect<F>(&self, min: impl Into<MintVec2>, max: impl Into<MintVec2>, f: F)
    where
        F: FnOnce(),
    {
        unsafe {
            sys::ImDrawList_PushClipRect(
                self.draw_list,
                min.into().into(),
                max.into().into(),
                false,
            )
        }
        f();
        unsafe { sys::ImDrawList_PopClipRect(self.draw_list) }
    }

    /// Push a clipping rectangle on the stack, run `f` and pop it.
    ///
    /// Clip all drawings done within the closure `f` in the given
    /// rectangle. Intersect with all clipping rectangle previously on
    /// the stack.
    #[doc(alias = "PushClipRect", alias = "PopClipRect")]
    pub fn with_clip_rect_intersect<F>(
        &self,
        min: impl Into<MintVec2>,
        max: impl Into<MintVec2>,
        f: F,
    ) where
        F: FnOnce(),
    {
        unsafe {
            sys::ImDrawList_PushClipRect(self.draw_list, min.into().into(), max.into().into(), true)
        }
        f();
        unsafe { sys::ImDrawList_PopClipRect(self.draw_list) }
    }
}

/// # Images
impl<'ui> DrawListMut<'ui> {
    /// Draw the specified image in the rect specified by `p_min` to
    /// `p_max`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use imgui::*;
    /// fn custom_button(ui: &Ui, img_id: TextureId) {
    ///     // Invisible button is good widget to customise with image
    ///     ui.invisible_button("custom_button", [100.0, 20.0]);
    ///
    ///     // Get draw list and draw image over invisible button
    ///     let draw_list = ui.get_window_draw_list();
    ///     draw_list
    ///         .add_image(img_id, ui.item_rect_min(), ui.item_rect_max())
    ///         .build();
    /// }
    /// ```
    pub fn add_image(
        &'ui self,
        texture_id: TextureId,
        p_min: impl Into<MintVec2>,
        p_max: impl Into<MintVec2>,
    ) -> Image<'_> {
        Image::new(self, texture_id, p_min, p_max)
    }

    /// Draw the specified image to a quad with the specified
    /// coordinates. Similar to [`DrawListMut::add_image`] but this
    /// method is able to draw non-rectangle images.
    pub fn add_image_quad(
        &'ui self,
        texture_id: TextureId,
        p1: impl Into<MintVec2>,
        p2: impl Into<MintVec2>,
        p3: impl Into<MintVec2>,
        p4: impl Into<MintVec2>,
    ) -> ImageQuad<'_> {
        ImageQuad::new(self, texture_id, p1, p2, p3, p4)
    }

    /// Draw the speciied image, with rounded corners
    pub fn add_image_rounded(
        &'ui self,
        texture_id: TextureId,
        p_min: impl Into<MintVec2>,
        p_max: impl Into<MintVec2>,
        rounding: f32,
    ) -> ImageRounded<'_> {
        ImageRounded::new(self, texture_id, p_min, p_max, rounding)
    }

    /// Draw the specified callback.
    ///
    /// Note: if this DrawList is never rendered the callback will leak because DearImGui
    /// does not provide a method to clean registered callbacks.
    pub fn add_callback<F: FnOnce() + 'static>(&'ui self, callback: F) -> Callback<'ui, F> {
        Callback::new(self, callback)
    }
}

/// Represents a line about to be drawn
#[must_use = "should call .build() to draw the object"]
pub struct Line<'ui> {
    p1: [f32; 2],
    p2: [f32; 2],
    color: ImColor32,
    thickness: f32,
    draw_list: &'ui DrawListMut<'ui>,
}

impl<'ui> Line<'ui> {
    fn new<C>(
        draw_list: &'ui DrawListMut<'_>,
        p1: impl Into<MintVec2>,
        p2: impl Into<MintVec2>,
        c: C,
    ) -> Self
    where
        C: Into<ImColor32>,
    {
        Self {
            p1: p1.into().into(),
            p2: p2.into().into(),
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
                self.draw_list.draw_list,
                self.p1.into(),
                self.p2.into(),
                self.color.into(),
                self.thickness,
            )
        }
    }
}

/// Represents a poly line about to be drawn
#[must_use = "should call .build() to draw the object"]
pub struct Polyline<'ui> {
    points: Vec<[f32; 2]>,
    thickness: f32,
    filled: bool,
    color: ImColor32,
    draw_list: &'ui DrawListMut<'ui>,
}

impl<'ui> Polyline<'ui> {
    fn new<C, P>(draw_list: &'ui DrawListMut<'_>, points: Vec<P>, c: C) -> Self
    where
        C: Into<ImColor32>,
        P: Into<MintVec2>,
    {
        Self {
            points: points.into_iter().map(|p| p.into().into()).collect(),
            color: c.into(),
            thickness: 1.0,
            filled: false,
            draw_list,
        }
    }

    /// Set line's thickness (default to 1.0 pixel). Has no effect if
    /// shape is filled
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Draw shape as filled convex polygon
    pub fn filled(mut self, filled: bool) -> Self {
        self.filled = filled;
        self
    }

    /// Draw the line on the window
    pub fn build(self) {
        if self.filled {
            unsafe {
                sys::ImDrawList_AddConvexPolyFilled(
                    self.draw_list.draw_list,
                    self.points.as_ptr() as *const sys::ImVec2,
                    self.points.len() as i32,
                    self.color.into(),
                )
            }
        } else {
            unsafe {
                sys::ImDrawList_AddPolyline(
                    self.draw_list.draw_list,
                    self.points.as_ptr() as *const sys::ImVec2,
                    self.points.len() as i32,
                    self.color.into(),
                    sys::ImDrawFlags::default(),
                    self.thickness,
                )
            }
        }
    }
}

/// Represents a rectangle about to be drawn
#[must_use = "should call .build() to draw the object"]
pub struct Rect<'ui> {
    p1: [f32; 2],
    p2: [f32; 2],
    color: ImColor32,
    rounding: f32,
    flags: DrawFlags,
    thickness: f32,
    filled: bool,
    draw_list: &'ui DrawListMut<'ui>,
}

impl<'ui> Rect<'ui> {
    fn new<C>(
        draw_list: &'ui DrawListMut<'_>,
        p1: impl Into<MintVec2>,
        p2: impl Into<MintVec2>,
        c: C,
    ) -> Self
    where
        C: Into<ImColor32>,
    {
        Self {
            p1: p1.into().into(),
            p2: p2.into().into(),
            color: c.into(),
            rounding: 0.0,
            flags: DrawFlags::ROUND_CORNERS_ALL,
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
        self.flags.set(DrawFlags::ROUND_CORNERS_TOP_LEFT, value);
        self
    }

    /// Set flag to indicate if rectangle's top-right corner will be rounded.
    pub fn round_top_right(mut self, value: bool) -> Self {
        self.flags.set(DrawFlags::ROUND_CORNERS_TOP_RIGHT, value);
        self
    }

    /// Set flag to indicate if rectangle's bottom-left corner will be rounded.
    pub fn round_bot_left(mut self, value: bool) -> Self {
        self.flags.set(DrawFlags::ROUND_CORNERS_BOT_LEFT, value);
        self
    }

    /// Set flag to indicate if rectangle's bottom-right corner will be rounded.
    pub fn round_bot_right(mut self, value: bool) -> Self {
        self.flags.set(DrawFlags::ROUND_CORNERS_BOT_RIGHT, value);
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
                    self.draw_list.draw_list,
                    self.p1.into(),
                    self.p2.into(),
                    self.color.into(),
                    self.rounding,
                    self.flags.bits() as i32,
                );
            }
        } else {
            unsafe {
                sys::ImDrawList_AddRect(
                    self.draw_list.draw_list,
                    self.p1.into(),
                    self.p2.into(),
                    self.color.into(),
                    self.rounding,
                    self.flags.bits() as i32,
                    self.thickness,
                );
            }
        }
    }
}

/// Represents a triangle about to be drawn on the window
#[must_use = "should call .build() to draw the object"]
pub struct Triangle<'ui> {
    p1: [f32; 2],
    p2: [f32; 2],
    p3: [f32; 2],
    color: ImColor32,
    thickness: f32,
    filled: bool,
    draw_list: &'ui DrawListMut<'ui>,
}

impl<'ui> Triangle<'ui> {
    fn new<C>(
        draw_list: &'ui DrawListMut<'_>,
        p1: impl Into<MintVec2>,
        p2: impl Into<MintVec2>,
        p3: impl Into<MintVec2>,
        c: C,
    ) -> Self
    where
        C: Into<ImColor32>,
    {
        Self {
            p1: p1.into().into(),
            p2: p2.into().into(),
            p3: p3.into().into(),
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
                    self.draw_list.draw_list,
                    self.p1.into(),
                    self.p2.into(),
                    self.p3.into(),
                    self.color.into(),
                )
            }
        } else {
            unsafe {
                sys::ImDrawList_AddTriangle(
                    self.draw_list.draw_list,
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
pub struct Circle<'ui> {
    center: [f32; 2],
    radius: f32,
    color: ImColor32,
    num_segments: u32,
    thickness: f32,
    filled: bool,
    draw_list: &'ui DrawListMut<'ui>,
}

impl<'ui> Circle<'ui> {
    /// Typically constructed by [`DrawListMut::add_circle`]
    pub fn new<C>(
        draw_list: &'ui DrawListMut<'_>,
        center: impl Into<MintVec2>,
        radius: f32,
        color: C,
    ) -> Self
    where
        C: Into<ImColor32>,
    {
        Self {
            center: center.into().into(),
            radius,
            color: color.into(),
            num_segments: 0,
            thickness: 1.0,
            filled: false,
            draw_list,
        }
    }

    /// Set number of segment used to draw the circle, default to 0.
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
                    self.draw_list.draw_list,
                    self.center.into(),
                    self.radius,
                    self.color.into(),
                    self.num_segments as i32,
                )
            }
        } else {
            unsafe {
                sys::ImDrawList_AddCircle(
                    self.draw_list.draw_list,
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
pub struct BezierCurve<'ui> {
    pos0: [f32; 2],
    cp0: [f32; 2],
    pos1: [f32; 2],
    cp1: [f32; 2],
    color: ImColor32,
    thickness: f32,
    /// If num_segments is not set, the bezier curve is auto-tessalated.
    num_segments: Option<u32>,
    draw_list: &'ui DrawListMut<'ui>,
}

impl<'ui> BezierCurve<'ui> {
    /// Typically constructed by [`DrawListMut::add_bezier_curve`]
    pub fn new<C>(
        draw_list: &'ui DrawListMut<'_>,
        pos0: impl Into<MintVec2>,
        cp0: impl Into<MintVec2>,
        cp1: impl Into<MintVec2>,
        pos1: impl Into<MintVec2>,
        c: C,
    ) -> Self
    where
        C: Into<ImColor32>,
    {
        Self {
            pos0: pos0.into().into(),
            cp0: cp0.into().into(),
            cp1: cp1.into().into(),
            pos1: pos1.into().into(),
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
            sys::ImDrawList_AddBezierCubic(
                self.draw_list.draw_list,
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

/// Image draw list primitive, not to be confused with the widget
/// [`imgui::Image`](crate::Image).
#[must_use = "should call .build() to draw the object"]
pub struct Image<'ui> {
    texture_id: TextureId,
    p_min: [f32; 2],
    p_max: [f32; 2],
    uv_min: [f32; 2],
    uv_max: [f32; 2],
    col: ImColor32,
    draw_list: &'ui DrawListMut<'ui>,
}

impl<'ui> Image<'ui> {
    /// Typically constructed by [`DrawListMut::add_image`]
    pub fn new(
        draw_list: &'ui DrawListMut<'_>,
        texture_id: TextureId,
        p_min: impl Into<MintVec2>,
        p_max: impl Into<MintVec2>,
    ) -> Self {
        Self {
            texture_id,
            p_min: p_min.into().into(),
            p_max: p_max.into().into(),
            uv_min: [0.0, 0.0],
            uv_max: [1.0, 1.0],
            col: [1.0, 1.0, 1.0, 1.0].into(),
            draw_list,
        }
    }

    /// Set uv_min (default `[0.0, 0.0]`)
    pub fn uv_min(mut self, uv_min: impl Into<MintVec2>) -> Self {
        self.uv_min = uv_min.into().into();
        self
    }
    /// Set uv_max (default `[1.0, 1.0]`)
    pub fn uv_max(mut self, uv_max: impl Into<MintVec2>) -> Self {
        self.uv_max = uv_max.into().into();
        self
    }

    /// Set color tint (default: no tint/white `[1.0, 1.0, 1.0, 1.0]`)
    pub fn col<C>(mut self, col: C) -> Self
    where
        C: Into<ImColor32>,
    {
        self.col = col.into();
        self
    }

    /// Draw the image on the window.
    pub fn build(self) {
        use std::os::raw::c_void;

        unsafe {
            sys::ImDrawList_AddImage(
                self.draw_list.draw_list,
                self.texture_id.id() as *mut c_void,
                self.p_min.into(),
                self.p_max.into(),
                self.uv_min.into(),
                self.uv_max.into(),
                self.col.into(),
            );
        }
    }
}

/// Represents a image about to be drawn
#[must_use = "should call .build() to draw the object"]
pub struct ImageQuad<'ui> {
    texture_id: TextureId,
    p1: [f32; 2],
    p2: [f32; 2],
    p3: [f32; 2],
    p4: [f32; 2],
    uv1: [f32; 2],
    uv2: [f32; 2],
    uv3: [f32; 2],
    uv4: [f32; 2],
    col: ImColor32,
    draw_list: &'ui DrawListMut<'ui>,
}

impl<'ui> ImageQuad<'ui> {
    /// Typically constructed by [`DrawListMut::add_image_quad`]
    pub fn new(
        draw_list: &'ui DrawListMut<'_>,
        texture_id: TextureId,
        p1: impl Into<MintVec2>,
        p2: impl Into<MintVec2>,
        p3: impl Into<MintVec2>,
        p4: impl Into<MintVec2>,
    ) -> Self {
        Self {
            texture_id,
            p1: p1.into().into(),
            p2: p2.into().into(),
            p3: p3.into().into(),
            p4: p4.into().into(),
            uv1: [0.0, 0.0],
            uv2: [1.0, 0.0],
            uv3: [1.0, 1.0],
            uv4: [0.0, 1.0],
            col: [1.0, 1.0, 1.0, 1.0].into(),
            draw_list,
        }
    }

    /// Set uv coordinates of each point of the quad. If not called, defaults are:
    ///
    /// ```text
    /// uv1: [0.0, 0.0],
    /// uv2: [1, 0],
    /// uv3: [1, 1],
    /// uv4: [0, 1],
    /// ```
    pub fn uv(
        mut self,
        uv1: impl Into<MintVec2>,
        uv2: impl Into<MintVec2>,
        uv3: impl Into<MintVec2>,
        uv4: impl Into<MintVec2>,
    ) -> Self {
        self.uv1 = uv1.into().into();
        self.uv2 = uv2.into().into();
        self.uv3 = uv3.into().into();
        self.uv4 = uv4.into().into();
        self
    }

    /// Set color tint (default: no tint/white `[1.0, 1.0, 1.0, 1.0]`)
    pub fn col<C>(mut self, col: C) -> Self
    where
        C: Into<ImColor32>,
    {
        self.col = col.into();
        self
    }

    /// Draw the image on the window.
    pub fn build(self) {
        use std::os::raw::c_void;

        unsafe {
            sys::ImDrawList_AddImageQuad(
                self.draw_list.draw_list,
                self.texture_id.id() as *mut c_void,
                self.p1.into(),
                self.p2.into(),
                self.p3.into(),
                self.p4.into(),
                self.uv1.into(),
                self.uv2.into(),
                self.uv3.into(),
                self.uv4.into(),
                self.col.into(),
            );
        }
    }
}

/// Represents a image about to be drawn. Similar to [`Image`] but
/// with corners rounded with a given radius
#[must_use = "should call .build() to draw the object"]
pub struct ImageRounded<'ui> {
    texture_id: TextureId,
    p_min: [f32; 2],
    p_max: [f32; 2],
    uv_min: [f32; 2],
    uv_max: [f32; 2],
    col: ImColor32,
    rounding: f32,
    draw_flags: DrawFlags,
    draw_list: &'ui DrawListMut<'ui>,
}

impl<'ui> ImageRounded<'ui> {
    /// Typically constructed by [`DrawListMut::add_image_rounded`]
    pub fn new(
        draw_list: &'ui DrawListMut<'_>,
        texture_id: TextureId,
        p_min: impl Into<MintVec2>,
        p_max: impl Into<MintVec2>,
        rounding: f32,
    ) -> Self {
        Self {
            texture_id,
            p_min: p_min.into().into(),
            p_max: p_max.into().into(),
            uv_min: [0.0, 0.0],
            uv_max: [1.0, 1.0],
            col: [1.0, 1.0, 1.0, 1.0].into(),
            rounding,
            draw_flags: DrawFlags::ROUND_CORNERS_ALL,
            draw_list,
        }
    }

    /// Set uv_min (default `[0.0, 0.0]`)
    pub fn uv_min(mut self, uv_min: impl Into<MintVec2>) -> Self {
        self.uv_min = uv_min.into().into();
        self
    }
    /// Set uv_max (default `[1.0, 1.0]`)
    pub fn uv_max(mut self, uv_max: impl Into<MintVec2>) -> Self {
        self.uv_max = uv_max.into().into();
        self
    }

    /// Set color tint (default: no tint/white `[1.0, 1.0, 1.0, 1.0]`)
    pub fn col<C>(mut self, col: C) -> Self
    where
        C: Into<ImColor32>,
    {
        self.col = col.into();
        self
    }

    /// Set flag to indicate rounding on all all corners.
    pub fn round_all(mut self, value: bool) -> Self {
        self.draw_flags.set(DrawFlags::ROUND_CORNERS_ALL, value);
        self
    }

    /// Set flag to indicate if image's top-left corner will be rounded.
    pub fn round_top_left(mut self, value: bool) -> Self {
        self.draw_flags
            .set(DrawFlags::ROUND_CORNERS_TOP_LEFT, value);
        self
    }

    /// Set flag to indicate if image's top-right corner will be rounded.
    pub fn round_top_right(mut self, value: bool) -> Self {
        self.draw_flags
            .set(DrawFlags::ROUND_CORNERS_TOP_RIGHT, value);
        self
    }

    /// Set flag to indicate if image's bottom-left corner will be rounded.
    pub fn round_bot_left(mut self, value: bool) -> Self {
        self.draw_flags
            .set(DrawFlags::ROUND_CORNERS_BOT_LEFT, value);
        self
    }

    /// Set flag to indicate if image's bottom-right corner will be rounded.
    pub fn round_bot_right(mut self, value: bool) -> Self {
        self.draw_flags
            .set(DrawFlags::ROUND_CORNERS_BOT_RIGHT, value);
        self
    }

    /// Draw the image on the window.
    pub fn build(self) {
        use std::os::raw::c_void;

        unsafe {
            sys::ImDrawList_AddImageRounded(
                self.draw_list.draw_list,
                self.texture_id.id() as *mut c_void,
                self.p_min.into(),
                self.p_max.into(),
                self.uv_min.into(),
                self.uv_max.into(),
                self.col.into(),
                self.rounding,
                self.draw_flags.bits() as i32,
            );
        }
    }
}

#[must_use = "should call .build() to draw the object"]
pub struct Callback<'ui, F> {
    draw_list: &'ui DrawListMut<'ui>,
    callback: F,
}

impl<'ui, F: FnOnce() + 'static> Callback<'ui, F> {
    /// Typically constructed by [`DrawListMut::add_callback`]
    pub fn new(draw_list: &'ui DrawListMut<'_>, callback: F) -> Self {
        Callback {
            draw_list,
            callback,
        }
    }
    /// Adds the callback to the draw-list so it will be run when the window is drawn
    pub fn build(self) {
        use std::os::raw::c_void;
        // F is Sized, so *mut F must be a thin pointer.
        let callback: *mut F = Box::into_raw(Box::new(self.callback));

        unsafe {
            sys::ImDrawList_AddCallback(
                self.draw_list.draw_list,
                Some(Self::run_callback),
                callback as *mut c_void,
            );
        }
    }
    unsafe extern "C" fn run_callback(_parent_list: *const ImDrawList, cmd: *const ImDrawCmd) {
        // We are modifying through a C const pointer, but that should be harmless.
        let cmd = &mut *(cmd as *mut ImDrawCmd);
        // Consume the pointer and leave a NULL behind to avoid a double-free or
        // calling twice an FnOnce. It should not happen, but better safe than sorry.
        let callback = std::mem::replace(&mut cmd.UserCallbackData, std::ptr::null_mut());
        if callback.is_null() {
            return;
        }
        let callback = Box::from_raw(callback as *mut F);
        callback();
    }
}
