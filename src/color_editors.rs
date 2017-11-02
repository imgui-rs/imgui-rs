use imgui_sys;
use std::marker::PhantomData;

use {ImGuiColorEditFlags, ImStr, Ui};

#[derive(Debug)]
pub enum EditableColor<'p> {
    Float3(&'p mut [f32; 3]),
    Float4(&'p mut [f32; 4]),
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
pub enum EditableColorFormat {
    U8,
    Float,
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
    pub fn alpha_preview(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::AlphaPreview, value);
        self
    }
    #[inline]
    pub fn alpha_preview_half(mut self, value: bool) -> Self {
        self.flags.set(ImGuiColorEditFlags::AlphaPreviewHalf, value);
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
