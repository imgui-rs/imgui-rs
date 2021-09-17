#![allow(clippy::float_cmp)]
use bitflags::bitflags;

use crate::input::mouse::MouseButton;
use crate::style::StyleColor;
use crate::Ui;
use crate::{sys, Direction};

bitflags! {
    /// Item hover check option flags
    #[repr(transparent)]
    pub struct ItemHoveredFlags: u32 {
        /// Return true even if a popup window is blocking access to this item
        const ALLOW_WHEN_BLOCKED_BY_POPUP = sys::ImGuiHoveredFlags_AllowWhenBlockedByPopup;
        /// Return true even if an active item is blocking access to this item
        const ALLOW_WHEN_BLOCKED_BY_ACTIVE_ITEM = sys::ImGuiHoveredFlags_AllowWhenBlockedByActiveItem;
        /// Return true even if the position is obstructed or overlapped by another window
        const ALLOW_WHEN_OVERLAPPED = sys::ImGuiHoveredFlags_AllowWhenOverlapped;
        /// Return true even if the item is disabled
        const ALLOW_WHEN_DISABLED = sys::ImGuiHoveredFlags_AllowWhenDisabled;
        const RECT_ONLY = sys::ImGuiHoveredFlags_RectOnly;
    }
}

/// # Item/widget utilities
impl<'ui> Ui<'ui> {
    /// Returns `true` if the last item is hovered
    #[doc(alias = "IsItemHovered")]
    pub fn is_item_hovered(&self) -> bool {
        unsafe { sys::igIsItemHovered(0) }
    }
    /// Returns `true` if the last item is hovered based on the given flags
    #[doc(alias = "IsItemHovered")]
    pub fn is_item_hovered_with_flags(&self, flags: ItemHoveredFlags) -> bool {
        unsafe { sys::igIsItemHovered(flags.bits() as i32) }
    }
    /// Returns `true` if the last item is active
    #[doc(alias = "IsItemActive")]
    pub fn is_item_active(&self) -> bool {
        unsafe { sys::igIsItemActive() }
    }
    #[doc(alias = "IsItemFocused")]
    /// Returns `true` if the last item is focused for keyboard/gamepad navigation
    pub fn is_item_focused(&self) -> bool {
        unsafe { sys::igIsItemFocused() }
    }
    /// Returns `true` if the last item is being clicked by `MouseButton::Left`.
    ///
    /// This is the same as [is_item_clicked_with_button](Self::is_item_clicked_with_button)
    /// with `button` set to `MouseButton::Left`.
    #[doc(alias = "IsItemClicked")]
    pub fn is_item_clicked(&self) -> bool {
        self.is_item_clicked_with_button(MouseButton::Left)
    }

