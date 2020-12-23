use bitflags::bitflags;
use std::borrow::Cow;
use std::ptr;
use std::thread;

use crate::context::Context;
use crate::string::ImStr;
use crate::sys;
use crate::Ui;

// TODO: support size constraints

/// Combo box height mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ComboBoxHeight {
    /// Max ~4 items visible.
    Small,
    /// Max ~8 items visible.
    Regular,
    /// Max ~20 items visible.
    Large,
    /// As many fitting items as possible visible.
    Largest,
}

/// Combo box preview mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ComboBoxPreviewMode {
    /// Show only a box with the preview value
    Label,
    /// Show only an arrow button
    ArrowButton,
    /// Show a box with the preview value and an arrow button
    Full,
}

bitflags!(
/// Flags for combo boxes
#[repr(transparent)]
pub struct ComboBoxFlags: u32 {
    /// Align the popup toward the left by default
    const POPUP_ALIGN_LEFT = sys::ImGuiComboFlags_PopupAlignLeft as u32;
    /// Max ~4 items visible.
    const HEIGHT_SMALL = sys::ImGuiComboFlags_HeightSmall as u32;
    /// Max ~8 items visible (default)
    const HEIGHT_REGULAR = sys::ImGuiComboFlags_HeightRegular as u32;
    /// Max ~20 items visible
    const HEIGHT_LARGE = sys::ImGuiComboFlags_HeightLarge as u32;
    /// As many fitting items as possible
    const HEIGHT_LARGEST = sys::ImGuiComboFlags_HeightLargest as u32;
    /// Display on the preview box without the square arrow button
    const NO_ARROW_BUTTON = sys::ImGuiComboFlags_NoArrowButton as u32;
    /// Display only a square arrow button
    const NO_PREVIEW = sys::ImGuiComboFlags_NoPreview as u32;
}
);

/// Builder for a combo box widget
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct ComboBox<'a> {
    label: &'a ImStr,
    preview_value: Option<&'a ImStr>,
    flags: ComboBoxFlags,
}

impl<'a> ComboBox<'a> {
    /// Constructs a new combo box builder.
    pub fn new(label: &'a ImStr) -> ComboBox<'a> {
        ComboBox {
            label,
            preview_value: None,
            flags: ComboBoxFlags::empty(),
        }
    }
    /// Sets the preview value displayed in the preview box (if visible).
    #[inline]
    pub fn preview_value(mut self, preview_value: &'a ImStr) -> Self {
        self.preview_value = Some(preview_value);
        self
    }
    /// Replaces all current settings with the given flags.
    #[inline]
    pub fn flags(mut self, flags: ComboBoxFlags) -> Self {
        self.flags = flags;
        self
    }
    /// Enables/disables aligning the combo box popup toward the left.
    ///
    /// Disabled by default.
    #[inline]
    pub fn popup_align_left(mut self, popup_align_left: bool) -> Self {
        self.flags
            .set(ComboBoxFlags::POPUP_ALIGN_LEFT, popup_align_left);
        self
    }
    /// Sets the combo box height.
    ///
    /// Default: `ComboBoxHeight::Regular`
    #[inline]
    pub fn height(mut self, height: ComboBoxHeight) -> Self {
        self.flags
            .set(ComboBoxFlags::HEIGHT_SMALL, height == ComboBoxHeight::Small);
        self.flags.set(
            ComboBoxFlags::HEIGHT_REGULAR,
            height == ComboBoxHeight::Regular,
        );
        self.flags
            .set(ComboBoxFlags::HEIGHT_LARGE, height == ComboBoxHeight::Large);
        self.flags.set(
            ComboBoxFlags::HEIGHT_LARGEST,
            height == ComboBoxHeight::Largest,
        );
        self
    }
    /// Sets the combo box preview mode.
    ///
    /// Default: `ComboBoxPreviewMode::Full`
    #[inline]
    pub fn preview_mode(mut self, preview_mode: ComboBoxPreviewMode) -> Self {
        self.flags.set(
            ComboBoxFlags::NO_ARROW_BUTTON,
            preview_mode == ComboBoxPreviewMode::Label,
        );
        self.flags.set(
            ComboBoxFlags::NO_PREVIEW,
            preview_mode == ComboBoxPreviewMode::ArrowButton,
        );
        self
    }
    /// Creates a combo box and starts appending to it.
    ///
    /// Returns `Some(ComboBoxToken)` if the combo box is open. After content has been
    /// rendered, the token must be ended by calling `.end()`.
    ///
    /// Returns `None` if the combo box is not open and no content should be rendered.
    #[must_use]
    pub fn begin(self, ui: &Ui) -> Option<ComboBoxToken> {
        let should_render = unsafe {
            sys::igBeginCombo(
                self.label.as_ptr(),
                self.preview_value.map(ImStr::as_ptr).unwrap_or(ptr::null()),
                self.flags.bits() as i32,
            )
        };
        if should_render {
            Some(ComboBoxToken { ctx: ui.ctx })
        } else {
            None
        }
    }
    /// Creates a combo box and runs a closure to construct the popup contents.
    ///
    /// Note: the closure is not called if the combo box is not open.
    pub fn build<F: FnOnce()>(self, ui: &Ui, f: F) {
        if let Some(combo) = self.begin(ui) {
            f();
            combo.end(ui);
        }
    }
}

/// Tracks a combo box that must be ended by calling `.end()`
#[must_use]
pub struct ComboBoxToken {
    ctx: *const Context,
}

impl ComboBoxToken {
    /// Ends a combo box
    pub fn end(mut self, _: &Ui) {
        self.ctx = ptr::null();
        unsafe { sys::igEndCombo() };
    }
}

impl Drop for ComboBoxToken {
    fn drop(&mut self) {
        if !self.ctx.is_null() && !thread::panicking() {
            panic!("A ComboBoxToken was leaked. Did you call .end()?");
        }
    }
}

/// # Convenience functions
impl<'a> ComboBox<'a> {
    /// Builds a simple combo box for choosing from a slice of values
    pub fn build_simple<T, L>(
        self,
        ui: &Ui,
        current_item: &mut usize,
        items: &[T],
        label_fn: &L,
    ) -> bool
    where
        for<'b> L: Fn(&'b T) -> Cow<'b, ImStr>,
    {
        use crate::widget::selectable::Selectable;
        let mut result = false;
        let mut cb = self;
        let preview_value = items.get(*current_item).map(label_fn);
        if cb.preview_value.is_none() {
            if let Some(preview_value) = preview_value.as_ref() {
                cb = cb.preview_value(preview_value);
            }
        }
        if let Some(_cb) = cb.begin(ui) {
            for (idx, item) in items.iter().enumerate() {
                let text = label_fn(item);
                let selected = idx == *current_item;
                if Selectable::new(&text).selected(selected).build(ui) {
                    *current_item = idx;
                    result = true;
                }
            }
            _cb.end(ui);
        }
        result
    }
    /// Builds a simple combo box for choosing from a slice of strings
    pub fn build_simple_string<S>(self, ui: &Ui, current_item: &mut usize, items: &[&S]) -> bool
    where
        S: AsRef<ImStr> + ?Sized,
    {
        self.build_simple(ui, current_item, items, &|&s| s.as_ref().into())
    }
}
