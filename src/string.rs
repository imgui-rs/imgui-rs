use std::borrow::{Borrow, Cow};
use std::ffi::CStr;
use std::fmt;
use std::ops::{Deref, Index, RangeFull};
use std::os::raw::c_char;
use std::str;

#[macro_export]
macro_rules! im_str {
    ($e:tt) => ({
        unsafe {
          $crate::ImStr::from_utf8_with_nul_unchecked(concat!($e, "\0").as_bytes())
        }
    });
    ($e:tt, $($arg:tt)*) => ({
        unsafe {
          $crate::ImString::from_utf8_with_nul_unchecked(format!(concat!($e, "\0"), $($arg)*).into_bytes())
        }
    })
}

/// A UTF-8 encoded, growable, implicitly null-terminated string.
#[derive(Clone, Hash, Ord, Eq, PartialOrd, PartialEq)]
pub struct ImString(pub(crate) Vec<u8>);

impl ImString {
    /// Creates a new `ImString` from an existing string.
    pub fn new<T: Into<String>>(value: T) -> ImString {
        unsafe { ImString::from_utf8_unchecked(value.into().into_bytes()) }
    }
    /// Creates a new empty `ImString` with a particular capacity
    pub fn with_capacity(capacity: usize) -> ImString {
        let mut v = Vec::with_capacity(capacity + 1);
        v.push(b'\0');
        ImString(v)
    }
    /// Converts a vector of bytes to a `ImString` without checking that the string contains valid
    /// UTF-8
    pub unsafe fn from_utf8_unchecked(mut v: Vec<u8>) -> ImString {
        v.push(b'\0');
        ImString(v)
    }
    /// Converts a vector of bytes to a `ImString` without checking that the string contains valid
    /// UTF-8
    pub unsafe fn from_utf8_with_nul_unchecked(v: Vec<u8>) -> ImString {
        ImString(v)
    }
    /// Truncates this `ImString`, removing all contents
    pub fn clear(&mut self) {
        self.0.clear();
        self.0.push(b'\0');
    }
    /// Appends the given character to the end of this `ImString`
    pub fn push(&mut self, ch: char) {
        let mut buf = [0; 4];
        self.push_str(ch.encode_utf8(&mut buf));
    }
    /// Appends a given string slice to the end of this `ImString`
    pub fn push_str(&mut self, string: &str) {
        self.refresh_len();
        self.0.extend_from_slice(string.as_bytes());
        self.0.push(b'\0');
    }
    /// Returns the capacity of this `ImString` in bytes
    pub fn capacity(&self) -> usize {
        self.0.capacity() - 1
    }
    /// Returns the capacity of this `ImString` in bytes, including the implicit null byte
    pub fn capacity_with_nul(&self) -> usize {
        self.0.capacity()
    }
    /// Ensures that the capacity of this `ImString` is at least `additional` bytes larger than the
    /// current length.
    ///
    /// The capacity may be increased by more than `additional` bytes.
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }
    /// Ensures that the capacity of this `ImString` is at least `additional` bytes larger than the
    /// current length
    pub fn reserve_exact(&mut self, additional: usize) {
        self.0.reserve_exact(additional);
    }
    pub fn as_ptr(&self) -> *const c_char {
        self.0.as_ptr() as *const _
    }
    pub fn as_mut_ptr(&mut self) -> *mut c_char {
        self.0.as_mut_ptr() as *mut _
    }
    /// Updates the buffer length based on the current contents.
    ///
    /// Dear imgui accesses pointers directly, so the length doesn't get updated when the contents
    /// change. This is normally OK, because Deref to ImStr always calculates the slice length
    /// based on contents. However, we need to refresh the length in some ImString functions.
    pub(crate) fn refresh_len(&mut self) {
        let len = self.to_str().len();
        unsafe {
            self.0.set_len(len);
        }
    }
}

impl<'a> Default for ImString {
    fn default() -> ImString {
        ImString(vec![b'\0'])
    }
}

impl From<String> for ImString {
    fn from(s: String) -> ImString {
        unsafe { ImString::from_utf8_unchecked(s.into_bytes()) }
    }
}

