#![warn(missing_docs)]
use sys;
use std::marker::PhantomData;
use std::ptr;

use {ImGuiColorEditFlags, ImStr, ImVec2, ImVec4, Ui};

/// Mutable reference to an editable color value.
#[derive(Debug)]
pub enum EditableColor<'p> {
    /// Color value with three float components (e.g. RGB).
    Float3(&'p mut [f32; 3]),
    /// Color value with four float components (e.g. RGBA).
    Float4(&'p mut [f32; 4]),
}

impl<'p> EditableColor<'p> {
    /// Returns an unsafe mutable pointer to the color slice's buffer.
    fn as_mut_ptr(&mut self) -> *mut f32 {
        match *self {
            EditableColor::Float3(ref mut value) => value.as_mut_ptr(),
            EditableColor::Float4(ref mut value) => value.as_mut_ptr(),
        }
    }
}

impl<'p> From<&'p mut [f32; 3]> for EditableColor<'p> {
    fn from(value: &'p mut [f32; 3]) -> EditableColor<'p> { EditableColor::Float3(value) }
}

impl<'p> From<&'p mut [f32; 4]> for EditableColor<'p> {
    fn from(value: &'p mut [f32; 4]) -> EditableColor<'p> { EditableColor::Float4(value) }
}

/// Color editor mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorEditMode {
    /// Edit as RGB(A).
    RGB,
    /// Edit as HSV(A).
    HSV,
    /// Edit as hex (e.g. #AABBCC(DD))
    HEX,
}

/// Color picker hue/saturation/value editor mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorPickerMode {
    /// Use a bar for hue, rectangle for saturation/value.
    HueBar,
    /// Use a wheel for hue, triangle for saturation/value.
    HueWheel,
}

/// Color component formatting.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorFormat {
    /// Display values formatted as 0..255.
    U8,
    /// Display values formatted as 0.0..1.0.
    Float,
}

/// Color editor preview style.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorPreview {
    /// Don't show the alpha component.
    Opaque,
    /// Half of the preview area shows the alpha component using a checkerboard pattern.
    HalfAlpha,
    /// Show the alpha component using a checkerboard pattern.
    Alpha,
}

