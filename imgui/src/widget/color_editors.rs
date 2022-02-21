use bitflags::bitflags;
use std::ptr;

use crate::math::MintVec2;
use crate::math::MintVec3;
use crate::math::MintVec4;
use crate::sys;
use crate::Ui;

// /// Mutable reference to an editable color value.
// #[derive(Debug)]
// pub enum EditableColor<'a, T> {
//     /// Color value with three float components (e.g. RGB).
//     Float3(&'a mut T),
//     /// Color value with four float components (e.g. RGBA).
//     Float4(&'a mut T),
// }

// impl<'a> EditableColor<'a> {
//     /// Returns an unsafe mutable pointer to the color slice's buffer.
//     fn as_mut_ptr(&mut self) -> *mut f32 {
//         match *self {
//             EditableColor::Float3(ref mut value) => value.as_mut_ptr(),
//             EditableColor::Float4(ref mut value) => value.as_mut_ptr(),
//         }
//     }
// }

// impl<'a> From<&'a mut [f32; 3]> for EditableColor<'a> {
//     #[inline]
//     fn from(value: &'a mut [f32; 3]) -> EditableColor<'a> {
//         EditableColor::Float3(value)
//     }
// }

// impl<'a> From<&'a mut [f32; 4]> for EditableColor<'a> {
//     #[inline]
//     fn from(value: &'a mut [f32; 4]) -> EditableColor<'a> {
//         EditableColor::Float4(value)
//     }
// }

/// Color editor input mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorEditInputMode {
    /// Edit as RGB(A).
    Rgb,
    /// Edit as HSV(A).
    Hsv,
}

impl ColorEditInputMode {
    // Note: Probably no point in deprecating these since they're ~0 maintance burden.
    /// Edit as RGB(A). Alias for [`Self::Rgb`] for backwards-compatibility.
    pub const RGB: Self = Self::Rgb;
    /// Edit as HSV(A). Alias for [`Self::Hsv`] for backwards-compatibility.
    pub const HSV: Self = Self::Hsv;
}

/// Color editor display mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorEditDisplayMode {
    /// Display as RGB(A).
    Rgb,
    /// Display as HSV(A).
    Hsv,
    /// Display as hex (e.g. `#AABBCC(DD)`)
    Hex,
}

impl ColorEditDisplayMode {
    // Note: Probably no point in deprecating these since they're ~0 maintance burden.
    /// Display as RGB(A). Alias for [`Self::Rgb`] for backwards-compatibility.
    pub const RGB: Self = Self::Rgb;
    /// Display as HSV(A). Alias for [`Self::Hsv`] for backwards-compatibility.
    pub const HSV: Self = Self::Hsv;
    /// Display as hex. Alias for [`Self::Hex`] for backwards-compatibility.
    pub const HEX: Self = Self::Hex;
}

/// Color picker hue/saturation/value editor mode
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorPickerMode {
    /// Use a bar for hue, rectangle for saturation/value.
    HueBar,
    /// Use a wheel for hue, triangle for saturation/value.
    HueWheel,
}

/// Color component formatting
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorFormat {
    /// Display values formatted as 0..255.
    U8,
    /// Display values formatted as 0.0..1.0.
    Float,
}

/// Color editor preview style
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorPreview {
    /// Don't show the alpha component.
    Opaque,
    /// Half of the preview area shows the alpha component using a checkerboard pattern.
    HalfAlpha,
    /// Show the alpha component using a checkerboard pattern.
    Alpha,
}

