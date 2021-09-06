use bitflags::bitflags;
use std::marker::PhantomData;
use std::ops::Range;
use std::os::raw::{c_char, c_int, c_void};

use crate::sys;
use crate::{ImStr, ImString, Ui};

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

pub trait TextCallbackHandler {
    /// Filters a char -- returning a `None` means that the char is removed,
    /// and returning another char substitutes it out.
    ///
    /// Because of upstream ImGui choices, you do not have access to the buffer
    /// during this callback (for some reason).
    fn char_filter(&mut self, c: char) -> Option<char> {
        Some(c)
    }

    /// Allows one to perform autocompletion work when the Tab key has been pressed.
    fn on_completion(&mut self, _: TextCallbackBuffer<'_>) {}

    /// Allows one to edit the inner buffer whenever the buffer has been changed.
    fn on_edit(&mut self, _: TextCallbackBuffer<'_>) {}

    /// A callback when one of the direction keys have been pressed.
    fn on_history(&mut self, _: EventDirection, _: TextCallbackBuffer<'_>) {}

    /// A callback which will always fire, each tick.
    fn on_always(&mut self, _: TextCallbackBuffer<'_>) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventDirection {
    Up,
    Down,
}

pub struct TextCallbackBuffer<'a> {
    buf: &'a mut str,
    dirty: &'a mut bool,
    cursor_pos: &'a mut i32,
    selection_start: &'a mut i32,
    selection_end: &'a mut i32,
    callback_data: *mut sys::ImGuiInputTextCallbackData,
}

impl TextCallbackBuffer<'_> {
    /// Get a reference to the text callback buffer's buf.
    pub fn buf(&self) -> &str {
        self.buf
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
    /// We present this string with a `str` interface immutably, which
    /// actually somewhat weakens `trunc` operations on the string.
    /// You can use [remove_chars] to handle this operation completely
    /// safely for you. However, if this is still too limiting,
    /// please submit an issue.
    pub unsafe fn buf_mut(&mut self) -> &mut [u8] {
        self.buf.as_bytes_mut()
    }

    /// Sets the dirty flag on the text to imgui, indicating that
    /// it should reapply this string to its internal state.
    ///
    /// **NB:** You only need to use this method if you're using `[buf_mut]`.
    /// If you use the helper methods [remove_chars] and [insert_chars],
    /// this will be set for you. However, this is no downside to setting
    /// the dirty flag spuriously except the minor CPU time imgui will spend.
    pub fn set_dirty(&mut self) {
        *self.dirty = true;
    }

    /// Gets a range of the selected text. See [selection_start_mut] and
    /// [selection_end_mut] to mutably edit these values.
    ///
    /// This Range is given in `usize` so that it might be used in indexing
    /// operations more easily. To quickly grab the selected text, use [selected].
    pub fn selection(&self) -> Range<usize> {
        *self.selection_start as usize..*self.selection_end as usize
    }

    /// Returns the selected text directly. Note that if no text is selected,
    /// an empty str slice will be returned.
    pub fn selected(&self) -> &str {
        &self.buf[self.selection()]
    }

    /// Sets the cursor to select all. This is always a valid operation,
    /// and so it takes an `&self`.
    pub fn select_all(&self) {
        unsafe {
            sys::ImGuiInputTextCallbackData_SelectAll(self.callback_data);
        }
    }

    /// Clears the selection. This is always a valid operation,
    /// and so it takes an `&self`.
    pub fn clear_selection(&self) {
        unsafe {
            sys::ImGuiInputTextCallbackData_ClearSelection(self.callback_data);
        }
    }

    /// Checks if there is a selection within the text.
    pub fn has_selection(&self) -> bool {
        unsafe { sys::ImGuiInputTextCallbackData_HasSelection(self.callback_data) }
    }

    /// Inserts the given string at the given position. If this
    /// would require the String to resize, it will be resized by calling the
    /// `CALLBACK_RESIZE` callback. This is automatically handled.
    ///
    /// ## Panics
    /// Panics if the `pos` is not a char_boundary.
    pub fn insert_chars(&mut self, pos: usize, s: &str) {
        assert!(self.buf.is_char_boundary(pos));
        unsafe {
            self.insert_chars_unsafe(pos, s);
        }
    }

    /// Inserts the given string at the given position, unsafely. If this
    /// would require the String to resize, it will be resized by calling the
    /// Callback_Resize callback. This is automatically handled.
    ///
    /// ## Safety
    ///
    /// It is up to the caller to confirm that the `pos` is a valid byte
    /// position, or use [insert_chars] which will panic if it isn't.
    pub unsafe fn insert_chars_unsafe(&mut self, pos: usize, s: &str) {
        let start = s.as_ptr();
        let end = start.add(s.len());

        sys::ImGuiInputTextCallbackData_InsertChars(
            self.callback_data,
            pos as i32,
            start as *const c_char,
            end as *const c_char,
        )
    }

    /// Clears the string.
    pub fn clear(&mut self) {
        unsafe {
            self.remove_chars_unchecked(0, self.buf().len());
        }
    }

    /// Removes the given number of characters from the string starting
    /// at some byte pos.
    ///
    /// ## Panics
    /// Panics if the `pos` is not a char boundary or if
    /// there are not enough chars remaining.
    pub fn remove_chars(&mut self, pos: usize, char_count: usize) {
        let inner = &self.buf[pos..];
        let byte_count = inner
            .char_indices()
            .nth(char_count)
            .expect("not enough characters in string")
            .0;

        unsafe {
            self.remove_chars_unchecked(pos, byte_count);
        }
    }

    /// Removes the given number of bytes from the string starting
    /// at some byte pos, without checking for utf8 validity. Use [remove_chars]
    /// for a safe variant.
    ///
    /// ## Safety
    ///
    /// It is up to the caller to ensure that the position is at a valid utf8 char_boundary
    /// and that there are enough bytes within the string remaining.
    pub unsafe fn remove_chars_unchecked(&mut self, pos: usize, byte_count: usize) {
        sys::ImGuiInputTextCallbackData_DeleteChars(
            self.callback_data,
            pos as i32,
            byte_count as i32,
        )
    }

    /// Get a reference to the text callback buffer's cursor pos.
    pub fn cursor_pos(&self) -> usize {
        *self.cursor_pos as usize
    }

    /// Set the text callback buffer's cursor pos.
    pub fn set_cursor_pos(&mut self, cursor_pos: usize) {
        *self.cursor_pos = cursor_pos as i32;
    }

    /// Get a mutable reference to the text callback buffer's selection start.
    pub fn selection_start_mut(&mut self) -> &mut i32 {
        self.selection_start
    }

    /// Get a mutable reference to the text callback buffer's selection start.
    pub fn selection_end_mut(&mut self) -> &mut i32 {
        self.selection_end
    }
}

#[repr(C)]
struct UserData<'a> {
    container: &'a mut ImString,
    cback_handler: &'a mut dyn TextCallbackHandler,
}

/// Currently, this might contain UB. We may be holdling two mutable pointers to the same
/// data. Not sure yet though.
extern "C" fn callback(data: *mut sys::ImGuiInputTextCallbackData) -> c_int {
    struct CallbackData<'a> {
        event_flag: InputTextFlags,
        user_data: &'a mut UserData<'a>,
    }

    let callback_data = unsafe {
        CallbackData {
            event_flag: InputTextFlags::from_bits((*data).EventFlag as u32).unwrap(),
            user_data: &mut *((*data).UserData as *mut UserData),
        }
    };

    let make_txt_data = || {
        // This safe in every callback EXCEPT RESIZE.
        unsafe {
            TextCallbackBuffer {
                // specifically, this will bork in resize
                buf: std::str::from_utf8_mut(std::slice::from_raw_parts_mut(
                    (*data).Buf as *mut u8,
                    (*data).BufTextLen as usize,
                ))
                .expect("internal imgui error -- it boofed a utf8"),

                dirty: &mut (*data).BufDirty,
                cursor_pos: &mut (*data).CursorPos,
                selection_start: &mut (*data).SelectionStart,
                selection_end: &mut (*data).SelectionEnd,
                callback_data: data,
            }
        }
    };

    // check this callback.
    match callback_data.event_flag {
        InputTextFlags::CALLBACK_ALWAYS => {
            let text_info = make_txt_data();
            callback_data.user_data.cback_handler.on_always(text_info);
        }
        InputTextFlags::CALLBACK_EDIT => {
            let text_info = make_txt_data();
            callback_data.user_data.cback_handler.on_edit(text_info);
        }
        InputTextFlags::CALLBACK_COMPLETION => {
            let text_info = make_txt_data();
            callback_data
                .user_data
                .cback_handler
                .on_completion(text_info);
        }
        InputTextFlags::CALLBACK_RESIZE => {
            unsafe {
                let requested_size = (*data).BufSize as usize;
                let buffer = &mut callback_data.user_data.container;
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
        InputTextFlags::CALLBACK_CHAR_FILTER => {
            let chr = unsafe { char::from_u32((*data).EventChar).unwrap() };
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
                    sys::ImGuiKey_UpArrow => EventDirection::Up,
                    sys::ImGuiKey_DownArrow => EventDirection::Down,
                    _ => panic!("Unexpected key"),
                }
            };
            let text_info = make_txt_data();

            callback_data
                .user_data
                .cback_handler
                .on_history(key, text_info);
        }

        _ => {}
    }

    0
}

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

static mut PASSTHROUGH_CALLBACK: PassthroughCallback = PassthroughCallback;
pub struct PassthroughCallback;
impl TextCallbackHandler for PassthroughCallback {}

#[must_use]
pub struct InputText<'ui, 'p> {
    label: &'p ImStr,
    hint: Option<&'p ImStr>,
    buf: &'p mut ImString,
    callback_handler: &'p mut dyn TextCallbackHandler,
    flags: InputTextFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> InputText<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p ImStr, buf: &'p mut ImString) -> Self {
        InputText {
            label,
            hint: None,
            // this is fine because no one else has access to this and imgui is single threaded.
            callback_handler: unsafe { &mut PASSTHROUGH_CALLBACK },
            buf,
            flags: InputTextFlags::CALLBACK_RESIZE,
            _phantom: PhantomData,
        }
    }