    /// Returns `true` if the last item is being clicked
    #[doc(alias = "IsItemClicked")]
    pub fn is_item_clicked_with_button(&self, button: MouseButton) -> bool {
        unsafe { sys::igIsItemClicked(button as i32) }
    }
    /// Returns `true` if the last item is visible
    #[doc(alias = "IsItemVisible")]
    pub fn is_item_visible(&self) -> bool {
        unsafe { sys::igIsItemVisible() }
    }
    /// Returns `true` if the last item modified its underlying value this frame or was pressed
    #[doc(alias = "IsItemEdited")]
    pub fn is_item_edited(&self) -> bool {
        unsafe { sys::igIsItemEdited() }
    }
    /// Returns `true` if the last item was just made active
    #[doc(alias = "IsItemActivated")]
    pub fn is_item_activated(&self) -> bool {
        unsafe { sys::igIsItemActivated() }
    }
    /// Returns `true` if the last item was just made inactive
    #[doc(alias = "IsItemDeactivated")]
    pub fn is_item_deactivated(&self) -> bool {
        unsafe { sys::igIsItemDeactivated() }
    }
    /// Returns `true` if the last item was just made inactive and made a value change when it was
    #[doc(alias = "IsItemDeactivatedAfterEdit")]
    /// active
    pub fn is_item_deactivated_after_edit(&self) -> bool {
        unsafe { sys::igIsItemDeactivatedAfterEdit() }
    }
    /// Returns `true` if the last item open state was toggled
    #[doc(alias = "IsItemToggledOpen")]
    pub fn is_item_toggled_open(&self) -> bool {
        unsafe { sys::igIsItemToggledOpen() }
    }
    /// Returns `true` if any item is hovered
    #[doc(alias = "IsAnyItemHovered")]
    pub fn is_any_item_hovered(&self) -> bool {
        unsafe { sys::igIsAnyItemHovered() }
    }
    /// Returns `true` if any item is active
    #[doc(alias = "IsAnyItemActive")]
    pub fn is_any_item_active(&self) -> bool {
        unsafe { sys::igIsAnyItemActive() }
    }
    /// Returns `true` if any item is focused
    #[doc(alias = "IsAnyItemFocused")]
    pub fn is_any_item_focused(&self) -> bool {
        unsafe { sys::igIsAnyItemFocused() }
    }
    /// Returns the upper-left bounding rectangle of the last item (in screen coordinates)
    #[doc(alias = "GetItemRectMin")]
    pub fn item_rect_min(&self) -> [f32; 2] {
        let mut out = sys::ImVec2::zero();
        unsafe { sys::igGetItemRectMin(&mut out) }
        out.into()
    }
    /// Returns the lower-right bounding rectangle of the last item (in screen coordinates)
    #[doc(alias = "GetItemRectMax")]
    pub fn item_rect_max(&self) -> [f32; 2] {
        let mut out = sys::ImVec2::zero();
        unsafe { sys::igGetItemRectMax(&mut out) }
        out.into()
    }
    /// Returns the size of the last item
    #[doc(alias = "GetItemRectSize")]
    pub fn item_rect_size(&self) -> [f32; 2] {
        let mut out = sys::ImVec2::zero();
        unsafe { sys::igGetItemRectSize(&mut out) }
        out.into()
    }
    /// Allows the last item to be overlapped by a subsequent item.
    ///
    /// Both may be activated during the same frame before the later one takes priority.
    #[doc(alias = "SetItemAllowOverlap")]
    pub fn set_item_allow_overlap(&self) {
        unsafe { sys::igSetItemAllowOverlap() };
    }
    /// Makes the last item the default focused item of the window
    #[doc(alias = "SetItemDefaultFocus")]
    pub fn set_item_default_focus(&self) {
        unsafe { sys::igSetItemDefaultFocus() };
    }
}

