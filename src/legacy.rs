#![allow(non_upper_case_globals)]
use bitflags::bitflags;
use std::ffi::CStr;
use std::os::raw::c_int;
use std::str;

use crate::fonts::atlas::{FontAtlas, FontAtlasTexture, FontConfig};
use crate::fonts::glyph_ranges::FontGlyphRanges;
use crate::fonts::font::Font;
use crate::input::keyboard::Key;
use crate::input::mouse::{MouseButton, MouseCursor};
use crate::render::draw_data::{DrawIdx, DrawVert};
use crate::render::renderer::TextureId;
use crate::internal::RawCast;
use crate::style::{StyleColor, StyleVar, Style};
use crate::{Context, Ui, Condition};

#[deprecated(since = "0.1.0", note = "use Font instead")]
pub type ImFont = Font;

#[deprecated(since = "0.1.0", note = "use Key instead")]
pub type ImGuiKey = Key;

#[deprecated(since = "0.1.0", note = "use MouseCursor instead")]
pub type ImGuiMouseCursor = MouseCursor;

#[deprecated(since = "0.1.0", note = "use MouseButton instead")]
pub type ImMouseButton = MouseButton;

#[deprecated(since = "0.1.0", note = "use FontConfig instead")]
pub type ImFontConfig = FontConfig;

#[deprecated(since = "0.1.0", note = "use FontAtlas instead")]
pub type ImFontAtlas = FontAtlas;

#[deprecated(since = "0.1.0", note = "use Context instead")]
pub type ImGui = Context;

#[deprecated(since = "0.1.0", note = "use Condition instead")]
pub type ImGuiCond = Condition;

#[deprecated(since = "0.1.0", note = "use StyleColor instead")]
pub type ImGuiCol = StyleColor;

#[deprecated(since = "0.1.0", note = "use TextureId instead")]
pub type ImTexture = TextureId;

#[deprecated(since = "0.1.0", note = "use Style instead")]
pub type ImGuiStyle = Style;

#[deprecated(since = "0.1.0", note = "use DrawIdx instead")]
pub type ImDrawIdx = DrawIdx;

#[deprecated(since = "0.1.0", note = "use DrawVert instead")]
pub type ImDrawVert = DrawVert;

#[deprecated(since = "0.1.0", note = "use FontAtlasTexture instead")]
pub type TextureHandle<'a> = FontAtlasTexture<'a>;

#[deprecated(since = "0.1.0", note = "use FontGlyphRanges instead")]
pub type FontGlyphRange = FontGlyphRanges;

bitflags!(
    /// Color edit flags
    #[repr(C)]
    pub struct ImGuiColorEditFlags: c_int {
        /// ColorEdit, ColorPicker, ColorButton: ignore Alpha component (read 3 components from the
        /// input pointer).
        const NoAlpha = 1;
        /// ColorEdit: disable picker when clicking on colored square.
        const NoPicker = 1 << 2;
        /// ColorEdit: disable toggling options menu when right-clicking on inputs/small preview.
        const NoOptions = 1 << 3;
        /// ColorEdit, ColorPicker: disable colored square preview next to the inputs. (e.g. to
        /// show only the inputs)
        const NoSmallPreview = 1 << 4;
        /// ColorEdit, ColorPicker: disable inputs sliders/text widgets (e.g. to show only the
        /// small preview colored square).
        const NoInputs = 1 << 5;
        /// ColorEdit, ColorPicker, ColorButton: disable tooltip when hovering the preview.
        const NoTooltip = 1 << 6;
        /// ColorEdit, ColorPicker: disable display of inline text label (the label is still
        /// forwarded to the tooltip and picker).
        const NoLabel = 1 << 7;
        /// ColorPicker: disable bigger color preview on right side of the picker, use small
        /// colored square preview instead.
        const NoSidePreview = 1 << 8;
        /// ColorEdit: disable drag and drop target. ColorButton: disable drag and drop source.
        const NoDragDrop = 1 << 9;

        /// ColorEdit, ColorPicker: show vertical alpha bar/gradient in picker.
        const AlphaBar = 1 << 16;
        /// ColorEdit, ColorPicker, ColorButton: display preview as a transparent color over a
        /// checkerboard, instead of opaque.
        const AlphaPreview = 1 << 17;
        /// ColorEdit, ColorPicker, ColorButton: display half opaque / half checkerboard, instead
        /// of opaque.
        const AlphaPreviewHalf= 1 << 18;
        /// (WIP) ColorEdit: Currently only disable 0.0f..1.0f limits in RGBA edition (note: you
        /// probably want to use ImGuiColorEditFlags::Float flag as well).
        const HDR = 1 << 19;
        /// ColorEdit: choose one among RGB/HSV/HEX. ColorPicker: choose any combination using
        /// RGB/HSV/HEX.
        const RGB = 1 << 20;
        const HSV = 1 << 21;
        const HEX = 1 << 22;
        /// ColorEdit, ColorPicker, ColorButton: _display_ values formatted as 0..255.
        const Uint8 = 1 << 23;
        /// ColorEdit, ColorPicker, ColorButton: _display_ values formatted as 0.0f..1.0f floats
        /// instead of 0..255 integers. No round-trip of value via integers.
        const Float = 1 << 24;
        /// ColorPicker: bar for Hue, rectangle for Sat/Value.
        const PickerHueBar = 1 << 25;
        /// ColorPicker: wheel for Hue, triangle for Sat/Value.
        const PickerHueWheel = 1 << 26;
    }
);