bitflags! {
    /// Color edit flags
    #[repr(transparent)]
    pub struct ColorEditFlags: u32 {
        /// ColorEdit, ColorPicker, ColorButton: ignore Alpha component (read only 3 components of
        /// the value).
        const NO_ALPHA = sys::ImGuiColorEditFlags_NoAlpha;
        /// ColorEdit: disable picker when clicking on colored square.
        const NO_PICKER = sys::ImGuiColorEditFlags_NoPicker;
        /// ColorEdit: disable toggling options menu when right-clicking on inputs/small preview.
        const NO_OPTIONS = sys::ImGuiColorEditFlags_NoOptions;
        /// ColorEdit, ColorPicker: disable colored square preview next to the inputs. (e.g. to
        /// show only the inputs)
        const NO_SMALL_PREVIEW = sys::ImGuiColorEditFlags_NoSmallPreview;
        /// ColorEdit, ColorPicker: disable inputs sliders/text widgets (e.g. to show only the
        /// small preview colored square).
        const NO_INPUTS = sys::ImGuiColorEditFlags_NoInputs;
        /// ColorEdit, ColorPicker, ColorButton: disable tooltip when hovering the preview.
        const NO_TOOLTIP = sys::ImGuiColorEditFlags_NoTooltip;
        /// ColorEdit, ColorPicker: disable display of inline text label (the label is still
        /// forwarded to the tooltip and picker).
        const NO_LABEL = sys::ImGuiColorEditFlags_NoLabel;
        /// ColorPicker: disable bigger color preview on right side of the picker, use small
        /// colored square preview instead.
        const NO_SIDE_PREVIEW = sys::ImGuiColorEditFlags_NoSidePreview;
        /// ColorEdit: disable drag and drop target. ColorButton: disable drag and drop source.
        const NO_DRAG_DROP = sys::ImGuiColorEditFlags_NoDragDrop;
        /// ColorButton: disable border (which is enforced by default).
        const NO_BORDER = sys::ImGuiColorEditFlags_NoBorder;

        /// ColorEdit, ColorPicker: show vertical alpha bar/gradient in picker.
        const ALPHA_BAR = sys::ImGuiColorEditFlags_AlphaBar;
        /// ColorEdit, ColorPicker, ColorButton: display preview as a transparent color over a
        /// checkerboard, instead of opaque.
        const ALPHA_PREVIEW = sys::ImGuiColorEditFlags_AlphaPreview;
        /// ColorEdit, ColorPicker, ColorButton: display half opaque / half checkerboard, instead
        /// of opaque.
        const ALPHA_PREVIEW_HALF = sys::ImGuiColorEditFlags_AlphaPreviewHalf;
        /// (WIP) ColorEdit: Currently onlys disable 0.0f..1.0f limits in RGBA editing (note: you
        /// probably want to use `ColorEditFlags::FLOAT` as well).
        const HDR = sys::ImGuiColorEditFlags_HDR;
        /// ColorEdit: display only as RGB. ColorPicker: Enable RGB display.
        const DISPLAY_RGB = sys::ImGuiColorEditFlags_DisplayRGB;
        /// ColorEdit: display only as HSV. ColorPicker: Enable HSV display.
        const DISPLAY_HSV = sys::ImGuiColorEditFlags_DisplayHSV;
        /// ColorEdit: display only as HEX. ColorPicker: Enable Hex display.
        const DISPLAY_HEX = sys::ImGuiColorEditFlags_DisplayHex;
        /// ColorEdit, ColorPicker, ColorButton: _display_ values formatted as 0..255.
        const UINT8 = sys::ImGuiColorEditFlags_Uint8;
        /// ColorEdit, ColorPicker, ColorButton: _display_ values formatted as 0.0f..1.0f floats
        /// instead of 0..255 integers. No round-trip of value via integers.
        const FLOAT = sys::ImGuiColorEditFlags_Float;
        /// ColorPicker: bar for Hue, rectangle for Sat/Value.
        const PICKER_HUE_BAR = sys::ImGuiColorEditFlags_PickerHueBar;
        /// ColorPicker: wheel for Hue, triangle for Sat/Value.
        const PICKER_HUE_WHEEL = sys::ImGuiColorEditFlags_PickerHueWheel;
        /// ColorEdit, ColorPicker: input and output data in RGB format.
        const INPUT_RGB = sys::ImGuiColorEditFlags_InputRGB;
        /// ColorEdit, ColorPicker: input and output data in HSV format.
        const INPUT_HSV = sys::ImGuiColorEditFlags_InputHSV;
    }
}

/// Builder for a color editor widget.
///
/// # Examples
///
/// ```no_run
/// # use imgui::*;
/// # let mut imgui = Context::create();
/// # let ui = imgui.frame();
/// # let mut color = [0.0, 0.0, 0.0, 1.0];
/// if ui.color_edit4("color_edit", &mut color) {
///   println!("The color was changed");
/// }
/// ```
#[derive(Debug)]
#[must_use]
pub struct ColorEdit3<'ui, 'a, Label, C> {
    label: Label,
    value: &'a mut C,
    flags: ColorEditFlags,
    ui: &'ui Ui,
}

