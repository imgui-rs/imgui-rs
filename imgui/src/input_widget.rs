use bitflags::bitflags;
use std::ops::Range;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

use crate::internal::DataTypeKind;
use crate::math::*;
use crate::sys;
use crate::Ui;

bitflags!(
    /// Flags for text inputs
    #[repr(C)]
    pub struct InputTextFlags: u32 {
        /// Allow 0123456789.+-*/
        const CHARS_DECIMAL = sys::ImGuiInputTextFlags_CharsDecimal;
        /// Allow 0123456789ABCDEFabcdef
        const CHARS_HEXADECIMAL = sys::ImGuiInputTextFlags_CharsHexadecimal;
        /// Turn a..z into A..Z
        const CHARS_UPPERCASE = sys::ImGuiInputTextFlags_CharsUppercase;
        /// Filter out spaces, tabs
        const CHARS_NO_BLANK = sys::ImGuiInputTextFlags_CharsNoBlank;
        /// Select entire text when first taking mouse focus
        const AUTO_SELECT_ALL = sys::ImGuiInputTextFlags_AutoSelectAll;
        /// Return 'true' when Enter is pressed (as opposed to when the value was modified)
        const ENTER_RETURNS_TRUE = sys::ImGuiInputTextFlags_EnterReturnsTrue;
        /// Call user function on pressing TAB (for completion handling)
        const CALLBACK_COMPLETION = sys::ImGuiInputTextFlags_CallbackCompletion;
        /// Call user function on pressing Up/Down arrows (for history handling)
        const CALLBACK_HISTORY = sys::ImGuiInputTextFlags_CallbackHistory;
        /// Call user function on pressing Up/Down arrows (for history handling)
        const CALLBACK_EDIT = sys::ImGuiInputTextFlags_CallbackEdit;
        /// Call user function every time. User code may query cursor position, modify text buffer.
        const CALLBACK_ALWAYS = sys::ImGuiInputTextFlags_CallbackAlways;
        /// Call user function to filter character.
        const CALLBACK_CHAR_FILTER = sys::ImGuiInputTextFlags_CallbackCharFilter;
        /// Pressing TAB input a '\t' character into the text field
        const ALLOW_TAB_INPUT = sys::ImGuiInputTextFlags_AllowTabInput;
        /// In multi-line mode, unfocus with Enter, add new line with Ctrl+Enter (default is
        /// opposite: unfocus with Ctrl+Enter, add line with Enter).
        const CTRL_ENTER_FOR_NEW_LINE = sys::ImGuiInputTextFlags_CtrlEnterForNewLine;
        /// Disable following the cursor horizontally
        const NO_HORIZONTAL_SCROLL = sys::ImGuiInputTextFlags_NoHorizontalScroll;
        /// Always overwrite (aka "insert mode").
        const ALWAYS_OVERWRITE = sys::ImGuiInputTextFlags_AlwaysOverwrite;
        /// Read-only mode
        const READ_ONLY = sys::ImGuiInputTextFlags_ReadOnly;
        /// Password mode, display all characters as '*'
        const PASSWORD = sys::ImGuiInputTextFlags_Password;
        /// Disable undo/redo.
        const NO_UNDO_REDO = sys::ImGuiInputTextFlags_NoUndoRedo;
        /// Allow 0123456789.+-*/eE (Scientific notation input)
        const CHARS_SCIENTIFIC = sys::ImGuiInputTextFlags_CharsScientific;
        /// Allow buffer capacity resize + notify when the string wants to be resized
        const CALLBACK_RESIZE = sys::ImGuiInputTextFlags_CallbackResize;
    }
);