bitflags!(
    /// Flags for combo boxes
    #[repr(C)]
    pub struct ImGuiComboFlags: c_int {
        /// Align the popup toward the left by default
        const PopupAlignLeft = 1;
        /// Max ~4 items visible.
        const HeightSmall = 1 << 1;
        /// Max ~8 items visible (default)
        const HeightRegular = 1 << 2;
        /// Max ~20 items visible
        const HeightLarge = 1 << 3;
        /// As many fitting items as possible
        const HeightLargest = 1 << 4;
        /// Display on the preview box without the square arrow button
        const NoArrowButton = 1 << 5;
        /// Display only a square arrow button
        const NoPreview = 1 << 6;

        const HeightMask     = ImGuiComboFlags::HeightSmall.bits
            | ImGuiComboFlags::HeightRegular.bits
            | ImGuiComboFlags::HeightLarge.bits
            | ImGuiComboFlags::HeightLargest.bits;
    }
);

bitflags!(
    /// Flags for igBeginDragDropSource(), igAcceptDragDropPayload()
    #[repr(C)]
    pub struct ImGuiDragDropFlags: c_int {
        /// By default, a successful call to igBeginDragDropSource opens a tooltip so you can
        /// display a preview or description of the source contents. This flag disable this
        /// behavior.
        const SourceNoPreviewTooltip = 1;
        /// By default, when dragging we clear data so that igIsItemHovered() will return false, to
        /// avoid subsequent user code submitting tooltips. This flag disable this behavior so you
        /// can still call igIsItemHovered() on the source item.
        const SourceNoDisableHover = 1 << 1;
        /// Disable the behavior that allows to open tree nodes and collapsing header by holding
        /// over them while dragging a source item.
        const SourceNoHoldToOpenOthers = 1 << 2;
        /// Allow items such as igText(), igImage() that have no unique identifier to be used as
        /// drag source, by manufacturing a temporary identifier based on their window-relative
        /// position. This is extremely unusual within the dear imgui ecosystem and so we made it
        /// explicit.
        const SourceAllowNullID = 1 << 3;
        /// External source (from outside of imgui), won't attempt to read current item/window
        /// info. Will always return true. Only one Extern source can be active simultaneously.
        const SourceExtern = 1 << 4;
        /// Automatically expire the payload if the source cease to be submitted (otherwise
        /// payloads are persisting while being dragged)
        const SourceAutoExpirePayload = 1 << 5;
        /// igAcceptDragDropPayload() will returns true even before the mouse button is released.
        /// You can then call igIsDelivery() to test if the payload needs to be delivered.
        const AcceptBeforeDelivery = 1 << 10;
        /// Do not draw the default highlight rectangle when hovering over target.
        const AcceptNoDrawDefaultRect = 1 << 11;
        /// Request hiding the igBeginDragDropSource tooltip from the igBeginDragDropTarget site.
        const AcceptNoPreviewTooltip = 1 << 12;
        /// For peeking ahead and inspecting the payload before delivery.
        const AcceptPeekOnly = ImGuiDragDropFlags::AcceptBeforeDelivery.bits
            | ImGuiDragDropFlags::AcceptNoDrawDefaultRect.bits;
    }
);

