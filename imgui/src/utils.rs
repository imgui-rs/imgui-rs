#![allow(clippy::float_cmp)]
use bitflags::bitflags;

use crate::input::mouse::MouseButton;
use crate::math::MintVec2;
use crate::style::StyleColor;
use crate::sys;
use crate::Style;
use crate::Ui;

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
    pub fn is_cursor_rect_visible(&self, size: impl Into<MintVec2>) -> bool {
        unsafe { sys::igIsRectVisible_Nil(size.into().into()) }
    }
    /// Returns `true` if the rectangle (in screen coordinates) is visible
    #[doc(alias = "IsRectVisibleNilVec2")]
    pub fn is_rect_visible(
        &self,
        rect_min: impl Into<MintVec2>,
        rect_max: impl Into<MintVec2>,
    ) -> bool {
        unsafe { sys::igIsRectVisible_Vec2(rect_min.into().into(), rect_max.into().into()) }
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
        unsafe { self.style() }.colors[style_color as usize]
    }

    /// Returns a shared reference to the current [`Style`].
    ///
    /// ## Safety
    ///
    /// This function is tagged as `unsafe` because pushing via
    /// [`push_style_color`](crate::Ui::push_style_color) or
    /// [`push_style_var`](crate::Ui::push_style_var) or popping via
    /// [`ColorStackToken::pop`](crate::ColorStackToken::pop) or
    /// [`StyleStackToken::pop`](crate::StyleStackToken::pop) will modify the values in the returned
    /// shared reference. Therefore, you should not retain this reference across calls to push and
    /// pop. The [`clone_style`](Ui::clone_style) version may instead be used to avoid `unsafe`.
    #[doc(alias = "GetStyle")]
    pub unsafe fn style(&self) -> &Style {
        // safe because Style is a transparent wrapper around sys::ImGuiStyle
        &*(sys::igGetStyle() as *const Style)
    }
}