impl<'ui, 'a, Label, C> ColorEdit3<'ui, 'a, Label, C>
where
    Label: AsRef<str>,
    C: Copy + Into<MintVec3>,
    MintVec3: Into<C> + Into<[f32; 3]>,
{
    /// Constructs a new color editor builder.
    #[deprecated(since = "0.9.0", note = "Use `ui.color_edit3(...)` instead")]
    pub fn new(ui: &'ui Ui, label: Label, value: &'a mut C) -> Self {
        ColorEdit3 {
            label,
            value,
            flags: ColorEditFlags::empty(),
            ui,
        }
    }
    /// Replaces all current settings with the given flags.
    #[inline]
    pub fn flags(mut self, flags: ColorEditFlags) -> Self {
        self.flags = flags;
        self
    }
    /// Enables/disables the use of the alpha component.
    #[inline]
    pub fn alpha(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_ALPHA, !value);
        self
    }
    /// Enables/disables the picker that appears when clicking on colored square.
    #[inline]
    pub fn picker(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_PICKER, !value);
        self
    }
    /// Enables/disables toggling of the options menu when right-clicking on inputs or the small
    /// preview.
    #[inline]
    pub fn options(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_OPTIONS, !value);
        self
    }
    /// Enables/disables the colored square preview next to the inputs.
    #[inline]
    pub fn small_preview(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_SMALL_PREVIEW, !value);
        self
    }
    /// Enables/disables the input sliders/text widgets.
    #[inline]
    pub fn inputs(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_INPUTS, !value);
        self
    }
    /// Enables/disables the tooltip that appears when hovering the preview.
    #[inline]
    pub fn tooltip(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_TOOLTIP, !value);
        self
    }
    /// Enables/disables display of the inline text label (the label is in any case forwarded to
    /// the tooltip and picker).
    #[inline]
    pub fn label(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_LABEL, !value);
        self
    }
    /// Enables/disables the vertical alpha bar/gradient in the color picker.
    #[inline]
    pub fn alpha_bar(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::ALPHA_BAR, value);
        self
    }
    /// Sets the preview style.
    #[inline]
    pub fn preview(mut self, preview: ColorPreview) -> Self {
        self.flags.set(
            ColorEditFlags::ALPHA_PREVIEW_HALF,
            preview == ColorPreview::HalfAlpha,
        );
        self.flags.set(
            ColorEditFlags::ALPHA_PREVIEW,
            preview == ColorPreview::Alpha,
        );
        self
    }
    /// (WIP) Currently only disables 0.0..1.0 limits in RGBA edition.
    ///
    /// Note: you probably want to use ColorFormat::Float as well.
    #[inline]
    pub fn hdr(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::HDR, value);
        self
    }
    /// Sets the data format for input and output data.
    #[inline]
    pub fn input_mode(mut self, input_mode: ColorEditInputMode) -> Self {
        self.flags.set(
            ColorEditFlags::INPUT_RGB,
            input_mode == ColorEditInputMode::RGB,
        );
        self.flags.set(
            ColorEditFlags::INPUT_HSV,
            input_mode == ColorEditInputMode::HSV,
        );
        self
    }
    /// Sets the color editor display mode.
    #[inline]
    pub fn display_mode(mut self, mode: ColorEditDisplayMode) -> Self {
        self.flags.set(
            ColorEditFlags::DISPLAY_RGB,
            mode == ColorEditDisplayMode::RGB,
        );
        self.flags.set(
            ColorEditFlags::DISPLAY_HSV,
            mode == ColorEditDisplayMode::HSV,
        );
        self.flags.set(
            ColorEditFlags::DISPLAY_HEX,
            mode == ColorEditDisplayMode::HEX,
        );
        self
    }
    /// Sets the formatting style of color components.
    #[inline]
    pub fn format(mut self, format: ColorFormat) -> Self {
        self.flags
            .set(ColorEditFlags::UINT8, format == ColorFormat::U8);
        self.flags
            .set(ColorEditFlags::FLOAT, format == ColorFormat::Float);
        self
    }
    /// Builds the color editor.
    ///
    /// Returns true if the color value was changed.
    pub fn build(mut self) -> bool {
        self.flags.insert(ColorEditFlags::NO_ALPHA);

        let as_vec3: MintVec3 = (*self.value).into();
        let mut as_vec3: [f32; 3] = as_vec3.into();

        let changed = unsafe {
            sys::igColorEdit3(
                self.ui.scratch_txt(self.label),
                as_vec3.as_mut_ptr(),
                self.flags.bits() as _,
            )
        };

        // and go backwards...
        if changed {
            let as_vec3: MintVec3 = as_vec3.into();

            *self.value = as_vec3.into();
        }

        changed
    }
}

impl Ui {
    /// Edits a color of 3 channels. Use [color_edit3_config](Self::color_edit3_config)
    /// for a builder to customize this widget.
    pub fn color_edit3<Label, C>(&self, label: Label, value: &mut C) -> bool
    where
        Label: AsRef<str>,
        C: Copy + Into<MintVec3>,
        MintVec3: Into<C> + Into<[f32; 3]>,
    {
        ColorEdit3 {
            label,
            value,
            flags: ColorEditFlags::empty(),
            ui: self,
        }
        .build()
    }

    /// Constructs a new color editor builder.
    pub fn color_edit3_config<'a, Label, C>(
        &self,
        label: Label,
        value: &'a mut C,
    ) -> ColorEdit3<'_, 'a, Label, C>
    where
        Label: AsRef<str>,
        C: Copy + Into<MintVec3>,
        MintVec3: Into<C> + Into<[f32; 3]>,
    {
        ColorEdit3 {
            label,
            value,
            flags: ColorEditFlags::empty(),
            ui: self,
        }
    }
}

