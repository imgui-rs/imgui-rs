use bitflags::bitflags;

use crate::input::mouse::MouseButton;
use crate::style::StyleColor;
use crate::sys;
use crate::Ui;

bitflags! {
    /// Item hover check option flags
    #[repr(transparent)]
    pub struct ItemHoveredFlags: u32 {
        /// Return true even if a popup window is blocking access to this item
        const ALLOW_WHEN_BLOCKED_BY_POPUP = sys::ImGuiHoveredFlags_AllowWhenBlockedByPopup as u32;
        /// Return true even if an active item is blocking access to this item
        const ALLOW_WHEN_BLOCKED_BY_ACTIVE_ITEM = sys::ImGuiHoveredFlags_AllowWhenBlockedByActiveItem as u32;
        /// Return true even if the position is obstructed or overlapped by another window
        const ALLOW_WHEN_OVERLAPPED = sys::ImGuiHoveredFlags_AllowWhenOverlapped as u32;
        /// Return true even if the item is disabled
        const ALLOW_WHEN_DISABLED = sys::ImGuiHoveredFlags_AllowWhenDisabled as u32;
        const RECT_ONLY = sys::ImGuiHoveredFlags_RectOnly as u32;
    }
}

/// # Item/widget utilities
impl<'ui> Ui<'ui> {
    /// Returns `true` if the last item is hovered
    pub fn is_item_hovered(&self) -> bool {
        unsafe { sys::igIsItemHovered(0) }
    }
    /// Returns `true` if the last item is hovered based on the given flags
    pub fn is_item_hovered_with_flags(&self, flags: ItemHoveredFlags) -> bool {
        unsafe { sys::igIsItemHovered(flags.bits() as i32) }
    }
    /// Returns `true` if the last item is active
    pub fn is_item_active(&self) -> bool {
        unsafe { sys::igIsItemActive() }
    }
    /// Returns `true` if the last item is focused for keyboard/gamepad navigation
    pub fn is_item_focused(&self) -> bool {
        unsafe { sys::igIsItemFocused() }
    }
    /// Returns `true` if the last item is being clicked
    pub fn is_item_clicked(&self, button: MouseButton) -> bool {
        unsafe { sys::igIsItemClicked(button as i32) }
    }
    /// Returns `true` if the last item is visible
    pub fn is_item_visible(&self) -> bool {
        unsafe { sys::igIsItemVisible() }
    }
    /// Returns `true` if the last item modified its underlying value this frame or was pressed
    pub fn is_item_edited(&self) -> bool {
        unsafe { sys::igIsItemEdited() }
    }
    /// Returns `true` if the last item was just made active
    pub fn is_item_activated(&self) -> bool {
        unsafe { sys::igIsItemActivated() }
    }
    /// Returns `true` if the last item was just made inactive
    pub fn is_item_deactivated(&self) -> bool {
        unsafe { sys::igIsItemDeactivated() }
    }
    /// Returns `true` if the last item was just made inactive and made a value change when it was
    /// active
    pub fn is_item_deactivated_after_edit(&self) -> bool {
        unsafe { sys::igIsItemDeactivatedAfterEdit() }
    }
    /// Returns `true` if the last item open state was toggled
    pub fn is_item_toggled_open(&self) -> bool {
        unsafe { sys::igIsItemToggledOpen() }
    }
    /// Returns `true` if any item is hovered
    pub fn is_any_item_hovered(&self) -> bool {
        unsafe { sys::igIsAnyItemHovered() }
    }
    /// Returns `true` if any item is active
    pub fn is_any_item_active(&self) -> bool {
        unsafe { sys::igIsAnyItemActive() }
    }
    /// Returns `true` if any item is focused
    pub fn is_any_item_focused(&self) -> bool {
        unsafe { sys::igIsAnyItemFocused() }
    }
    /// Returns the upper-left bounding rectangle of the last item (in screen coordinates)
    pub fn item_rect_min(&self) -> [f32; 2] {
        let mut out = sys::ImVec2::zero();
        unsafe { sys::igGetItemRectMin(&mut out) }
        out.into()
    }
    /// Returns the lower-right bounding rectangle of the last item (in screen coordinates)
    pub fn item_rect_max(&self) -> [f32; 2] {
        let mut out = sys::ImVec2::zero();
        unsafe { sys::igGetItemRectMax(&mut out) }
        out.into()
    }
    /// Returns the size of the last item
    pub fn item_rect_size(&self) -> [f32; 2] {
        let mut out = sys::ImVec2::zero();
        unsafe { sys::igGetItemRectSize(&mut out) }
        out.into()
    }
    /// Allows the last item to be overlapped by a subsequent item.
    ///
    /// Both may be activated during the same frame before the later one takes priority.
    pub fn set_item_allow_overlap(&self) {
        unsafe { sys::igSetItemAllowOverlap() };
    }
    /// Makes the last item the default focused item of the window
    pub fn set_item_default_focus(&self) {
        unsafe { sys::igSetItemDefaultFocus() };
    }
}

/// # Miscellaneous utilities
impl<'ui> Ui<'ui> {
    /// Returns `true` if the rectangle (of given size, starting from cursor position) is visible
    pub fn is_cursor_rect_visible(&self, size: [f32; 2]) -> bool {
        unsafe { sys::igIsRectVisibleNil(size.into()) }
    }
    /// Returns `true` if the rectangle (in screen coordinates) is visible
    pub fn is_rect_visible(&self, rect_min: [f32; 2], rect_max: [f32; 2]) -> bool {
        unsafe { sys::igIsRectVisibleVec2(rect_min.into(), rect_max.into()) }
    }
    /// Returns the global imgui-rs time.
    ///
    /// Incremented by Io::delta_time every frame.
    pub fn time(&self) -> f64 {
        unsafe { sys::igGetTime() }
    }
    /// Returns the global imgui-rs frame count.
    ///
    /// Incremented by 1 every frame.
    pub fn frame_count(&self) -> i32 {
        unsafe { sys::igGetFrameCount() }
    }
    /// Returns a single style color from the user interface style.
    ///
    /// Use this function if you need to access the colors, but don't want to clone the entire
    /// style object.
    pub fn style_color(&self, style_color: StyleColor) -> [f32; 4] {
        self.ctx.style()[style_color]
    }
}