bitflags!(
    /// Flags for indictating which corner of a rectangle should be rounded
    #[repr(C)]
    pub struct ImDrawCornerFlags: c_int {
        const TopLeft = 1;
        const TopRight = 1 << 1;
        const BotLeft = 1 << 2;
        const BotRight = 1 << 3;
        const Top = ImDrawCornerFlags::TopLeft.bits
            | ImDrawCornerFlags::TopRight.bits;
        const Bot = ImDrawCornerFlags::BotLeft.bits
            | ImDrawCornerFlags::BotRight.bits;
        const Left = ImDrawCornerFlags::TopLeft.bits
            | ImDrawCornerFlags::BotLeft.bits;
        const Right = ImDrawCornerFlags::TopRight.bits
            | ImDrawCornerFlags::BotRight.bits;
        const All = 0xF;
    }
);

bitflags!(
    /// Draw list flags
    #[repr(C)]
    pub struct ImDrawListFlags: c_int {
        const AntiAliasedLines = 1;
        const AntiAliasedFill = 1 << 1;
    }
);

bitflags!(
    /// Flags for window focus checks
    #[repr(C)]
    pub struct ImGuiFocusedFlags: c_int {
        /// Return true if any children of the window is focused
        const ChildWindows = 1;
        /// Test from root window (top most parent of the current hierarchy)
        const RootWindow = 1 << 1;
        /// Return true if any window is focused
        const AnyWindow = 1 << 2;

        const RootAndChildWindows =
            ImGuiFocusedFlags::RootWindow.bits | ImGuiFocusedFlags::ChildWindows.bits;
    }
);

bitflags!(
    /// Flags for hover checks
    #[repr(C)]
    pub struct ImGuiHoveredFlags: c_int {
        /// Window hover checks only: Return true if any children of the window is hovered
        const ChildWindows = 1;
        /// Window hover checks only: Test from root window (top most parent of the current hierarchy)
        const RootWindow = 1 << 1;
        /// Window hover checks only: Return true if any window is hovered
        const AnyWindow = 1 << 2;
        /// Return true even if a popup window is normally blocking access to this item/window
        const AllowWhenBlockedByPopup = 1 << 3;
        /// Return true even if an active item is blocking access to this item/window. Useful for
        /// Drag and Drop patterns.
        const AllowWhenBlockedByActiveItem = 1 << 5;
        /// Return true even if the position is overlapped by another window
        const AllowWhenOverlapped = 1 << 6;
        /// Return true even if the item is disabled
        const AllowWhenDisabled = 1 << 7;

        const RectOnly = ImGuiHoveredFlags::AllowWhenBlockedByPopup.bits
            | ImGuiHoveredFlags::AllowWhenBlockedByActiveItem.bits
            | ImGuiHoveredFlags::AllowWhenOverlapped.bits;
        const RootAndChildWindows = ImGuiFocusedFlags::RootWindow.bits
            | ImGuiFocusedFlags::ChildWindows.bits;
    }
);