    /// Sets the hint displayed in the input text background.
    #[inline]
    pub fn hint(mut self, hint: &'p ImStr) -> Self {
        self.hint = Some(hint);
        self
    }

    impl_text_flags!(InputText);

    /// By default (as of 0.8.0), imgui-rs will automatically handle string resizes
    /// for `InputText` and `InputTextMultiline`.
    ///
    /// If, for some reason, you don't want this, you can run this function to prevent this.
    /// In that case, edits which would cause a resize will not occur.
    #[inline]
    pub fn do_not_resize(mut self) -> Self {
        self.flags.remove(InputTextFlags::CALLBACK_RESIZE);
        self
    }

    #[inline]
    pub fn callback(
        mut self,
        callbacks: InputTextCallback,
        callback: &'p mut dyn TextCallbackHandler,
    ) -> Self {
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
        self.callback_handler = callback;
        self
    }

    pub fn build(self) -> bool {
        let (ptr, capacity) = (self.buf.as_mut_ptr(), self.buf.capacity_with_nul());

        let mut data = UserData {
            container: self.buf,
            cback_handler: self.callback_handler,
        };
        let data = &mut data as *mut _ as *mut c_void;

        unsafe {
            let result = if let Some(hint) = self.hint {
                sys::igInputTextWithHint(
                    self.label.as_ptr(),
                    hint.as_ptr(),
                    ptr,
                    capacity,
                    self.flags.bits() as i32,
                    Some(callback),
                    data,
                )
            } else {
                sys::igInputText(
                    self.label.as_ptr(),
                    ptr,
                    capacity,
                    self.flags.bits() as i32,
                    Some(callback),
                    data,
                )
            };
            self.buf.refresh_len();
            result
        }
    }
}

