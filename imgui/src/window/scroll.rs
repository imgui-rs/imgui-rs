use crate::sys;
use crate::Ui;

/// # Window scrolling
impl<'ui> Ui<'ui> {
    /// Returns the horizontal scrolling position.
    ///
    /// Value is between 0.0 and self.scroll_max_x().
    pub fn scroll_x(&self) -> f32 {
        unsafe { sys::igGetScrollX() }
    }
    /// Returns the vertical scrolling position.
    ///
    /// Value is between 0.0 and self.scroll_max_y().
    pub fn scroll_y(&self) -> f32 {
        unsafe { sys::igGetScrollY() }
    }
    /// Returns the maximum horizontal scrolling position.
    ///
    /// Roughly equal to content size X - window size X.
    pub fn scroll_max_x(&self) -> f32 {
        unsafe { sys::igGetScrollMaxX() }
    }
    /// Returns the maximum vertical scrolling position.
    ///
    /// Roughly equal to content size Y - window size Y.
    pub fn scroll_max_y(&self) -> f32 {
        unsafe { sys::igGetScrollMaxY() }
    }
    /// Sets the horizontal scrolling position
    pub fn set_scroll_x(&self, scroll_x: f32) {
        unsafe { sys::igSetScrollXFloat(scroll_x) };
    }
    /// Sets the vertical scroll position
    pub fn set_scroll_y(&self, scroll_y: f32) {
        unsafe { sys::igSetScrollYFloat(scroll_y) };
    }
    /// Adjusts the horizontal scroll position to make the current cursor position visible
    pub fn set_scroll_here_x(&self) {
        unsafe { sys::igSetScrollHereX(0.5) };
    }
    /// Adjusts the horizontal scroll position to make the current cursor position visible.
    ///
    /// center_x_ratio:
    ///
    /// - `0.0`: left
    /// - `0.5`: center
    /// - `1.0`: right
    pub fn set_scroll_here_x_with_ratio(&self, center_x_ratio: f32) {
        unsafe { sys::igSetScrollHereX(center_x_ratio) };
    }
    /// Adjusts the vertical scroll position to make the current cursor position visible
    pub fn set_scroll_here_y(&self) {
        unsafe { sys::igSetScrollHereY(0.5) };
    }
    /// Adjusts the vertical scroll position to make the current cursor position visible.
    ///
    /// center_y_ratio:
    ///
    /// - `0.0`: top
    /// - `0.5`: center
    /// - `1.0`: bottom
    pub fn set_scroll_here_y_with_ratio(&self, center_y_ratio: f32) {
        unsafe { sys::igSetScrollHereY(center_y_ratio) };
    }
    /// Adjusts the horizontal scroll position to make the given position visible
    pub fn set_scroll_from_pos_x(&self, local_x: f32) {
        unsafe { sys::igSetScrollFromPosXFloat(local_x, 0.5) };
    }
    /// Adjusts the horizontal scroll position to make the given position visible.
    ///
    /// center_x_ratio:
    ///
    /// - `0.0`: left
    /// - `0.5`: center
    /// - `1.0`: right
    pub fn set_scroll_from_pos_x_with_ratio(&self, local_x: f32, center_x_ratio: f32) {
        unsafe { sys::igSetScrollFromPosXFloat(local_x, center_x_ratio) };
    }
    /// Adjusts the vertical scroll position to make the given position visible
    pub fn set_scroll_from_pos_y(&self, local_y: f32) {
        unsafe { sys::igSetScrollFromPosYFloat(local_y, 0.5) };
    }
    /// Adjusts the vertical scroll position to make the given position visible.
    ///
    /// center_y_ratio:
    ///
    /// - `0.0`: top
    /// - `0.5`: center
    /// - `1.0`: bottom
    pub fn set_scroll_from_pos_y_with_ratio(&self, local_y: f32, center_y_ratio: f32) {
        unsafe { sys::igSetScrollFromPosYFloat(local_y, center_y_ratio) };
    }
}
