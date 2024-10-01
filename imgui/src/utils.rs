#![allow(clippy::float_cmp)]

use bitflags::bitflags;

use crate::input::mouse::MouseButton;
use crate::math::MintVec2;
use crate::style::StyleColor;
use crate::sys;
use crate::Style;
use crate::Ui;
bitflags! {
    /// Flags for [`Ui::is_item_hovered`], [`Ui::is_window_hovered`]
    /// Note: if you are trying to check whether your mouse should be dispatched to
    /// `Dear ImGui` or to your app, you should use [`Io::want_capture_mouse`](crate::Io::want_capture_mouse)
    /// instead! Please read the FAQ!
    ///
    /// Note: windows with the [`WindowFlags::NO_INPUTS`](crate::WindowFlags::NO_INPUTS) flag
    /// are ignored by [`Ui::is_window_hovered`] calls.
    ///
    /// Note: [`HoveredFlags::empty`] will return true in the above functions
    /// if directly over the item/window, not obstructed by another window, not obstructed by an active popup or modal blocking inputs under them.
    #[repr(transparent)]
    pub struct HoveredFlags: u32 {
        /// [`Ui::is_item_hovered`] only: Return true if any children of the window is hovered
        const CHILD_WINDOWS = sys::ImGuiHoveredFlags_ChildWindows;
        /// [`Ui::is_item_hovered`] only: Test from root window (top most parent of the current hierarchy)
        const ROOT_WINDOW = sys::ImGuiHoveredFlags_RootWindow;
        /// [`Ui::is_item_hovered`] only: Return true if any window is hovered
        const ANY_WINDOW = sys::ImGuiHoveredFlags_AnyWindow;
        /// [`Ui::is_item_hovered`] only: Do not consider popup hierarchy (do not treat popup emitter as parent of popup) (when used with _ChildWindows or _RootWindow)
        const NO_POPUP_HIERARCHY = sys::ImGuiHoveredFlags_NoPopupHierarchy;
        /// Return true even if a popup window is normally blocking access to this item/window
        const ALLOW_WHEN_BLOCKED_BY_POPUP = sys::ImGuiHoveredFlags_AllowWhenBlockedByPopup;
        /// Return true even if an active item is blocking access to this item/window. Useful for Drag and Drop patterns.
        const ALLOW_WHEN_BLOCKED_BY_ACTIVE_ITEM = sys::ImGuiHoveredFlags_AllowWhenBlockedByActiveItem;
        /// [`Ui::is_item_hovered`] only: Return true even if the item uses AllowOverlap mode and is overlapped by another hoverable item.
        const ALLOW_WHEN_OVERLAPPED_BY_ITEM = sys::ImGuiHoveredFlags_AllowWhenOverlappedByItem;
        /// [`Ui::is_item_hovered`] only: Return true even if the position is obstructed or overlapped by another window.
        const ALLOW_WHEN_OVERLAPPED_BY_WINDOW = sys::ImGuiHoveredFlags_AllowWhenOverlappedByWindow;
        /// [`Ui::is_item_hovered`] only: Return true even if the item is disabled
        const ALLOW_WHEN_DISABLED = sys::ImGuiHoveredFlags_AllowWhenDisabled;
        /// [`Ui::is_item_hovered`] only: Disable using gamepad/keyboard navigation state when active, always query mouse
        const NO_NAV_OVERRIDE = sys::ImGuiHoveredFlags_NoNavOverride;

        /// Union of [`HoveredFlags::ALLOW_WHEN_OVERLAPPED_BY_ITEM`] and [`HoveredFlags::ALLOW_WHEN_OVERLAPPED_BY_WINDOW`],
        const ALLOW_WHEN_OVERLAPPED = Self::ALLOW_WHEN_OVERLAPPED_BY_ITEM.bits | Self::ALLOW_WHEN_OVERLAPPED_BY_WINDOW.bits;

        /// Union of [`HoveredFlags::ALLOW_WHEN_BLOCKED_BY_POPUP`], [`HoveredFlags::ALLOW_WHEN_BLOCKED_BY_ACTIVE_ITEM`],
        /// [`HoveredFlags::ALLOW_WHEN_OVERLAPPED_BY_ITEM`], and [`HoveredFlags::ALLOW_WHEN_OVERLAPPED_BY_WINDOW`],
        const RECT_ONLY = Self::ALLOW_WHEN_BLOCKED_BY_POPUP.bits | Self::ALLOW_WHEN_BLOCKED_BY_ACTIVE_ITEM.bits | Self::ALLOW_WHEN_OVERLAPPED.bits;

        /// Union of [`HoveredFlags::ROOT_WINDOW`], [`HoveredFlags::CHILD_WINDOWS`],
        const ROOT_AND_CHILD_WINDOWS = Self::ROOT_WINDOW.bits | Self::CHILD_WINDOWS.bits;

        /// Tooltips mode
        /// - typically used in [`Ui::is_item_hovered`] + [`Ui::tooltip`] sequence.
        /// - this is a shortcut to pull flags from [`Style::hover_flags_for_tooltip_mouse`] or
        ///   [`Style::hover_flags_for_tooltip_nav`] where you can reconfigure desired behavior.
        /// - for frequently actioned or hovered items providing a tooltip, you want may to use
        ///   [`HoveredFlags::FOR_TOOLTIP`] (stationary + delay) so the tooltip doesn't show too often.
        /// - for items which main purpose is to be hovered, or items with low affordance, or in less
        ///   consistent apps, prefer no delay or shorter delay.
        const FOR_TOOLTIP = sys::ImGuiHoveredFlags_ForTooltip;
        /// Require mouse to be stationary for [`Style::hover_stationary_delay`] (~0.15 sec)
        /// _at least one time_. After this, can move on same item/window.
        /// Using the stationary test tends to reduces the need for a long delay.
        const STATIONARY = sys::ImGuiHoveredFlags_Stationary;
        /// [`Ui::is_item_hovered`] only: Return true immediately (default).
        /// As this is the default you generally ignore this.
        const DELAY_NONE = sys::ImGuiHoveredFlags_DelayNone;
        /// [`Ui::is_item_hovered`] only: Return true after [`Style::hover_delay_short`]
        /// elapsed (~0.15 sec) (shared between items) + requires mouse to be stationary
        /// for [`Style::hover_stationary_delay`] (once per item).
        const DELAY_SHORT = sys::ImGuiHoveredFlags_DelayShort;
        // [`Ui::is_item_hovered`] only: Return true after [`Style::hover_delay_normal`]
        // elapsed (~0.40 sec) (shared between items) + requires mouse to be stationary
        /// for [`Style::hover_stationary_delay`] (once per item).
        const DELAY_NORMAL = sys::ImGuiHoveredFlags_DelayNormal;
        /// [`Ui::is_item_hovered`] only: Disable shared delay system where moving from one item to the next keeps
        /// the previous timer for a short time (standard for tooltips with long delays)
        const NO_SHARED_DELAY = sys::ImGuiHoveredFlags_NoSharedDelay;
    }
}

/// # Item/widget utilities
impl Ui {
    /// Returns `true` if the last item is hovered
    #[doc(alias = "IsItemHovered")]
    pub fn is_item_hovered(&self) -> bool {
        unsafe { sys::igIsItemHovered(0) }
    }
    /// Returns `true` if the last item is hovered based on the given flags
    #[doc(alias = "IsItemHovered")]
    pub fn is_item_hovered_with_flags(&self, flags: HoveredFlags) -> bool {
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
    #[doc(alias = "SetNextItemAllowOverlap")]
    pub fn set_item_allow_overlap(&self) {
        unsafe { sys::igSetNextItemAllowOverlap() };
    }
    /// Makes the last item the default focused item of the window
    #[doc(alias = "SetItemDefaultFocus")]
    pub fn set_item_default_focus(&self) {
        unsafe { sys::igSetItemDefaultFocus() };
    }
}

/// # Miscellaneous utilities
impl Ui {
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

    /// Gets the name of some style color.
    ///
    /// This is just a wrapper around calling [`name`] on [StyleColor].
    ///
    /// [`name`]: StyleColor::name
    #[doc(alias = "GetStyleColorName")]
    pub fn style_color_name(&self, style_color: StyleColor) -> &'static str {
        style_color.name()
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
