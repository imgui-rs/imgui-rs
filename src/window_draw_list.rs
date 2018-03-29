use sys;
use sys::{ImDrawCornerFlags, ImDrawList, ImU32};

use super::{ImVec2, ImVec4, Ui};

use std::marker::PhantomData;

/// Wrap `ImU32` (a type typically used by ImGui to store packed colors)
/// This type is used to represent the color of drawing primitives in ImGui's
/// custom drawing API.
///
/// The type implements `From<ImU32>`, `From<ImVec4>`, `From<[f32; 4]>`,
/// `From<[f32; 3]>`, `From<(f32, f32, f32, f32)>` and `From<(f32, f32, f32)>`
/// for convenience. If alpha is not provided, it is assumed to be 1.0 (255).
#[derive(Copy, Clone)]
pub struct ImColor(ImU32);

impl From<ImColor> for ImU32 {
    fn from(color: ImColor) -> Self { color.0 }
}

impl From<ImU32> for ImColor {
    fn from(color: ImU32) -> Self { ImColor(color) }
}

impl From<ImVec4> for ImColor {
    fn from(v: ImVec4) -> Self { ImColor(unsafe { sys::igColorConvertFloat4ToU32(v) }) }
}

impl From<[f32; 4]> for ImColor {
    fn from(v: [f32; 4]) -> Self { ImColor(unsafe { sys::igColorConvertFloat4ToU32(v.into()) }) }
}

impl From<(f32, f32, f32, f32)> for ImColor {
    fn from(v: (f32, f32, f32, f32)) -> Self {
        ImColor(unsafe { sys::igColorConvertFloat4ToU32(v.into()) })
    }
}

impl From<[f32; 3]> for ImColor {
    fn from(v: [f32; 3]) -> Self { [v[0], v[1], v[2], 1.0].into() }
}

impl From<(f32, f32, f32)> for ImColor {
    fn from(v: (f32, f32, f32)) -> Self { [v.0, v.1, v.2, 1.0].into() }
}

/// All types from which ImGui's custom draw API can be used implement this
/// trait. This trait is internal to this library and implemented by
/// `WindowDrawList` and `ChannelsSplit`.
pub trait DrawAPI<'ui> {
    /// Get draw_list object
    fn draw_list(&self) -> *mut ImDrawList;
}

/// Object implementing the custom draw API.
pub struct WindowDrawList<'ui> {
    draw_list: *mut ImDrawList,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui> DrawAPI<'ui> for WindowDrawList<'ui> {
    fn draw_list(&self) -> *mut ImDrawList { self.draw_list }
}

impl<'ui> WindowDrawList<'ui> {
    pub fn new(_: &Ui<'ui>) -> Self {
        Self {
            draw_list: unsafe { sys::igGetWindowDrawList() },
            _phantom: PhantomData,
        }
    }

    /// Split into *channels_count* drawing channels.
    /// At the end of the closure, the channels are merged. The objects
    /// are then drawn in the increasing order of their channel number, and not
    /// in the all order they were called.
    ///
    /// # Example
    ///
    /// ```
    /// fn custom_drawing(ui: &Ui) {
    ///     ui.with_window_draw_list(|draw_list| {
    ///         draw_list.channels_split(2, |draw_list| {
    ///             draw_list.channels_set_current(1);
    ///             // ... Draw channel 1
    ///             draw_list.channels_set_current(0);
    ///             // ... Draw channel 0
    ///         });
    ///     });
    /// }
    /// ```
    pub fn channels_split<F: FnOnce(&ChannelsSplit)>(&self, channels_count: u32, f: F) {
        unsafe { sys::ImDrawList_ChannelsSplit(self.draw_list, channels_count as i32) };
        f(&ChannelsSplit(self));
        unsafe { sys::ImDrawList_ChannelsMerge(self.draw_list) };
    }
}

/// Represent the drawing interface within a call to `channels_split`.
pub struct ChannelsSplit<'ui>(&'ui WindowDrawList<'ui>);

impl<'ui> DrawAPI<'ui> for ChannelsSplit<'ui> {
    fn draw_list(&self) -> *mut ImDrawList { self.0.draw_list }
}

impl<'ui> ChannelsSplit<'ui> {
    /// Change current channel
    pub fn channels_set_current(&self, channel_index: u32) {
        unsafe { sys::ImDrawList_ChannelsSetCurrent(self.draw_list(), channel_index as i32) };
    }
}

