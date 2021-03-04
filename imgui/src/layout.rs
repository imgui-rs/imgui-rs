use crate::sys;
use crate::Ui;

create_token!(
    /// Tracks a layout group that can be ended with `end` or by dropping.
    pub struct GroupToken<'ui>;

    /// Drops the layout group manually. You can also just allow this token
    /// to drop on its own.
    drop { sys::igEndGroup() }
);

/// # Cursor / Layout
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
    ///
    /// This is equivalent to calling [same_line_with_pos](Self::same_line_with_pos)
    /// with the `pos` set to 0.0, which uses `Style::item_spacing`.
    pub fn same_line(&self) {
        self.same_line_with_pos(0.0);
    }

    /// Call between widgets or groups to layout them horizontally.
    ///
    /// X position is given in window coordinates.
    ///
    /// This is equivalent to calling [same_line_with_spacing](Self::same_line_with_spacing)
    /// with the `spacing` set to -1.0, which means no extra spacing.
    pub fn same_line_with_pos(&self, pos_x: f32) {
        self.same_line_with_spacing(pos_x, -1.0)
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
    /// Fills a space of `size` in pixels with nothing on the current window.
    ///
    /// Can be used to move the cursor on the window.
    pub fn dummy(&self, size: [f32; 2]) {
        unsafe { sys::igDummy(size.into()) }
    }

    /// Moves content position to the right by `Style::indent_spacing`
    ///
    /// This is equivalent to [indent_by](Self::indent_by) with `width` set to
    /// `Style::ident_spacing`.
    pub fn indent(&self) {
        self.indent_by(0.0)
    }

    /// Moves content position to the right by `width`
    pub fn indent_by(&self, width: f32) {
        unsafe { sys::igIndent(width) };
    }
    /// Moves content position to the left by `Style::indent_spacing`
    ///
    /// This is equivalent to [unindent_by](Self::unindent_by) with `width` set to
    /// `Style::ident_spacing`.
    pub fn unindent(&self) {
        self.unindent_by(0.0)
    }
    /// Moves content position to the left by `width`
    pub fn unindent_by(&self, width: f32) {
        unsafe { sys::igUnindent(width) };
    }
    /// Groups items together as a single item.
    ///
    /// May be useful to handle the same mouse event on a group of items, for example.
    ///
    /// Returns a `GroupToken` that must be ended by calling `.end()`
    pub fn begin_group(&self) -> GroupToken {
        unsafe { sys::igBeginGroup() };
        GroupToken::new(self)
    }
    /// Creates a layout group and runs a closure to construct the contents.
    ///
    /// May be useful to handle the same mouse event on a group of items, for example.
    pub fn group<R, F: FnOnce() -> R>(&self, f: F) -> R {
        let group = self.begin_group();
        let result = f();
        group.end();
        result
    }
    /// Returns the cursor position (in window coordinates)
    pub fn cursor_pos(&self) -> [f32; 2] {
        let mut out = sys::ImVec2::zero();
        unsafe { sys::igGetCursorPos(&mut out) };
        out.into()
    }
    /// Sets the cursor position (in window coordinates).
    ///
    /// This sets the point on which the next widget will be drawn.
    pub fn set_cursor_pos(&self, pos: [f32; 2]) {
        unsafe { sys::igSetCursorPos(pos.into()) };
    }
    /// Returns the initial cursor position (in window coordinates)
    pub fn cursor_start_pos(&self) -> [f32; 2] {
        let mut out = sys::ImVec2::zero();
        unsafe { sys::igGetCursorStartPos(&mut out) };
        out.into()
    }
    /// Returns the cursor position (in absolute screen coordinates).
    ///
    /// This is especially useful for drawing, as the drawing API uses screen coordinates.
    pub fn cursor_screen_pos(&self) -> [f32; 2] {
        let mut out = sys::ImVec2::zero();
        unsafe { sys::igGetCursorScreenPos(&mut out) };
        out.into()
    }
    /// Sets the cursor position (in absolute screen coordinates)
    pub fn set_cursor_screen_pos(&self, pos: [f32; 2]) {
        unsafe { sys::igSetCursorScreenPos(pos.into()) }
    }
    /// Vertically aligns text baseline so that it will align properly to regularly frame items.
    ///
    /// Call this if you have text on a line before a framed item.
    pub fn align_text_to_frame_padding(&self) {
        unsafe { sys::igAlignTextToFramePadding() };
    }
    pub fn text_line_height(&self) -> f32 {
        unsafe { sys::igGetTextLineHeight() }
    }
    pub fn text_line_height_with_spacing(&self) -> f32 {
        unsafe { sys::igGetTextLineHeightWithSpacing() }
    }
    pub fn frame_height(&self) -> f32 {
        unsafe { sys::igGetFrameHeight() }
    }
    pub fn frame_height_with_spacing(&self) -> f32 {
        unsafe { sys::igGetFrameHeightWithSpacing() }
    }
}