/// Builder for a color editor widget.
///
/// # Examples
///
/// ```no_run
/// # use imgui::*;
/// # let mut imgui = Context::create();
/// # let ui = imgui.frame();
/// # let mut color = [0.0, 0.0, 0.0, 1.0];
/// if ui.color_edit4("color_edit", &mut color) {
///   println!("The color was changed");
/// }
/// ```
#[derive(Debug)]
#[must_use]
pub struct ColorEdit4<'ui, 'a, T, C> {
    label: T,
    value: &'a mut C,
    flags: ColorEditFlags,
    ui: &'ui Ui,
}

impl<'ui, 'a, T, C> ColorEdit4<'ui, 'a, T, C>
where
    T: AsRef<str>,
    C: Copy + Into<MintVec4>,
    MintVec4: Into<C> + Into<[f32; 4]>,
{
    /// Constructs a new color editor builder.
    #[deprecated(since = "0.9.0", note = "Use `ui.color_edit4(...)` instead")]
    pub fn new(ui: &'ui Ui, label: T, value: &'a mut C) -> Self {
        Self {
            label,
            value,
            flags: ColorEditFlags::empty(),
            ui,
        }
    }
    /// Replaces all current settings with the given flags.
    #[inline]
    pub fn flags(mut self, flags: ColorEditFlags) -> Self {
        self.flags = flags;
        self
    }
    /// Enables/disables the use of the alpha component.
    #[inline]
    pub fn alpha(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_ALPHA, !value);
        self
    }
    /// Enables/disables the picker that appears when clicking on colored square.
    #[inline]
    pub fn picker(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_PICKER, !value);
        self
    }
    /// Enables/disables toggling of the options menu when right-clicking on inputs or the small
    /// preview.
    #[inline]
    pub fn options(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_OPTIONS, !value);
        self
    }
    /// Enables/disables the colored square preview next to the inputs.
    #[inline]
    pub fn small_preview(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_SMALL_PREVIEW, !value);
        self
    }
    /// Enables/disables the input sliders/text widgets.
    #[inline]
    pub fn inputs(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_INPUTS, !value);
        self
    }
    /// Enables/disables the tooltip that appears when hovering the preview.
    #[inline]
    pub fn tooltip(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_TOOLTIP, !value);
        self
    }
    /// Enables/disables display of the inline text label (the label is in any case forwarded to
    /// the tooltip and picker).
    #[inline]
    pub fn label(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_LABEL, !value);
        self
    }
    /// Enables/disables the vertical alpha bar/gradient in the color picker.
    #[inline]
    pub fn alpha_bar(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::ALPHA_BAR, value);
        self
    }
    /// Sets the preview style.
    #[inline]
    pub fn preview(mut self, preview: ColorPreview) -> Self {
        self.flags.set(
            ColorEditFlags::ALPHA_PREVIEW_HALF,
            preview == ColorPreview::HalfAlpha,
        );
        self.flags.set(
            ColorEditFlags::ALPHA_PREVIEW,
            preview == ColorPreview::Alpha,
        );
        self
    }
    /// (WIP) Currently only disables 0.0..1.0 limits in RGBA edition.
    ///
    /// Note: you probably want to use ColorFormat::Float as well.
    #[inline]
    pub fn hdr(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::HDR, value);
        self
    }
    /// Sets the data format for input and output data.
    #[inline]
    pub fn input_mode(mut self, input_mode: ColorEditInputMode) -> Self {
        self.flags.set(
            ColorEditFlags::INPUT_RGB,
            input_mode == ColorEditInputMode::RGB,
        );
        self.flags.set(
            ColorEditFlags::INPUT_HSV,
            input_mode == ColorEditInputMode::HSV,
        );
        self
    }
    /// Sets the color editor display mode.
    #[inline]
    pub fn display_mode(mut self, mode: ColorEditDisplayMode) -> Self {
        self.flags.set(
            ColorEditFlags::DISPLAY_RGB,
            mode == ColorEditDisplayMode::RGB,
        );
        self.flags.set(
            ColorEditFlags::DISPLAY_HSV,
            mode == ColorEditDisplayMode::HSV,
        );
        self.flags.set(
            ColorEditFlags::DISPLAY_HEX,
            mode == ColorEditDisplayMode::HEX,
        );
        self
    }
    /// Sets the formatting style of color components.
    #[inline]
    pub fn format(mut self, format: ColorFormat) -> Self {
        self.flags
            .set(ColorEditFlags::UINT8, format == ColorFormat::U8);
        self.flags
            .set(ColorEditFlags::FLOAT, format == ColorFormat::Float);
        self
    }
    /// Builds the color editor.
    ///
    /// Returns true if the color value was changed.
    pub fn build(self) -> bool {
        let as_vec4: MintVec4 = (*self.value).into();
        let mut as_vec4: [f32; 4] = as_vec4.into();

        let changed = unsafe {
            sys::igColorEdit4(
                self.ui.scratch_txt(self.label),
                as_vec4.as_mut_ptr(),
                self.flags.bits() as _,
            )
        };

        // and go backwards...
        if changed {
            let as_vec4: MintVec4 = as_vec4.into();

            *self.value = as_vec4.into();
        }

        changed
    }
}

