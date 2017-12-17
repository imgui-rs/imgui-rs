use sys;
use std::marker::PhantomData;
use std::ptr;

use super::{ImGuiInputTextFlags, ImString, Ui};

macro_rules! impl_text_flags {
    ($InputType:ident) => {
        #[inline]
        pub fn flags(mut self, flags: ImGuiInputTextFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn chars_decimal(mut self, value: bool) -> Self {
            self.flags.set(ImGuiInputTextFlags::CharsDecimal, value);
            self
        }

        #[inline]
        pub fn chars_hexadecimal(mut self, value: bool) -> Self {
            self.flags.set(ImGuiInputTextFlags::CharsHexadecimal, value);
            self
        }

        #[inline]
        pub fn chars_uppercase(mut self, value: bool) -> Self {
            self.flags.set(ImGuiInputTextFlags::CharsUppercase, value);
            self
        }

        #[inline]
        pub fn chars_noblank(mut self, value: bool) -> Self {
            self.flags.set(ImGuiInputTextFlags::CharsNoBlank, value);
            self
        }

        #[inline]
        pub fn auto_select_all(mut self, value: bool) -> Self {
            self.flags.set(ImGuiInputTextFlags::AutoSelectAll, value);
            self
        }

        #[inline]
        pub fn enter_returns_true(mut self, value: bool) -> Self {
            self.flags.set(ImGuiInputTextFlags::EnterReturnsTrue, value);
            self
        }

        #[inline]
        pub fn callback_completion(mut self, value: bool) -> Self {
            self.flags.set(ImGuiInputTextFlags::CallbackCompletion, value);
            self
        }

        #[inline]
        pub fn callback_history(mut self, value: bool) -> Self {
            self.flags.set(ImGuiInputTextFlags::CallbackHistory, value);
            self
        }

        #[inline]
        pub fn callback_always(mut self, value: bool) -> Self {
            self.flags.set(ImGuiInputTextFlags::CallbackAlways, value);
            self
        }

        #[inline]
        pub fn callback_char_filter(mut self, value: bool) -> Self {
            self.flags.set(ImGuiInputTextFlags::CallbackCharFilter, value);
            self
        }

        #[inline]
        pub fn allow_tab_input(mut self, value: bool) -> Self {
            self.flags.set(ImGuiInputTextFlags::AllowTabInput, value);
            self
        }

        #[inline]
        pub fn no_horizontal_scroll(mut self, value: bool) -> Self {
            self.flags.set(ImGuiInputTextFlags::NoHorizontalScroll, value);
            self
        }

        #[inline]
        pub fn always_insert_mode(mut self, value: bool) -> Self {
            self.flags.set(ImGuiInputTextFlags::AlwaysInsertMode, value);
            self
        }
    }
}

macro_rules! impl_step_params {
    ($InputType:ident, $Value:ty) => {
        #[inline]
        pub fn step(mut self, value: $Value) -> Self {
            self.step = value;
            self
        }

        #[inline]
        pub fn step_fast(mut self, value: $Value) -> Self {
            self.step_fast = value;
            self
        }
    }
}

macro_rules! impl_precision_params {
    ($InputType:ident) => {
        #[inline]
        pub fn decimal_precision(mut self, value: i32) -> Self {
            self.decimal_precision = value;
            self
        }
    }
}

#[must_use]
pub struct InputText<'ui, 'p> {
    label: &'p str,
    buf: &'p mut ImString,
    flags: ImGuiInputTextFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> InputText<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p str, buf: &'p mut ImString) -> Self {
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
            sys::igInputText(
                sys::ImStr::from(self.label),
                self.buf.as_mut_ptr(),
                self.buf.capacity_with_nul(),
                self.flags,
                None,
                ptr::null_mut(),
            )
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
    pub fn new(_: &Ui<'ui>, label: &'p str, value: &'p mut i32) -> Self {
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
            sys::igInputInt(
                sys::ImStr::from(self.label),
                self.value as *mut i32,
                self.step,
                self.step_fast,
                self.flags,
            )
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
    pub fn new(_: &Ui<'ui>, label: &'p str, value: &'p mut f32) -> Self {
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
            sys::igInputFloat(
                sys::ImStr::from(self.label),
                self.value as *mut f32,
                self.step,
                self.step_fast,
                self.decimal_precision,
                self.flags,
            )
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
            pub fn new(_: &Ui<'ui>, label: &'p str, value: &'p mut [f32;$N]) -> Self {
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
                    sys::$igInputFloatN(
                        sys::ImStr::from(self.label),
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
            pub fn new(_: &Ui<'ui>, label: &'p str, value: &'p mut [i32;$N]) -> Self {
                $InputIntN {
                    label: label,
                    value: value,
                    flags: ImGuiInputTextFlags::empty(),
                    _phantom: PhantomData
                }
            }

            pub fn build(self) -> bool {
                unsafe {
                    sys::$igInputIntN(
                        sys::ImStr::from(self.label),
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
