use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::c_char;
use std::panic::catch_unwind;
use std::process;
use std::ptr;

// use crate::string::{ImStr, ImString};
use crate::Ui;

/// Trait for clipboard backends
pub trait ClipboardBackend: 'static {
    /// Returns the current clipboard contents as an owned imgui-rs string, or None if the
    /// clipboard is empty or inaccessible
    fn get(&mut self) -> Option<String>;
    /// Sets the clipboard contents to the given imgui-rs string slice.
    fn set(&mut self, value: &str);
}

pub(crate) struct ClipboardContext {
    backend: Box<dyn ClipboardBackend>,
    // this is needed to keep ownership of the value when the raw C callback is called
    last_value: CString,
}

impl ClipboardContext {
    /// Creates a new [ClipboardContext]. This function previously took a `Box`, but now
    /// is generic over the T it takes and boxes itself (which should be less strange).
    pub fn new<T: ClipboardBackend>(backend: T) -> ClipboardContext {
        ClipboardContext {
            backend: Box::new(backend) as Box<dyn ClipboardBackend>,
            last_value: CString::default(),
        }
    }

    pub fn dummy() -> ClipboardContext {
        Self {
            backend: Box::new(DummyClipboardContext),
            last_value: CString::default(),
        }
    }
}
/// Non-functioning placeholder
pub struct DummyClipboardContext;
impl ClipboardBackend for DummyClipboardContext {
    fn get(&mut self) -> Option<String> {
        None
    }

    fn set(&mut self, _: &str) {
        // empty
    }
}

impl fmt::Debug for ClipboardContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClipboardContext")
            // beautiful code, no?
            .field("backend", &(&(*self.backend) as *const _))
            .field("last_value", &self.last_value)
            .finish()
    }
}

pub(crate) unsafe extern "C" fn get_clipboard_text(
    _user_data: *mut sys::ImGuiContext,
) -> *const c_char {
    let result = catch_unwind(|| {
        let user_data = unsafe { (*sys::igGetPlatformIO()).Platform_ClipboardUserData };

        let ctx = &mut *(user_data as *mut ClipboardContext);
        match ctx.backend.get() {
            Some(text) => {
                ctx.last_value = CString::new(text).unwrap();
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

pub(crate) unsafe extern "C" fn set_clipboard_text(
    _user_data: *mut sys::ImGuiContext,
    text: *const c_char,
) {
    let result = catch_unwind(|| {
        let user_data = unsafe { (*sys::igGetPlatformIO()).Platform_ClipboardUserData };

        let ctx = &mut *(user_data as *mut ClipboardContext);
        let text = CStr::from_ptr(text).to_owned();
        ctx.backend.set(text.to_str().unwrap());
    });
    result.unwrap_or_else(|_| {
        eprintln!("Clipboard setter panicked");
        process::abort();
    });
}

/// # Clipboard
#[allow(clippy::fn_address_comparisons)] // This is allowed because although function addresses wont be unique, we just care if its OURS
impl Ui {
    /// Returns the current clipboard contents as text, or None if the clipboard is empty or cannot
    /// be accessed
    pub fn clipboard_text(&self) -> Option<String> {
        let platform_io = unsafe { sys::igGetPlatformIO() };

        let current_clipboard_text_fn = unsafe { (*platform_io).Platform_GetClipboardTextFn };

        current_clipboard_text_fn.and_then(|get_clipboard_text_fn| {
            // Bypass FFI if we end up calling our own function anyway
            if get_clipboard_text_fn == get_clipboard_text {
                let ctx = unsafe {
                    &mut *((*platform_io).Platform_ClipboardUserData as *mut ClipboardContext)
                };
                ctx.backend.get()
            } else {
                unsafe {
                    let text_ptr = get_clipboard_text_fn(sys::igGetCurrentContext());
                    if text_ptr.is_null() || *text_ptr == b'\0' as c_char {
                        None
                    } else {
                        Some(
                            CStr::from_ptr(text_ptr)
                                .to_owned()
                                .to_str()
                                .ok()?
                                .to_owned(),
                        )
                    }
                }
            }
        })
    }

    /// Sets the clipboard contents.
    ///
    /// Does nothing if the clipboard cannot be accessed.
    pub fn set_clipboard_text(&self, text: impl AsRef<str>) {
        let platform_io = unsafe { sys::igGetPlatformIO() };
        let set_clipboard_text_fn = unsafe { (*platform_io).Platform_SetClipboardTextFn };

        if let Some(set_clipboard_text_fn) = set_clipboard_text_fn {
            // Bypass FFI if we end up calling our own function anyway
            if set_clipboard_text_fn == set_clipboard_text {
                let ctx = unsafe {
                    &mut *((*platform_io).Platform_ClipboardUserData as *mut ClipboardContext)
                };
                ctx.backend.set(text.as_ref());
            } else {
                unsafe {
                    set_clipboard_text_fn(sys::igGetCurrentContext(), self.scratch_txt(text));
                }
            }
        }
    }
}