impl<'a> From<ImString> for Cow<'a, ImStr> {
    fn from(s: ImString) -> Cow<'a, ImStr> {
        Cow::Owned(s)
    }
}

impl<'a> From<&'a ImString> for Cow<'a, ImStr> {
    fn from(s: &'a ImString) -> Cow<'a, ImStr> {
        Cow::Borrowed(s)
    }
}

impl<'a, T: ?Sized + AsRef<ImStr>> From<&'a T> for ImString {
    fn from(s: &'a T) -> ImString {
        s.as_ref().to_owned()
    }
}

impl AsRef<ImStr> for ImString {
    fn as_ref(&self) -> &ImStr {
        self
    }
}

impl Borrow<ImStr> for ImString {
    fn borrow(&self) -> &ImStr {
        self
    }
}

impl AsRef<str> for ImString {
    fn as_ref(&self) -> &str {
        self.to_str()
    }
}

impl Borrow<str> for ImString {
    fn borrow(&self) -> &str {
        self.to_str()
    }
}

impl Index<RangeFull> for ImString {
    type Output = ImStr;
    fn index(&self, _index: RangeFull) -> &ImStr {
        self
    }
}

impl fmt::Debug for ImString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.to_str(), f)
    }
}

impl fmt::Display for ImString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.to_str(), f)
    }
}

impl Deref for ImString {
    type Target = ImStr;
    fn deref(&self) -> &ImStr {
        // as_ptr() is used, because we need to look at the bytes to figure out the length
        // self.0.len() is incorrect, because there might be more than one nul byte in the end
        unsafe {
            &*(CStr::from_ptr(self.0.as_ptr() as *const c_char) as *const CStr as *const ImStr)
        }
    }
}

/// A UTF-8 encoded, implicitly null-terminated string slice.
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImStr(CStr);

impl<'a> Default for &'a ImStr {
    fn default() -> &'a ImStr {
        static SLICE: &'static [u8] = &[0];
        unsafe { ImStr::from_utf8_with_nul_unchecked(SLICE) }
    }
}

impl fmt::Debug for ImStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for ImStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.to_str(), f)
    }
}

impl ImStr {
    /// Wraps a raw UTF-8 encoded C string
    pub unsafe fn from_ptr_unchecked<'a>(ptr: *const c_char) -> &'a ImStr {
        ImStr::from_cstr_unchecked(CStr::from_ptr(ptr))
    }
    /// Converts a slice of bytes to an imgui-rs string slice without checking for valid UTF-8 or
    /// null termination.
    pub unsafe fn from_utf8_with_nul_unchecked(bytes: &[u8]) -> &ImStr {
        &*(bytes as *const [u8] as *const ImStr)
    }
    /// Converts a CStr reference to an imgui-rs string slice without checking for valid UTF-8.
    pub unsafe fn from_cstr_unchecked(value: &CStr) -> &ImStr {
        &*(value as *const CStr as *const ImStr)
    }
    /// Converts an imgui-rs string slice to a raw pointer
    pub fn as_ptr(&self) -> *const c_char {
        self.0.as_ptr()
    }
    /// Converts an imgui-rs string slice to a normal string slice
    pub fn to_str(&self) -> &str {
        // CStr::to_bytes does *not* include the null terminator
        unsafe { str::from_utf8_unchecked(self.0.to_bytes()) }
    }
    /// Returns true if the imgui-rs string slice is empty
    pub fn is_empty(&self) -> bool {
        self.0.to_bytes().is_empty()
    }
}

impl AsRef<CStr> for ImStr {
    fn as_ref(&self) -> &CStr {
        &self.0
    }
}

impl AsRef<ImStr> for ImStr {
    fn as_ref(&self) -> &ImStr {
        self
    }
}

impl AsRef<str> for ImStr {
    fn as_ref(&self) -> &str {
        self.to_str()
    }
}

impl<'a> From<&'a ImStr> for Cow<'a, ImStr> {
    fn from(s: &'a ImStr) -> Cow<'a, ImStr> {
        Cow::Borrowed(s)
    }
}

impl ToOwned for ImStr {
    type Owned = ImString;
    fn to_owned(&self) -> ImString {
        ImString(self.0.to_owned().into_bytes())
    }
}
