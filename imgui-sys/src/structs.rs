use std::os::raw::c_float;

use crate::enums::{ImGuiCol, ImGuiDir};
use crate::{ImVec2, ImVec4};

/// Runtime data for styling/colors
#[repr(C)]
#[derive(Clone)]
pub struct ImGuiStyle {
    /// Global alpha applies to everything in ImGui.
    pub alpha: c_float,
    /// Padding within a window.
    pub window_padding: ImVec2,
    /// Radius of window corners rounding. Set to 0.0 to have rectangular windows.
    pub window_rounding: c_float,
    /// Thickness of border around windows. Generally set to 0.0 or 1.0. (Other values are not well
    /// tested and more CPU/GPU costly).
    pub window_border_size: c_float,
    /// Minimum window size. This is a global setting. If you want to constraint individual
    /// windows, use igSetNextWindowSizeConstraints().
    pub window_min_size: ImVec2,
    /// Alignment for title bar text. Defaults to (0.0, 0.5) for left-aligned, vertically centered.
    pub window_title_align: ImVec2,
    pub window_menu_button_position: ImGuiDir,
    /// Radius of child window corners rounding. Set to 0.0 to have rectangular windows.
    pub child_rounding: c_float,
    /// Thickness of border around child windows. Generally set to 0.0 or 1.0. (Other values are
    /// not well tested and more CPU/GPU costly).
    pub child_border_size: c_float,
    /// Radius of popup window corners rounding. (Note that tooltip windows use window_rounding)
    pub popup_rounding: c_float,
    /// Thickness of border around popup/tooltip windows. Generally set to 0.0 or 1.0. (Other
    /// values are not well tested and more CPU/GPU costly).
    pub popup_border_size: c_float,
    /// Padding within a framed rectangle (used by most widgets).
    pub frame_padding: ImVec2,
    /// Radius of frame corners rounding. Set to 0.0 to have rectangular frame (used by most
    /// widgets).
    pub frame_rounding: c_float,
    /// Thickness of border around frames. Generally set to 0.0 or 1.0. (Other values are not well
    /// tested and more CPU/GPU costly).
    pub frame_border_size: c_float,
    /// Horizontal and vertical spacing between widgets/lines.
    pub item_spacing: ImVec2,
    /// Horizontal and vertical spacing between within elements of a composed widget (e.g. a slider
    /// and its label).
    pub item_inner_spacing: ImVec2,
    /// Expand reactive bounding box for touch-based system where touch position is not accurate
    /// enough. Unfortunately we don't sort widgets so priority on overlap will always be given to
    /// the first widget. So don't grow this too much!
    pub touch_extra_padding: ImVec2,
    /// Horizontal indentation when e.g. entering a tree node. Generally == (FontSize +
    /// FramePadding.x*2).
    pub indent_spacing: c_float,
    /// Minimum horizontal spacing between two columns.
    pub columns_min_spacing: c_float,
    /// Width of the vertical scrollbar, Height of the horizontal scrollbar.
    pub scrollbar_size: c_float,
    /// Radius of grab corners for scrollbar.
    pub scrollbar_rounding: c_float,
    /// Minimum width/height of a grab box for slider/scrollbar.
    pub grab_min_size: c_float,
    /// Radius of grabs corners rounding. Set to 0.0 to have rectangular slider grabs.
    pub grab_rounding: c_float,
    pub tab_rounding: c_float,
    pub tab_border_size: c_float,
    /// Alignment of button text when button is larger than text. Defaults to (0.5, 0.5) for
    /// horizontally+vertically centered.
    pub button_text_align: ImVec2,
    pub selectable_text_align: ImVec2,
    /// Window position are clamped to be visible within the display area by at least this amount.
    /// Only applies to regular windows.
    pub display_window_padding: ImVec2,
    /// If you cannot see the edges of your screen (e.g. on a TV) increase the safe area padding.
    /// Apply to popups/tooltips as well regular windows. NB: Prefer configuring your TV sets
    /// correctly!
    pub display_safe_area_padding: ImVec2,
    /// Scale software rendered mouse cursor (when io.mouse_draw_cursor is enabled). May be removed
    /// later.
    pub mouse_cursor_scale: c_float,
    /// Enable anti-aliasing on lines/borders. Disable if you are really tight on CPU/GPU.
    pub anti_aliased_lines: bool,
    /// Enable anti-aliasing on filled shapes (rounded rectangles, circles, etc.)
    pub anti_aliased_fill: bool,
    /// Tessellation tolerance when using igPathBezierCurveTo() without a specific number of
    /// segments. Decrease for highly tessellated curves (higher quality, more polygons), increase
    /// to reduce quality.
    pub curve_tessellation_tol: c_float,
    /// Colors for the user interface
    pub colors: [ImVec4; ImGuiCol::COUNT],
}

// ImGuiStyle
extern "C" {
    pub fn ImGuiStyle_ScaleAllSizes(this: *mut ImGuiStyle, scale_factor: c_float);
}
