use std::fmt;
use std::os::raw::{c_char, c_void};
use std::panic::catch_unwind;
use std::process;
use std::ptr;

use crate::string::{ImStr, ImString};
use crate::Ui;

/// Trait for clipboard backends
pub trait ClipboardBackend {
    /// Returns the current clipboard contents as an owned imgui-rs string, or None if the
    /// clipboard is empty or inaccessible
    fn get(&mut self) -> Option<ImString>;
    /// Sets the clipboard contents to the given imgui-rs string slice.
    fn set(&mut self, value: &ImStr);
}

pub(crate) struct ClipboardContext {
    backend: Box<dyn ClipboardBackend>,
    // this is needed to keep ownership of the value when the raw C callback is called
    last_value: ImString,
}

impl ClipboardContext {
    #[inline]
    pub fn new(backend: Box<dyn ClipboardBackend>) -> ClipboardContext {
        ClipboardContext {
            backend,
            last_value: ImString::default(),
        }
    }
}

impl fmt::Debug for ClipboardContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ClipboardContext({:?}, {:?})",
            &(*self.backend) as *const _,
            self.last_value
        )
    }
}

pub(crate) unsafe extern "C" fn get_clipboard_text(user_data: *mut c_void) -> *const c_char {
    let result = catch_unwind(|| {
        let ctx = &mut *(user_data as *mut ClipboardContext);
        match ctx.backend.get() {
            Some(text) => {
                ctx.last_value = text;
                ctx.last_value.as_ptr()
            }
            None => ptr::null(),
        }
    });
    result.unwrap_or_else(|_| {
        eprintln!("Clipboard getter panicked");
        process::abort();
    })
}

pub(crate) unsafe extern "C" fn set_clipboard_text(user_data: *mut c_void, text: *const c_char) {
    let result = catch_unwind(|| {
        let ctx = &mut *(user_data as *mut ClipboardContext);
        let text = ImStr::from_ptr_unchecked(text);
        ctx.backend.set(text);
    });
    result.unwrap_or_else(|_| {
        eprintln!("Clipboard setter panicked");
        process::abort();
    });
}

/// # Clipboard
#[allow(clippy::fn_address_comparisons)] // This is allowed because although function addresses wont be unique, we just care if its OURS
impl<'ui> Ui<'ui> {
    /// Returns the current clipboard contents as text, or None if the clipboard is empty or cannot
    /// be accessed
    pub fn clipboard_text(&self) -> Option<ImString> {
        let io = self.io();
        io.get_clipboard_text_fn.and_then(|get_clipboard_text_fn| {
            // Bypass FFI if we end up calling our own function anyway
            if get_clipboard_text_fn == get_clipboard_text {
                let ctx = unsafe { &mut *(io.clipboard_user_data as *mut ClipboardContext) };
                ctx.backend.get()
            } else {
                unsafe {
                    let text_ptr = get_clipboard_text_fn(io.clipboard_user_data);
                    if text_ptr.is_null() || *text_ptr == b'\0' as c_char {
                        None
                    } else {
                        Some(ImStr::from_ptr_unchecked(text_ptr).to_owned())
                    }
                }
            }
        })
    }
    /// Sets the clipboard contents.
    ///
    /// Does nothing if the clipboard cannot be accessed.
    pub fn set_clipboard_text(&self, text: &ImStr) {
        let io = self.io();
        if let Some(set_clipboard_text_fn) = io.set_clipboard_text_fn {
            // Bypass FFI if we end up calling our own function anyway
            if set_clipboard_text_fn == set_clipboard_text {
                let ctx = unsafe { &mut *(io.clipboard_user_data as *mut ClipboardContext) };
                ctx.backend.set(text);
            } else {
                unsafe {
                    set_clipboard_text_fn(io.clipboard_user_data, text.as_ptr());
                }
            }
        }
    }
}
