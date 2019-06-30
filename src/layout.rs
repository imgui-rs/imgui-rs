use crate::sys;
use crate::Ui;

/// # Cursor/layout
impl<'ui> Ui<'ui> {
    /// Renders a separator (generally horizontal).
    ///
    /// This becomes a vertical separator inside a menu bar or in horizontal layout mode.
    pub fn separator(&self) {
        unsafe { sys::igSeparator() }
    }
    /// Call between widgets or groups to layout them horizontally.
    ///
    /// X position is given in window coordinates.
    pub fn same_line(&self, pos_x: f32) {
        unsafe { sys::igSameLine(pos_x, -1.0f32) }
    }
    /// Call between widgets or groups to layout them horizontally.
    ///
    /// X position is given in window coordinates.
    pub fn same_line_with_spacing(&self, pos_x: f32, spacing_w: f32) {
        unsafe { sys::igSameLine(pos_x, spacing_w) }
    }
    /// Undo a `same_line` call or force a new line when in horizontal layout mode
    pub fn new_line(&self) {
        unsafe { sys::igNewLine() }
    }
    /// Adds vertical spacing
    pub fn spacing(&self) {
        unsafe { sys::igSpacing() }
    }
    /// Fill a space of `size` in pixels with nothing on the current window.
    ///
    /// Can be used to move the cursor on the window.
    pub fn dummy(&self, size: [f32; 2]) {
        unsafe { sys::igDummy(size.into()) }
    }
    /// Move content position to the right by `Style::indent_spacing`
    pub fn indent(&self) {
        unsafe { sys::igIndent(0.0) };
    }
    /// Move content position to the right by `width`
    pub fn indent_by(&self, width: f32) {
        unsafe { sys::igIndent(width) };
    }
    /// Move content position to the left by `Style::indent_spacing`
    pub fn unindent(&self) {
        unsafe { sys::igUnindent(0.0) };
    }
    /// Move content position to the left by `width`
    pub fn unindent_by(&self, width: f32) {
        unsafe { sys::igUnindent(width) };
    }
}
