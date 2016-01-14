use imgui_sys;
use libc::size_t;
use std::marker::PhantomData;
use std::ptr;

use super::{
    Ui,
    ImGuiInputTextFlags, ImGuiInputTextFlags_CharsDecimal, ImGuiInputTextFlags_CharsHexadecimal,
    ImGuiInputTextFlags_CharsUppercase, ImGuiInputTextFlags_CharsNoBlank,
    ImGuiInputTextFlags_AutoSelectAll, ImGuiInputTextFlags_EnterReturnsTrue,
    ImGuiInputTextFlags_CallbackCompletion, ImGuiInputTextFlags_CallbackHistory,
    ImGuiInputTextFlags_CallbackAlways, ImGuiInputTextFlags_CallbackCharFilter,
    ImGuiInputTextFlags_AllowTabInput, //ImGuiInputTextFlags_CtrlEnterForNewLine,
    ImGuiInputTextFlags_NoHorizontalScroll, ImGuiInputTextFlags_AlwaysInsertMode,
    ImStr
};

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

#[must_use]
pub struct InputText<'ui, 'p> {
    label: ImStr<'p>,
    buf: &'p mut str,
    flags: ImGuiInputTextFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>
}

impl<'ui, 'p> InputText<'ui, 'p> {
    pub fn new(label: ImStr<'p>, buf: &'p mut str) -> Self {
        InputText {
            label: label,
            buf: buf,
            flags: ImGuiInputTextFlags::empty(),
            _phantom: PhantomData
        }
    }

    impl_text_flags!(InputText);

    // TODO: boxed closure...?
    // pub fn callback(self) -> Self { }

    pub fn build(self) -> bool {
        unsafe {
            imgui_sys::igInputText(
                self.label.as_ptr(),
                // TODO: this is evil. Perhaps something else than &mut str is better
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
    label: ImStr<'p>,
    value: &'p mut i32,
    step: i32,
    step_fast: i32,
    flags: ImGuiInputTextFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>
}

impl<'ui, 'p> InputInt<'ui, 'p> {
    pub fn new(label: ImStr<'p>, value: &'p mut i32) -> Self {
        InputInt {
            label: label,
            value: value,
            step: 1,
            step_fast: 100,
            flags: ImGuiInputTextFlags::empty(),
            _phantom: PhantomData
        }
    }

    pub fn build(self) -> bool {
        unsafe {
            imgui_sys::igInputInt(
                self.label.as_ptr(),
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
    label: ImStr<'p>,
    value: &'p mut f32,
    step: f32,
    step_fast: f32,
    decimal_precision: i32,
    flags: ImGuiInputTextFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>
}

impl<'ui, 'p> InputFloat<'ui, 'p> {
    pub fn new(label: ImStr<'p>, value: &'p mut f32) -> Self {
        InputFloat {
            label: label,
            value: value,
            step: 0.0,
            step_fast: 0.0,
            decimal_precision: -1,
            flags: ImGuiInputTextFlags::empty(),
            _phantom: PhantomData
        }
    }

    pub fn decimal_precision(self, value: i32) -> Self {
        InputFloat {
            decimal_precision: value,
            .. self
        }
    }

    pub fn build(self) -> bool {
        unsafe {
            imgui_sys::igInputFloat(
                self.label.as_ptr(),
                self.value as *mut f32,
                self.step,
                self.step_fast,
                self.decimal_precision,
                self.flags)
        }
    }

    impl_step_params!(InputFloat, f32);
    impl_text_flags!(InputFloat);

}