#[must_use]
pub struct InputTextMultiline<'ui, 'p> {
    label: &'p ImStr,
    buf: &'p mut ImString,
    flags: InputTextFlags,
    size: [f32; 2],
    callback_handler: &'p mut dyn TextCallbackHandler,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> InputTextMultiline<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p ImStr, buf: &'p mut ImString, size: [f32; 2]) -> Self {
        InputTextMultiline {
            label,
            buf,
            flags: InputTextFlags::CALLBACK_RESIZE,
            size,
            // this is safe because
            callback_handler: unsafe { &mut PASSTHROUGH_CALLBACK },
            _phantom: PhantomData,
        }
    }

    impl_text_flags!(InputText);

    /// By default (as of 0.8.0), imgui-rs will automatically handle string resizes
    /// for `InputText` and `InputTextMultiline`.
    ///
    /// If, for some reason, you don't want this, you can run this function to prevent this.
    /// In that case, edits which would cause a resize will not occur.
    #[inline]
    pub fn do_not_resize(mut self) -> Self {
        self.flags.remove(InputTextFlags::CALLBACK_RESIZE);
        self
    }

    #[inline]
    pub fn callback(
        mut self,
        callbacks: InputTextMultilineCallback,
        callback: &'p mut dyn TextCallbackHandler,
    ) -> Self {
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
        self.callback_handler = callback;
        self
    }

    pub fn build(self) -> bool {
        let (ptr, capacity) = (self.buf.as_mut_ptr(), self.buf.capacity_with_nul());

        let mut data = UserData {
            container: self.buf,
            cback_handler: self.callback_handler,
        };
        let data = &mut data as *mut _ as *mut c_void;

        unsafe {
            let result = sys::igInputTextMultiline(
                self.label.as_ptr(),
                ptr,
                capacity,
                self.size.into(),
                self.flags.bits() as i32,
                Some(callback),
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
    flags: InputTextFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> InputInt<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p ImStr, value: &'p mut i32) -> Self {
        InputInt {
            label,
            value,
            step: 1,
            step_fast: 100,
            flags: InputTextFlags::empty(),
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
                self.flags.bits() as i32,
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
    flags: InputTextFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> InputFloat<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p ImStr, value: &'p mut f32) -> Self {
        InputFloat {
            label,
            value,
            step: 0.0,
            step_fast: 0.0,
            flags: InputTextFlags::empty(),
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
                self.flags.bits() as i32,
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
            flags: InputTextFlags,
            _phantom: PhantomData<&'ui Ui<'ui>>,
        }

        impl<'ui, 'p> $InputFloatN<'ui, 'p> {
            pub fn new(_: &Ui<'ui>, label: &'p ImStr, value: &'p mut [f32; $N]) -> Self {
                $InputFloatN {
                    label,
                    value,
                    flags: InputTextFlags::empty(),
                    _phantom: PhantomData,
                }
            }

            pub fn build(self) -> bool {
                unsafe {
                    sys::$igInputFloatN(
                        self.label.as_ptr(),
                        self.value.as_mut_ptr(),
                        b"%.3f\0".as_ptr() as *const _,
                        self.flags.bits() as i32,
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
            flags: InputTextFlags,
            _phantom: PhantomData<&'ui Ui<'ui>>,
        }

        impl<'ui, 'p> $InputIntN<'ui, 'p> {
            pub fn new(_: &Ui<'ui>, label: &'p ImStr, value: &'p mut [i32; $N]) -> Self {
                $InputIntN {
                    label,
                    value,
                    flags: InputTextFlags::empty(),
                    _phantom: PhantomData,
                }
            }

            pub fn build(self) -> bool {
                unsafe {
                    sys::$igInputIntN(
                        self.label.as_ptr(),
                        self.value.as_mut_ptr(),
                        self.flags.bits() as i32,
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
