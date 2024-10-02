use bitflags::bitflags;

use crate::math::MintVec2;
use crate::sys;
use crate::Ui;

bitflags!(
    /// Flags for selectables
    #[repr(transparent)]
    pub struct SelectableFlags: u32 {
        /// Clicking this don't close parent popup window
        const NO_AUTO_CLOSE_POPUPS = sys::ImGuiSelectableFlags_NoAutoClosePopups;
        /// Selectable frame can span all columns (text will still fit in current column)
        const SPAN_ALL_COLUMNS = sys::ImGuiSelectableFlags_SpanAllColumns;
        /// Generate press events on double clicks too
        const ALLOW_DOUBLE_CLICK = sys::ImGuiSelectableFlags_AllowDoubleClick;
        /// Cannot be selected, display greyed out text
        const DISABLED = sys::ImGuiSelectableFlags_Disabled;
        /// Hit testing to allow subsequent willdgets to overlap this one
        const ALLOW_OVERLAP = sys::ImGuiSelectableFlags_AllowOverlap;
    }
);

impl Ui {
    /// Constructs a new simple selectable.
    ///
    /// Use [selectable_config] for a builder with additional options.
    ///
    /// [selectable_config]: Self::selectable_config
    #[doc(alias = "Selectable")]
    pub fn selectable<T: AsRef<str>>(&self, label: T) -> bool {
        self.selectable_config(label).build()
    }

    /// Constructs a new selectable builder.
    #[doc(alias = "Selectable")]
    pub fn selectable_config<T: AsRef<str>>(&self, label: T) -> Selectable<'_, T> {
        Selectable {
            label,
            selected: false,
            flags: SelectableFlags::empty(),
            size: [0.0, 0.0],
            ui: self,
        }
    }
}

/// Builder for a selectable widget.
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct Selectable<'ui, T> {
    label: T,
    selected: bool,
    flags: SelectableFlags,
    size: [f32; 2],
    ui: &'ui Ui,
}

impl<'ui, T: AsRef<str>> Selectable<'ui, T> {
    /// Constructs a new selectable builder.
    #[doc(alias = "Selectable")]
    #[deprecated(
        since = "0.9.0",
        note = "use `ui.selectable` or `ui.selectable_config`"
    )]
    pub fn new(label: T, ui: &'ui Ui) -> Self {
        Selectable {
            label,
            selected: false,
            flags: SelectableFlags::empty(),
            size: [0.0, 0.0],
            ui,
        }
    }
    /// Replaces all current settings with the given flags
    pub fn flags(mut self, flags: SelectableFlags) -> Self {
        self.flags = flags;
        self
    }
    /// Sets the selected state of the selectable
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
    /// Enables/disables closing parent popup window on click.
    ///
    /// Default: enabled
    pub fn close_popups(mut self, value: bool) -> Self {
        self.flags.set(SelectableFlags::NO_AUTO_CLOSE_POPUPS, !value);
        self
    }
    /// Enables/disables full column span (text will still fit in the current column).
    ///
    /// Default: disabled
    pub fn span_all_columns(mut self, value: bool) -> Self {
        self.flags.set(SelectableFlags::SPAN_ALL_COLUMNS, value);
        self
    }
    /// Enables/disables click event generation on double clicks.
    ///
    /// Default: disabled
    pub fn allow_double_click(mut self, value: bool) -> Self {
        self.flags.set(SelectableFlags::ALLOW_DOUBLE_CLICK, value);
        self
    }
    /// Enables/disables the selectable.
    ///
    /// When disabled, it cannot be selected and the text uses the disabled text color.
    ///
    /// Default: disabled
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
    pub fn size(mut self, size: impl Into<MintVec2>) -> Self {
        self.size = size.into().into();
        self
    }

    /// Builds the selectable.
    ///
    /// Returns true if the selectable was clicked.
    pub fn build(self) -> bool {
        unsafe {
            sys::igSelectable_Bool(
                self.ui.scratch_txt(self.label),
                self.selected,
                self.flags.bits() as i32,
                self.size.into(),
            )
        }
    }

    /// Builds the selectable using a mutable reference to selected state.
    pub fn build_with_ref(self, selected: &mut bool) -> bool {
        if self.selected(*selected).build() {
            *selected = !*selected;
            true
        } else {
            false
        }
    }
}
