use bitflags::bitflags;
use std::os::raw::c_void;

use crate::internal::DataTypeKind;
use crate::math::MintVec2;
use crate::sys;
use crate::Ui;

bitflags!(
    /// Flags for sliders
    #[repr(transparent)]
    pub struct SliderFlags: u32 {
        /// Clamp value to min/max bounds when input manually with CTRL+Click.
        ///
        /// By default CTRL+click allows going out of bounds.
        const ALWAYS_CLAMP = sys::ImGuiSliderFlags_AlwaysClamp;
        /// Make the widget logarithmic instead of linear
        const LOGARITHMIC = sys::ImGuiSliderFlags_Logarithmic;
        /// Disable rounding underlying value to match precision of the display format string
        const NO_ROUND_TO_FORMAT = sys::ImGuiSliderFlags_NoRoundToFormat;
        /// Disable CTRL+Click or Enter key allowing to input text directly into the widget
        const NO_INPUT = sys::ImGuiSliderFlags_NoInput;
    }
);

impl Ui {
    /// Creates a new slider widget. Returns true if the value has been edited.
    pub fn slider<T: AsRef<str>, K: DataTypeKind>(
        &self,
        label: T,
        min: K,
        max: K,
        value: &mut K,
    ) -> bool {
        self.slider_config(label, min, max).build(value)
    }

    /// Creates an new ubuilt Slider.
    pub fn slider_config<T: AsRef<str>, K: DataTypeKind>(
        &self,
        label: T,
        min: K,
        max: K,
    ) -> Slider<'_, T, K> {
        Slider {
            label,
            min,
            max,
            display_format: Option::<&'static str>::None,
            flags: SliderFlags::empty(),
            ui: self,
        }
    }
}

/// Builder for a slider widget.
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct Slider<'ui, Label, Data, Format = &'static str> {
    label: Label,
    min: Data,
    max: Data,
    display_format: Option<Format>,
    flags: SliderFlags,
    ui: &'ui Ui,
}

impl<'ui, T: AsRef<str>, K: DataTypeKind> Slider<'ui, T, K> {
    /// Constructs a new slider builder with the given range.
    #[doc(alias = "SliderScalar", alias = "SliderScalarN")]
    #[deprecated(note = "Use `Ui::slider` or `Ui::slider_config`.", since = "0.9.0")]
    pub fn new(ui: &'ui Ui, label: T, min: K, max: K) -> Self {
        Slider {
            label,
            min,
            max,
            display_format: None,
            flags: SliderFlags::empty(),
            ui,
        }
    }
}

impl<'ui, Label, Data, Format> Slider<'ui, Label, Data, Format>
where
    Label: AsRef<str>,
    Data: DataTypeKind,
    Format: AsRef<str>,
{
    /// Sets the range inclusively, such that both values given
    /// are valid values which the slider can be dragged to.
    ///
    /// ```no_run
    /// # let mut ctx = imgui::Context::create();
    /// # let ui = ctx.frame();
    /// ui.slider_config("Example", i8::MIN, i8::MAX)
    ///     .range(4, 8)
    ///     // Remember to call .build()
    ///     ;
    /// ```
    ///
    /// It is safe, though up to C++ Dear ImGui, on how to handle when
    /// `min > max`.
    ///
    /// Note for f32 and f64 sliders, Dear ImGui limits the available
    /// range to half their full range (e.g `f32::MIN/2.0 .. f32::MAX/2.0`)
    /// Specifying a value above this will cause an abort.
    /// For large ranged values, consider using [`Ui::input_scalar`] instead
    #[inline]
    pub fn range(mut self, min: Data, max: Data) -> Self {
        self.min = min;
        self.max = max;
        self
    }
    /// Sets the display format using *a C-style printf string*
    #[inline]
    pub fn display_format<Format2: AsRef<str>>(
        self,
        display_format: Format2,
    ) -> Slider<'ui, Label, Data, Format2> {
        Slider {
            label: self.label,
            min: self.min,
            max: self.max,
            display_format: Some(display_format),
            flags: self.flags,
            ui: self.ui,
        }
    }
    /// Replaces all current settings with the given flags
    #[inline]
    pub fn flags(mut self, flags: SliderFlags) -> Self {
        self.flags = flags;
        self
    }
    /// Builds a slider that is bound to the given value.
    ///
    /// Returns true if the slider value was changed.
    pub fn build(self, value: &mut Data) -> bool {
        unsafe {
            let (label, display_format) = self
                .ui
                .scratch_txt_with_opt(self.label, self.display_format);

            sys::ImGui_SliderScalar(
                label,
                Data::KIND as i32,
                value as *mut Data as *mut c_void,
                &self.min as *const Data as *const c_void,
                &self.max as *const Data as *const c_void,
                display_format,
                self.flags.bits() as i32,
            )
        }
    }
    /// Builds a horizontal array of multiple sliders attached to the given slice.
    ///
    /// Returns true if any slider value was changed.
    pub fn build_array(self, values: &mut [Data]) -> bool {
        unsafe {
            let (label, display_format) = self
                .ui
                .scratch_txt_with_opt(self.label, self.display_format);

            sys::ImGui_SliderScalarN(
                label,
                Data::KIND as i32,
                values.as_mut_ptr() as *mut c_void,
                values.len() as i32,
                &self.min as *const Data as *const c_void,
                &self.max as *const Data as *const c_void,
                display_format,
                self.flags.bits() as i32,
            )
        }
    }
}

/// Builder for a vertical slider widget.
#[derive(Clone, Debug)]
#[must_use]
pub struct VerticalSlider<Label, Data, Format = &'static str> {
    label: Label,
    size: [f32; 2],
    min: Data,
    max: Data,
    display_format: Option<Format>,
    flags: SliderFlags,
}

