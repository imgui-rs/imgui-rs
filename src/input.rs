use imgui_sys;
use libc::size_t;
use std::marker::PhantomData;
use std::ptr;

use super::{ImGuiInputTextFlags,
            ImGuiInputTextFlags_AllowTabInput /* ImGuiInputTextFlags_CtrlEnterForNewLine, */,
            ImGuiInputTextFlags_AlwaysInsertMode, ImGuiInputTextFlags_AutoSelectAll,
            ImGuiInputTextFlags_CallbackAlways, ImGuiInputTextFlags_CallbackCharFilter,
            ImGuiInputTextFlags_CallbackCompletion, ImGuiInputTextFlags_CallbackHistory,
            ImGuiInputTextFlags_CharsDecimal, ImGuiInputTextFlags_CharsHexadecimal,
            ImGuiInputTextFlags_CharsNoBlank, ImGuiInputTextFlags_CharsUppercase,
            ImGuiInputTextFlags_EnterReturnsTrue, ImGuiInputTextFlags_NoHorizontalScroll, Ui};

macro_rules! impl_text_flags {
    ($InputType:ident) => {
        #[inline]
        pub fn flags(self, flags: ImGuiInputTextFlags) -> Self {
            $InputType {
                flags: flags,
                .. self
            }
        }

        #[inline]
        pub fn chars_decimal(self, value: bool) -> Self {
            $InputType {
                flags: self.flags.with(ImGuiInputTextFlags_CharsDecimal, value),
                .. self
            }
        }

        #[inline]
        pub fn chars_hexadecimal(self, value: bool) -> Self {
            $InputType {
                flags: self.flags.with(ImGuiInputTextFlags_CharsHexadecimal, value),
                .. self
            }
        }

        #[inline]
        pub fn chars_uppercase(self, value: bool) -> Self {
            $InputType {
                flags: self.flags.with(ImGuiInputTextFlags_CharsUppercase, value),
                .. self
            }
        }

        #[inline]
        pub fn chars_noblank(self, value: bool) -> Self {
            $InputType {
                flags: self.flags.with(ImGuiInputTextFlags_CharsNoBlank, value),
                .. self
            }
        }

        #[inline]
        pub fn auto_select_all(self, value: bool) -> Self {
            $InputType {
                flags: self.flags.with(ImGuiInputTextFlags_AutoSelectAll, value),
                .. self
            }
        }

        #[inline]
        pub fn enter_returns_true(self, value: bool) -> Self {
            $InputType {
                flags: self.flags.with(ImGuiInputTextFlags_EnterReturnsTrue, value),
                .. self
            }
        }

        #[inline]
        pub fn callback_completion(self, value: bool) -> Self {
            $InputType {
                flags: self.flags.with(ImGuiInputTextFlags_CallbackCompletion, value),
                .. self
            }
        }

        #[inline]
        pub fn callback_history(self, value: bool) -> Self {
            $InputType {
                flags: self.flags.with(ImGuiInputTextFlags_CallbackHistory, value),
                .. self
            }
        }

        #[inline]
        pub fn callback_always(self, value: bool) -> Self {
            $InputType {
                flags: self.flags.with(ImGuiInputTextFlags_CallbackAlways, value),
                .. self
            }
        }

        #[inline]
        pub fn callback_char_filter(self, value: bool) -> Self {
            $InputType {
                flags: self.flags.with(ImGuiInputTextFlags_CallbackCharFilter, value),
                .. self
            }
        }

        #[inline]
        pub fn allow_tab_input(self, value: bool) -> Self {
            $InputType {
                flags: self.flags.with(ImGuiInputTextFlags_AllowTabInput, value),
                .. self
            }
        }

        #[inline]
        pub fn no_horizontal_scroll(self, value: bool) -> Self {
            $InputType {
                flags: self.flags.with(ImGuiInputTextFlags_NoHorizontalScroll, value),
                .. self
            }
        }

        #[inline]
        pub fn always_insert_mode(self, value: bool) -> Self {
            $InputType {
                flags: self.flags.with(ImGuiInputTextFlags_AlwaysInsertMode, value),
                .. self
            }
        }

    }
}

macro_rules! impl_step_params {
    ($InputType:ident, $Value:ty) => {
        #[inline]
        pub fn step(self, value: $Value) -> Self {
            $InputType {
                step: value,
                .. self
            }
        }

        #[inline]
        pub fn step_fast(self, value: $Value) -> Self {
            $InputType {
                step_fast: value,
                .. self
            }
        }
    }
}

macro_rules! impl_precision_params {
    ($InputType:ident) => {
        #[inline]
        pub fn decimal_precision(self, value: i32) -> Self {
            $InputType {
                decimal_precision: value,
                .. self
            }
        }
    }
}