/// Builder for a color editor widget.
#[must_use]
pub struct ColorEdit<'ui, 'p> {
    label: &'p ImStr,
    value: EditableColor<'p>,
    flags: ImGuiColorEditFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> ColorEdit<'ui, 'p> {
    /// Constructs a new color editor builder.
    pub fn new(_: &Ui<'ui>, label: &'p ImStr, value: EditableColor<'p>) -> Self {
        ColorEdit {
            label,
            value,
            flags: ImGuiColorEditFlags::empty(),
            _phantom: PhantomData,
        }
    }
    /// Replaces all current settings with the given flags.
    #[inline]
    pub fn flags(mut self, flags: ImGuiColorEditFlags) -> Self {
        self.flags = flags;
        self
    }
    /// Enables/disables the use of the alpha component.
    #[inline]
    pub fn alpha(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoAlpha, !value);
        self
    }
    /// Enables/disables the picker that appears when clicking on colored square.
    #[inline]
    pub fn picker(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoPicker, !value);
        self
    }
    /// Enables/disables toggling of the options menu when right-clicking on inputs or the small
    /// preview.
    #[inline]
    pub fn options(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoOptions, !value);
        self
    }
    /// Enables/disables the colored square preview next to the inputs.
    #[inline]
    pub fn small_preview(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoSmallPreview, !value);
        self
    }
    /// Enables/disables the input sliders/text widgets.
    #[inline]
    pub fn inputs(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoInputs, !value);
        self
    }
    /// Enables/disables the tooltip that appears when hovering the preview.
    #[inline]
    pub fn tooltip(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoTooltip, !value);
        self
    }
    /// Enables/disables display of the inline text label (the label is in any case forwarded to
    /// the tooltip and picker).
    #[inline]
    pub fn label(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoLabel, !value);
        self
    }
    /// Enables/disables the vertical alpha bar/gradient in the color picker.
    #[inline]
    pub fn alpha_bar(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::AlphaBar, value);
        self
    }
    /// Sets the preview style.
    #[inline]
    pub fn preview(mut self, preview: ColorPreview) -> Self {
        self.flags.set(
            ImGuiColorEditFlags::AlphaPreviewHalf,
            preview == ColorPreview::HalfAlpha,
        );
        self.flags.set(
            ImGuiColorEditFlags::AlphaPreview,
            preview == ColorPreview::Alpha,
        );
        self
    }
    /// (WIP) Currently only disables 0.0..1.0 limits in RGBA edition.
    ///
    /// Note: you probably want to use ColorFormat::Float as well.
    #[inline]
    pub fn hdr(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::HDR, value);
        self
    }
    /// Sets the color editor mode.
    #[inline]
    pub fn mode(mut self, mode: ColorEditMode) -> Self {
        self.flags.set(
            ImGuiColorEditFlags::RGB,
            mode == ColorEditMode::RGB,
        );
        self.flags.set(
            ImGuiColorEditFlags::HSV,
            mode == ColorEditMode::HSV,
        );
        self.flags.set(
            ImGuiColorEditFlags::HEX,
            mode == ColorEditMode::HEX,
        );
        self
    }
    /// Sets the formatting style of color components.
    #[inline]
    pub fn format(mut self, format: ColorFormat) -> Self {
        self.flags.set(
            ImGuiColorEditFlags::Uint8,
            format == ColorFormat::U8,
        );
        self.flags.set(
            ImGuiColorEditFlags::Float,
            format == ColorFormat::Float,
        );
        self
    }
    /// Builds the color editor.
    pub fn build(self) -> bool {
        match self.value {
            EditableColor::Float3(value) => unsafe {
                sys::igColorEdit3(self.label.as_ptr(), value.as_mut_ptr(), self.flags)
            },
            EditableColor::Float4(value) => unsafe {
                sys::igColorEdit4(self.label.as_ptr(), value.as_mut_ptr(), self.flags)
            },
        }
    }
}