impl<Label, Data> VerticalSlider<Label, Data>
where
    Label: AsRef<str>,
    Data: DataTypeKind,
{
    /// Constructs a new vertical slider builder with the given size and range.
    ///
    /// ```rust
    /// imgui::VerticalSlider::new("Example", [20.0, 20.0], i8::MIN, i8::MAX)
    ///     .range(4, 8)
    ///     // Remember to call .build(&ui)
    ///     ;
    /// ```
    ///
    /// It is safe, though up to C++ Dear ImGui, on how to handle when
    /// `min > max`.
    #[doc(alias = "VSliderScalar")]
    pub fn new(label: Label, size: impl Into<MintVec2>, min: Data, max: Data) -> Self {
        VerticalSlider {
            label,
            size: size.into().into(),
            min,
            max,
            display_format: None,
            flags: SliderFlags::empty(),
        }
    }
}

impl<Label, Data, Format> VerticalSlider<Label, Data, Format>
where
    Label: AsRef<str>,
    Data: DataTypeKind,
    Format: AsRef<str>,
{
    /// Sets the range for the vertical slider.
    ///
    /// ```rust
    /// imgui::VerticalSlider::new("Example", [20.0, 20.0], i8::MIN, i8::MAX)
    ///     .range(4, 8)
    ///     // Remember to call .build(&ui)
    ///     ;
    /// ```
    ///
    /// It is safe, though up to C++ Dear ImGui, on how to handle when
    /// `min > max`.
    #[inline]
    pub fn range(mut self, min: Data, max: Data) -> Self {
        self.min = min;
        self.max = max;
        self
    }
    /// Sets the display format using *a C-style printf string*
    #[inline]
    pub fn display_format<Format2: AsRef<str>>(
        self,
        display_format: Format2,
    ) -> VerticalSlider<Label, Data, Format2> {
        VerticalSlider {
            label: self.label,
            size: self.size,
            min: self.min,
            max: self.max,
            display_format: Some(display_format),
            flags: self.flags,
        }
    }
    /// Replaces all current settings with the given flags
    #[inline]
    pub fn flags(mut self, flags: SliderFlags) -> Self {
        self.flags = flags;
        self
    }
    /// Builds a vertical slider that is bound to the given value.
    ///
    /// Returns true if the slider value was changed.
    pub fn build(self, ui: &Ui, value: &mut Data) -> bool {
        unsafe {
            let (label, display_format) = ui.scratch_txt_with_opt(self.label, self.display_format);

            sys::ImGui_VSliderScalar(
                label,
                self.size.into(),
                Data::KIND as i32,
                value as *mut Data as *mut c_void,
                &self.min as *const Data as *const c_void,
                &self.max as *const Data as *const c_void,
                display_format,
                self.flags.bits() as i32,
            )
        }
    }
}

/// Builder for an angle slider widget.
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct AngleSlider<Label, Format = &'static str> {
    label: Label,
    min_degrees: f32,
    max_degrees: f32,
    display_format: Format,
    flags: SliderFlags,
}

impl<Label> AngleSlider<Label>
where
    Label: AsRef<str>,
{
    /// Constructs a new angle slider builder, where its minimum defaults to -360.0 and
    /// maximum defaults to 360.0
    #[doc(alias = "SliderAngle")]
    pub fn new(label: Label) -> Self {
        AngleSlider {
            label,
            min_degrees: -360.0,
            max_degrees: 360.0,
            display_format: "%.0f deg",
            flags: SliderFlags::empty(),
        }
    }
}

impl<Label, Format> AngleSlider<Label, Format>
where
    Label: AsRef<str>,
    Format: AsRef<str>,
{
    /// Sets the range in degrees (inclusive)
    /// ```rust
    /// imgui::AngleSlider::new("Example")
    ///     .range_degrees(-20.0, 20.0)
    ///     // Remember to call .build(&ui)
    ///     ;
    /// ```
    ///
    /// It is safe, though up to C++ Dear ImGui, on how to handle when
    /// `min > max`.
    #[inline]
    pub fn range_degrees(mut self, min_degrees: f32, max_degrees: f32) -> Self {
        self.min_degrees = min_degrees;
        self.max_degrees = max_degrees;
        self
    }
    /// Sets the minimum value (in degrees)
    #[inline]
    pub fn min_degrees(mut self, min_degrees: f32) -> Self {
        self.min_degrees = min_degrees;
        self
    }
    /// Sets the maximum value (in degrees)
    #[inline]
    pub fn max_degrees(mut self, max_degrees: f32) -> Self {
        self.max_degrees = max_degrees;
        self
    }
    /// Sets the display format using *a C-style printf string*
    #[inline]
    pub fn display_format<Format2: AsRef<str>>(
        self,
        display_format: Format2,
    ) -> AngleSlider<Label, Format2> {
        AngleSlider {
            label: self.label,
            min_degrees: self.min_degrees,
            max_degrees: self.max_degrees,
            display_format,
            flags: self.flags,
        }
    }
    /// Replaces all current settings with the given flags
    #[inline]
    pub fn flags(mut self, flags: SliderFlags) -> Self {
        self.flags = flags;
        self
    }
    /// Builds an angle slider that is bound to the given value (in radians).
    ///
    /// Returns true if the slider value was changed.
    pub fn build(self, ui: &Ui, value_rad: &mut f32) -> bool {
        unsafe {
            let (label, display_format) = ui.scratch_txt_two(self.label, self.display_format);

            sys::ImGui_SliderAngle(
                label,
                value_rad as *mut _,
                self.min_degrees,
                self.max_degrees,
                display_format,
                self.flags.bits() as i32,
            )
        }
    }
}