impl Ui {
    /// Edits a color of 4 channels. Use [color_edit4_config](Self::color_edit4_config)
    /// for a builder to customize this widget.
    pub fn color_edit4<Label, C>(&self, label: Label, value: &mut C) -> bool
    where
        Label: AsRef<str>,
        C: Copy + Into<MintVec4>,
        MintVec4: Into<C> + Into<[f32; 4]>,
    {
        ColorEdit4 {
            label,
            value,
            flags: ColorEditFlags::empty(),
            ui: self,
        }
        .build()
    }

    /// Constructs a new color editor builder.
    pub fn color_edit4_config<'a, Label, C>(
        &self,
        label: Label,
        value: &'a mut C,
    ) -> ColorEdit4<'_, 'a, Label, C>
    where
        Label: AsRef<str>,
        C: Copy + Into<MintVec4>,
        MintVec4: Into<C> + Into<[f32; 4]>,
    {
        ColorEdit4 {
            label,
            value,
            flags: ColorEditFlags::empty(),
            ui: self,
        }
    }
}

/// Builder for a color picker widget.
///
/// # Examples
///
/// ```no_run
/// # use imgui::*;
/// # let mut imgui = Context::create();
/// # let ui = imgui.frame();
/// # let mut color = [0.0, 0.0, 0.0, 1.0];
/// if ui.color_picker4("color_picker", &mut color) {
///   println!("A color was picked");
/// }
/// ```
#[derive(Debug)]
#[must_use]
pub struct ColorPicker3<'ui, 'a, Label, Color> {
    label: Label,
    value: &'a mut Color,
    flags: ColorEditFlags,
    ui: &'ui Ui,
}

impl<'ui, 'a, Label, Color> ColorPicker3<'ui, 'a, Label, Color>
where
    Label: AsRef<str>,
    Color: Copy + Into<MintVec3>,
    MintVec3: Into<Color> + Into<[f32; 3]>,
{
    /// Constructs a new color picker builder.
    #[deprecated(since = "0.9.0", note = "Use `ui.color_picker3(...)` instead")]
    pub fn new(ui: &'ui Ui, label: Label, value: &'a mut Color) -> Self {
        ColorPicker3 {
            label,
            value,
            flags: ColorEditFlags::empty(),
            ui,
        }
    }
    /// Replaces all current settings with the given flags.
    #[inline]
    pub fn flags(mut self, flags: ColorEditFlags) -> Self {
        self.flags = flags;
        self
    }
    /// Enables/disables the use of the alpha component.
    #[inline]
    pub fn alpha(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_ALPHA, !value);
        self
    }
    /// Enables/disables toggling of the options menu when right-clicking on inputs or the small
    /// preview.
    #[inline]
    pub fn options(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_OPTIONS, !value);
        self
    }
    /// Enables/disables the colored square preview next to the inputs.
    #[inline]
    pub fn small_preview(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_SMALL_PREVIEW, !value);
        self
    }
    /// Enables/disables the input sliders/text widgets.
    #[inline]
    pub fn inputs(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_INPUTS, !value);
        self
    }
    /// Enables/disables the tooltip that appears when hovering the preview.
    #[inline]
    pub fn tooltip(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_TOOLTIP, !value);
        self
    }
    /// Enables/disables display of the inline text label (the label is in any case forwarded to
    /// the tooltip and picker).
    #[inline]
    pub fn label(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_LABEL, !value);
        self
    }
    /// Enables/disables the bigger color preview on the right side of the picker.
    #[inline]
    pub fn side_preview(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_SIDE_PREVIEW, !value);
        self
    }
    /// Enables/disables the vertical alpha bar/gradient in the color picker.
    #[inline]
    pub fn alpha_bar(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::ALPHA_BAR, value);
        self
    }
    /// Sets the preview style.
    #[inline]
    pub fn preview(mut self, preview: ColorPreview) -> Self {
        self.flags.set(
            ColorEditFlags::ALPHA_PREVIEW_HALF,
            preview == ColorPreview::HalfAlpha,
        );
        self.flags.set(
            ColorEditFlags::ALPHA_PREVIEW,
            preview == ColorPreview::Alpha,
        );
        self
    }
    /// Sets the data format for input and output data.
    #[inline]
    pub fn input_mode(mut self, input_mode: ColorEditInputMode) -> Self {
        self.flags.set(
            ColorEditFlags::INPUT_RGB,
            input_mode == ColorEditInputMode::RGB,
        );
        self.flags.set(
            ColorEditFlags::INPUT_HSV,
            input_mode == ColorEditInputMode::HSV,
        );
        self
    }
    /// Enables/disables displaying the value as RGB.
    #[inline]
    pub fn display_rgb(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::DISPLAY_RGB, value);
        self
    }
    /// Enables/disables displaying the value as HSV.
    #[inline]
    pub fn display_hsv(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::DISPLAY_HSV, value);
        self
    }
    /// Enables/disables displaying the value as hex.
    #[inline]
    pub fn display_hex(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::DISPLAY_HEX, value);
        self
    }
    /// Sets the hue/saturation/value editor mode.
    #[inline]
    pub fn mode(mut self, mode: ColorPickerMode) -> Self {
        self.flags.set(
            ColorEditFlags::PICKER_HUE_BAR,
            mode == ColorPickerMode::HueBar,
        );
        self.flags.set(
            ColorEditFlags::PICKER_HUE_WHEEL,
            mode == ColorPickerMode::HueWheel,
        );
        self
    }
    /// Sets the formatting style of color components.
    #[inline]
    pub fn format(mut self, format: ColorFormat) -> Self {
        self.flags
            .set(ColorEditFlags::UINT8, format == ColorFormat::U8);
        self.flags
            .set(ColorEditFlags::FLOAT, format == ColorFormat::Float);
        self
    }

    /// Builds the color picker.
    ///
    /// Returns true if the color value was changed.
    pub fn build(mut self) -> bool {
        self.flags.insert(ColorEditFlags::NO_ALPHA);
        let mut value: [f32; 3] = (*self.value).into().into();
        let changed = unsafe {
            sys::igColorPicker3(
                self.ui.scratch_txt(self.label),
                value.as_mut_ptr(),
                self.flags.bits() as _,
            )
        };

        if changed {
            let as_vec3: MintVec3 = value.into();

            *self.value = as_vec3.into();
        }

        changed
    }
}

