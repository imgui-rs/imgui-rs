use bitflags::bitflags;

use crate::string::ImStr;
use crate::sys;
use crate::Ui;

bitflags!(
    /// Flags for selectables
    #[repr(transparent)]
    pub struct SelectableFlags: u32 {
        /// Clicking this don't close parent popup window
        const DONT_CLOSE_POPUPS = sys::ImGuiSelectableFlags_DontClosePopups as u32;
        /// Selectable frame can span all columns (text will still fit in current column)
        const SPAN_ALL_COLUMNS = sys::ImGuiSelectableFlags_SpanAllColumns as u32;
        /// Generate press events on double clicks too
        const ALLOW_DOUBLE_CLICK = sys::ImGuiSelectableFlags_AllowDoubleClick as u32;
        /// Cannot be selected, display greyed out text
        const DISABLED = sys::ImGuiSelectableFlags_Disabled as u32;
        /// (WIP) Hit testing to allow subsequent willdgets to overlap this one
        const ALLOW_ITEM_OVERLAP = sys::ImGuiSelectableFlags_AllowItemOverlap as u32;
    }
);

/// Builder for a selectable widget.
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct Selectable<'a> {
    label: &'a ImStr,
    selected: bool,
    flags: SelectableFlags,
    size: [f32; 2],
}

impl<'a> Selectable<'a> {
    /// Constructs a new selectable builder.
    pub fn new(label: &ImStr) -> Selectable {
        Selectable {
            label,
            selected: false,
            flags: SelectableFlags::empty(),
            size: [0.0, 0.0],
        }
    }
    /// Replaces all current settings with the given flags
    #[inline]
    pub fn flags(mut self, flags: SelectableFlags) -> Self {
        self.flags = flags;
        self
    }
    /// Sets the selected state of the selectable
    #[inline]
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
    /// Enables/disables closing parent popup window on click.
    ///
    /// Default: enabled
    #[inline]
    pub fn close_popups(mut self, value: bool) -> Self {
        self.flags.set(SelectableFlags::DONT_CLOSE_POPUPS, !value);
        self
    }
    /// Enables/disables full column span (text will still fit in the current column).
    ///
    /// Default: disabled
    #[inline]
    pub fn span_all_columns(mut self, value: bool) -> Self {
        self.flags.set(SelectableFlags::SPAN_ALL_COLUMNS, value);
        self
    }
    /// Enables/disables click event generation on double clicks.
    ///
    /// Default: disabled
    #[inline]
    pub fn allow_double_click(mut self, value: bool) -> Self {
        self.flags.set(SelectableFlags::ALLOW_DOUBLE_CLICK, value);
        self
    }
    /// Enables/disables the selectable.
    ///
    /// When disabled, it cannot be selected and the text uses the disabled text color.
    ///
    /// Default: disabled
    #[inline]
    pub fn disabled(mut self, value: bool) -> Self {
        self.flags.set(SelectableFlags::DISABLED, value);
        self
    }
    /// Sets the size of the selectable.
    ///
    /// For the X axis:
    ///
    /// - `> 0.0`: use given width
    /// - `= 0.0`: use remaining width
    ///
    /// For the Y axis:
    ///
    /// - `> 0.0`: use given height
    /// - `= 0.0`: use label height
    #[inline]
    pub fn size(mut self, size: [f32; 2]) -> Self {
        self.size = size;
        self
    }
    /// Builds the selectable.
    ///
    /// Returns true if the selectable was clicked.
    pub fn build(self, _: &Ui) -> bool {
        unsafe {
            sys::igSelectableBool(
                self.label.as_ptr(),
                self.selected,
                self.flags.bits() as i32,
                self.size.into(),
            )
        }
    }
}

/// # Convenience functions
impl<'a> Selectable<'a> {
    /// Builds the selectable using a mutable reference to selected state.
    pub fn build_with_ref(self, ui: &Ui, selected: &mut bool) -> bool {
        if self.selected(*selected).build(ui) {
            *selected = !*selected;
            true
        } else {
            false
        }
    }
}