/// Builder for a color picker widget.
#[must_use]
pub struct ColorPicker<'ui, 'p> {
    label: &'p ImStr,
    value: EditableColor<'p>,
    flags: ImGuiColorEditFlags,
    ref_color: Option<&'p [f32; 4]>,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> ColorPicker<'ui, 'p> {
    /// Constructs a new color picker builder.
    pub fn new(_: &Ui<'ui>, label: &'p ImStr, value: EditableColor<'p>) -> Self {
        ColorPicker {
            label,
            value,
            flags: ImGuiColorEditFlags::empty(),
            ref_color: None,
            _phantom: PhantomData,
        }
    }
    /// Replaces all current settings with the given flags.
    #[inline]
    pub fn flags(mut self, flags: ImGuiColorEditFlags) -> Self {
        self.flags = flags;
        self
    }
    /// Enables/disables the use of the alpha component.
    #[inline]
    pub fn alpha(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoAlpha, !value);
        self
    }
    /// Enables/disables the colored square preview next to the inputs.
    #[inline]
    pub fn small_preview(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoSmallPreview, !value);
        self
    }
    /// Enables/disables the input sliders/text widgets.
    #[inline]
    pub fn inputs(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoInputs, !value);
        self
    }
    /// Enables/disables the tooltip that appears when hovering the preview.
    #[inline]
    pub fn tooltip(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoTooltip, !value);
        self
    }
    /// Enables/disables display of the inline text label (the label is in any case forwarded to
    /// the tooltip and picker).
    #[inline]
    pub fn label(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoLabel, !value);
        self
    }
    /// Enables/disables the bigger color preview on the right side of the picker.
    #[inline]
    pub fn side_preview(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoSidePreview, !value);
        self
    }
    /// Enables/disables the vertical alpha bar/gradient in the color picker.
    #[inline]
    pub fn alpha_bar(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::AlphaBar, value);
        self
    }
    /// Sets the preview style.
    #[inline]
    pub fn preview(mut self, preview: ColorPreview) -> Self {
        self.flags.set(
            ImGuiColorEditFlags::AlphaPreviewHalf,
            preview == ColorPreview::HalfAlpha,
        );
        self.flags.set(
            ImGuiColorEditFlags::AlphaPreview,
            preview == ColorPreview::Alpha,
        );
        self
    }
    /// Enables/disables the RGB inputs.
    #[inline]
    pub fn rgb(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::RGB, value);
        self
    }
    /// Enables/disables the HSV inputs.
    #[inline]
    pub fn hsv(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::HSV, value);
        self
    }
    /// Enables/disables the HEX input.
    #[inline]
    pub fn hex(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::HEX, value);
        self
    }
    /// Sets the hue/saturation/value editor mode.
    #[inline]
    pub fn mode(mut self, mode: ColorPickerMode) -> Self {
        self.flags.set(
            ImGuiColorEditFlags::PickerHueBar,
            mode == ColorPickerMode::HueBar,
        );
        self.flags.set(
            ImGuiColorEditFlags::PickerHueWheel,
            mode == ColorPickerMode::HueWheel,
        );
        self
    }
    /// Sets the formatting style of color components.
    #[inline]
    pub fn format(mut self, format: ColorFormat) -> Self {
        self.flags.set(
            ImGuiColorEditFlags::Uint8,
            format == ColorFormat::U8,
        );
        self.flags.set(
            ImGuiColorEditFlags::Float,
            format == ColorFormat::Float,
        );
        self
    }
    /// Sets the shown reference color.
    #[inline]
    pub fn reference_color(mut self, ref_color: &'p [f32; 4]) -> Self {
        self.ref_color = Some(ref_color);
        self
    }
    /// Builds the color picker.
    pub fn build(mut self) -> bool {
        if let EditableColor::Float3(_) = self.value {
            self.flags.insert(ImGuiColorEditFlags::NoAlpha);
        }
        let ref_color = self.ref_color.map(|c| c.as_ptr()).unwrap_or(ptr::null());
        unsafe {
            sys::igColorPicker4(
                self.label.as_ptr(),
                self.value.as_mut_ptr(),
                self.flags,
                ref_color,
            )
        }
    }
}

/// Builder for a color button widget.
#[must_use]
pub struct ColorButton<'ui, 'p> {
    desc_id: &'p ImStr,
    color: ImVec4,
    flags: ImGuiColorEditFlags,
    size: ImVec2,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> ColorButton<'ui, 'p> {
    /// Constructs a new color button builder.
    pub fn new(_: &Ui<'ui>, desc_id: &'p ImStr, color: ImVec4) -> Self {
        ColorButton {
            desc_id,
            color,
            flags: ImGuiColorEditFlags::empty(),
            size: ImVec2::zero(),
            _phantom: PhantomData,
        }
    }
    /// Replaces all current settings with the given flags.
    #[inline]
    pub fn flags(mut self, flags: ImGuiColorEditFlags) -> Self {
        self.flags = flags;
        self
    }
    /// Enables/disables the use of the alpha component.
    #[inline]
    pub fn alpha(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoAlpha, !value);
        self
    }
    /// Enables/disables the tooltip that appears when hovering the preview.
    #[inline]
    pub fn tooltip(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoTooltip, !value);
        self
    }
    /// Sets the preview style.
    #[inline]
    pub fn preview(mut self, preview: ColorPreview) -> Self {
        self.flags.set(
            ImGuiColorEditFlags::AlphaPreviewHalf,
            preview == ColorPreview::HalfAlpha,
        );
        self.flags.set(
            ImGuiColorEditFlags::AlphaPreview,
            preview == ColorPreview::Alpha,
        );
        self
    }
    /// Sets the button size.
    ///
    /// Use 0.0 for width and/or height to use the default size.
    #[inline]
    pub fn size<S: Into<ImVec2>>(mut self, size: S) -> Self {
        self.size = size.into();
        self
    }
    /// Builds the color button.
    pub fn build(self) -> bool {
        unsafe { sys::igColorButton(self.desc_id.as_ptr(), self.color, self.flags, self.size) }
    }
}
