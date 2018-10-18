#[macro_use]
extern crate lazy_static;
extern crate parking_lot;

pub extern crate imgui_sys as sys;

mod context;
mod string;
mod style;
#[cfg(test)]
mod test;

use std::ffi::CStr;
use std::str;

pub use context::*;
pub use string::*;
pub use style::*;

/// Returns the underlying Dear ImGui library version
pub fn get_dear_imgui_version() -> &'static str {
    unsafe {
        let bytes = CStr::from_ptr(sys::igGetVersion()).to_bytes();
        str::from_utf8_unchecked(bytes)
    }
}

#[test]
fn test_get_version() {
    assert_eq!(get_dear_imgui_version(), "1.67");
}
