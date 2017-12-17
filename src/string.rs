use std::borrow::Borrow;
use std::fmt;
use std::os::raw::c_char;
use std::str;

#[derive(Clone, Hash, Ord, Eq, PartialOrd, PartialEq)]
pub struct ImString(Vec<u8>);

impl ImString {
    pub fn new<T: Into<String>>(value: T) -> ImString {
        unsafe { ImString::from_utf8_unchecked(value.into().into_bytes()) }
    }
    pub fn with_capacity(capacity: usize) -> ImString {
        let mut v = Vec::with_capacity(capacity + 1);
        v.push(b'\0');
        ImString(v)
    }
    pub unsafe fn from_utf8_unchecked(mut v: Vec<u8>) -> ImString {
        v.push(b'\0');
        ImString(v)
    }
    pub unsafe fn from_utf8_with_nul_unchecked(v: Vec<u8>) -> ImString { ImString(v) }
    pub fn clear(&mut self) {
        self.0.clear();
        self.0.push(b'\0');
    }
    pub fn push(&mut self, ch: char) {
        let mut buf = [0; 4];
        self.push_str(ch.encode_utf8(&mut buf));
    }
    pub fn push_str(&mut self, string: &str) {
        self.refresh_len();
        self.0.extend_from_slice(string.as_bytes());
        self.0.push(b'\0');
    }
    pub fn capacity(&self) -> usize { self.0.capacity() - 1 }
    pub fn capacity_with_nul(&self) -> usize { self.0.capacity() }
    pub fn reserve(&mut self, additional: usize) { self.0.reserve(additional); }
    pub fn reserve_exact(&mut self, additional: usize) { self.0.reserve_exact(additional); }
    pub fn as_ptr(&self) -> *const c_char { self.0.as_ptr() as *const _ }
    pub fn as_mut_ptr(&mut self) -> *mut c_char { self.0.as_mut_ptr() as *mut _ }

    pub fn as_str(&self) -> &str {
        let len = if let Some(index) = self.0.iter().position(|&c| c == b'\0') {
            index
        } else {
            0
        };
        str::from_utf8(&self.0[0..len]).unwrap()
    }

    /// Updates the buffer length based on the current contents.
    ///
    /// Dear imgui accesses pointers directly, so the length doesn't get updated when the contents
    /// change. This is normally OK, because Deref to ImStr always calculates the slice length
    /// based on contents. However, we need to refresh the length in some ImString functions.
    fn refresh_len(&mut self) {
        let len = self.as_str().len();
        unsafe {
            self.0.set_len(len);
        }
    }
}

impl<'a> Default for ImString {
    fn default() -> ImString { unsafe { ImString::from_utf8_with_nul_unchecked(vec![0]) } }
}

impl From<String> for ImString {
    fn from(s: String) -> ImString { ImString::new(s) }
}

impl AsRef<str> for ImString {
    fn as_ref(&self) -> &str { self.as_str() }
}

impl Borrow<str> for ImString {
    fn borrow(&self) -> &str { self.as_str() }
}

impl fmt::Debug for ImString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Debug::fmt(self.as_str(), f) }
}
