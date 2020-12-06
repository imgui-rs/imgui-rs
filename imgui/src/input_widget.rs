use std::marker::PhantomData;
use std::os::raw::{c_int, c_void};
use std::ptr;

use crate::legacy::ImGuiInputTextFlags;
use crate::sys;
use crate::{ImStr, ImString, Ui};

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
            self.flags
                .set(ImGuiInputTextFlags::CallbackCompletion, value);
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
            self.flags
                .set(ImGuiInputTextFlags::CallbackCharFilter, value);
            self
        }

        #[inline]
        pub fn resize_buffer(mut self, value: bool) -> Self {
            self.flags.set(ImGuiInputTextFlags::CallbackResize, value);
            self
        }

        #[inline]
        pub fn allow_tab_input(mut self, value: bool) -> Self {
            self.flags.set(ImGuiInputTextFlags::AllowTabInput, value);
            self
        }

        #[inline]
        pub fn no_horizontal_scroll(mut self, value: bool) -> Self {
            self.flags
                .set(ImGuiInputTextFlags::NoHorizontalScroll, value);
            self
        }

        #[inline]
        pub fn always_insert_mode(mut self, value: bool) -> Self {
            self.flags.set(ImGuiInputTextFlags::AlwaysInsertMode, value);
            self
        }

        #[inline]
        pub fn read_only(mut self, value: bool) -> Self {
            self.flags.set(ImGuiInputTextFlags::ReadOnly, value);
            self
        }

        #[inline]
        pub fn password(mut self, value: bool) -> Self {
            self.flags.set(ImGuiInputTextFlags::Password, value);
            self
        }

        #[inline]
        pub fn no_undo_redo(mut self, value: bool) -> Self {
            self.flags.set(ImGuiInputTextFlags::NoUndoRedo, value);
            self
        }
    };
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
    };
}

extern "C" fn resize_callback(data: *mut sys::ImGuiInputTextCallbackData) -> c_int {
    unsafe {
        if (*data).EventFlag == ImGuiInputTextFlags::CallbackResize.bits() {
            if let Some(buffer) = ((*data).UserData as *mut ImString).as_mut() {
                let requested_size = (*data).BufSize as usize;
                if requested_size > buffer.capacity_with_nul() {
                    // Refresh the buffer's length to take into account changes made by dear imgui.
                    buffer.refresh_len();
                    buffer.reserve(requested_size - buffer.0.len());
                    debug_assert!(buffer.capacity_with_nul() >= requested_size);
                    (*data).Buf = buffer.as_mut_ptr();
                    (*data).BufDirty = true;
                }
            }
        }
        0
    }
}

#[must_use]
pub struct InputText<'ui, 'p> {
    label: &'p ImStr,
    buf: &'p mut ImString,
    flags: ImGuiInputTextFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> InputText<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p ImStr, buf: &'p mut ImString) -> Self {
        InputText {
            label,
            buf,
            flags: ImGuiInputTextFlags::empty(),
            _phantom: PhantomData,
        }
    }

    impl_text_flags!(InputText);

    // TODO: boxed closure...?
    // pub fn callback(self) -> Self { }

    pub fn build(self) -> bool {
        let (ptr, capacity) = (self.buf.as_mut_ptr(), self.buf.capacity_with_nul());
        let (callback, data): (sys::ImGuiInputTextCallback, _) = {
            if self.flags.contains(ImGuiInputTextFlags::CallbackResize) {
                (Some(resize_callback), self.buf as *mut _ as *mut c_void)
            } else {
                (None, ptr::null_mut())
            }
        };

        unsafe {
            let result = sys::igInputText(
                self.label.as_ptr(),
                ptr,
                capacity,
                self.flags.bits(),
                callback,
                data,
            );
            self.buf.refresh_len();
            result
        }
    }
}

#[must_use]
pub struct InputTextMultiline<'ui, 'p> {
    label: &'p ImStr,
    buf: &'p mut ImString,
    flags: ImGuiInputTextFlags,
    size: [f32; 2],
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> InputTextMultiline<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p ImStr, buf: &'p mut ImString, size: [f32; 2]) -> Self {
        InputTextMultiline {
            label,
            buf,
            flags: ImGuiInputTextFlags::empty(),
            size,
            _phantom: PhantomData,
        }
    }

    impl_text_flags!(InputText);

    // TODO: boxed closure...?
    // pub fn callback(self) -> Self { }

    pub fn build(self) -> bool {
        let (ptr, capacity) = (self.buf.as_mut_ptr(), self.buf.capacity_with_nul());
        let (callback, data): (sys::ImGuiInputTextCallback, _) = {
            if self.flags.contains(ImGuiInputTextFlags::CallbackResize) {
                (Some(resize_callback), self.buf as *mut _ as *mut c_void)
            } else {
                (None, ptr::null_mut())
            }
        };

        unsafe {
            let result = sys::igInputTextMultiline(
                self.label.as_ptr(),
                ptr,
                capacity,
                self.size.into(),
                self.flags.bits(),
                callback,
                data,
            );
            self.buf.refresh_len();
            result
        }
    }
}

