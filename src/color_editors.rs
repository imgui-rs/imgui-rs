use imgui_sys;
use std::marker::PhantomData;
use std::ptr;

use {ImGuiColorEditFlags, ImStr, Ui};

#[derive(Debug)]
pub enum EditableColor<'p> {
    Float3(&'p mut [f32; 3]),
    Float4(&'p mut [f32; 4]),
}

impl<'p> EditableColor<'p> {
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorEditMode {
    RGB,
    HSV,
    HEX,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorPickerMode {
    HueBar,
    HueWheel,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EditableColorFormat {
    U8,
    Float,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EditableColorPreview {
    Opaque,
    HalfAlpha,
    Alpha,
}

#[must_use]
pub struct ColorEdit<'ui, 'p> {
    label: &'p ImStr,
    value: EditableColor<'p>,
    flags: ImGuiColorEditFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> ColorEdit<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p ImStr, value: EditableColor<'p>) -> Self {
        ColorEdit {
            label,
            value,
            flags: ImGuiColorEditFlags::empty(),
            _phantom: PhantomData,
        }
    }
    #[inline]
    pub fn flags(mut self, flags: ImGuiColorEditFlags) -> Self {
        self.flags = flags;
        self
    }
    #[inline]
    pub fn alpha(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoAlpha, !value);
        self
    }
    #[inline]
    pub fn picker(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoPicker, !value);
        self
    }
    #[inline]
    pub fn options(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoOptions, !value);
        self
    }
    #[inline]
    pub fn small_preview(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoSmallPreview, !value);
        self
    }
    #[inline]
    pub fn inputs(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoInputs, !value);
        self
    }
    #[inline]
    pub fn tooltip(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoTooltip, !value);
        self
    }
    #[inline]
    pub fn label(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoLabel, !value);
        self
    }
    #[inline]
    pub fn alpha_bar(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::AlphaBar, value);
        self
    }
    #[inline]
    pub fn preview(mut self, preview: EditableColorPreview) -> Self {
        self.flags.set(
            ImGuiColorEditFlags::AlphaPreviewHalf,
            preview == EditableColorPreview::HalfAlpha,
        );
        self.flags.set(
            ImGuiColorEditFlags::AlphaPreview,
            preview == EditableColorPreview::Alpha,
        );
        self
    }
    #[inline]
    pub fn hdr(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::HDR, value);
        self
    }
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
    #[inline]
    pub fn format(mut self, format: EditableColorFormat) -> Self {
        self.flags.set(
            ImGuiColorEditFlags::Uint8,
            format == EditableColorFormat::U8,
        );
        self.flags.set(
            ImGuiColorEditFlags::Float,
            format == EditableColorFormat::Float,
        );
        self
    }
    pub fn build(self) -> bool {
        match self.value {
            EditableColor::Float3(value) => unsafe {
                imgui_sys::igColorEdit3(self.label.as_ptr(), value.as_mut_ptr(), self.flags)
            },
            EditableColor::Float4(value) => unsafe {
                imgui_sys::igColorEdit4(self.label.as_ptr(), value.as_mut_ptr(), self.flags)
            },
        }
    }
}


#[must_use]
pub struct ColorPicker<'ui, 'p> {
    label: &'p ImStr,
    value: EditableColor<'p>,
    flags: ImGuiColorEditFlags,
    ref_color: Option<&'p [f32; 4]>,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> ColorPicker<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p ImStr, value: EditableColor<'p>) -> Self {
        ColorPicker {
            label,
            value,
            flags: ImGuiColorEditFlags::empty(),
            ref_color: None,
            _phantom: PhantomData,
        }
    }
    #[inline]
    pub fn flags(mut self, flags: ImGuiColorEditFlags) -> Self {
        self.flags = flags;
        self
    }
    #[inline]
    pub fn alpha(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoAlpha, !value);
        self
    }
    #[inline]
    pub fn small_preview(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoSmallPreview, !value);
        self
    }
    #[inline]
    pub fn inputs(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoInputs, !value);
        self
    }
    #[inline]
    pub fn tooltip(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoTooltip, !value);
        self
    }
    #[inline]
    pub fn label(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoLabel, !value);
        self
    }
    #[inline]
    pub fn side_preview(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::NoSidePreview, !value);
        self
    }
    #[inline]
    pub fn alpha_bar(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::AlphaBar, value);
        self
    }
    #[inline]
    pub fn preview(mut self, preview: EditableColorPreview) -> Self {
        self.flags.set(
            ImGuiColorEditFlags::AlphaPreviewHalf,
            preview == EditableColorPreview::HalfAlpha,
        );
        self.flags.set(
            ImGuiColorEditFlags::AlphaPreview,
            preview == EditableColorPreview::Alpha,
        );
        self
    }
    #[inline]
    pub fn rgb(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::RGB, value);
        self
    }
    #[inline]
    pub fn hsv(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::HSV, value);
        self
    }
    #[inline]
    pub fn hex(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::HEX, value);
        self
    }
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
    #[inline]
    pub fn format(mut self, format: EditableColorFormat) -> Self {
        self.flags.set(
            ImGuiColorEditFlags::Uint8,
            format == EditableColorFormat::U8,
        );
        self.flags.set(
            ImGuiColorEditFlags::Float,
            format == EditableColorFormat::Float,
        );
        self
    }
    #[inline]
    pub fn reference_color(mut self, ref_color: &'p [f32; 4]) -> Self {
        self.ref_color = Some(ref_color);
        self
    }
    pub fn build(mut self) -> bool {
        if let EditableColor::Float3(_) = self.value {
            self.flags.insert(ImGuiColorEditFlags::NoAlpha);
        }
        let ref_color = self.ref_color.map(|c| c.as_ptr()).unwrap_or(
            ptr::null(),
        );
        unsafe {
            imgui_sys::igColorPicker4(
                self.label.as_ptr(),
                self.value.as_mut_ptr(),
                self.flags,
                ref_color,
            )
        }
    }
}
