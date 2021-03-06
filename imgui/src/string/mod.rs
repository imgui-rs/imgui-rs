#[macro_use]
mod legacy;
pub use legacy::{ImStr, ImString};
use std::os::raw::c_char;

pub trait AsImStr: sealed::Sealed {
    // Note: #[doc(hidden)] + doc helps since r-a doesn't really know about
    // doc(hidden) always... The only part we use a doc comment for is the scary
    // part though.
    //
    /// `__write_priv` is not part of our public API!
    ///
    /// If external code calls it, it will likely break!
    //
    // For example: It may need to change for the imgui branch that supports
    // string with lengths. Internal code shoudldn't use it directly either
    // unless it's part of this file.
    //
    // impl notes: `Buf` is always `SmallCStrBuf`, but specified viaa an
    // equivalent-ish trait to avoid needing that to be `#[doc(hidden)] pub`
    // too. also, this is basically a weirdly shaped `Display` trait...
    #[doc(hidden)]
    fn __write_priv<Buf: core::fmt::Write>(self, buf: &mut Buf);
}

impl<T: ?Sized + AsRef<str>> AsImStr for &T {
    fn __write_priv<B: core::fmt::Write>(self, buf: &mut B) {
        let res = buf.write_str(self.as_ref());
        debug_assert!(res.is_ok());
    }
}

impl AsImStr for core::fmt::Arguments<'_> {
    fn __write_priv<B: core::fmt::Write>(self, buf: &mut B) {
        let res = std::fmt::write(buf, self);
        debug_assert!(res.is_ok());
    }
}

mod sealed {
    /// A trait that exists to prevent external implementations of AsImStr
    pub trait Sealed {}
    impl<T: ?Sized + AsRef<str>> Sealed for &T {}
    impl Sealed for core::fmt::Arguments<'_> {}
}

/// Internal macro to help wrangle SmallCStrBuf/AsImStr. see `test_with_cstr`
/// below for more thorough usage, but basically the main idea:
///
/// ```
/// # use imgui::AsImStr;
/// # use std::os::raw::c_cstr;
/// // if you have one or more `AsImStr`s, like:
///
/// fn foo(_a: impl AsImStr) { /* ??? your code here ??? */ }
///
/// // And you want to call a function that takes a `*const c_char`:
///
/// fn bar(_: *const c_char) {}
///
/// # #[cfg(FALSE)] mod _rustoc_ignore_is_bad {
/// // you do it like:
/// fn foo(a: impl AsImStr) {
///     // fake closure. used both for input and output.
///     with_cstr!(|a| bar(a));
/// }
///
/// unsafe fn bar_dangerously(_: *const c_char) {}
///
/// // Note also:
/// fn foo_dangerously(a: impl AsImStr) {
///     with_cstr!(unsafe |a| bar_dangerously(a));
///     // which is shorthand for:
///     with_cstr!(|a| unsafe { bar_dangerously(a) });
/// }
/// # }
/// ```
macro_rules! with_cstr {
    (|$first:ident $(,)?| $body:expr) => {
        with_cstr!(@one $first, |$first| $body)
    };
    (|$first:ident $(, $rest:ident)* $(,)?| $body:expr) => {
        with_cstr!(@one $first, |$first| with_cstr!(|$($rest),*| $body))
    };

    // most of our uses will have unsafe bodies (why do we need the pointer if
    // we aren't going to pass them to something), so i did this fake "unsafe
    // closure" syntax:
    //
    // - `with_cstr!(unsafe |foo| bar())` is the same as:
    // - `with_cstr!(|foo| unsafe { bar() })` but with less nesting and
    //   indentation.
    (unsafe |$first:ident $(,)?| $body:expr) => {
        with_cstr!(@one $first, |$first| unsafe { $body })
    };
    (unsafe |$first:ident $(, $rest:ident)* $(,)?| $body:expr) => {
        with_cstr!(@one $first, |$first| with_cstr!(|$($rest),*| unsafe { $body }))
    };

    // most important one: the above just call this one recursively.
    // note that it doesn't require `$string` and `$ptr_name` be the same, altho
    // for us they always are.
    (@one $string:expr, |$ptr_name:ident| $body:expr) => {{
        // manual usage of `SmallCStrBuf` is like this. put the string on the stack
        let mut buf = crate::string::SmallCStrBuf::new();
        // call `buf.cstr_for(something)` where `something: impl AsImStr` to get
        // pointer. (Note: declaring the type is just to help explain, not
        // needed)
        let $ptr_name: *const std::os::raw::c_char = buf.cstr_for($string);
        // then do something with it.
        $body
        // There are a lot of ways to mess that up which lead to dangling
        // pointers (same issues that exist with std::ffi::CString), so
        // we use this macro.
    }};
}

#[test]
fn test_with_cstr() {
    unsafe fn check_ptrs(a: *const c_char, b: *const c_char, expect: (&str, &str)) {
        assert_eq!(std::ffi::CStr::from_ptr(a).to_str().unwrap(), expect.0);
        assert_eq!(std::ffi::CStr::from_ptr(b).to_str().unwrap(), expect.1);
    }
    fn two_args(a: impl AsImStr, b: impl AsImStr, expect: (&str, &str)) {
        // Takes some identifiers that are `impl AsImStr`
        with_cstr!(|a, b| unsafe {
            // and mucks around with the cstrbuf and their trait a bit,
            // but afterwards, a and b are *const c_char here.
            let _: [*const c_char; 2] = [a, b];
            check_ptrs(a, b, expect);
        });
    }

    two_args(
        format_args!("a {} b", 123),
        &"hello hello hello".repeat(50),
        ("a 123 b", &"hello hello hello".repeat(50)),
    );
    two_args(
        im_str!("abcde"),
        &im_str!("hijkl {:?}", (5, 30.3)),
        ("abcde", "hijkl (5, 30.3)"),
    );

    fn single(only_one: impl AsImStr, v: &str) {
        // Note: "`unsafe` closure" style works with multiarg too, it just
        // allows avoiding the brackets,
        with_cstr!(unsafe |only_one| check_ptrs(only_one, only_one, (v, v)));
    }
    single("12345", "12345");
}

/// Similar to std::ffi::CString, but avoids heap allocating if the string is
/// small enough. This is actually overkill for what we need, and should be
/// simplified to only handle the initialization case.
///
/// Don't use directly, use via the `with_cstr!` macro.
#[derive(Clone, Default)]
pub(crate) struct SmallCStrBuf(smallvec::SmallVec<[u8; 128]>);

impl SmallCStrBuf {
    #[inline]
    pub(crate) fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub(crate) fn cstr_for(&mut self, arg: impl AsImStr) -> *const c_char {
        arg.__write_priv(self);
        if self.0.is_empty() {
            b"\0".as_ptr().cast()
        } else {
            self.0.push(0u8);
            self.0.as_ptr().cast()
        }
    }
}

impl core::fmt::Write for SmallCStrBuf {
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0.reserve(s.len() + 1);
        // If there are any internal nuls, replace them with "\\0". This is
        // dodgy kinda, but should be fixed when we move to the explicit-length
        // string branch, so I'd rather not complicate the API worrying about it
        // that much. (it would proably be reasonable to debug_assert about it
        // tho)
        let mut first = true;
        for chunk in s.split('\0') {
            if !first {
                self.0.extend_from_slice(b"\\0");
            }
            first = false;
            self.0.extend_from_slice(chunk.as_bytes());
        }
        Ok(())
    }
}
