use std::fmt;
use std::os::raw::{c_char, c_void};
use std::panic::catch_unwind;
use std::process;

use crate::string::ImStr;

pub trait Clipboard {
    fn get(&mut self) -> &ImStr;
    fn set(&mut self, value: &ImStr);
}

pub(crate) struct ClipboardContext(pub Box<dyn Clipboard>);

impl fmt::Debug for ClipboardContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClipboardContext({:?})", &(*self.0) as *const _)
    }
}

pub(crate) unsafe extern "C" fn get_clipboard_text(user_data: *mut c_void) -> *const c_char {
    let result = catch_unwind(|| {
        let ctx = &mut *(user_data as *mut ClipboardContext);
        ctx.0.get().as_ptr()
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
        ctx.0.set(text);
    });
    result.unwrap_or_else(|_| {
        eprintln!("Clipboard setter panicked");
        process::abort();
    });
}