bitflags!(
    /// Flags for text inputs
    #[repr(C)]
    pub struct ImGuiInputTextFlags: c_int {
        /// Allow 0123456789.+-*/
        const CharsDecimal = 1;
        /// Allow 0123456789ABCDEFabcdef
        const CharsHexadecimal = 1 << 1;
        /// Turn a..z into A..Z
        const CharsUppercase = 1 << 2;
        /// Filter out spaces, tabs
        const CharsNoBlank = 1 << 3;
        /// Select entire text when first taking mouse focus
        const AutoSelectAll = 1 << 4;
        /// Return 'true' when Enter is pressed (as opposed to when the value was modified)
        const EnterReturnsTrue = 1 << 5;
        /// Call user function on pressing TAB (for completion handling)
        const CallbackCompletion = 1 << 6;
        /// Call user function on pressing Up/Down arrows (for history handling)
        const CallbackHistory = 1 << 7;
        /// Call user function every time. User code may query cursor position, modify text buffer.
        const CallbackAlways = 1 << 8;
        /// Call user function to filter character.
        const CallbackCharFilter = 1 << 9;
        /// Pressing TAB input a '\t' character into the text field
        const AllowTabInput = 1 << 10;
        /// In multi-line mode, unfocus with Enter, add new line with Ctrl+Enter (default is
        /// opposite: unfocus with Ctrl+Enter, add line with Enter).
        const CtrlEnterForNewLine = 1 << 11;
        /// Disable following the cursor horizontally
        const NoHorizontalScroll = 1 << 12;
        /// Insert mode
        const AlwaysInsertMode = 1 << 13;
        /// Read-only mode
        const ReadOnly = 1 << 14;
        /// Password mode, display all characters as '*'
        const Password = 1 << 15;
        /// Disable undo/redo.
        const NoUndoRedo = 1 << 16;
        /// Allow 0123456789.+-*/eE (Scientific notation input)
        const CharsScientific = 1 << 17;
        /// Allow buffer capacity resize + notify when the string wants to be resized
        const CallbackResize = 1 << 18;
    }
);

bitflags!(
    /// Flags for selectables
    #[repr(C)]
    pub struct ImGuiSelectableFlags: c_int {
        /// Clicking this don't close parent popup window
        const DontClosePopups = 1;
        /// Selectable frame can span all columns (text will still fit in current column)
        const SpanAllColumns = 1 << 1;
        /// Generate press events on double clicks too
        const AllowDoubleClick = 1 << 2;
        /// Cannot be selected, display greyed out text
        const Disabled = 1 << 3;
    }
);

bitflags!(
    /// Flags for trees and collapsing headers
    #[repr(C)]
    pub struct ImGuiTreeNodeFlags: c_int {
        /// Draw as selected
        const Selected = 1;
        /// Full colored frame (e.g. for collapsing header)
        const Framed = 1 << 1;
        /// Hit testing to allow subsequent widgets to overlap this one
        const AllowItemOverlap = 1 << 2;
        /// Don't do a tree push when open (e.g. for collapsing header) = no extra indent nor
        /// pushing on ID stack
        const NoTreePushOnOpen = 1 << 3;
        /// Don't automatically and temporarily open node when Logging is active (by default
        /// logging will automatically open tree nodes)
        const NoAutoOpenOnLog = 1 << 4;
        /// Default node to be open
        const DefaultOpen = 1 << 5;
        /// Need double-click to open node
        const OpenOnDoubleClick = 1 << 6;
        /// Only open when clicking on the arrow part. If OpenOnDoubleClick is also set,
        /// single-click arrow or double-click all box to open.
        const OpenOnArrow = 1 << 7;
        /// No collapsing, no arrow (use as a convenience for leaf nodes).
        const Leaf = 1 << 8;
        /// Display a bullet instead of arrow
        const Bullet = 1 << 9;
        /// Use FramePadding (even for an unframed text node) to vertically align text baseline to
        /// regular widget height.
        const FramePadding = 1 << 10;
        const NavLeftJumpsBackHere = 1 << 13;

        const CollapsingHeader  =
            ImGuiTreeNodeFlags::Framed.bits | ImGuiTreeNodeFlags::NoTreePushOnOpen.bits |
            ImGuiTreeNodeFlags::NoAutoOpenOnLog.bits;
    }
);