macro_rules! impl_text_flags {
    ($InputType:ident) => {
        #[inline]
        pub fn flags(mut self, flags: InputTextFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn chars_decimal(mut self, value: bool) -> Self {
            self.flags.set(InputTextFlags::CHARS_DECIMAL, value);
            self
        }

        #[inline]
        pub fn chars_hexadecimal(mut self, value: bool) -> Self {
            self.flags.set(InputTextFlags::CHARS_HEXADECIMAL, value);
            self
        }

        #[inline]
        pub fn chars_uppercase(mut self, value: bool) -> Self {
            self.flags.set(InputTextFlags::CHARS_UPPERCASE, value);
            self
        }

        #[inline]
        pub fn chars_noblank(mut self, value: bool) -> Self {
            self.flags.set(InputTextFlags::CHARS_NO_BLANK, value);
            self
        }

        #[inline]
        pub fn auto_select_all(mut self, value: bool) -> Self {
            self.flags.set(InputTextFlags::AUTO_SELECT_ALL, value);
            self
        }

        #[inline]
        pub fn enter_returns_true(mut self, value: bool) -> Self {
            self.flags.set(InputTextFlags::ENTER_RETURNS_TRUE, value);
            self
        }

        #[inline]
        pub fn allow_tab_input(mut self, value: bool) -> Self {
            self.flags.set(InputTextFlags::ALLOW_TAB_INPUT, value);
            self
        }

        #[inline]
        pub fn no_horizontal_scroll(mut self, value: bool) -> Self {
            self.flags.set(InputTextFlags::NO_HORIZONTAL_SCROLL, value);
            self
        }

        /// Note: this is equivalent to `always_overwrite`
        #[inline]
        pub fn always_insert_mode(self, value: bool) -> Self {
            self.always_overwrite(value)
        }

        #[inline]
        #[allow(deprecated)]
        pub fn always_overwrite(mut self, value: bool) -> Self {
            self.flags.set(InputTextFlags::ALWAYS_OVERWRITE, value);
            self
        }

        #[inline]
        pub fn read_only(mut self, value: bool) -> Self {
            self.flags.set(InputTextFlags::READ_ONLY, value);
            self
        }

        #[inline]
        pub fn password(mut self, value: bool) -> Self {
            self.flags.set(InputTextFlags::PASSWORD, value);
            self
        }

        #[inline]
        pub fn no_undo_redo(mut self, value: bool) -> Self {
            self.flags.set(InputTextFlags::NO_UNDO_REDO, value);
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

#[must_use]
pub struct InputText<'ui, 'p, L, H = &'static str, T = PassthroughCallback> {
    label: L,
    hint: Option<H>,
    buf: &'p mut String,
    callback_handler: T,
    flags: InputTextFlags,
    ui: &'ui Ui,
}

impl<'ui, 'p, L: AsRef<str>> InputText<'ui, 'p, L> {
    /// Creates a new input text widget to edit the given string.
    ///
    /// # String Editing
    ///
    /// Please note, ImGui requires this string to be null-terminated. We accomplish this
    /// by appending and then removing a null terminator (`\0`) from the String you pass in.
    /// This has several consequences:
    /// 1. The string's backing buffer may be resized and relocated even without edits as result
    /// of this pushed char.
    /// 2. **The string will appear truncated if the string contains `\0` inside it.** This will not
    /// cause memory *unsafety*, but it will limit your usage. If that's the case, please pre-process
    /// your string.
    /// 3. Truncations by ImGui appear to be done primarily by insertions of `\0` to the truncation point.
    /// We will handle this for you and edit the string "properly" too, but this might show up in callbacks.
    pub fn new(ui: &'ui Ui, label: L, buf: &'p mut String) -> Self {
        InputText {
            label,
            hint: None,
            // this is fine because no one else has access to this and imgui is single threaded.
            callback_handler: PassthroughCallback,
            buf,
            flags: InputTextFlags::CALLBACK_RESIZE,
            ui,
        }
    }
}

impl<'ui, 'p, T, L, H> InputText<'ui, 'p, L, H, T>
where
    L: AsRef<str>,
    H: AsRef<str>,
    T: InputTextCallbackHandler,
{
    /// Sets the hint displayed in the input text background.
    #[inline]
    pub fn hint<H2: AsRef<str>>(self, hint: H2) -> InputText<'ui, 'p, L, H2, T> {
        InputText {
            label: self.label,
            hint: Some(hint),
            buf: self.buf,
            callback_handler: self.callback_handler,
            flags: self.flags,
            ui: self.ui,
        }
    }

    impl_text_flags!(InputText);

    // I am commenting this ability out for now -- because we need to push `\0` for imgui,
    // we may resize the buffer no matter what, and we must do that.
    // The solution for this will be, I suspect, to build a second api channel that takes
    // an `&mut CStr`, which is ugly! I suspect few to none will want no-resizing, so I'm deferring
    // fixing the problem. -- sanbox-irl 09/15/2021, see #523 for more
    //
    // /// By default (as of 0.8.0), imgui-rs will automatically handle string resizes
    // /// for [InputText] and [InputTextMultiline].
    // ///
    // /// If, for some reason, you don't want this, you can run this function to prevent this.
    // /// In that case, edits which would cause a resize will not occur.
    // /// #[inline]
    // pub unsafe fn do_not_resize(mut self) -> Self {
    //     self.flags.remove(InputTextFlags::CALLBACK_RESIZE);
    //     self
    // }

    #[inline]
    pub fn callback<T2: InputTextCallbackHandler>(
        mut self,
        callbacks: InputTextCallback,
        callback: T2,
    ) -> InputText<'ui, 'p, L, H, T2> {
        if callbacks.contains(InputTextCallback::COMPLETION) {
            self.flags.insert(InputTextFlags::CALLBACK_COMPLETION);
        }
        if callbacks.contains(InputTextCallback::HISTORY) {
            self.flags.insert(InputTextFlags::CALLBACK_HISTORY);
        }
        if callbacks.contains(InputTextCallback::ALWAYS) {
            self.flags.insert(InputTextFlags::CALLBACK_ALWAYS);
        }
        if callbacks.contains(InputTextCallback::CHAR_FILTER) {
            self.flags.insert(InputTextFlags::CALLBACK_CHAR_FILTER);
        }
        if callbacks.contains(InputTextCallback::EDIT) {
            self.flags.insert(InputTextFlags::CALLBACK_EDIT);
        }
        InputText {
            callback_handler: callback,
            label: self.label,
            hint: self.hint,
            buf: self.buf,
            flags: self.flags,
            ui: self.ui,
        }
    }

    /// Builds the string editor, performing string editing operations.
    ///
    /// # String Editing
    ///
    /// Please note, ImGui requires this string to be null-terminated. We accomplish this
    /// by appending and then removing a null terminator (`\0`) from the String you pass in.
    /// This has several consequences:
    /// 1. The string's backing buffer may be resized and relocated even without edits as result
    /// of this pushed char.
    /// 2. **The string will appear truncated if the string contains `\0` inside it.** This will not
    /// cause memory *unsafety*, but it will limit your usage. If that's the case, please pre-process
    /// your string.
    /// 3. Truncations by ImGui appear to be done primarily by insertions of `\0` to the truncation point.
    /// We will handle this for you and edit the string "properly" too, but this might show up in callbacks.
    pub fn build(self) -> bool {
        // needs to be null-terminated! this is a hack!
        self.buf.push('\0');

        let (ptr, capacity) = (self.buf.as_mut_ptr(), self.buf.capacity());

        let mut data = UserData {
            container: self.buf,
            cback_handler: self.callback_handler,
        };
        let data = &mut data as *mut _ as *mut c_void;

        let o = unsafe {
            if let Some(hint) = self.hint {
                let (label, hint) = self.ui.scratch_txt_two(self.label, hint);
                sys::igInputTextWithHint(
                    label,
                    hint,
                    ptr as *mut sys::cty::c_char,
                    capacity,
                    self.flags.bits() as i32,
                    Some(callback::<T>),
                    data,
                )
            } else {
                let label = self.ui.scratch_txt(self.label);

                sys::igInputText(
                    label,
                    ptr as *mut sys::cty::c_char,
                    capacity,
                    self.flags.bits() as i32,
                    Some(callback::<T>),
                    data,
                )
            }
        };

        let cap = self.buf.capacity();

        // SAFETY: this slice is simply a view into the underlying buffer
        // of a String. We MAY be holding onto a view of uninitialized memory,
        // however, since we're holding this as a u8 slice, I think it should be
        // alright...
        // additionally, we can go over the bytes directly, rather than char indices,
        // because NUL will never appear in any UTF8 outside the NUL character (ie, within
        // a char).
        let buf = unsafe { std::slice::from_raw_parts(self.buf.as_ptr(), cap) };
        if let Some(len) = buf.iter().position(|x| *x == b'\0') {
            // `len` is the position of the first `\0` byte in the String
            unsafe {
                self.buf.as_mut_vec().set_len(len);
            }
        } else {
            // There is no null terminator, the best we can do is to not
            // update the string length.
        }

        o
    }
}

#[must_use]
pub struct InputTextMultiline<'ui, 'p, L, T = PassthroughCallback> {
    label: L,
    buf: &'p mut String,
    flags: InputTextFlags,
    size: [f32; 2],
    callback_handler: T,
    ui: &'ui Ui,
}

impl<'ui, 'p, L: AsRef<str>> InputTextMultiline<'ui, 'p, L, PassthroughCallback> {
    /// Creates a new input text widget to edit the given string.
    ///
    /// # String Editing
    ///
    /// Please note, ImGui requires this string to be null-terminated. We accomplish this
    /// by appending and then removing a null terminator (`\0`) from the String you pass in.
    /// This has several consequences:
    /// 1. The string's backing buffer may be resized and relocated even without edits as result
    /// of this pushed char.
    /// 2. **The string will appear truncated if the string contains `\0` inside it.** This will not
    /// cause memory *unsafety*, but it will limit your usage. If that's the case, please pre-process
    /// your string.
    /// 3. Truncations by ImGui appear to be done primarily by insertions of `\0` to the truncation point.
    /// We will handle this for you and edit the string "properly" too, but this might show up in callbacks.
    pub fn new(ui: &'ui Ui, label: L, buf: &'p mut String, size: impl Into<MintVec2>) -> Self {
        InputTextMultiline {
            label,
            buf,
            flags: InputTextFlags::CALLBACK_RESIZE,
            size: size.into().into(),
            callback_handler: PassthroughCallback,
            ui,
        }
    }
}

impl<'ui, 'p, T: InputTextCallbackHandler, L: AsRef<str>> InputTextMultiline<'ui, 'p, L, T> {
    impl_text_flags!(InputText);

    // I am commenting this ability out for now -- because we need to push `\0` for imgui,
    // we may resize the buffer no matter what, and we must do that.
    // The solution for this will be, I suspect, to build a second api channel that takes
    // an `&mut CStr`, which is ugly! I suspect few to none will want no-resizing, so I'm deferring
    // fixing the problem. -- sanbox-irl 09/15/2021, see #523 for more
    // /// By default (as of 0.8.0), imgui-rs will automatically handle string resizes
    // /// for [InputText] and [InputTextMultiline].
    // ///
    // /// If, for some reason, you don't want this, you can run this function to prevent this.
    // /// In that case, edits which would cause a resize will not occur.
    // #[inline]
    // pub fn do_not_resize(mut self) -> Self {
    //     self.flags.remove(InputTextFlags::CALLBACK_RESIZE);
    //     self
    // }

    #[inline]
    pub fn callback<T2: InputTextCallbackHandler>(
        mut self,
        callbacks: InputTextMultilineCallback,
        callback_handler: T2,
    ) -> InputTextMultiline<'ui, 'p, L, T2> {
        if callbacks.contains(InputTextMultilineCallback::COMPLETION) {
            self.flags.insert(InputTextFlags::CALLBACK_COMPLETION);
        }
        if callbacks.contains(InputTextMultilineCallback::ALWAYS) {
            self.flags.insert(InputTextFlags::CALLBACK_ALWAYS);
        }
        if callbacks.contains(InputTextMultilineCallback::CHAR_FILTER) {
            self.flags.insert(InputTextFlags::CALLBACK_CHAR_FILTER);
        }
        if callbacks.contains(InputTextMultilineCallback::EDIT) {
            self.flags.insert(InputTextFlags::CALLBACK_EDIT);
        }

        InputTextMultiline {
            label: self.label,
            buf: self.buf,
            flags: self.flags,
            size: self.size,
            callback_handler,
            ui: self.ui,
        }
    }

    /// Builds the string editor, performing string editing operations.
    ///
    /// # String Editing
    ///
    /// Please note, ImGui requires this string to be null-terminated. We accomplish this
    /// by appending and then removing a null terminator (`\0`) from the String you pass in.
    /// This has several consequences:
    /// 1. The string's backing buffer may be resized and relocated even without edits as result
    /// of this pushed char.
    /// 2. **The string will appear truncated if the string contains `\0` inside it.** This will not
    /// cause memory *unsafety*, but it will limit your usage. If that's the case, please pre-process
    /// your string.
    /// 3. Truncations by ImGui appear to be done primarily by insertions of `\0` to the truncation point.
    /// We will handle this for you and edit the string "properly" too, but this might show up in callbacks.
    pub fn build(self) -> bool {
        // needs to be null-terminated! this is a hack!
        self.buf.push('\0');
        let (ptr, capacity) = (self.buf.as_mut_ptr(), self.buf.capacity());

        let mut data = UserData {
            container: self.buf,
            cback_handler: self.callback_handler,
        };
        let data = &mut data as *mut _ as *mut c_void;

        let o = unsafe {
            sys::igInputTextMultiline(
                self.ui.scratch_txt(self.label),
                ptr as *mut sys::cty::c_char,
                capacity,
                self.size.into(),
                self.flags.bits() as i32,
                Some(callback::<T>),
                data,
            )
        };

        let cap = self.buf.capacity();

        // SAFETY: this slice is simply a view into the underlying buffer
        // of a String. We MAY be holding onto a view of uninitialized memory,
        // however, since we're holding this as a u8 slice, I think it should be
        // alright...
        // additionally, we can go over the bytes directly, rather than char indices,
        // because NUL will never appear in any UTF8 outside the NUL character (ie, within
        // a char).
        let buf = unsafe { std::slice::from_raw_parts(self.buf.as_ptr(), cap) };
        if let Some(len) = buf.iter().position(|x| *x == b'\0') {
            // `len` is the position of the first `\0` byte in the String
            unsafe {
                self.buf.as_mut_vec().set_len(len);
            }
        } else {
            // There is no null terminator, the best we can do is to not
            // update the string length.
        }

        o
    }
}

#[must_use]
pub struct InputInt<'ui, 'p, L> {
    label: L,
    value: &'p mut i32,
    step: i32,
    step_fast: i32,
    flags: InputTextFlags,
    ui: &'ui Ui,
}

impl<'ui, 'p, L: AsRef<str>> InputInt<'ui, 'p, L> {
    #[deprecated(
        since = "0.9.0",
        note = "use `ui.input_int` or `ui.input_scalar` instead"
    )]
    pub fn new(ui: &'ui Ui, label: L, value: &'p mut i32) -> Self {
        InputInt {
            label,
            value,
            step: 1,
            step_fast: 100,
            flags: InputTextFlags::empty(),
            ui,
        }
    }

    pub fn build(self) -> bool {
        unsafe {
            sys::igInputInt(
                self.ui.scratch_txt(self.label),
                self.value as *mut i32,
                self.step,
                self.step_fast,
                self.flags.bits() as i32,
            )
        }
    }

    impl_step_params!(InputInt, i32);
    impl_text_flags!(InputInt);
}

#[must_use]
pub struct InputFloat<'ui, 'p, L, F = &'static str> {
    label: L,
    value: &'p mut f32,
    step: f32,
    step_fast: f32,
    display_format: Option<F>,
    flags: InputTextFlags,
    ui: &'ui Ui,
}