/// # Miscellaneous utilities
impl<'ui> Ui<'ui> {
    /// Returns `true` if the rectangle (of given size, starting from cursor position) is visible
    #[doc(alias = "IsRectVisibleNil")]
    pub fn is_cursor_rect_visible(&self, size: [f32; 2]) -> bool {
        unsafe { sys::igIsRectVisible_Nil(size.into()) }
    }
    /// Returns `true` if the rectangle (in screen coordinates) is visible
    #[doc(alias = "IsRectVisibleNilVec2")]
    pub fn is_rect_visible(&self, rect_min: [f32; 2], rect_max: [f32; 2]) -> bool {
        unsafe { sys::igIsRectVisible_Vec2(rect_min.into(), rect_max.into()) }
    }
    /// Returns the global imgui-rs time.
    ///
    /// Incremented by Io::delta_time every frame.
    #[doc(alias = "GetTime")]
    pub fn time(&self) -> f64 {
        unsafe { sys::igGetTime() }
    }
    /// Returns the global imgui-rs frame count.
    ///
    /// Incremented by 1 every frame.
    #[doc(alias = "GetFrameCount")]
    pub fn frame_count(&self) -> i32 {
        unsafe { sys::igGetFrameCount() }
    }
    /// Returns a single style color from the user interface style.
    ///
    /// Use this function if you need to access the colors, but don't want to clone the entire
    /// style object.
    #[doc(alias = "GetStyle")]
    pub fn style_color(&self, style_color: StyleColor) -> [f32; 4] {
        self.ctx.style()[style_color]
    }
    /// Returns the current [`Style::alpha`](crate::Style::alpha).
    #[doc(alias = "GetStyle")]
    pub fn style_alpha(&self) -> f32 {
        self.ctx.style().alpha
    }
    /// Returns the current [`Style::disabled_alpha`](crate::Style::disabled_alpha).
    #[doc(alias = "GetStyle")]
    pub fn style_disabled_alpha(&self) -> f32 {
        self.ctx.style().disabled_alpha
    }
    /// Returns the current [`Style::window_padding`](crate::Style::window_padding).
    #[doc(alias = "GetStyle")]
    pub fn style_window_padding(&self) -> [f32; 2] {
        self.ctx.style().window_padding
    }
    /// Returns the current [`Style::window_rounding`](crate::Style::window_rounding).
    #[doc(alias = "GetStyle")]
    pub fn style_window_rounding(&self) -> f32 {
        self.ctx.style().window_rounding
    }
    /// Returns the current [`Style::window_border_size`](crate::Style::window_border_size).
    #[doc(alias = "GetStyle")]
    pub fn style_window_border_size(&self) -> f32 {
        self.ctx.style().window_border_size
    }
    /// Returns the current [`Style::window_min_size`](crate::Style::window_min_size).
    #[doc(alias = "GetStyle")]
    pub fn style_window_min_size(&self) -> [f32; 2] {
        self.ctx.style().window_min_size
    }
    /// Returns the current [`Style::window_title_align`](crate::Style::window_title_align).
    #[doc(alias = "GetStyle")]
    pub fn style_window_title_align(&self) -> [f32; 2] {
        self.ctx.style().window_title_align
    }
    /// Returns the current [`Style::window_menu_button_position`](crate::Style::window_menu_button_position).
    #[doc(alias = "GetStyle")]
    pub fn style_window_menu_button_position(&self) -> Direction {
        self.ctx.style().window_menu_button_position
    }
    /// Returns the current [`Style::child_rounding`](crate::Style::child_rounding).
    #[doc(alias = "GetStyle")]
    pub fn style_child_rounding(&self) -> f32 {
        self.ctx.style().child_rounding
    }
    /// Returns the current [`Style::child_border_size`](crate::Style::child_border_size).
    #[doc(alias = "GetStyle")]
    pub fn style_child_border_size(&self) -> f32 {
        self.ctx.style().child_border_size
    }
    /// Returns the current [`Style::popup_rounding`](crate::Style::popup_rounding).
    #[doc(alias = "GetStyle")]
    pub fn style_popup_rounding(&self) -> f32 {
        self.ctx.style().popup_rounding
    }
    /// Returns the current [`Style::popup_border_size`](crate::Style::popup_border_size).
    #[doc(alias = "GetStyle")]
    pub fn style_popup_border_size(&self) -> f32 {
        self.ctx.style().popup_border_size
    }
    /// Returns the current [`Style::frame_padding`](crate::Style::frame_padding).
    #[doc(alias = "GetStyle")]
    pub fn style_frame_padding(&self) -> [f32; 2] {
        self.ctx.style().frame_padding
    }
    /// Returns the current [`Style::frame_rounding`](crate::Style::frame_rounding).
    #[doc(alias = "GetStyle")]
    pub fn style_frame_rounding(&self) -> f32 {
        self.ctx.style().frame_rounding
    }
    /// Returns the current [`Style::frame_border_size`](crate::Style::frame_border_size).
    #[doc(alias = "GetStyle")]
    pub fn style_frame_border_size(&self) -> f32 {
        self.ctx.style().frame_border_size
    }
    /// Returns the current [`Style::item_spacing`](crate::Style::item_spacing).
    #[doc(alias = "GetStyle")]
    pub fn style_item_spacing(&self) -> [f32; 2] {
        self.ctx.style().item_spacing
    }
    /// Returns the current [`Style::item_inner_spacing`](crate::Style::item_inner_spacing).
    #[doc(alias = "GetStyle")]
    pub fn style_item_inner_spacing(&self) -> [f32; 2] {
        self.ctx.style().item_inner_spacing
    }
    /// Returns the current [`Style::cell_padding`](crate::Style::cell_padding).
    #[doc(alias = "GetStyle")]
    pub fn style_cell_padding(&self) -> [f32; 2] {
        self.ctx.style().cell_padding
    }
    /// Returns the current [`Style::touch_extra_padding`](crate::Style::touch_extra_padding).
    #[doc(alias = "GetStyle")]
    pub fn style_touch_extra_padding(&self) -> [f32; 2] {
        self.ctx.style().touch_extra_padding
    }
    /// Returns the current [`Style::indent_spacing`](crate::Style::indent_spacing).
    #[doc(alias = "GetStyle")]
    pub fn style_indent_spacing(&self) -> f32 {
        self.ctx.style().indent_spacing
    }
    /// Returns the current [`Style::columns_min_spacing`](crate::Style::columns_min_spacing).
    #[doc(alias = "GetStyle")]
    pub fn style_columns_min_spacing(&self) -> f32 {
        self.ctx.style().columns_min_spacing
    }
    /// Returns the current [`Style::scrollbar_size`](crate::Style::scrollbar_size).
    #[doc(alias = "GetStyle")]
    pub fn style_scrollbar_size(&self) -> f32 {
        self.ctx.style().scrollbar_size
    }
    /// Returns the current [`Style::scrollbar_rounding`](crate::Style::scrollbar_rounding).
    #[doc(alias = "GetStyle")]
    pub fn style_scrollbar_rounding(&self) -> f32 {
        self.ctx.style().scrollbar_rounding
    }
    /// Returns the current [`Style::grab_min_size`](crate::Style::grab_min_size).
    #[doc(alias = "GetStyle")]
    pub fn style_grab_min_size(&self) -> f32 {
        self.ctx.style().grab_min_size
    }
    /// Returns the current [`Style::grab_rounding`](crate::Style::grab_rounding).
    #[doc(alias = "GetStyle")]
    pub fn style_grab_rounding(&self) -> f32 {
        self.ctx.style().grab_rounding
    }
    /// Returns the current [`Style::log_slider_deadzone`](crate::Style::log_slider_deadzone).
    #[doc(alias = "GetStyle")]
    pub fn style_log_slider_deadzone(&self) -> f32 {
        self.ctx.style().log_slider_deadzone
    }
    /// Returns the current [`Style::tab_rounding`](crate::Style::tab_rounding).
    #[doc(alias = "GetStyle")]
    pub fn style_tab_rounding(&self) -> f32 {
        self.ctx.style().tab_rounding
    }
    /// Returns the current [`Style::tab_border_size`](crate::Style::tab_border_size).
    #[doc(alias = "GetStyle")]
    pub fn style_tab_border_size(&self) -> f32 {
        self.ctx.style().tab_border_size
    }
    /// Returns the current [`Style::tab_min_width_for_close_button`](crate::Style::tab_min_width_for_close_button).
    #[doc(alias = "GetStyle")]
    pub fn style_tab_min_width_for_close_button(&self) -> f32 {
        self.ctx.style().tab_min_width_for_close_button
    }
    /// Returns the current [`Style::color_button_position`](crate::Style::color_button_position).
    #[doc(alias = "GetStyle")]
    pub fn style_color_button_position(&self) -> Direction {
        self.ctx.style().color_button_position
    }
    /// Returns the current [`Style::button_text_align`](crate::Style::button_text_align).
    #[doc(alias = "GetStyle")]
    pub fn style_button_text_align(&self) -> [f32; 2] {
        self.ctx.style().button_text_align
    }
    /// Returns the current [`Style::selectable_text_align`](crate::Style::selectable_text_align).
    #[doc(alias = "GetStyle")]
    pub fn style_selectable_text_align(&self) -> [f32; 2] {
        self.ctx.style().selectable_text_align
    }
    /// Returns the current [`Style::display_window_padding`](crate::Style::display_window_padding).
    #[doc(alias = "GetStyle")]
    pub fn style_display_window_padding(&self) -> [f32; 2] {
        self.ctx.style().display_window_padding
    }
    /// Returns the current [`Style::display_safe_area_padding`](crate::Style::display_safe_area_padding).
    #[doc(alias = "GetStyle")]
    pub fn style_display_safe_area_padding(&self) -> [f32; 2] {
        self.ctx.style().display_safe_area_padding
    }
    /// Returns the current [`Style::mouse_cursor_scale`](crate::Style::mouse_cursor_scale).
    #[doc(alias = "GetStyle")]
    pub fn style_mouse_cursor_scale(&self) -> f32 {
        self.ctx.style().mouse_cursor_scale
    }
    /// Returns the current [`Style::anti_aliased_lines`](crate::Style::anti_aliased_lines).
    #[doc(alias = "GetStyle")]
    pub fn style_anti_aliased_lines(&self) -> bool {
        self.ctx.style().anti_aliased_lines
    }
    /// Returns the current [`Style::anti_aliased_lines_use_tex`](crate::Style::anti_aliased_lines_use_tex).
    #[doc(alias = "GetStyle")]
    pub fn style_anti_aliased_lines_use_tex(&self) -> bool {
        self.ctx.style().anti_aliased_lines_use_tex
    }
    /// Returns the current [`Style::anti_aliased_fill`](crate::Style::anti_aliased_fill).
    #[doc(alias = "GetStyle")]
    pub fn style_anti_aliased_fill(&self) -> bool {
        self.ctx.style().anti_aliased_fill
    }
    /// Returns the current [`Style::curve_tessellation_tol`](crate::Style::curve_tessellation_tol).
    #[doc(alias = "GetStyle")]
    pub fn style_curve_tessellation_tol(&self) -> f32 {
        self.ctx.style().curve_tessellation_tol
    }
    /// Returns the current [`Style::circle_tesselation_max_error`](crate::Style::circle_tesselation_max_error).
    #[doc(alias = "GetStyle")]
    pub fn style_circle_tesselation_max_error(&self) -> f32 {
        self.ctx.style().circle_tesselation_max_error
    }
}