impl Ui {
    /// Edits a color of 3 channels. Use [color_picker3](Self::color_picker3)
    /// for a builder to customize this widget.
    pub fn color_picker3<Label, C>(&self, label: Label, value: &mut C) -> bool
    where
        Label: AsRef<str>,
        C: Copy + Into<MintVec3>,
        MintVec3: Into<C> + Into<[f32; 3]>,
    {
        ColorPicker3 {
            label,
            value,
            flags: ColorEditFlags::empty(),
            ui: self,
        }
        .build()
    }

    /// Constructs a new color picker editor builder.
    pub fn color_picker3_config<'a, Label, C>(
        &self,
        label: Label,
        value: &'a mut C,
    ) -> ColorPicker3<'_, 'a, Label, C>
    where
        Label: AsRef<str>,
        C: Copy + Into<MintVec3>,
        MintVec3: Into<C> + Into<[f32; 3]>,
    {
        ColorPicker3 {
            label,
            value,
            flags: ColorEditFlags::empty(),
            ui: self,
        }
    }
}

// Builder for a color picker widget.
///
/// # Examples
///
/// ```no_run
/// # use imgui::*;
/// # let mut imgui = Context::create();
/// # let ui = imgui.frame();
/// # let mut color = [0.0, 0.0, 0.0, 1.0];
/// if ui.color_picker4("color_picker", &mut color) {
///   println!("A color was picked");
/// }
/// ```
#[derive(Debug)]
#[must_use]
pub struct ColorPicker4<'ui, 'a, Label, Color> {
    label: Label,
    value: &'a mut Color,
    flags: ColorEditFlags,
    ref_color: Option<[f32; 4]>,
    ui: &'ui Ui,
}