impl<'ui, 'p, L: AsRef<str>> InputFloat<'ui, 'p, L> {
    #[deprecated(
        since = "0.9.0",
        note = "use `ui.input_float` or `ui.input_scalar` instead"
    )]
    pub fn new(ui: &'ui Ui, label: L, value: &'p mut f32) -> Self {
        InputFloat {
            label,
            value,
            step: 0.0,
            step_fast: 0.0,
            display_format: None,
            flags: InputTextFlags::empty(),
            ui,
        }
    }

    pub fn display_format<F2: AsRef<str>>(self, display_format: F2) -> InputFloat<'ui, 'p, L, F2> {
        InputFloat {
            value: self.value,
            label: self.label,
            step: self.step,
            step_fast: self.step_fast,
            display_format: Some(display_format),
            flags: self.flags,
            ui: self.ui,
        }
    }

    pub fn build(self) -> bool {
        let (one, two) = self
            .ui
            .scratch_txt_with_opt(self.label, self.display_format);

        unsafe {
            sys::igInputFloat(
                one,
                self.value as *mut f32,
                self.step,
                self.step_fast,
                two,
                self.flags.bits() as i32,
            )
        }
    }

    impl_step_params!(InputFloat, f32);
    impl_text_flags!(InputFloat);
}

macro_rules! impl_input_floatn {
    ($InputFloatN:ident, $MINT_TARGET:ty, $N:expr, $igInputFloatN:ident) => {
        #[must_use]
        pub struct $InputFloatN<'ui, 'p, L, T, F = &'static str> {
            label: L,
            value: &'p mut T,
            display_format: Option<F>,
            flags: InputTextFlags,
            ui: &'ui Ui,
        }

        impl<'ui, 'p, L, T> $InputFloatN<'ui, 'p, L, T>
        where
            L: AsRef<str>,
            T: Copy + Into<$MINT_TARGET>,
            $MINT_TARGET: Into<T> + Into<[f32; $N]>,
        {
            pub fn new(ui: &'ui Ui, label: L, value: &'p mut T) -> Self {
                $InputFloatN {
                    label,
                    value,
                    display_format: None,
                    flags: InputTextFlags::empty(),
                    ui,
                }
            }

            pub fn display_format<F2: AsRef<str>>(
                self,
                display_format: F2,
            ) -> $InputFloatN<'ui, 'p, L, T, F2> {
                $InputFloatN {
                    label: self.label,
                    value: self.value,
                    display_format: Some(display_format),
                    flags: self.flags,
                    ui: self.ui,
                }
            }

            pub fn build(self) -> bool {
                let value: $MINT_TARGET = (*self.value).into();
                let mut value: [f32; $N] = value.into();

                let (one, two) = self
                    .ui
                    .scratch_txt_with_opt(self.label, self.display_format);

                let changed = unsafe {
                    sys::$igInputFloatN(one, value.as_mut_ptr(), two, self.flags.bits() as i32)
                };

                if changed {
                    let value: $MINT_TARGET = value.into();
                    *self.value = value.into();
                }

                changed
            }

            impl_text_flags!($InputFloatN);
        }
    };
}