bitflags!(
    /// Window flags
    #[repr(C)]
    pub struct ImGuiWindowFlags: c_int {
        /// Disable title-bar.
        const NoTitleBar = 1;
        /// Disable user resizing with the lower-right grip.
        const NoResize = 1 << 1;
        /// Disable user moving the window.
        const NoMove = 1 << 2;
        /// Disable scrollbars (window can still scroll with mouse or programatically).
        const NoScrollbar = 1 << 3;
        /// Disable user vertically scrolling with mouse wheel. On child window, mouse wheel will
        /// be forwarded to the parent unless NoScrollbar is also set.
        const NoScrollWithMouse = 1 << 4;
        /// Disable user collapsing window by double-clicking on it.
        const NoCollapse = 1 << 5;
        /// Resize every window to its content every frame.
        const AlwaysAutoResize = 1 << 6;
        /// Disable drawing background color (WindowBg, etc.) and outside border
        const NoBackground = 1 << 7;
        /// Never load/save settings in .ini file.
        const NoSavedSettings = 1 << 8;
        /// Disable catching mouse, hovering test with pass through.
        const NoMouseInputs = 1 << 9;
        /// Has a menu-bar.
        const MenuBar = 1 << 10;
        /// Allow horizontal scrollbar to appear (off by default).
        const HorizontalScrollbar = 1 << 11;
        /// Disable taking focus when transitioning from hidden to visible state.
        const NoFocusOnAppearing = 1 << 12;
        /// Disable bringing window to front when taking focus (e.g. clicking on it or
        /// programmatically giving it focus).
        const NoBringToFrontOnFocus = 1 << 13;
        /// Always show vertical scrollbar.
        const AlwaysVerticalScrollbar = 1 << 14;
        /// Always show horizontal scrollbar.
        const AlwaysHorizontalScrollbar = 1<< 15;
        /// Ensure child windows without border use window padding (ignored by default for
        /// non-bordered child windows, because more convenient).
        const AlwaysUseWindowPadding = 1 << 16;
        /// No gamepad/keyboard navigation within the window.
        const NoNavInputs = 1 << 18;
        /// No focusing toward this window with gamepad/keyboard navigation (e.g. skipped by
        /// CTRL+TAB).
        const NoNavFocus = 1 << 19;

        const NoNav = ImGuiWindowFlags::NoNavInputs.bits | ImGuiWindowFlags::NoNavFocus.bits;
        const NoDecoration = ImGuiWindowFlags::NoTitleBar.bits | ImGuiWindowFlags::NoResize.bits
            | ImGuiWindowFlags::NoScrollbar.bits | ImGuiWindowFlags::NoCollapse.bits;
        const NoInputs = ImGuiWindowFlags::NoMouseInputs.bits | ImGuiWindowFlags::NoNavInputs.bits
            | ImGuiWindowFlags::NoNavFocus.bits;
    }
);