#[must_use]
pub struct InputText<'ui, 'p> {
    label: &'p str,
    buf: &'p mut str,
    flags: ImGuiInputTextFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> InputText<'ui, 'p> {
    pub fn new(label: &'p str, buf: &'p mut str) -> Self {
        InputText {
            label: label,
            buf: buf,
            flags: ImGuiInputTextFlags::empty(),
            _phantom: PhantomData,
        }
    }

    impl_text_flags!(InputText);

    // TODO: boxed closure...?
    // pub fn callback(self) -> Self { }

    pub fn build(self) -> bool {
        unsafe {
            imgui_sys::igInputText(imgui_sys::ImStr::from(self.label),
                                   // TODO: this is evil.
                                   // Perhaps something else than &mut str is better
                                   self.buf.as_ptr() as *mut i8,
                                   self.buf.len() as size_t,
                                   self.flags,
                                   None,
                                   ptr::null_mut())
        }
    }
}

#[must_use]
pub struct InputInt<'ui, 'p> {
    label: &'p str,
    value: &'p mut i32,
    step: i32,
    step_fast: i32,
    flags: ImGuiInputTextFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> InputInt<'ui, 'p> {
    pub fn new(label: &'p str, value: &'p mut i32) -> Self {
        InputInt {
            label: label,
            value: value,
            step: 1,
            step_fast: 100,
            flags: ImGuiInputTextFlags::empty(),
            _phantom: PhantomData,
        }
    }

    pub fn build(self) -> bool {
        unsafe {
            imgui_sys::igInputInt(imgui_sys::ImStr::from(self.label),
                                  self.value as *mut i32,
                                  self.step,
                                  self.step_fast,
                                  self.flags)
        }
    }

    impl_step_params!(InputInt, i32);
    impl_text_flags!(InputInt);
}

#[must_use]
pub struct InputFloat<'ui, 'p> {
    label: &'p str,
    value: &'p mut f32,
    step: f32,
    step_fast: f32,
    decimal_precision: i32,
    flags: ImGuiInputTextFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> InputFloat<'ui, 'p> {
    pub fn new(label: &'p str, value: &'p mut f32) -> Self {
        InputFloat {
            label: label,
            value: value,
            step: 0.0,
            step_fast: 0.0,
            decimal_precision: -1,
            flags: ImGuiInputTextFlags::empty(),
            _phantom: PhantomData,
        }
    }

    pub fn build(self) -> bool {
        unsafe {
            imgui_sys::igInputFloat(imgui_sys::ImStr::from(self.label),
                                    self.value as *mut f32,
                                    self.step,
                                    self.step_fast,
                                    self.decimal_precision,
                                    self.flags)
        }
    }

    impl_step_params!(InputFloat, f32);
    impl_precision_params!(InputFloat);
    impl_text_flags!(InputFloat);
}

macro_rules! impl_input_floatn {
    ($InputFloatN:ident, $N:expr, $igInputFloatN:ident) => {
        #[must_use]
        pub struct $InputFloatN<'ui, 'p> {
            label: &'p str,
            value: &'p mut [f32;$N],
            decimal_precision: i32,
            flags: ImGuiInputTextFlags,
            _phantom: PhantomData<&'ui Ui<'ui>>
        }

        impl<'ui, 'p> $InputFloatN<'ui, 'p> {
            pub fn new(label: &'p str, value: &'p mut [f32;$N]) -> Self {
                $InputFloatN {
                    label: label,
                    value: value,
                    decimal_precision: -1,
                    flags: ImGuiInputTextFlags::empty(),
                    _phantom: PhantomData
                }
            }

            pub fn build(self) -> bool {
                unsafe {
                    imgui_sys::$igInputFloatN(
                        imgui_sys::ImStr::from(self.label),
                        self.value.as_mut_ptr(),
                        self.decimal_precision,
                        self.flags)
                }
            }

            impl_precision_params!($InputFloatN);
            impl_text_flags!($InputFloatN);
        }
    }
}

impl_input_floatn!(InputFloat2, 2, igInputFloat2);
impl_input_floatn!(InputFloat3, 3, igInputFloat3);
impl_input_floatn!(InputFloat4, 4, igInputFloat4);

macro_rules! impl_input_intn {
    ($InputIntN:ident, $N:expr, $igInputIntN:ident) => {
        #[must_use]
        pub struct $InputIntN<'ui, 'p> {
            label: &'p str,
            value: &'p mut [i32;$N],
            flags: ImGuiInputTextFlags,
            _phantom: PhantomData<&'ui Ui<'ui>>
        }

        impl<'ui, 'p> $InputIntN<'ui, 'p> {
            pub fn new(label: &'p str, value: &'p mut [i32;$N]) -> Self {
                $InputIntN {
                    label: label,
                    value: value,
                    flags: ImGuiInputTextFlags::empty(),
                    _phantom: PhantomData
                }
            }

            pub fn build(self) -> bool {
                unsafe {
                    imgui_sys::$igInputIntN(
                        imgui_sys::ImStr::from(self.label),
                        self.value.as_mut_ptr(),
                        self.flags)
                }
            }

            impl_text_flags!($InputIntN);
        }
    }
}

impl_input_intn!(InputInt2, 2, igInputInt2);
impl_input_intn!(InputInt3, 3, igInputInt3);
impl_input_intn!(InputInt4, 4, igInputInt4);

#[must_use]
pub struct ColorEdit3<'ui, 'p> {
    label: &'p str,
    value: &'p mut [f32; 3],
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> ColorEdit3<'ui, 'p> {
    pub fn new(label: &'p str, value: &'p mut [f32; 3]) -> Self {
        ColorEdit3 {
            label: label,
            value: value,
            _phantom: PhantomData,
        }
    }

    pub fn build(self) -> bool {
        unsafe {
            imgui_sys::igColorEdit3(imgui_sys::ImStr::from(self.label), self.value.as_mut_ptr())
        }
    }
}

#[must_use]
pub struct ColorEdit4<'ui, 'p> {
    label: &'p str,
    value: &'p mut [f32; 4],
    show_alpha: bool,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> ColorEdit4<'ui, 'p> {
    pub fn new(label: &'p str, value: &'p mut [f32; 4]) -> Self {
        ColorEdit4 {
            label: label,
            value: value,
            show_alpha: true,
            _phantom: PhantomData,
        }
    }

    pub fn build(self) -> bool {
        unsafe {
            imgui_sys::igColorEdit4(imgui_sys::ImStr::from(self.label),
                                    self.value.as_mut_ptr(),
                                    self.show_alpha)
        }
    }
}