impl_input_floatn!(InputFloat2, MintVec2, 2, igInputFloat2);
impl_input_floatn!(InputFloat3, MintVec3, 3, igInputFloat3);
impl_input_floatn!(InputFloat4, MintVec4, 4, igInputFloat4);

macro_rules! impl_input_intn {
    ($InputIntN:ident, $MINT_TARGET:ident, $N:expr, $igInputIntN:ident) => {
        #[must_use]
        pub struct $InputIntN<'ui, 'p, L, T> {
            label: L,
            value: &'p mut T,
            flags: InputTextFlags,
            ui: &'ui Ui,
        }

        impl<'ui, 'p, L, T> $InputIntN<'ui, 'p, L, T>
        where
            L: AsRef<str>,
            T: Copy + Into<$MINT_TARGET>,
            $MINT_TARGET: Into<T> + Into<[i32; $N]>,
        {
            pub fn new(ui: &'ui Ui, label: L, value: &'p mut T) -> Self {
                $InputIntN {
                    label,
                    value,
                    flags: InputTextFlags::empty(),
                    ui,
                }
            }

            pub fn build(self) -> bool {
                let value: $MINT_TARGET = (*self.value).into();
                let mut value: [i32; $N] = value.into();

                let changed = unsafe {
                    sys::$igInputIntN(
                        self.ui.scratch_txt(self.label),
                        value.as_mut_ptr(),
                        self.flags.bits() as i32,
                    )
                };

                if changed {
                    let value: $MINT_TARGET = value.into();
                    *self.value = value.into();
                }

                changed
            }

            impl_text_flags!($InputIntN);
        }
    };
}

