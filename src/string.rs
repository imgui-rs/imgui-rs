use std::borrow::Borrow;
use std::ffi::{CStr, CString, NulError};
use std::fmt;
use std::mem;
use std::ops::Deref;
use std::os::raw::c_char;
use std::str;

#[derive(Clone, Default, Hash, Ord, Eq, PartialOrd, PartialEq)]
pub struct ImString(Vec<u8>);

impl ImString {
    pub fn new<T: Into<String>>(t: T) -> Result<ImString, NulError> {
        CString::new(t.into()).map(|cstring| ImString(cstring.into_bytes_with_nul()))
    }
    pub fn with_capacity(capacity: usize) -> ImString {
        let mut v = Vec::with_capacity(capacity + 1);
        v.push(b'\0');
        ImString(v)
    }
    pub unsafe fn from_string_unchecked(s: String) -> ImString {
        ImString::from_vec_unchecked(s.into())
    }
    pub unsafe fn from_vec_unchecked(mut v: Vec<u8>) -> ImString {
        v.push(b'\0');
        ImString(v)
    }
    pub fn clear(&mut self) {
        self.0.clear();
        self.0.push(b'\0');
    }
    pub fn push_str(&mut self, string: &str) {
        self.0.pop();
        self.0.extend_from_slice(string.as_bytes());
        self.0.push(b'\0');
    }
    pub fn capacity(&self) -> usize { self.0.capacity() - 1 }
    pub fn capacity_with_nul(&self) -> usize { self.0.capacity() }
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }
    pub fn reserve_exact(&mut self, additional: usize) {
        self.0.reserve_exact(additional);
    }
}

impl AsRef<ImStr> for ImString {
    fn as_ref(&self) -> &ImStr { self }
}

impl Borrow<ImStr> for ImString {
    fn borrow(&self) -> &ImStr { self }
}

impl fmt::Debug for ImString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: &str = self;
        fmt::Debug::fmt(s, f)
    }
}

impl Deref for ImString {
    type Target = ImStr;
    fn deref(&self) -> &ImStr {
        // as_ptr() is used, because we need to look at the bytes to figure out the length
        // self.0.len() is incorrect, because there might be more than one nul byte in the end
        unsafe { mem::transmute(CStr::from_ptr(self.0.as_ptr() as *const c_char)) }
    }
}

impl<'a> From<&'a ImStr> for ImString {
    fn from(value: &'a ImStr) -> ImString { value.to_owned() }
}

#[derive(Hash)]
pub struct ImStr(CStr);

impl<'a> Default for &'a ImStr {
    fn default() -> &'a ImStr {
        static SLICE: &'static [u8] = &[0];
        unsafe { ImStr::from_bytes_unchecked(SLICE) }
    }
}

impl fmt::Debug for ImStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl ImStr {
    pub unsafe fn from_bytes_unchecked<'a>(bytes: &'a [u8]) -> &'a ImStr {
        mem::transmute(bytes)
    }
    pub fn as_ptr(&self) -> *const c_char { self.0.as_ptr() as *const c_char }
    pub fn to_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.as_bytes()) }
    }
}

impl<'a> Into<&'a CStr> for &'a ImStr {
    fn into(self) -> &'a CStr { &self.0 }
}

impl AsRef<CStr> for ImStr {
    fn as_ref(&self) -> &CStr { &self.0 }
}

impl AsRef<ImStr> for ImStr {
    fn as_ref(&self) -> &ImStr { self }
}

impl ToOwned for ImStr {
    type Owned = ImString;
    fn to_owned(&self) -> ImString { ImString(self.0.to_owned().into_bytes()) }
}

impl Deref for ImStr {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.0.to_bytes()) }
    }
}