impl<'ui, 'a, Label, Color> ColorPicker4<'ui, 'a, Label, Color>
where
    Label: AsRef<str>,
    Color: Copy + Into<MintVec4>,
    MintVec4: Into<Color> + Into<[f32; 4]>,
{
    /// Constructs a new color picker builder.
    #[deprecated(since = "0.9.0", note = "Use `ui.color_picker4(...)` instead")]
    pub fn new(ui: &'ui Ui, label: Label, value: &'a mut Color) -> Self {
        Self {
            label,
            value,
            flags: ColorEditFlags::empty(),
            ref_color: None,
            ui,
        }
    }
    /// Replaces all current settings with the given flags.
    #[inline]
    pub fn flags(mut self, flags: ColorEditFlags) -> Self {
        self.flags = flags;
        self
    }
    /// Enables/disables the use of the alpha component.
    #[inline]
    pub fn alpha(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_ALPHA, !value);
        self
    }
    /// Enables/disables toggling of the options menu when right-clicking on inputs or the small
    /// preview.
    #[inline]
    pub fn options(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_OPTIONS, !value);
        self
    }
    /// Enables/disables the colored square preview next to the inputs.
    #[inline]
    pub fn small_preview(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_SMALL_PREVIEW, !value);
        self
    }
    /// Enables/disables the input sliders/text widgets.
    #[inline]
    pub fn inputs(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_INPUTS, !value);
        self
    }
    /// Enables/disables the tooltip that appears when hovering the preview.
    #[inline]
    pub fn tooltip(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_TOOLTIP, !value);
        self
    }
    /// Enables/disables display of the inline text label (the label is in any case forwarded to
    /// the tooltip and picker).
    #[inline]
    pub fn label(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_LABEL, !value);
        self
    }
    /// Enables/disables the bigger color preview on the right side of the picker.
    #[inline]
    pub fn side_preview(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_SIDE_PREVIEW, !value);
        self
    }
    /// Enables/disables the vertical alpha bar/gradient in the color picker.
    #[inline]
    pub fn alpha_bar(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::ALPHA_BAR, value);
        self
    }
    /// Sets the preview style.
    #[inline]
    pub fn preview(mut self, preview: ColorPreview) -> Self {
        self.flags.set(
            ColorEditFlags::ALPHA_PREVIEW_HALF,
            preview == ColorPreview::HalfAlpha,
        );
        self.flags.set(
            ColorEditFlags::ALPHA_PREVIEW,
            preview == ColorPreview::Alpha,
        );
        self
    }
    /// Sets the data format for input and output data.
    #[inline]
    pub fn input_mode(mut self, input_mode: ColorEditInputMode) -> Self {
        self.flags.set(
            ColorEditFlags::INPUT_RGB,
            input_mode == ColorEditInputMode::RGB,
        );
        self.flags.set(
            ColorEditFlags::INPUT_HSV,
            input_mode == ColorEditInputMode::HSV,
        );
        self
    }
    /// Enables/disables displaying the value as RGB.
    #[inline]
    pub fn display_rgb(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::DISPLAY_RGB, value);
        self
    }
    /// Enables/disables displaying the value as HSV.
    #[inline]
    pub fn display_hsv(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::DISPLAY_HSV, value);
        self
    }
    /// Enables/disables displaying the value as hex.
    #[inline]
    pub fn display_hex(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::DISPLAY_HEX, value);
        self
    }
    /// Sets the hue/saturation/value editor mode.
    #[inline]
    pub fn mode(mut self, mode: ColorPickerMode) -> Self {
        self.flags.set(
            ColorEditFlags::PICKER_HUE_BAR,
            mode == ColorPickerMode::HueBar,
        );
        self.flags.set(
            ColorEditFlags::PICKER_HUE_WHEEL,
            mode == ColorPickerMode::HueWheel,
        );
        self
    }
    /// Sets the formatting style of color components.
    #[inline]
    pub fn format(mut self, format: ColorFormat) -> Self {
        self.flags
            .set(ColorEditFlags::UINT8, format == ColorFormat::U8);
        self.flags
            .set(ColorEditFlags::FLOAT, format == ColorFormat::Float);
        self
    }
    /// Sets the shown reference color.
    #[inline]
    pub fn reference_color(mut self, ref_color: impl Into<MintVec4>) -> Self {
        self.ref_color = Some(ref_color.into().into());
        self
    }

    /// Builds the color picker.
    ///
    /// Returns true if the color value was changed.
    pub fn build(self) -> bool {
        let mut value: [f32; 4] = (*self.value).into().into();
        let ref_color = self.ref_color.map(|c| c.as_ptr()).unwrap_or(ptr::null());

        let changed = unsafe {
            sys::igColorPicker4(
                self.ui.scratch_txt(self.label),
                value.as_mut_ptr(),
                self.flags.bits() as _,
                ref_color,
            )
        };

        if changed {
            let as_vec3: MintVec4 = value.into();

            *self.value = as_vec3.into();
        }

        changed
    }
}

impl Ui {
    /// Edits a color of 4 channels. Use [color_picker4_config](Self::color_picker4_config)
    /// for a builder to customize this widget.
    pub fn color_picker4<Label, C>(&self, label: Label, value: &mut C) -> bool
    where
        Label: AsRef<str>,
        C: Copy + Into<MintVec4>,
        MintVec4: Into<C> + Into<[f32; 4]>,
    {
        ColorPicker4 {
            label,
            value,
            flags: ColorEditFlags::empty(),
            ref_color: None,
            ui: self,
        }
        .build()
    }

    /// Constructs a new color picker editor builder.
    pub fn color_picker4_config<'a, Label, C>(
        &self,
        label: Label,
        value: &'a mut C,
    ) -> ColorPicker4<'_, 'a, Label, C>
    where
        Label: AsRef<str>,
        C: Copy + Into<MintVec4>,
        MintVec4: Into<C> + Into<[f32; 4]>,
    {
        ColorPicker4 {
            label,
            value,
            flags: ColorEditFlags::empty(),
            ref_color: None,
            ui: self,
        }
    }
}