impl_input_intn!(InputInt2, MintIVec2, 2, igInputInt2);
impl_input_intn!(InputInt3, MintIVec3, 3, igInputInt3);
impl_input_intn!(InputInt4, MintIVec4, 4, igInputInt4);

/// Builder for an input scalar widget.
#[must_use]
pub struct InputScalar<'ui, 'p, T, L, F = &'static str> {
    value: &'p mut T,
    label: L,
    step: Option<T>,
    step_fast: Option<T>,
    display_format: Option<F>,
    flags: InputTextFlags,
    ui: &'ui Ui,
}

impl<'ui, 'p, L: AsRef<str>, T: DataTypeKind> InputScalar<'ui, 'p, T, L> {
    /// Constructs a new input scalar builder.
    #[doc(alias = "InputScalar", alias = "InputScalarN")]
    pub fn new(ui: &'ui Ui, label: L, value: &'p mut T) -> Self {
        InputScalar {
            value,
            label,
            step: None,
            step_fast: None,
            display_format: None,
            flags: InputTextFlags::empty(),
            ui,
        }
    }
}

impl<'ui, 'p, L: AsRef<str>, T: DataTypeKind, F: AsRef<str>> InputScalar<'ui, 'p, T, L, F> {
    /// Sets the display format using *a C-style printf string*
    pub fn display_format<F2: AsRef<str>>(
        self,
        display_format: F2,
    ) -> InputScalar<'ui, 'p, T, L, F2> {
        InputScalar {
            value: self.value,
            label: self.label,
            step: self.step,
            step_fast: self.step_fast,
            display_format: Some(display_format),
            flags: self.flags,
            ui: self.ui,
        }
    }
    /// Builds an input scalar that is bound to the given value.
    ///
    /// Returns true if the value was changed.
    pub fn build(self) -> bool {
        unsafe {
            let (one, two) = self
                .ui
                .scratch_txt_with_opt(self.label, self.display_format);

            sys::igInputScalar(
                one,
                T::KIND as i32,
                self.value as *mut T as *mut c_void,
                self.step
                    .as_ref()
                    .map(|step| step as *const T)
                    .unwrap_or(ptr::null()) as *const c_void,
                self.step_fast
                    .as_ref()
                    .map(|step| step as *const T)
                    .unwrap_or(ptr::null()) as *const c_void,
                two,
                self.flags.bits() as i32,
            )
        }
    }

    #[inline]
    pub fn step(mut self, value: T) -> Self {
        self.step = Some(value);
        self
    }

    #[inline]
    pub fn step_fast(mut self, value: T) -> Self {
        self.step_fast = Some(value);
        self
    }

    impl_text_flags!(InputScalar);
}

/// Builder for an input scalar widget.
#[must_use]
pub struct InputScalarN<'ui, 'p, T, L, F = &'static str> {
    values: &'p mut [T],
    label: L,
    step: Option<T>,
    step_fast: Option<T>,
    display_format: Option<F>,
    flags: InputTextFlags,
    ui: &'ui Ui,
}