macro_rules! impl_draw_list_methods {
    ($T: ident) => {
        impl<'ui> $T<'ui>
        where
            $T<'ui>: DrawAPI<'ui>,
        {
            /// Returns a line from point `p1` to `p2` with color `c`.
            pub fn add_line<P1, P2, C>(&self, p1: P1, p2: P2, c: C) -> Line<'ui, $T>
            where
                P1: Into<ImVec2>,
                P2: Into<ImVec2>,
                C: Into<ImColor>,
            {
                Line::new(self, p1, p2, c)
            }

            /// Returns a rectangle whose upper-left corner is at point `p1`
            /// and lower-right corner is at point `p2`, with color `c`.
            pub fn add_rect<P1, P2, C>(&self, p1: P1, p2: P2, c: C) -> Rect<'ui, $T>
            where
                P1: Into<ImVec2>,
                P2: Into<ImVec2>,
                C: Into<ImColor>,
            {
                Rect::new(self, p1, p2, c)
            }

            /// Draw a rectangle whose upper-left corner is at point `p1`
            /// and lower-right corner is at point `p2`.
            /// The remains parameters are the respective color of the corners
            /// in the counter-clockwise starting from the upper-left corner
            /// first.
            pub fn add_rect_filled_multicolor<P1, P2, C1, C2, C3, C4>(
                &self,
                p1: P1,
                p2: P2,
                col_upr_left: C1,
                col_upr_right: C2,
                col_bot_right: C3,
                col_bot_left: C4,
            ) where
                P1: Into<ImVec2>,
                P2: Into<ImVec2>,
                C1: Into<ImColor>,
                C2: Into<ImColor>,
                C3: Into<ImColor>,
                C4: Into<ImColor>,
            {
                unsafe {
                    sys::ImDrawList_AddRectFilledMultiColor(
                        self.draw_list(),
                        p1.into(),
                        p2.into(),
                        col_upr_left.into().into(),
                        col_upr_right.into().into(),
                        col_bot_right.into().into(),
                        col_bot_left.into().into(),
                    );
                }
            }

            /// Returns a circle with the given `center`, `radius` and `color`.
            pub fn add_circle<P, C>(&self, center: P, radius: f32, color: C) -> Circle<'ui, $T>
            where
                P: Into<ImVec2>,
                C: Into<ImColor>,
            {
                Circle::new(self, center, radius, color)
            }
        }
    };
}

impl_draw_list_methods!(WindowDrawList);
impl_draw_list_methods!(ChannelsSplit);

/// Represents a line about to be drawn
pub struct Line<'ui, D: 'ui> {
    p1: ImVec2,
    p2: ImVec2,
    color: ImColor,
    thickness: f32,
    draw_list: &'ui D,
}

impl<'ui, D: DrawAPI<'ui>> Line<'ui, D> {
    fn new<P1, P2, C>(draw_list: &'ui D, p1: P1, p2: P2, c: C) -> Self
    where
        P1: Into<ImVec2>,
        P2: Into<ImVec2>,
        C: Into<ImColor>,
    {
        Self {
            p1: p1.into(),
            p2: p2.into(),
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
                self.draw_list.draw_list(),
                self.p1,
                self.p2,
                self.color.into(),
                self.thickness,
            )
        }
    }
}

/// Represents a rectangle about to be drawn
pub struct Rect<'ui, D: 'ui> {
    p1: ImVec2,
    p2: ImVec2,
    color: ImColor,
    rounding: f32,
    flags: ImDrawCornerFlags,
    thickness: f32,
    filled: bool,
    draw_list: &'ui D,
}

impl<'ui, D: DrawAPI<'ui>> Rect<'ui, D> {
    fn new<P1, P2, C>(draw_list: &'ui D, p1: P1, p2: P2, c: C) -> Self
    where
        P1: Into<ImVec2>,
        P2: Into<ImVec2>,
        C: Into<ImColor>,
    {
        Self {
            p1: p1.into(),
            p2: p2.into(),
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
                    self.draw_list.draw_list(),
                    self.p1,
                    self.p2,
                    self.color.into(),
                    self.rounding,
                    self.flags,
                );
            }
        } else {
            unsafe {
                sys::ImDrawList_AddRect(
                    self.draw_list.draw_list(),
                    self.p1,
                    self.p2,
                    self.color.into(),
                    self.rounding,
                    self.flags,
                    self.thickness,
                );
            }
        }
    }
}

/// Represents a circle about to be drawn
pub struct Circle<'ui, D: 'ui> {
    center: ImVec2,
    radius: f32,
    color: ImColor,
    num_segments: u32,
    thickness: f32,
    filled: bool,
    draw_list: &'ui D,
}

impl<'ui, D: DrawAPI<'ui>> Circle<'ui, D> {
    pub fn new<P, C>(draw_list: &'ui D, center: P, radius: f32, color: C) -> Self
    where
        P: Into<ImVec2>,
        C: Into<ImColor>,
    {
        Self {
            center: center.into(),
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
                    self.draw_list.draw_list(),
                    self.center,
                    self.radius,
                    self.color.into(),
                    self.num_segments as i32,
                )
            }
        } else {
            unsafe {
                sys::ImDrawList_AddCircle(
                    self.draw_list.draw_list(),
                    self.center,
                    self.radius,
                    self.color.into(),
                    self.num_segments as i32,
                    self.thickness,
                )
            }
        }
    }
}