/// Builder for a color button widget.
///
/// # Examples
///
/// ```no_run
/// # use imgui::*;
/// # let mut imgui = Context::create();
/// # let ui = imgui.frame();
/// if ui.color_button("color_button", [1.0, 0.0, 0.0, 1.0]) {
///     println!("pressed!");
/// }
/// ```
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct ColorButton<'ui, T> {
    desc_id: T,
    color: [f32; 4],
    flags: ColorEditFlags,
    size: [f32; 2],
    ui: &'ui Ui,
}

impl<'ui, T: AsRef<str>> ColorButton<'ui, T> {
    /// Constructs a new color button builder.
    #[deprecated(since = "0.9.0", note = "Use `ui.color_button_config(...)` instead")]
    pub fn new(ui: &'ui Ui, desc_id: T, color: impl Into<MintVec4>) -> Self {
        ColorButton {
            desc_id,
            color: color.into().into(),
            flags: ColorEditFlags::empty(),
            size: [0.0, 0.0],
            ui,
        }
    }
    /// Replaces all current settings with the given flags.
    #[inline]
    pub fn flags(mut self, flags: ColorEditFlags) -> Self {
        self.flags = flags;
        self
    }
    /// Enables/disables the use of the alpha component.
    #[inline]
    pub fn alpha(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_ALPHA, !value);
        self
    }
    /// Enables/disables the tooltip that appears when hovering the preview.
    #[inline]
    pub fn tooltip(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_TOOLTIP, !value);
        self
    }
    /// Sets the preview style.
    #[inline]
    pub fn preview(mut self, preview: ColorPreview) -> Self {
        self.flags.set(
            ColorEditFlags::ALPHA_PREVIEW_HALF,
            preview == ColorPreview::HalfAlpha,
        );
        self.flags.set(
            ColorEditFlags::ALPHA_PREVIEW,
            preview == ColorPreview::Alpha,
        );
        self
    }
    /// Sets the data format for input data.
    #[inline]
    pub fn input_mode(mut self, input_mode: ColorEditInputMode) -> Self {
        self.flags.set(
            ColorEditFlags::INPUT_RGB,
            input_mode == ColorEditInputMode::RGB,
        );
        self.flags.set(
            ColorEditFlags::INPUT_HSV,
            input_mode == ColorEditInputMode::HSV,
        );
        self
    }
    /// Enables/disables using the button as drag&drop source.
    ///
    /// Enabled by default.
    pub fn drag_drop(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_DRAG_DROP, !value);
        self
    }
    /// Enables/disables the button border.
    ///
    /// Enabled by default.
    pub fn border(mut self, value: bool) -> Self {
        self.flags.set(ColorEditFlags::NO_BORDER, !value);
        self
    }
    /// Sets the button size.
    ///
    /// Use 0.0 for width and/or height to use the default size.
    #[inline]
    pub fn size(mut self, size: impl Into<MintVec2>) -> Self {
        self.size = size.into().into();
        self
    }
    /// Builds the color button.
    ///
    /// Returns true if this color button was clicked.
    pub fn build(self) -> bool {
        unsafe {
            sys::igColorButton(
                self.ui.scratch_txt(self.desc_id),
                self.color.into(),
                self.flags.bits() as _,
                self.size.into(),
            )
        }
    }
}

impl Ui {
    /// Builds a [ColorButton].
    ///
    /// Returns true if this color button was clicked.
    pub fn color_button<Label: AsRef<str>>(
        &self,
        desc_id: Label,
        color: impl Into<MintVec4>,
    ) -> bool {
        ColorButton {
            desc_id,
            color: color.into().into(),
            flags: ColorEditFlags::empty(),
            size: [0.0, 0.0],
            ui: self,
        }
        .build()
    }

    /// Returns a [ColorButton] builder.
    pub fn color_button_config<Label: AsRef<str>>(
        &self,
        desc_id: Label,
        color: impl Into<MintVec4>,
    ) -> ColorButton<'_, Label> {
        ColorButton {
            desc_id,
            color: color.into().into(),
            flags: ColorEditFlags::empty(),
            size: [0.0, 0.0],
            ui: self,
        }
    }
}

/// # Widgets: Color Editor/Picker
impl Ui {
    /// Initializes current color editor/picker options (generally on application startup) if you
    /// want to select a default format, picker type, etc. Users will be able to change many
    /// settings, unless you use .options(false) in your widget builders.
    #[doc(alias = "SetColorEditOptions")]
    pub fn set_color_edit_options(&self, flags: ColorEditFlags) {
        unsafe {
            sys::igSetColorEditOptions(flags.bits() as i32);
        }
    }
}