impl Context {
    #[deprecated(since = "0.1.0", note = "Access Io::ini_saving_rate directly instead")]
    pub fn set_ini_saving_rate(&mut self, value: f32) {
        let io = self.io_mut();
        io.ini_saving_rate = value;
    }
    #[deprecated(
        since = "0.1.0",
        note = "Access Io::font_global_scale directly instead"
    )]
    pub fn set_font_global_scale(&mut self, value: f32) {
        let io = self.io_mut();
        io.font_global_scale = value;
    }
    #[deprecated(
        since = "0.1.0",
        note = "Access Io::mouse_double_click_time directly instead"
    )]
    pub fn set_mouse_double_click_time(&mut self, value: f32) {
        let io = self.io_mut();
        io.mouse_double_click_time = value;
    }
    #[deprecated(
        since = "0.1.0",
        note = "Access Io::mouse_double_click_max_dist directly instead"
    )]
    pub fn set_mouse_double_click_max_dist(&mut self, value: f32) {
        let io = self.io_mut();
        io.mouse_double_click_max_dist = value;
    }
    #[deprecated(
        since = "0.1.0",
        note = "Access Io::mouse_drag_threshold directly instead"
    )]
    pub fn set_mouse_drag_threshold(&mut self, value: f32) {
        let io = self.io_mut();
        io.mouse_drag_threshold = value;
    }
    #[deprecated(since = "0.1.0", note = "Access Io::key_repeat_delay directly instead")]
    pub fn set_key_repeat_delay(&mut self, value: f32) {
        let io = self.io_mut();
        io.key_repeat_delay = value;
    }
    #[deprecated(since = "0.1.0", note = "Access Io::key_repeat_rate directly instead")]
    pub fn set_key_repeat_rate(&mut self, value: f32) {
        let io = self.io_mut();
        io.key_repeat_rate = value;
    }
    #[deprecated(since = "0.1.0", note = "Access Io::display_size directly instead")]
    pub fn display_size(&self) -> (f32, f32) {
        let io = self.io();
        (io.display_size[0], io.display_size[1])
    }
    #[deprecated(
        since = "0.1.0",
        note = "Access Io::display_framebuffer_scale directly instead"
    )]
    pub fn display_framebuffer_scale(&self) -> (f32, f32) {
        let io = self.io();
        (
            io.display_framebuffer_scale[0],
            io.display_framebuffer_scale[1],
        )
    }
    #[deprecated(since = "0.1.0", note = "Access Io::mouse_pos directly instead")]
    pub fn mouse_pos(&self) -> (f32, f32) {
        let io = self.io();
        (io.mouse_pos[0], io.mouse_pos[1])
    }
    #[deprecated(since = "0.1.0", note = "Access Io::mouse_pos directly instead")]
    pub fn set_mouse_pos(&mut self, x: f32, y: f32) {
        let io = self.io_mut();
        io.mouse_pos = [x, y];
    }
    /// Get mouse's position's delta between the current and the last frame.
    #[deprecated(since = "0.1.0", note = "Access Io::mouse_delta directly instead")]
    pub fn mouse_delta(&self) -> (f32, f32) {
        let io = self.io();
        (io.mouse_delta[0], io.mouse_delta[1])
    }
    #[deprecated(since = "0.1.0", note = "Access Io::mouse_down directly instead")]
    pub fn mouse_down(&self) -> [bool; 5] {
        let io = self.io();
        io.mouse_down
    }
    #[deprecated(since = "0.1.0", note = "Access Io::mouse_down directly instead")]
    pub fn set_mouse_down(&mut self, states: [bool; 5]) {
        let io = self.io_mut();
        io.mouse_down = states;
    }
    #[deprecated(since = "0.1.0", note = "Access Io::mouse_wheel directly instead")]
    pub fn set_mouse_wheel(&mut self, value: f32) {
        let io = self.io_mut();
        io.mouse_wheel = value;
    }
    /// Get mouse wheel delta
    #[deprecated(since = "0.1.0", note = "Access Io::mouse_wheel directly instead")]
    pub fn mouse_wheel(&self) -> f32 {
        let io = self.io();
        io.mouse_wheel
    }
    #[deprecated(since = "0.1.0", note = "Use Ui::mouse_drag_delta instead")]
    pub fn mouse_drag_delta(&self, button: MouseButton) -> (f32, f32) {
        let delta = unsafe { sys::igGetMouseDragDelta_nonUDT2(button as c_int, -1.0) };
        delta.into()
    }
    /// Set to `true` to have ImGui draw the cursor in software.
    /// If `false`, the OS cursor is used (default to `false`).
    #[deprecated(
        since = "0.1.0",
        note = "Access Io::mouse_draw_cursor directly instead"
    )]
    pub fn set_mouse_draw_cursor(&mut self, value: bool) {
        let io = self.io_mut();
        io.mouse_draw_cursor = value;
    }
    #[deprecated(
        since = "0.1.0",
        note = "Access Io::mouse_draw_cursor directly instead"
    )]
    pub fn mouse_draw_cursor(&self) -> bool {
        let io = self.io();
        io.mouse_draw_cursor
    }
    /// Returns `true` if mouse is currently dragging with the `button` provided
    /// as argument.
    #[deprecated(since = "0.1.0", note = "Use Ui::is_mouse_dragging instead")]
    pub fn is_mouse_dragging(&self, button: MouseButton) -> bool {
        unsafe { sys::igIsMouseDragging(button as c_int, -1.0) }
    }
    /// Returns `true` if the `button` provided as argument is currently down.
    #[deprecated(since = "0.1.0", note = "Use Ui::is_mouse_down instead")]
    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        unsafe { sys::igIsMouseDown(button as c_int) }
    }
    /// Returns `true` if the `button` provided as argument is being clicked.
    #[deprecated(since = "0.1.0", note = "Use Ui::is_mouse_clicked instead")]
    pub fn is_mouse_clicked(&self, button: MouseButton) -> bool {
        unsafe { sys::igIsMouseClicked(button as c_int, false) }
    }
    /// Returns `true` if the `button` provided as argument is being double-clicked.
    #[deprecated(since = "0.1.0", note = "Use Ui::is_mouse_double_clicked instead")]
    pub fn is_mouse_double_clicked(&self, button: MouseButton) -> bool {
        unsafe { sys::igIsMouseDoubleClicked(button as c_int) }
    }
    /// Returns `true` if the `button` provided as argument was released
    #[deprecated(since = "0.1.0", note = "Use Ui::is_mouse_released instead")]
    pub fn is_mouse_released(&self, button: MouseButton) -> bool {
        unsafe { sys::igIsMouseReleased(button as c_int) }
    }
    #[deprecated(since = "0.1.0", note = "Access Io::key_ctrl directly instead")]
    pub fn key_ctrl(&self) -> bool {
        let io = self.io();
        io.key_ctrl
    }
    #[deprecated(since = "0.1.0", note = "Access Io::key_ctrl directly instead")]
    pub fn set_key_ctrl(&mut self, value: bool) {
        let io = self.io_mut();
        io.key_ctrl = value;
    }
    #[deprecated(since = "0.1.0", note = "Access Io::key_shift directly instead")]
    pub fn key_shift(&self) -> bool {
        let io = self.io();
        io.key_shift
    }
    #[deprecated(since = "0.1.0", note = "Access Io::key_shift directly instead")]
    pub fn set_key_shift(&mut self, value: bool) {
        let io = self.io_mut();
        io.key_shift = value;
    }
    #[deprecated(since = "0.1.0", note = "Access Io::key_alt directly instead")]
    pub fn key_alt(&self) -> bool {
        let io = self.io();
        io.key_alt
    }
    #[deprecated(since = "0.1.0", note = "Access Io::key_alt directly instead")]
    pub fn set_key_alt(&mut self, value: bool) {
        let io = self.io_mut();
        io.key_alt = value;
    }
    #[deprecated(since = "0.1.0", note = "Access Io::key_super directly instead")]
    pub fn key_super(&self) -> bool {
        let io = self.io();
        io.key_super
    }
    #[deprecated(since = "0.1.0", note = "Access Io::key_super directly instead")]
    pub fn set_key_super(&mut self, value: bool) {
        let io = self.io_mut();
        io.key_super = value;
    }
    #[deprecated(since = "0.1.0", note = "Access Io::keys_down directly instead")]
    pub fn set_key(&mut self, key: u8, pressed: bool) {
        let io = self.io_mut();
        io.keys_down[key as usize] = pressed;
    }
    #[deprecated(since = "0.1.0", note = "Index Io::key_map with the key instead")]
    pub fn set_imgui_key(&mut self, key: Key, mapping: u8) {
        let io = self.io_mut();
        io.key_map[key as usize] = u32::from(mapping);
    }
    /// Map [`Key`] values into user's key index
    #[deprecated(since = "0.1.0", note = "Index Io::key_map with the key instead")]
    pub fn get_key_index(&self, key: Key) -> usize {
        unsafe { sys::igGetKeyIndex(key as i32) as usize }
    }
    /// Return whether specific key is being held
    ///
    /// # Example
    ///
    /// ```rust
    /// use imgui::{Key, Ui};
    ///
    /// fn test(ui: &Ui) {
    ///     let delete_key_index = ui.imgui().get_key_index(Key::Delete);
    ///     if ui.imgui().is_key_down(delete_key_index) {
    ///         println!("Delete is being held!");
    ///     }
    /// }
    /// ```
    #[deprecated(since = "0.1.0", note = "Use Ui::is_key_down instead")]
    pub fn is_key_down(&self, user_key_index: usize) -> bool {
        unsafe { sys::igIsKeyDown(user_key_index as c_int) }
    }
    /// Return whether specific key was pressed
    #[deprecated(since = "0.1.0", note = "Use Ui::is_key_pressed instead")]
    pub fn is_key_pressed(&self, user_key_index: usize) -> bool {
        unsafe { sys::igIsKeyPressed(user_key_index as c_int, true) }
    }
    /// Return whether specific key was released
    #[deprecated(since = "0.1.0", note = "Use Ui::is_key_released instead")]
    pub fn is_key_released(&self, user_key_index: usize) -> bool {
        unsafe { sys::igIsKeyReleased(user_key_index as c_int) }
    }
    #[deprecated(since = "0.1.0", note = "Use Io::add_input_character instead")]
    pub fn add_input_character(&mut self, character: char) {
        let mut buf = [0; 5];
        character.encode_utf8(&mut buf);
        unsafe {
            sys::ImGuiIO_AddInputCharactersUTF8(self.io_mut().raw_mut(), buf.as_ptr() as *const _);
        }
    }
    #[deprecated(since = "0.1.0", note = "Access Io::framerate directly instead")]
    pub fn get_frame_rate(&self) -> f32 {
        self.io().framerate
    }
}