impl<'ui, 'p, L: AsRef<str>, T: DataTypeKind> InputScalarN<'ui, 'p, T, L> {
    /// Constructs a new input scalar builder.
    #[doc(alias = "InputScalarN")]
    pub fn new(ui: &'ui Ui, label: L, values: &'p mut [T]) -> Self {
        InputScalarN {
            values,
            label,
            step: None,
            step_fast: None,
            display_format: None,
            flags: InputTextFlags::empty(),
            ui,
        }
    }
}

impl<'ui, 'p, L: AsRef<str>, T: DataTypeKind, F: AsRef<str>> InputScalarN<'ui, 'p, T, L, F> {
    /// Sets the display format using *a C-style printf string*
    pub fn display_format<F2: AsRef<str>>(
        self,
        display_format: F2,
    ) -> InputScalarN<'ui, 'p, T, L, F2> {
        InputScalarN {
            values: self.values,
            label: self.label,
            step: self.step,
            step_fast: self.step_fast,
            display_format: Some(display_format),
            flags: self.flags,
            ui: self.ui,
        }
    }
    /// Builds a horizontal array of multiple input scalars attached to the given slice.
    ///
    /// Returns true if any value was changed.
    pub fn build(self) -> bool {
        unsafe {
            let (one, two) = self
                .ui
                .scratch_txt_with_opt(self.label, self.display_format);

            sys::igInputScalarN(
                one,
                T::KIND as i32,
                self.values.as_mut_ptr() as *mut c_void,
                self.values.len() as i32,
                self.step
                    .as_ref()
                    .map(|step| step as *const T)
                    .unwrap_or(ptr::null()) as *const c_void,
                self.step_fast
                    .as_ref()
                    .map(|step| step as *const T)
                    .unwrap_or(ptr::null()) as *const c_void,
                two,
                self.flags.bits() as i32,
            )
        }
    }

    #[inline]
    pub fn step(mut self, value: T) -> Self {
        self.step = Some(value);
        self
    }

    #[inline]
    pub fn step_fast(mut self, value: T) -> Self {
        self.step_fast = Some(value);
        self
    }

    impl_text_flags!(InputScalar);
}

bitflags!(
    /// Callback flags for an `InputText` widget. These correspond to
    /// the general textflags.
    pub struct InputTextCallback: u32 {
        /// Call user function on pressing TAB (for completion handling)
        const COMPLETION = sys::ImGuiInputTextFlags_CallbackCompletion;
        /// Call user function on pressing Up/Down arrows (for history handling)
        const HISTORY = sys::ImGuiInputTextFlags_CallbackHistory;
        /// Call user function every time. User code may query cursor position, modify text buffer.
        const ALWAYS = sys::ImGuiInputTextFlags_CallbackAlways;
        /// Call user function to filter character.
        const CHAR_FILTER = sys::ImGuiInputTextFlags_CallbackCharFilter;
        /// Callback on buffer edit (note that InputText already returns true on edit, the
        /// callback is useful mainly to manipulate the underlying buffer while focus is active)
        const EDIT = sys::ImGuiInputTextFlags_CallbackEdit;
    }
);

bitflags!(
    /// Callback flags for an `InputTextMultiline` widget. These correspond to the
    /// general textflags.
    pub struct InputTextMultilineCallback: u32 {
        /// Call user function on pressing TAB (for completion handling)
        const COMPLETION = sys::ImGuiInputTextFlags_CallbackCompletion;
        /// Call user function every time. User code may query cursor position, modify text buffer.
        const ALWAYS = sys::ImGuiInputTextFlags_CallbackAlways;
        /// Call user function to filter character.
        const CHAR_FILTER = sys::ImGuiInputTextFlags_CallbackCharFilter;
        /// Callback on buffer edit (note that InputText already returns true on edit, the
        /// callback is useful mainly to manipulate the underlying buffer while focus is active)
        const EDIT = sys::ImGuiInputTextFlags_CallbackEdit;
    }
);

/// This trait provides an interface which ImGui will call on `InputText`
/// and `InputTextMultiline` callbacks.
///
/// Each method is called *if and only if* the corresponding flag for each
/// method is passed to ImGui in the `callback` builder.
///
/// Each method here lists the flag required to call it, and this module begins
/// with an example of callbacks being used.
pub trait InputTextCallbackHandler {
    /// Filters a char -- returning a `None` means that the char is removed,
    /// and returning another char substitutes it out.
    ///
    /// Because of upstream ImGui choices, you do not have access to the buffer
    /// during this callback (for some reason).
    ///
    /// To make ImGui run this callback, use [InputTextCallback::CHAR_FILTER] or
    /// [InputTextMultilineCallback::CHAR_FILTER].
    fn char_filter(&mut self, c: char) -> Option<char> {
        Some(c)
    }

    /// Allows one to perform autocompletion work when the Tab key has been pressed.
    ///
    /// To make ImGui run this callback, use [InputTextCallback::COMPLETION] or
    /// [InputTextMultilineCallback::COMPLETION].
    fn on_completion(&mut self, _: TextCallbackData) {}

    /// Allows one to edit the inner buffer whenever the buffer has been changed.
    ///    
    /// To make ImGui run this callback, use [InputTextCallback::EDIT] or
    /// [InputTextMultilineCallback::EDIT].
    fn on_edit(&mut self, _: TextCallbackData) {}

