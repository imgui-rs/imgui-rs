use bitflags::bitflags;
use std::os::raw::{c_int, c_void};
use std::ptr;
use std::{convert::TryFrom, marker::PhantomData};

use crate::sys;
use crate::{ImStr, ImString, Ui};

/// @angelofsol -- notice the "CALLBACK_EDIT" flag that I added too. We need to cover that also...probably.

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
        /// Call user function every time. User code may query cursor position, modify text buffer.
        const CALLBACK_ALWAYS = sys::ImGuiInputTextFlags_CallbackAlways;
        /// Call user function to filter character.
        const CALLBACK_CHAR_FILTER = sys::ImGuiInputTextFlags_CallbackCharFilter;
        /// Callback on buffer edit (note that InputText() already returns true on edit, the callback is useful mainly to manipulate the underlying buffer while focus is active)
        const CALLBACK_EDIT = sys::ImGuiInputTextFlags_CallbackEdit;
        /// Pressing TAB input a '\t' character into the text field
        const ALLOW_TAB_INPUT = sys::ImGuiInputTextFlags_AllowTabInput;
        /// In multi-line mode, unfocus with Enter, add new line with Ctrl+Enter (default is
        /// opposite: unfocus with Ctrl+Enter, add line with Enter).
        const CTRL_ENTER_FOR_NEW_LINE = sys::ImGuiInputTextFlags_CtrlEnterForNewLine;
        /// Disable following the cursor horizontally
        const NO_HORIZONTAL_SCROLL = sys::ImGuiInputTextFlags_NoHorizontalScroll;
        /// Insert mode
        const ALWAYS_INSERT_MODE = sys::ImGuiInputTextFlags_AlwaysInsertMode;
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
        pub fn callback_completion(mut self, value: bool) -> Self {
            self.flags.set(InputTextFlags::CALLBACK_COMPLETION, value);
            self
        }

        #[inline]
        pub fn callback_history(mut self, value: bool) -> Self {
            self.flags.set(InputTextFlags::CALLBACK_HISTORY, value);
            self
        }

        #[inline]
        pub fn callback_always(mut self, value: bool) -> Self {
            self.flags.set(InputTextFlags::CALLBACK_ALWAYS, value);
            self
        }

        #[inline]
        pub fn callback_char_filter(mut self, value: bool) -> Self {
            self.flags.set(InputTextFlags::CALLBACK_CHAR_FILTER, value);
            self
        }

        #[inline]
        pub fn resize_buffer(mut self, value: bool) -> Self {
            self.flags.set(InputTextFlags::CALLBACK_RESIZE, value);
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

        #[inline]
        pub fn always_insert_mode(mut self, value: bool) -> Self {
            self.flags.set(InputTextFlags::ALWAYS_INSERT_MODE, value);
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

/// This is the callback that we *always* provide. It will, helpfully, pass along
/// some data, which we use to figure out how to handle callbacks.
///
/// This modules role is essentially creating that payload and handling it as needed.
///
/// ## Safety
/// The complexity of this module is to make sure that this operation is safe. Users should
/// not be worried about safety here.
///
/// @angelofsol -- this code doesn't actually compile right now, but hey, it's getting there
unsafe extern "C" fn callback(data: *mut sys::ImGuiInputTextCallbackData) -> c_int {
    // rebind for ease
    let data = &mut *data;
    let event_flag = CallbackType::try_from(data.EventFlag as u32)
        .expect("imgui-rs error -- couldn't convert callback flag. Please submit an issue.");

    let handler = &mut *(data.UserData as *mut InternalCallback);

    let mut cback_data = CallbackData { inner: data };

    match event_flag {
        CallbackType::Completion => handler.user_callbacks.completion_callback(&mut cback_data),
        CallbackType::History => {
            let key = match data.EventKey as u32 {
                sys::ImGuiKey_UpArrow => HistoryKey::Up,
                sys::ImGuiKey_DownArrow => HistoryKey::Down,
                other => unimplemented!("internal imgui-rs -- couldn't convert {} to HistoryKey. Please submit an issue.", other),
            };

            handler
                .user_callbacks
                .history_callback(key, &mut cback_data)
        }
        CallbackType::CharacterFilter => {
            // uhhhhh here's hoping this works!
            let character = std::char::decode_utf16(std::iter::once(data.EventChar))
                .next()
                .unwrap()
                .unwrap();

            match handler
                .user_callbacks
                .filter_callback(character, &mut cback_data)
            {
                Some(chr) => {
                    data.EventChar = chr as u16;
                }
                None => {
                    // setting to 0 is a magic way to reject the character
                    data.EventChar = 0;
                }
            }
        }
        CallbackType::Edit => todo!("no support yet..."),
        CallbackType::Resize => {
            // this is inline, but probably shoudl be in a function
            let buffer = &mut *handler.buffer_to_resize;
            let requested_size = data.BufSize as usize;
            if requested_size > buffer.capacity_with_nul() {
                // Refresh the buffer's length to take into account changes made by dear imgui.
                buffer.refresh_len();
                buffer.reserve(requested_size - buffer.0.len());
                debug_assert!(buffer.capacity_with_nul() >= requested_size);
                data.Buf = buffer.as_mut_ptr();
                data.BufDirty = true;
            }
        }
    }

    0
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub enum CallbackType {
    Completion,
    History,
    CharacterFilter,
    Edit,
    Resize,
}

// This is just a quick InputTextCallback From that should remain entirely internal to this
// module. We will panic on Err, so it's no big deal to not have any information in it.
impl TryFrom<u32> for CallbackType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let found_flag = match value {
            sys::ImGuiInputTextFlags_CallbackCompletion => Self::Completion,
            sys::ImGuiInputTextFlags_CallbackHistory => Self::History,
            sys::ImGuiInputTextFlags_CallbackAlways => todo!("what do"),
            sys::ImGuiInputTextFlags_CallbackEdit => Self::Edit,
            sys::ImGuiInputTextFlags_CallbackCharFilter => Self::CharacterFilter,
            _ => return Err(()),
        };

        Ok(found_flag)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryKey {
    Up,
    Down,
}

// tighten the data provided to each event type
// so that the user can understand what they need to use to do what they want
// don't allow resize to be registered, and just let the normal callback

pub struct CallbackData {
    inner: *mut sys::ImGuiInputTextCallbackData,
}

impl CallbackData {
    // todo make this more intuitive
    pub fn insert_characters(&mut self, pos: i32, text: &ImStr) {
        unsafe {
            sys::ImGuiInputTextCallbackData_InsertChars(
                self.inner,
                pos,
                text.as_ptr(),
                ptr::null(), // imgui will read strlen for us based on nul character, though if we want to calc this our selves we can
            );
        }
    }
    // TODO make this more intuitive
    // things like deleting to the last X should be easy todo?
    // imgui repo says this code path isnt executed a lot
    // so we might consider not using this, and letting the user
    // do this kind of stuff through the &mut ImString
    pub fn delete_characters(&mut self, pos: i32, bytes_count: i32) {
        unsafe { sys::ImGuiInputTextCallbackData_DeleteChars(self.inner, pos, bytes_count) }
    }

    pub fn has_selection(&mut self) -> bool {
        unsafe { sys::ImGuiInputTextCallbackData_HasSelection(self.inner) }
    }

    pub fn clear_selection(&mut self) {
        unsafe { sys::ImGuiInputTextCallbackData_ClearSelection(self.inner) }
    }

    pub fn select_all(&mut self) {
        unsafe { sys::ImGuiInputTextCallbackData_SelectAll(self.inner) }
    }

    pub fn cursor_pos(&self) -> usize {
        let byte_position = unsafe { (*self.inner).CursorPos } as usize;

        self.byte_to_index(byte_position).unwrap()
    }

    pub fn set_cursor_pos(&mut self, index: usize) -> bool {
        if let Some(position) = self.index_to_byte(index) {
            unsafe {
                (*self.inner).CursorPos = position as i32;
            }
            true
        } else {
            false
        }
    }

    pub fn event_char(&self) -> char {
        // pretty sure EventChar is utf16 but i need to double check
        std::char::decode_utf16(std::iter::once(unsafe { (*self.inner).EventChar }))
            .next()
            .unwrap()
            .unwrap()
    }

    pub fn event_key(&self) -> crate::Key {
        let _ = unsafe { (*self.inner).EventKey };
        todo!()
    }

    // from what I can tell, selection_start and end represent byte positions
    // rather than utf8 scalar values, so we need to call get unchecked here
    // I looked at the https://github.com/ocornut/imgui/blob/master/imgui_widgets.cpp#L4354
    // to understand the caclulations, so it looks like these values should be safe to call
    // get_unchcked with
    // it might be reasonable to change the API of selection_start and selection_end to calculate
    // the utf8 boundaries, but im not sure how to do that off the top of my head
    pub fn selection_start(&self) -> usize {
        let byte_position = unsafe { (*self.inner).SelectionStart } as usize;

        self.byte_to_index(byte_position).unwrap()
    }
    pub fn set_selection_start(&mut self, index: usize) -> bool {
        if let Some(position) = self.index_to_byte(index) {
            unsafe {
                (*self.inner).SelectionStart = position as i32;
            }
            true
        } else {
            false
        }
    }

    pub fn selection_end(&self) -> usize {
        let byte_position = unsafe { (*self.inner).SelectionEnd } as usize;

        self.byte_to_index(byte_position).unwrap()
    }
    pub fn set_selection_end(&mut self, index: usize) -> bool {
        if let Some(position) = self.index_to_byte(index) {
            unsafe {
                (*self.inner).SelectionEnd = position as i32;
            }
            true
        } else {
            false
        }
    }

    // these 2 are probably good helper functions for users
    pub fn delete_selection(&mut self) -> bool {
        todo!()
    }
    pub fn replace_selection(&mut self, value: &'_ str) {
        todo!()
    }

    // this functionality probably makes sense to implement ourselves?
    // i kind of presume that autocomplete is a word by word basis
    // so this would be deleting all the data from the cursor to the most recent
    // whitespace, and then insert the new value at the new cursor position

    pub fn autocomplete_with(&mut self, value: &'_ str) {
        todo!()
    }

    // these seem slow cause of the iteration
    // but it also doesnt seem like theyre going to be called often enough to be
    // a pain point?
    fn byte_to_index(&self, byte_position: usize) -> Option<usize> {
        // alternative impl
        // str.get(..byte_position).map(|s| s.chars().count())
        self.buffer()
            .to_str()
            .char_indices()
            .enumerate()
            .find(|(_, (byte_offset, _))| *byte_offset == byte_position)
            .map(|(idx, _)| idx)
    }
    fn index_to_byte(&self, index: usize) -> Option<usize> {
        self.buffer()
            .to_str()
            .char_indices()
            .nth(index)
            .map(|(value, _)| value)
    }

    pub fn selection(&self) -> &str {
        &self.buffer().to_str()[self.selection_start()..self.selection_end()]
    }

    pub fn is_buffer_dirty(&self) -> bool {
        unsafe { (*self.inner).BufDirty }
    }

    // this values is in bytes, not in unicode scalar values
    pub fn text_len(&self) -> i32 {
        // this might not make sense to even keep around?
        // our wrapper should probably update this after the callback is over to match what
        // is in the &mut ImString buffer
        // this number is a byte size regardless
        // and im not sure the users actually care
        unsafe { (*self.inner).BufTextLen }
    }

    // interestingly we pass a usize to imgui
    // but this struct will give us back an i32
    // not sure if this behavior is actually correct
    // it seems like we cast this straight to i32 in most cases
    // but I'm not sure how good of an idea that actually is given
    // usize != i32 in terms of range
    // its unlikely someone is gonna use a buffer size greater than i32::MAX
    // but we should note down somewhere that this is an "issue"
    //
    // we should probably just let the buffer method take care of this
    // as ImString/ImStr have capacity/lenghth info
    pub fn buffer_size(&self) -> i32 {
        unsafe { (*self.inner).BufSize }
    }

    pub fn buffer(&self) -> &ImString {
        unsafe { &*((*self.inner).Buf as *mut ImString) }
    }

    /// calling this function automatically sets the buffer dirty flag.
    // it's also not clear to me if we should let users modify this instance at will
    // and instead force them to use the add/delete character subfuncs
    // as far as i can tell thats what the intention in the C++ API is
    // though I'm not sure if its enforced
    //
    // looking through the imgui code
    // it seems like the funcs above actually just do in place buffer manipulation
    // so it might be worth it to not expose those and just let the user
    // manipulate them through the &mut ImString
    /* the following comment is from the imgui_widgets.cpp file:
        // Public API to manipulate UTF-8 text
        // We expose UTF-8 to the user (unlike the STB_TEXTEDIT_* functions which are manipulating wchar)
        // FIXME: The existence of this rarely exercised code path is a bit of a nuisance.
        void ImGuiInputTextCallbackData::DeleteChars(int pos, int bytes_count) { ... }
    */
    // it might be worth it to just let the user mutate this
    // and then set all the particulars for textlen, capacity et al
    // after the users callback is done
    // that way we can just let the user manipulate the &mut ImString as normal
    // the only weirdness here is updating the selection i32s
    // imgui has some logic in the InsertCharacters routine
    // that we might be better if we duplicate instead
    pub fn buffer_mut(&mut self) -> &mut ImString {
        self.set_buffer_dirty();
        unsafe { &mut *((*self.inner).Buf as *mut ImString) }
    }

    pub fn set_buffer_dirty(&mut self) {
        unsafe {
            (*self.inner).BufDirty = true;
        }
    }

    pub fn callback_type(&self) -> CallbackType {
        unsafe {
            let event = (*self.inner).EventFlag;

            if event == InputTextFlags::CALLBACK_RESIZE.bits() as i32 {
                CallbackType::Resize
            } else if event == InputTextFlags::CALLBACK_CHAR_FILTER.bits() as i32 {
                CallbackType::CharacterFilter
            } else if event == InputTextFlags::CALLBACK_HISTORY.bits() as i32 {
                CallbackType::History
            } else if event == sys::ImGuiInputTextFlags_CallbackEdit as i32 {
                CallbackType::Edit
            } else if event == InputTextFlags::CALLBACK_COMPLETION.bits() as i32 {
                CallbackType::Completion
            } else {
                panic!("Imgui returned unexpected callback type");
            }
        }
    }
}

// when callback_type == CharacterFilter
// the return value of this function tells imgui whether or not to keep the character
// 0 -> ignore, 1 -> keep
extern "C" fn generic_callback(data: *mut sys::ImGuiInputTextCallbackData) -> c_int {
    unsafe {
        // this has to be an FnMut, because imgui will call the closure multiple times
        // this also means we have to deal with the resize callback here
        // and the etc

        // 99% sure the API we want to provide, is giving an enum back to the user
        // explaining which call this was
        // and also passing a object that represents the rest of the callback info, but something rusty
        // also need to implement the functions it provides

        let callback = &mut *((*data).UserData as *mut InternalCallback);
        let mut callback_data = CallbackData { inner: data };

        if let Some(buffer) = callback.buffer_to_resize.and_then(|buffer| buffer.as_mut()) {
            if (*data).EventFlag == InputTextFlags::CALLBACK_RESIZE.bits() as i32 {
                let requested_size = (*data).BufSize as usize;
                if requested_size > buffer.capacity_with_nul() {
                    // Refresh the buffer's length to take into account changes made by dear imgui.
                    buffer.refresh_len();
                    buffer.reserve(requested_size - buffer.0.len());
                    debug_assert!(buffer.capacity_with_nul() >= requested_size);
                    // the Buf passed back to us by Imgui, and the one we hold onto, are the same?
                    // when exactly do we dealloc the buffer?
                    (*data).Buf = buffer.as_mut_ptr();
                    (*data).BufDirty = true;
                }
            }
        }

        if let Some(user_callback) = &mut callback.user_callbacks {
            if callback.buffer_to_resize.is_none()
                || (*data).EventFlag != InputTextFlags::CALLBACK_RESIZE.bits() as i32
            {
                return if user_callback(&mut callback_data) {
                    1
                } else {
                    0
                };
            }
        }
        0
    }
}

#[must_use]
pub struct InputText<'ui, 'p> {
    label: &'p ImStr,
    hint: Option<&'p ImStr>,
    buf: &'p mut ImString,
    flags: InputTextFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> InputText<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p ImStr, buf: &'p mut ImString) -> Self {
        InputText {
            label,
            hint: None,
            buf,
            flags: InputTextFlags::empty(),
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

    // TODO: boxed closure...?
    // pub fn callback(self) -> Self { }

    unsafe fn build_internal(
        self,
        callback: sys::ImGuiInputTextCallback,
        data: *mut c_void,
    ) -> bool {
        let (ptr, capacity) = (self.buf.as_mut_ptr(), self.buf.capacity_with_nul());

        let result = if let Some(hint) = self.hint {
            sys::igInputTextWithHint(
                self.label.as_ptr(),
                hint.as_ptr(),
                ptr,
                capacity,
                self.flags.bits() as i32,
                callback,
                data,
            )
        } else {
            sys::igInputText(
                self.label.as_ptr(),
                ptr,
                capacity,
                self.flags.bits() as i32,
                callback,
                data,
            )
        };
        self.buf.refresh_len();
        result
    }

    pub fn build(self) -> bool {
        let mut user_data = InternalCallback {
            user_callbacks: None,
            buffer_to_resize: self
                .flags
                .contains(InputTextFlags::CALLBACK_RESIZE)
                .then(|| self.buf as *mut _),
        };
        let (callback, data): (sys::ImGuiInputTextCallback, _) = {
            (
                Some(generic_callback),
                &mut user_data as *mut _ as *mut c_void,
            )
        };

        unsafe { self.build_internal(callback, data) }
    }

    pub fn build_with_callback<F: FnMut(&mut CallbackData) -> bool>(self, mut f: F) -> bool {
        let mut user_data = InternalCallback {
            user_callbacks: Some(&mut f),
            buffer_to_resize: self
                .flags
                .contains(InputTextFlags::CALLBACK_RESIZE)
                .then(|| self.buf as *mut _),
        };

        let (callback, data): (sys::ImGuiInputTextCallback, _) = {
            (
                Some(generic_callback),
                &mut user_data as *mut _ as *mut c_void,
            )
        };
        unsafe { self.build_internal(callback, data) }
    }
}

struct InternalCallback<'a> {
    user_callbacks: CallbackRefs<'a>,
    buffer_to_resize: Option<*mut ImString>,
}

pub struct UserCallbacks<CompletionFunc, HistoryFunc, CharacterFilterFunc, EditFunc, ResizeFunc> {
    completion: CompletionFunc,
    history: HistoryFunc,
    filter: CharacterFilterFunc,
    edit: EditFunc,
    resize: ResizeFunc,
}

impl<
        CompletionFunc: ToDynFnMut,
        HistoryFunc: ToDynFnMut,
        CharacterFilterFunc,
        EditFunc,
        ResizeFunc,
    > UserCallbacks<CompletionFunc, HistoryFunc, CharacterFilterFunc, EditFunc, ResizeFunc>
{
    fn get_refs(&mut self) -> CallbackRefs<'_> {
        CallbackRefs {
            completion: self.completion.as_dyn_fn_mut(),
            history: self.history.as_dyn_fn_mut(),
        }
    }
}

pub struct CallbackRefs<'a> {
    completion: Option<&'a mut dyn FnMut(&mut CallbackData)>,
    history: Option<&'a mut dyn FnMut(&mut CallbackData)>,
}

pub trait ToDynFnMut {
    fn as_dyn_fn_mut(&mut self) -> Option<&mut dyn FnMut(&mut CallbackData)>;
}

impl ToDynFnMut for () {
    fn as_dyn_fn_mut(&mut self) -> Option<&mut dyn FnMut(&mut CallbackData)> {
        None
    }
}

impl<F: FnMut(&mut CallbackData)> ToDynFnMut for F {
    fn as_dyn_fn_mut(&mut self) -> Option<&mut dyn FnMut(&mut CallbackData)> {
        Some(self)
    }
}

impl UserCallbacks<(), (), (), (), ()> {
    pub fn new() -> Self {
        Self {
            completion: (),
            history: (),
            filter: (),
            edit: (),
            resize: (),
        }
    }
}

#[test]
fn test() {
    let mut x = UserCallbacks::new().history(|_| {});
    let callbacks = x.get_refs();
}

impl<HistoryFunc, CharacterFilterFunc, EditFunc, ResizeFunc>
    UserCallbacks<(), HistoryFunc, CharacterFilterFunc, EditFunc, ResizeFunc>
{
    pub fn completion<F: FnMut(&mut CallbackData)>(
        self,
        f: F,
    ) -> UserCallbacks<F, HistoryFunc, CharacterFilterFunc, EditFunc, ResizeFunc> {
        UserCallbacks {
            completion: f,
            history: self.history,
            filter: self.filter,
            edit: self.edit,
            resize: self.resize,
        }
    }
}

impl<CompletionFunc, CharacterFilterFunc, EditFunc, ResizeFunc>
    UserCallbacks<CompletionFunc, (), CharacterFilterFunc, EditFunc, ResizeFunc>
{
    pub fn history<F: FnMut(&mut CallbackData)>(
        self,
        f: F,
    ) -> UserCallbacks<CompletionFunc, F, CharacterFilterFunc, EditFunc, ResizeFunc> {
        UserCallbacks {
            completion: self.completion,
            history: f,
            filter: self.filter,
            edit: self.edit,
            resize: self.resize,
        }
    }
}

#[must_use]
pub struct InputTextMultiline<'ui, 'p> {
    label: &'p ImStr,
    buf: &'p mut ImString,
    flags: InputTextFlags,
    size: [f32; 2],
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> InputTextMultiline<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p ImStr, buf: &'p mut ImString, size: [f32; 2]) -> Self {
        InputTextMultiline {
            label,
            buf,
            flags: InputTextFlags::empty(),
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
            if self.flags.contains(InputTextFlags::CALLBACK_RESIZE) {
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
                self.flags.bits() as i32,
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