impl<'ui> Ui<'ui> {
    #[deprecated(
        since = "0.1.0",
        note = "This function is potentially unsafe and will be removed"
    )]
    pub fn imgui(&self) -> &Context {
        self.ctx
    }
    #[deprecated(
        since = "0.1.0",
        note = "Access Io::want_capture_mouse directly instead"
    )]
    pub fn want_capture_mouse(&self) -> bool {
        let io = self.io();
        io.want_capture_mouse
    }
    #[deprecated(
        since = "0.1.0",
        note = "Access Io::want_capture_keyboard directly instead"
    )]
    pub fn want_capture_keyboard(&self) -> bool {
        let io = self.io();
        io.want_capture_keyboard
    }
    #[deprecated(since = "0.1.0", note = "Access Io::framerate directly instead")]
    pub fn framerate(&self) -> f32 {
        let io = self.io();
        io.framerate
    }
    #[deprecated(
        since = "0.1.0",
        note = "Access Io::metrics_render_vertices directly instead"
    )]
    pub fn metrics_render_vertices(&self) -> i32 {
        let io = self.io();
        io.metrics_render_vertices
    }
    #[deprecated(
        since = "0.1.0",
        note = "Access Io::metrics_render_indices directly instead"
    )]
    pub fn metrics_render_indices(&self) -> i32 {
        let io = self.io();
        io.metrics_render_indices
    }
    #[deprecated(
        since = "0.1.0",
        note = "Access Io::metrics_active_windows directly instead"
    )]
    pub fn metrics_active_windows(&self) -> i32 {
        let io = self.io();
        io.metrics_active_windows
    }
}