#[must_use]
pub struct InputInt<'ui, 'p> {
    label: &'p ImStr,
    value: &'p mut i32,
    step: i32,
    step_fast: i32,
    flags: ImGuiInputTextFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> InputInt<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p ImStr, value: &'p mut i32) -> Self {
        InputInt {
            label,
            value,
            step: 1,
            step_fast: 100,
            flags: ImGuiInputTextFlags::empty(),
            _phantom: PhantomData,
        }
    }

    pub fn build(self) -> bool {
        unsafe {
            sys::igInputInt(
                self.label.as_ptr(),
                self.value as *mut i32,
                self.step,
                self.step_fast,
                self.flags.bits(),
            )
        }
    }

    impl_step_params!(InputInt, i32);
    impl_text_flags!(InputInt);
}

#[must_use]
pub struct InputFloat<'ui, 'p> {
    label: &'p ImStr,
    value: &'p mut f32,
    step: f32,
    step_fast: f32,
    flags: ImGuiInputTextFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> InputFloat<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p ImStr, value: &'p mut f32) -> Self {
        InputFloat {
            label,
            value,
            step: 0.0,
            step_fast: 0.0,
            flags: ImGuiInputTextFlags::empty(),
            _phantom: PhantomData,
        }
    }

    pub fn build(self) -> bool {
        unsafe {
            sys::igInputFloat(
                self.label.as_ptr(),
                self.value as *mut f32,
                self.step,
                self.step_fast,
                b"%.3f\0".as_ptr() as *const _,
                self.flags.bits(),
            )
        }
    }

    impl_step_params!(InputFloat, f32);
    impl_text_flags!(InputFloat);
}

macro_rules! impl_input_floatn {
    ($InputFloatN:ident, $N:expr, $igInputFloatN:ident) => {
        #[must_use]
        pub struct $InputFloatN<'ui, 'p> {
            label: &'p ImStr,
            value: &'p mut [f32; $N],
            flags: ImGuiInputTextFlags,
            _phantom: PhantomData<&'ui Ui<'ui>>,
        }

        impl<'ui, 'p> $InputFloatN<'ui, 'p> {
            pub fn new(_: &Ui<'ui>, label: &'p ImStr, value: &'p mut [f32; $N]) -> Self {
                $InputFloatN {
                    label,
                    value,
                    flags: ImGuiInputTextFlags::empty(),
                    _phantom: PhantomData,
                }
            }

            pub fn build(self) -> bool {
                unsafe {
                    sys::$igInputFloatN(
                        self.label.as_ptr(),
                        self.value.as_mut_ptr(),
                        b"%.3f\0".as_ptr() as *const _,
                        self.flags.bits(),
                    )
                }
            }

            impl_text_flags!($InputFloatN);
        }
    };
}

impl_input_floatn!(InputFloat2, 2, igInputFloat2);
impl_input_floatn!(InputFloat3, 3, igInputFloat3);
impl_input_floatn!(InputFloat4, 4, igInputFloat4);

macro_rules! impl_input_intn {
    ($InputIntN:ident, $N:expr, $igInputIntN:ident) => {
        #[must_use]
        pub struct $InputIntN<'ui, 'p> {
            label: &'p ImStr,
            value: &'p mut [i32; $N],
            flags: ImGuiInputTextFlags,
            _phantom: PhantomData<&'ui Ui<'ui>>,
        }

        impl<'ui, 'p> $InputIntN<'ui, 'p> {
            pub fn new(_: &Ui<'ui>, label: &'p ImStr, value: &'p mut [i32; $N]) -> Self {
                $InputIntN {
                    label,
                    value,
                    flags: ImGuiInputTextFlags::empty(),
                    _phantom: PhantomData,
                }
            }

            pub fn build(self) -> bool {
                unsafe {
                    sys::$igInputIntN(
                        self.label.as_ptr(),
                        self.value.as_mut_ptr(),
                        self.flags.bits(),
                    )
                }
            }

            impl_text_flags!($InputIntN);
        }
    };
}

impl_input_intn!(InputInt2, 2, igInputInt2);
impl_input_intn!(InputInt3, 3, igInputInt3);
impl_input_intn!(InputInt4, 4, igInputInt4);
