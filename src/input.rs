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

#[must_use]
pub struct InputText<'ui, 'p> {
    label: &'p str,
    buf: &'p mut str,
    flags: ImGuiInputTextFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>
}

impl<'ui, 'p> InputText<'ui, 'p> {
    pub fn new(label: &'p str, buf: &'p mut str) -> Self {
        InputText {
            label: label,
            buf: buf,
            flags: ImGuiInputTextFlags::empty(),
            _phantom: PhantomData
        }
    }

    #[inline]
    pub fn flags(self, flags: ImGuiInputTextFlags) -> Self {
        InputText {
            flags: flags,
            .. self
        }
    }

    #[inline]
    pub fn chars_decimal(self, value: bool) -> Self {
        InputText {
            flags: self.flags.with(ImGuiInputTextFlags_CharsDecimal, value),
            .. self
        }
    }

    #[inline]
    pub fn chars_hexadecimal(self, value: bool) -> Self {
        InputText {
            flags: self.flags.with(ImGuiInputTextFlags_CharsHexadecimal, value),
            .. self
        }
    }

    #[inline]
    pub fn chars_uppercase(self, value: bool) -> Self {
        InputText {
            flags: self.flags.with(ImGuiInputTextFlags_CharsUppercase, value),
            .. self
        }
    }

    #[inline]
    pub fn chars_noblank(self, value: bool) -> Self {
        InputText {
            flags: self.flags.with(ImGuiInputTextFlags_CharsNoBlank, value),
            .. self
        }
    }

    #[inline]
    pub fn auto_select_all(self, value: bool) -> Self {
        InputText {
            flags: self.flags.with(ImGuiInputTextFlags_AutoSelectAll, value),
            .. self
        }
    }

    #[inline]
    pub fn enter_returns_true(self, value: bool) -> Self {
        InputText {
            flags: self.flags.with(ImGuiInputTextFlags_EnterReturnsTrue, value),
            .. self
        }
    }

    #[inline]
    pub fn callback_completion(self, value: bool) -> Self {
        InputText {
            flags: self.flags.with(ImGuiInputTextFlags_CallbackCompletion, value),
            .. self
        }
    }

    #[inline]
    pub fn callback_history(self, value: bool) -> Self {
        InputText {
            flags: self.flags.with(ImGuiInputTextFlags_CallbackHistory, value),
            .. self
        }
    }

    #[inline]
    pub fn callback_always(self, value: bool) -> Self {
        InputText {
            flags: self.flags.with(ImGuiInputTextFlags_CallbackAlways, value),
            .. self
        }
    }

    #[inline]
    pub fn callback_char_filter(self, value: bool) -> Self {
        InputText {
            flags: self.flags.with(ImGuiInputTextFlags_CallbackCharFilter, value),
            .. self
        }
    }

    #[inline]
    pub fn allow_tab_input(self, value: bool) -> Self {
        InputText {
            flags: self.flags.with(ImGuiInputTextFlags_AllowTabInput, value),
            .. self
        }
    }

    #[inline]
    pub fn no_horizontal_scroll(self, value: bool) -> Self {
        InputText {
            flags: self.flags.with(ImGuiInputTextFlags_NoHorizontalScroll, value),
            .. self
        }
    }

    #[inline]
    pub fn always_insert_mode(self, value: bool) -> Self {
        InputText {
            flags: self.flags.with(ImGuiInputTextFlags_AlwaysInsertMode, value),
            .. self
        }
    }

    // TODO: boxed closure...?
    // pub fn callback(self) -> Self { }

    pub fn build(self) -> bool {
        unsafe {
            imgui_sys::igInputText(
                imgui_sys::ImStr::from(self.label),
                // TODO: this is evil. Perhaps something else than &mut str is better
                self.buf.as_ptr() as *mut i8,
                self.buf.len() as size_t,
                self.flags,
                None,
                ptr::null_mut())
        }
    }
}
