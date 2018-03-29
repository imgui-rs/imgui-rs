use sys;
use sys::{ImDrawList, ImU32};

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