#[deprecated(since = "0.1.0", note = "Use dear_imgui_version instead")]
pub fn get_version() -> &'static str {
    unsafe {
        let bytes = CStr::from_ptr(sys::igGetVersion()).to_bytes();
        str::from_utf8_unchecked(bytes)
    }
}

impl<'ui> Ui<'ui> {
    #[deprecated(since = "0.1.0", note = "use Ui::push_style_color instead")]
    pub fn with_color_var<T, F>(&self, style_color: StyleColor, color: [f32; 4], f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let _token = self.push_style_color(style_color, color);
        f()
    }
    #[deprecated(since = "0.1.0", note = "use Ui::push_style_colors instead")]
    pub fn with_color_vars<'a, T, F, I>(&self, style_colors: I, f: F) -> T
    where
        F: FnOnce() -> T,
        I: IntoIterator<Item = &'a (StyleColor, [f32; 4])>,
    {
        let _token = self.push_style_colors(style_colors);
        f()
    }
    #[deprecated(since = "0.1.0", note = "use Ui::push_style_var instead")]
    pub fn with_style_var<T, F>(&self, style_var: StyleVar, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let _token = self.push_style_var(style_var);
        f()
    }
    #[deprecated(since = "0.1.0", note = "use Ui::push_style_vars instead")]
    pub fn with_style_vars<'a, T, F, I>(&self, style_vars: I, f: F) -> T
    where
        F: FnOnce() -> T,
        I: IntoIterator<Item = &'a StyleVar>,
    {
        let _token = self.push_style_vars(style_vars);
        f()
    }
    #[deprecated(since = "0.1.0", note = "use Ui::push_item_width instead")]
    pub fn with_item_width<T, F>(&self, item_width: f32, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let _token = self.push_item_width(item_width);
        f()
    }
    #[deprecated(since = "0.1.0", note = "use Ui::push_text_wrap_pos instead")]
    pub fn with_text_wrap_pos<T, F>(&self, wrap_pos_x: f32, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let _token = self.push_text_wrap_pos(wrap_pos_x);
        f()
    }
}
