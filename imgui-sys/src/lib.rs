#![allow(non_upper_case_globals)]

extern crate libc;

#[cfg(feature = "gfx")]
#[macro_use]
extern crate gfx;

#[cfg(feature = "glium")]
extern crate glium;

#[cfg(feature = "gfx")]
mod gfx_support;

#[cfg(feature = "glium")]
mod glium_support;

pub mod ffi;
pub use ffi::*;
