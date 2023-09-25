use crate::math::MintVec2;
use crate::sys;
use crate::Ui;

create_token!(
    /// Tracks a layout group that can be ended with `end` or by dropping.
    pub struct GroupToken<'ui>;

    /// Drops the layout group manually. You can also just allow this token
    /// to drop on its own.
    drop { sys::ImGui_EndGroup() }
);

/// # Cursor / Layout
impl Ui {
    /// Renders a separator (generally horizontal).
    ///
    /// This becomes a vertical separator inside a menu bar or in horizontal layout mode.
    #[doc(alias = "Separator")]
    pub fn separator(&self) {
        unsafe { sys::ImGui_Separator() }
    }

    /// Call between widgets or groups to layout them horizontally.
    ///
    /// X position is given in window coordinates.
    ///
    /// This is equivalent to calling [same_line_with_pos](Self::same_line_with_pos)
    /// with the `pos` set to 0.0, which uses `Style::item_spacing`.
    #[doc(alias = "SameLine")]
    pub fn same_line(&self) {
        self.same_line_with_pos(0.0);
    }

    /// Call between widgets or groups to layout them horizontally.
    ///
    /// X position is given in window coordinates.
    ///
    /// This is equivalent to calling [same_line_with_spacing](Self::same_line_with_spacing)
    /// with the `spacing` set to -1.0, which means no extra spacing.
    #[doc(alias = "SameLine")]
    pub fn same_line_with_pos(&self, pos_x: f32) {
        self.same_line_with_spacing(pos_x, -1.0)
    }

    /// Call between widgets or groups to layout them horizontally.
    ///
    /// X position is given in window coordinates.
    #[doc(alias = "SameLine")]
    pub fn same_line_with_spacing(&self, pos_x: f32, spacing_w: f32) {
        unsafe { sys::ImGui_SameLineEx(pos_x, spacing_w) }
    }

    /// Undo a `same_line` call or force a new line when in horizontal layout mode
    #[doc(alias = "NewLine")]
    pub fn new_line(&self) {
        unsafe { sys::ImGui_NewLine() }
    }
    /// Adds vertical spacing
    #[doc(alias = "Spacing")]
    pub fn spacing(&self) {
        unsafe { sys::ImGui_Spacing() }
    }
    /// Fills a space of `size` in pixels with nothing on the current window.
    ///
    /// Can be used to move the cursor on the window.
    #[doc(alias = "Dummy")]
    pub fn dummy(&self, size: impl Into<MintVec2>) {
        unsafe { sys::ImGui_Dummy(size.into().into()) }
    }

    /// Moves content position to the right by `Style::indent_spacing`
    ///
    /// This is equivalent to [indent_by](Self::indent_by) with `width` set to
    /// `Style::ident_spacing`.
    #[doc(alias = "Indent")]
    pub fn indent(&self) {
        self.indent_by(0.0)
    }

    /// Moves content position to the right by `width`
    #[doc(alias = "Indent")]
    pub fn indent_by(&self, width: f32) {
        unsafe { sys::ImGui_IndentEx(width) };
    }
    /// Moves content position to the left by `Style::indent_spacing`
    ///
    /// This is equivalent to [unindent_by](Self::unindent_by) with `width` set to
    /// `Style::ident_spacing`.
    #[doc(alias = "Unindent")]
    pub fn unindent(&self) {
        self.unindent_by(0.0)
    }
    /// Moves content position to the left by `width`
    #[doc(alias = "Unindent")]
    pub fn unindent_by(&self, width: f32) {
        unsafe { sys::ImGui_UnindentEx(width) };
    }
    /// Groups items together as a single item.
    ///
    /// May be useful to handle the same mouse event on a group of items, for example.
    ///
    /// Returns a `GroupToken` that must be ended by calling `.end()`
    #[doc(alias = "BeginGroup")]
    pub fn begin_group(&self) -> GroupToken<'_> {
        unsafe { sys::ImGui_BeginGroup() };
        GroupToken::new(self)
    }
    /// Creates a layout group and runs a closure to construct the contents.
    ///
    /// May be useful to handle the same mouse event on a group of items, for example.
    #[doc(alias = "BeginGroup")]
    pub fn group<R, F: FnOnce() -> R>(&self, f: F) -> R {
        let group = self.begin_group();
        let result = f();
        group.end();
        result
    }
    /// Returns the cursor position (in window coordinates)
    #[doc(alias = "GetCursorPos")]
    pub fn cursor_pos(&self) -> [f32; 2] {
        unsafe { sys::ImGui_GetCursorPos().into() }
    }
    /// Sets the cursor position (in window coordinates).
    ///
    /// This sets the point on which the next widget will be drawn.
    #[doc(alias = "SetCursorPos")]
    pub fn set_cursor_pos(&self, pos: impl Into<MintVec2>) {
        unsafe { sys::ImGui_SetCursorPos(pos.into().into()) };
    }
    /// Returns the initial cursor position (in window coordinates)
    #[doc(alias = "GetCursorStartPos")]
    pub fn cursor_start_pos(&self) -> [f32; 2] {
        unsafe { sys::ImGui_GetCursorStartPos().into() }
    }
    /// Returns the cursor position (in absolute screen coordinates).
    ///
    /// This is especially useful for drawing, as the drawing API uses screen coordinates.
    #[doc(alias = "GetCursorScreenPos")]
    pub fn cursor_screen_pos(&self) -> [f32; 2] {
        unsafe { sys::ImGui_GetCursorScreenPos().into() }
    }
    /// Sets the cursor position (in absolute screen coordinates)
    #[doc(alias = "SetCursorScreenPos")]
    pub fn set_cursor_screen_pos(&self, pos: impl Into<MintVec2>) {
        unsafe { sys::ImGui_SetCursorScreenPos(pos.into().into()) }
    }
    /// Vertically aligns text baseline so that it will align properly to regularly frame items.
    ///
    /// Call this if you have text on a line before a framed item.
    #[doc(alias = "AlignTextToFramePadding")]
    pub fn align_text_to_frame_padding(&self) {
        unsafe { sys::ImGui_AlignTextToFramePadding() };
    }
    #[doc(alias = "GetTextLineHeight")]
    pub fn text_line_height(&self) -> f32 {
        unsafe { sys::ImGui_GetTextLineHeight() }
    }
    #[doc(alias = "GetTextLineHeightWithSpacing")]
    pub fn text_line_height_with_spacing(&self) -> f32 {
        unsafe { sys::ImGui_GetTextLineHeightWithSpacing() }
    }
    #[doc(alias = "GetFrameHeight")]
    pub fn frame_height(&self) -> f32 {
        unsafe { sys::ImGui_GetFrameHeight() }
    }
    #[doc(alias = "GetFrameLineHeightWithSpacing")]
    pub fn frame_height_with_spacing(&self) -> f32 {
        unsafe { sys::ImGui_GetFrameHeightWithSpacing() }
    }
}