    /// A callback when one of the direction keys have been pressed.
    ///
    /// To make ImGui run this callback, use [InputTextCallback::HISTORY]. It appears
    /// that this callback will not be ran in a multiline input widget at all.
    fn on_history(&mut self, _: HistoryDirection, _: TextCallbackData) {}

    /// A callback which will always fire, each tick.
    ///
    /// To make ImGui run this callback, use [InputTextCallback::ALWAYS] or
    /// [InputTextMultilineCallback::ALWAYS].
    fn on_always(&mut self, _: TextCallbackData) {}
}

/// The arrow key a user pressed to trigger the `on_history` callback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HistoryDirection {
    Up,
    Down,
}

/// This struct provides methods to edit the underlying text buffer that
/// Dear ImGui manipulates. Primarily, it gives [remove_chars](Self::remove_chars),
/// [insert_chars](Self::insert_chars), and mutable access to what text is selected.
pub struct TextCallbackData(*mut sys::ImGuiInputTextCallbackData);

impl TextCallbackData {
    /// Creates the buffer.
    unsafe fn new(data: *mut sys::ImGuiInputTextCallbackData) -> Self {
        Self(data)
    }

    /// Get a reference to the text callback buffer's str.
    pub fn str(&self) -> &str {
        unsafe {
            std::str::from_utf8(std::slice::from_raw_parts(
                (*(self.0)).Buf as *const _,
                (*(self.0)).BufTextLen as usize,
            ))
            .expect("internal imgui error -- it boofed a utf8")
        }
    }

    /// Gives access to the underlying byte array MUTABLY.
    ///
    /// ## Safety
    ///
    /// This is very unsafe, and the following invariants must be
    /// upheld:
    /// 1. Keep the data utf8 valid.
    /// 2. After editing the string, call [set_dirty].
    ///
    /// To truncate the string, please use [remove_chars]. To extend
    /// the string, please use [insert_chars] and [push_str].
    ///
    /// This function should have highly limited usage, but could be for
    /// editing certain characters in the buffer based on some external condition.
    ///
    /// [remove_chars]: Self::remove_chars
    /// [set_dirty]: Self::set_dirty
    /// [insert_chars]: Self::insert_chars
    /// [push_str]: Self::push_str
    pub unsafe fn str_as_bytes_mut(&mut self) -> &mut [u8] {
        let str = std::str::from_utf8_mut(std::slice::from_raw_parts_mut(
            (*(self.0)).Buf as *const _ as *mut _,
            (*(self.0)).BufTextLen as usize,
        ))
        .expect("internal imgui error -- it boofed a utf8");

        str.as_bytes_mut()
    }

    /// Sets the dirty flag on the text to imgui, indicating that
    /// it should reapply this string to its internal state.
    ///
    /// **NB:** You only need to use this method if you're using `[str_as_bytes_mut]`.
    /// If you use the helper methods [remove_chars] and [insert_chars],
    /// this will be set for you. However, this is no downside to setting
    /// the dirty flag spuriously except the minor CPU time imgui will spend.
    ///
    /// [str_as_bytes_mut]: Self::str_as_bytes_mut
    /// [remove_chars]: Self::remove_chars
    /// [insert_chars]: Self::insert_chars
    pub fn set_dirty(&mut self) {
        unsafe {
            (*(self.0)).BufDirty = true;
        }
    }

    /// Gets a range of the selected text. See [selection_start_mut](Self::selection_start_mut)
    /// and [selection_end_mut](Self::selection_end_mut) to mutably edit these values.
    ///
    /// This Range is given in `usize` so that it might be used in indexing
    /// operations more easily. To quickly grab the selected text, use [selected](Self::selected).
    pub fn selection(&self) -> Range<usize> {
        unsafe { (*(self.0)).SelectionStart as usize..(*(self.0)).SelectionEnd as usize }
    }

    /// Returns the selected text directly. Note that if no text is selected,
    /// an empty str slice will be returned.
    pub fn selected(&self) -> &str {
        &self.str()[self.selection()]
    }

    /// Sets the cursor to select all.
    pub fn select_all(&mut self) {
        unsafe {
            sys::ImGuiInputTextCallbackData_SelectAll(self.0);
        }
    }

    /// Clears the selection.
    pub fn clear_selection(&mut self) {
        unsafe {
            sys::ImGuiInputTextCallbackData_ClearSelection(self.0);
        }
    }

    /// Checks if there is a selection within the text.
    pub fn has_selection(&self) -> bool {
        !self.selection().is_empty()
    }

    /// Pushes the given str to the end of this buffer. If this
    /// would require the String to resize, it will be resized.
    /// This is automatically handled.
    pub fn push_str(&mut self, s: &str) {
        // this is safe because the ench of a self.str is a char_boundary.
        unsafe {
            self.insert_chars_unsafe((*self.0).BufTextLen as usize, s);
        }
    }

    /// Inserts the given string at the given position. If this
    /// would require the String to resize, it will be resized
    /// automatically.
    ///
    /// ## Panics
    /// Panics if the `pos` is not a char_boundary.
    pub fn insert_chars(&mut self, pos: usize, s: &str) {
        assert!(self.str().is_char_boundary(pos));
        unsafe {
            self.insert_chars_unsafe(pos, s);
        }
    }

    /// Inserts the given string at the given position, unsafely. If this
    /// would require the String to resize, it will be resized automatically.
    ///
    /// ## Safety
    ///
    /// It is up to the caller to confirm that the `pos` is a valid byte
    /// position, or use [insert_chars](Self::insert_chars) which will panic
    /// if it isn't.
    pub unsafe fn insert_chars_unsafe(&mut self, pos: usize, s: &str) {
        let start = s.as_ptr();
        let end = start.add(s.len());

        sys::ImGuiInputTextCallbackData_InsertChars(
            self.0,
            pos as i32,
            start as *const c_char,
            end as *const c_char,
        );
    }

    /// Clears the string to an empty buffer.
    pub fn clear(&mut self) {
        unsafe {
            self.remove_chars_unchecked(0, (*self.0).BufTextLen as usize);
        }
    }

    /// Removes the given number of characters from the string starting
    /// at some byte pos.
    ///
    /// ## Panics
    /// Panics if the `pos` is not a char boundary.
    pub fn remove_chars(&mut self, pos: usize, char_count: usize) {
        let inner = &self.str()[pos..];
        let byte_count = inner
            .char_indices()
            .nth(char_count)
            .map(|v| v.0)
            .unwrap_or_else(|| inner.len());

        unsafe {
            self.remove_chars_unchecked(pos, byte_count);
        }
    }

    /// Removes the given number of bytes from the string starting
    /// at some byte pos, without checking for utf8 validity. Use
    /// [remove_chars](Self::remove_chars) for a safe variant.
    ///
    /// ## Safety
    ///
    /// It is up to the caller to ensure that the position is at a valid utf8 char_boundary
    /// and that there are enough bytes within the string remaining.
    pub unsafe fn remove_chars_unchecked(&mut self, pos: usize, byte_count: usize) {
        sys::ImGuiInputTextCallbackData_DeleteChars(self.0, pos as i32, byte_count as i32);
    }

    /// Get a reference to the text callback buffer's cursor pos.
    pub fn cursor_pos(&self) -> usize {
        unsafe { (*self.0).CursorPos as usize }
    }

    /// Set the text callback buffer's cursor pos.
    pub fn set_cursor_pos(&mut self, cursor_pos: usize) {
        unsafe {
            (*self.0).CursorPos = cursor_pos as i32;
        }
    }

    /// Get a mutable reference to the text callback buffer's selection start.
    pub fn selection_start_mut(&mut self) -> &mut i32 {
        unsafe { &mut (*self.0).SelectionStart }
    }

    /// Get a mutable reference to the text callback buffer's selection end..
    pub fn selection_end_mut(&mut self) -> &mut i32 {
        unsafe { &mut (*self.0).SelectionEnd }
    }
}

#[repr(C)]
struct UserData<T> {
    container: *mut String,
    cback_handler: T,
}

/// This is our default callback.
extern "C" fn callback<T: InputTextCallbackHandler>(
    data: *mut sys::ImGuiInputTextCallbackData,
) -> c_int {
    struct CallbackData<'a, T> {
        event_flag: InputTextFlags,
        user_data: &'a mut UserData<T>,
    }

    let callback_data = unsafe {
        CallbackData {
            event_flag: InputTextFlags::from_bits((*data).EventFlag as u32).unwrap(),
            user_data: &mut *((*data).UserData as *mut UserData<T>),
        }
    };

    // check this callback.
    match callback_data.event_flag {
        InputTextFlags::CALLBACK_ALWAYS => {
            let text_info = unsafe { TextCallbackData::new(&mut *data) };
            callback_data.user_data.cback_handler.on_always(text_info);
        }
        InputTextFlags::CALLBACK_EDIT => {
            let text_info = unsafe { TextCallbackData::new(&mut *data) };

            callback_data.user_data.cback_handler.on_edit(text_info);
        }
        InputTextFlags::CALLBACK_COMPLETION => {
            let text_info = unsafe { TextCallbackData::new(&mut *data) };
            callback_data
                .user_data
                .cback_handler
                .on_completion(text_info);
        }
        InputTextFlags::CALLBACK_RESIZE => {
            unsafe {
                let requested_size = (*data).BufSize as usize;
                let buffer = &mut *callback_data.user_data.container;

                // just confirm that we ARE working with our string.
                debug_assert_eq!(buffer.as_ptr() as *const _, (*data).Buf);

                if requested_size > buffer.capacity() {
                    let additional_bytes = requested_size - buffer.len();

                    // reserve more data...
                    buffer.reserve(additional_bytes);

                    (*data).Buf = buffer.as_mut_ptr() as *mut _;
                    (*data).BufDirty = true;
                }
            }
        }
        InputTextFlags::CALLBACK_CHAR_FILTER => {
            let chr = unsafe { std::char::from_u32((*data).EventChar).unwrap() };
            let new_data = match callback_data.user_data.cback_handler.char_filter(chr) {
                Some(value) => u32::from(value),
                // 0 means "do not use this char" in imgui docs
                None => 0,
            };
            // set the new char...
            unsafe {
                (*data).EventChar = new_data;
            }
        }
        InputTextFlags::CALLBACK_HISTORY => {
            let key = unsafe {
                let key = (*data).EventKey as u32;
                match key {
                    sys::ImGuiKey_UpArrow => HistoryDirection::Up,
                    sys::ImGuiKey_DownArrow => HistoryDirection::Down,
                    _ => panic!("Unexpected key"),
                }
            };
            let text_info = unsafe { TextCallbackData::new(&mut *data) };

            callback_data
                .user_data
                .cback_handler
                .on_history(key, text_info);
        }

        _ => {}
    }

    0
}

/// This is a Zst which implements TextCallbackHandler as a passthrough.
///
/// If you do not set a callback handler, this will be used (but will never
/// actually run, since you will not have pass imgui any flags).
pub struct PassthroughCallback;
impl InputTextCallbackHandler for PassthroughCallback {}
