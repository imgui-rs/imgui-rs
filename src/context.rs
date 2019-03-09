use parking_lot::ReentrantMutex;
use std::cell::RefCell;
use std::ffi::CStr;
use std::ops::Drop;
use std::ptr;
use std::rc::Rc;

use crate::clipboard::{Clipboard, ClipboardContext};
use crate::font_atlas::{FontAtlas, FontAtlasRefMut, SharedFontAtlas};
use crate::io::Io;
use crate::string::{ImStr, ImString};
use crate::style::Style;
use crate::sys;
use crate::Ui;

/// An imgui-rs context.
///
/// A context needs to be created to access most library functions. Due to current Dear ImGui
/// design choices, at most one active Context can exist at any time. This limitation will likely
/// be removed in a future Dear ImGui version.
///
/// If you need more than one context, you can use suspended contexts. As long as only one context
/// is active at a time, it's possible to have multiple independent contexts.
///
/// # Examples
///
/// Creating a new active context:
/// ```
/// let ctx = imgui::Context::create();
/// // ctx is dropped naturally when it goes out of scope, which deactivates and destroys the
/// // context
/// ```
///
/// Never try to create an active context when another one is active:
///
/// ```should_panic
/// let ctx1 = imgui::Context::create();
///
/// let ctx2 = imgui::Context::create(); // PANIC
/// ```
///
/// Suspending an active context allows you to create another active context:
///
/// ```
/// let ctx1 = imgui::Context::create();
/// let suspended1 = ctx1.suspend();
/// let ctx2 = imgui::Context::create(); // this is now OK
/// ```

#[derive(Debug)]
pub struct Context {
    raw: *mut sys::ImGuiContext,
    shared_font_atlas: Option<Rc<RefCell<SharedFontAtlas>>>,
    ini_filename: Option<ImString>,
    log_filename: Option<ImString>,
    platform_name: Option<ImString>,
    renderer_name: Option<ImString>,
    clipboard_ctx: Option<Box<ClipboardContext>>,
}

lazy_static! {
    // This mutex needs to be used to guard all public functions that can affect the underlying
    // Dear ImGui active context
    static ref CTX_MUTEX: ReentrantMutex<()> = ReentrantMutex::new(());
}

fn clear_current_context() {
    unsafe {
        sys::igSetCurrentContext(ptr::null_mut());
    }
}
fn no_current_context() -> bool {
    let ctx = unsafe { sys::igGetCurrentContext() };
    ctx.is_null()
}

impl Context {
    /// Creates a new active imgui-rs context.
    ///
    /// # Panics
    ///
    /// Panics if an active context already exists
    pub fn create() -> Self {
        Self::create_internal(None)
    }
    /// Creates a new active imgui-rs context with a shared font atlas.
    ///
    /// # Panics
    ///
    /// Panics if an active context already exists
    pub fn create_with_shared_font_atlas(shared_font_atlas: Rc<RefCell<SharedFontAtlas>>) -> Self {
        Self::create_internal(Some(shared_font_atlas))
    }
    /// Suspends this context so another context can be the active context.
    pub fn suspend(self) -> SuspendedContext {
        let _guard = CTX_MUTEX.lock();
        assert!(
            self.is_current_context(),
            "context to be suspended is not the active context"
        );
        clear_current_context();
        SuspendedContext(self)
    }
    pub fn ini_filename(&self) -> Option<&ImStr> {
        let io = self.io();
        if io.ini_filename.is_null() {
            None
        } else {
            unsafe { Some(ImStr::from_ptr_unchecked(io.ini_filename)) }
        }
    }
    pub fn set_ini_filename<T: Into<Option<ImString>>>(&mut self, ini_filename: T) {
        let ini_filename = ini_filename.into();
        self.io_mut().ini_filename = ini_filename
            .as_ref()
            .map(|x| x.as_ptr())
            .unwrap_or(ptr::null());
        self.ini_filename = ini_filename;
    }
    pub fn log_filename(&self) -> Option<&ImStr> {
        let io = self.io();
        if io.log_filename.is_null() {
            None
        } else {
            unsafe { Some(ImStr::from_ptr_unchecked(io.log_filename)) }
        }
    }
    pub fn set_log_filename<T: Into<Option<ImString>>>(&mut self, log_filename: T) {
        let log_filename = log_filename.into();
        self.io_mut().log_filename = log_filename
            .as_ref()
            .map(|x| x.as_ptr())
            .unwrap_or(ptr::null());
        self.log_filename = log_filename;
    }
    pub fn platform_name(&self) -> Option<&ImStr> {
        let io = self.io();
        if io.backend_platform_name.is_null() {
            None
        } else {
            unsafe { Some(ImStr::from_ptr_unchecked(io.backend_platform_name)) }
        }
    }
    pub fn set_platform_name<T: Into<Option<ImString>>>(&mut self, platform_name: T) {
        let platform_name = platform_name.into();
        self.io_mut().backend_platform_name = platform_name
            .as_ref()
            .map(|x| x.as_ptr())
            .unwrap_or(ptr::null());
        self.platform_name = platform_name;
    }
    pub fn renderer_name(&self) -> Option<&ImStr> {
        let io = self.io();
        if io.backend_renderer_name.is_null() {
            None
        } else {
            unsafe { Some(ImStr::from_ptr_unchecked(io.backend_renderer_name)) }
        }
    }
    pub fn set_renderer_name<T: Into<Option<ImString>>>(&mut self, renderer_name: T) {
        let renderer_name = renderer_name.into();
        self.io_mut().backend_renderer_name = renderer_name
            .as_ref()
            .map(|x| x.as_ptr())
            .unwrap_or(ptr::null());
        self.renderer_name = renderer_name;
    }
    pub fn load_ini_settings(&mut self, data: &str) {
        unsafe { sys::igLoadIniSettingsFromMemory(data.as_ptr() as *const _, data.len()) }
    }
    pub fn save_ini_settings(&mut self, buf: &mut String) {
        let data = unsafe { CStr::from_ptr(sys::igSaveIniSettingsToMemory(ptr::null_mut())) };
        buf.push_str(&data.to_string_lossy());
    }
    pub fn set_clipboard(&mut self, clipboard: Box<dyn Clipboard>) {
        use std::borrow::BorrowMut;
        let mut clipboard_ctx = Box::new(ClipboardContext::new(clipboard));
        let io = self.io_mut();
        io.set_clipboard_text_fn = Some(crate::clipboard::set_clipboard_text);
        io.get_clipboard_text_fn = Some(crate::clipboard::get_clipboard_text);
        io.clipboard_user_data = clipboard_ctx.borrow_mut() as *mut ClipboardContext as *mut _;
        self.clipboard_ctx.replace(clipboard_ctx);
    }
    fn create_internal(shared_font_atlas: Option<Rc<RefCell<SharedFontAtlas>>>) -> Self {
        let _guard = CTX_MUTEX.lock();
        assert!(
            no_current_context(),
            "A new active context cannot be created, because another one already exists"
        );
        // Dear ImGui implicitly sets the current context during igCreateContext if the current
        // context doesn't exist
        let raw = unsafe { sys::igCreateContext(ptr::null_mut()) };
        Context {
            raw,
            shared_font_atlas,
            ini_filename: None,
            log_filename: None,
            platform_name: None,
            renderer_name: None,
            clipboard_ctx: None,
        }
    }
    fn is_current_context(&self) -> bool {
        let ctx = unsafe { sys::igGetCurrentContext() };
        self.raw == ctx
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        let _guard = CTX_MUTEX.lock();
        // If this context is the active context, Dear ImGui automatically deactivates it during
        // destruction
        unsafe {
            sys::igDestroyContext(self.raw);
        }
    }
}

/// A suspended imgui-rs context.
///
/// A suspended context retains its state, but is not usable without activating it first.
///
/// # Examples
///
/// Suspended contexts are not directly very useful, but you can activate them:
///
/// ```
/// let suspended = imgui::SuspendedContext::create();
/// match suspended.activate() {
///   Ok(ctx) => {
///     // ctx is now the active context
///   },
///   Err(suspended) => {
///     // activation failed, so you get the suspended context back
///   }
/// }
/// ```
#[derive(Debug)]
pub struct SuspendedContext(Context);

impl SuspendedContext {
    /// Creates a new suspended imgui-rs context.
    pub fn create() -> Self {
        Self::create_internal(None)
    }
    /// Creates a new suspended imgui-rs context with a shared font atlas.
    pub fn create_with_shared_font_atlas(shared_font_atlas: Rc<RefCell<SharedFontAtlas>>) -> Self {
        Self::create_internal(Some(shared_font_atlas))
    }
    /// Attempts to activate this suspended context.
    ///
    /// If there is no active context, this suspended context is activated and `Ok` is returned,
    /// containing the activated context.
    /// If there is already an active context, nothing happens and `Err` is returned, containing
    /// the original suspended context.
    pub fn activate(self) -> Result<Context, SuspendedContext> {
        let _guard = CTX_MUTEX.lock();
        if no_current_context() {
            unsafe {
                sys::igSetCurrentContext(self.0.raw);
            }
            Ok(self.0)
        } else {
            Err(self)
        }
    }
    fn create_internal(shared_font_atlas: Option<Rc<RefCell<SharedFontAtlas>>>) -> Self {
        let _guard = CTX_MUTEX.lock();
        let raw = unsafe { sys::igCreateContext(ptr::null_mut()) };
        let ctx = Context {
            raw,
            shared_font_atlas,
            ini_filename: None,
            log_filename: None,
            platform_name: None,
            renderer_name: None,
            clipboard_ctx: None,
        };
        if ctx.is_current_context() {
            // Oops, the context was activated -> deactivate
            clear_current_context();
        }
        SuspendedContext(ctx)
    }
}

#[test]
fn test_one_context() {
    let _guard = crate::test::TEST_MUTEX.lock();
    let _ctx = Context::create();
    assert!(!no_current_context());
}

#[test]
fn test_drop_clears_current_context() {
    let _guard = crate::test::TEST_MUTEX.lock();
    {
        let _ctx1 = Context::create();
        assert!(!no_current_context());
    }
    assert!(no_current_context());
    {
        let _ctx2 = Context::create();
        assert!(!no_current_context());
    }
    assert!(no_current_context());
}

#[test]
fn test_new_suspended() {
    let _guard = crate::test::TEST_MUTEX.lock();
    let ctx = Context::create();
    let _suspended = SuspendedContext::create();
    assert!(ctx.is_current_context());
    ::std::mem::drop(_suspended);
    assert!(ctx.is_current_context());
}

#[test]
fn test_suspend() {
    let _guard = crate::test::TEST_MUTEX.lock();
    let ctx = Context::create();
    assert!(!no_current_context());
    let _suspended = ctx.suspend();
    assert!(no_current_context());
    let _ctx2 = Context::create();
}

#[test]
fn test_drop_suspended() {
    let _guard = crate::test::TEST_MUTEX.lock();
    let suspended = Context::create().suspend();
    assert!(no_current_context());
    let ctx2 = Context::create();
    ::std::mem::drop(suspended);
    assert!(ctx2.is_current_context());
}

#[test]
fn test_suspend_activate() {
    let _guard = crate::test::TEST_MUTEX.lock();
    let suspended = Context::create().suspend();
    assert!(no_current_context());
    let ctx = suspended.activate().unwrap();
    assert!(ctx.is_current_context());
}

#[test]
fn test_suspend_failure() {
    let _guard = crate::test::TEST_MUTEX.lock();
    let suspended = Context::create().suspend();
    let _ctx = Context::create();
    assert!(suspended.activate().is_err());
}

#[test]
fn test_shared_font_atlas() {
    let _guard = crate::test::TEST_MUTEX.lock();
    let atlas = Rc::new(RefCell::new(SharedFontAtlas::create()));
    let suspended1 = SuspendedContext::create_with_shared_font_atlas(atlas.clone());
    let mut ctx2 = Context::create_with_shared_font_atlas(atlas.clone());
    {
        let _borrow = ctx2.fonts();
    }
    let _suspended2 = ctx2.suspend();
    let mut ctx = suspended1.activate().unwrap();
    let _borrow = ctx.fonts();
}

#[test]
#[should_panic]
fn test_shared_font_atlas_borrow_panic() {
    let _guard = crate::test::TEST_MUTEX.lock();
    let atlas = Rc::new(RefCell::new(SharedFontAtlas::create()));
    let _suspended = SuspendedContext::create_with_shared_font_atlas(atlas.clone());
    let mut ctx = Context::create_with_shared_font_atlas(atlas.clone());
    let _borrow1 = atlas.borrow();
    let _borrow2 = ctx.fonts();
}

#[test]
fn test_ini_load_save() {
    let (_guard, mut ctx) = crate::test::test_ctx();
    let data = "[Window][Debug##Default]
Pos=60,60
Size=400,400
Collapsed=0";
    ctx.load_ini_settings(&data);
    let mut buf = String::new();
    ctx.save_ini_settings(&mut buf);
    assert_eq!(data.trim(), buf.trim());
}

impl Context {
    /// Returns an immutable reference to the inputs/outputs object
    pub fn io(&self) -> &Io {
        unsafe {
            // safe because Io is a transparent wrapper around sys::ImGuiIO
            &*(sys::igGetIO() as *const Io)
        }
    }
    /// Returns a mutable reference to the inputs/outputs object
    pub fn io_mut(&mut self) -> &mut Io {
        unsafe {
            // safe because Io is a transparent wrapper around sys::ImGuiIO
            &mut *(sys::igGetIO() as *mut Io)
        }
    }
    /// Returns an immutable reference to the user interface style
    pub fn style(&self) -> &Style {
        unsafe {
            // safe because Style is a transparent wrapper around sys::ImGuiStyle
            &*(sys::igGetStyle() as *const Style)
        }
    }
    /// Returns a mutable reference to the user interface style
    pub fn style_mut(&mut self) -> &mut Style {
        unsafe {
            // safe because Style is a transparent wrapper around sys::ImGuiStyle
            &mut *(sys::igGetStyle() as *mut Style)
        }
    }
    /// Returns a mutable reference to the font atlas.
    ///
    /// # Panics
    ///
    /// Panics if the context uses a shared font atlas that is already borrowed
    pub fn fonts(&mut self) -> FontAtlasRefMut {
        match self.shared_font_atlas {
            Some(ref font_atlas) => FontAtlasRefMut::Shared(font_atlas.borrow_mut()),
            None => unsafe {
                // safe because FontAtlas is a transparent wrapper around sys::ImFontAtlas
                let fonts = &mut *(self.io_mut().fonts as *mut FontAtlas);
                FontAtlasRefMut::Owned(fonts)
            },
        }
    }
    /// # Panics
    ///
    /// Panics if the context uses a shared font atlas that is already borrowed
    pub fn frame<'ui, 'a: 'ui>(&'a mut self) -> Ui<'ui> {
        // NewFrame/Render/EndFrame mutate the font atlas so we need exclusive access to it
        let font_atlas = self
            .shared_font_atlas
            .as_ref()
            .map(|font_atlas| font_atlas.borrow_mut());
        // TODO: precondition checks
        unsafe {
            sys::igNewFrame();
        }
        Ui {
            ctx: self,
            font_atlas,
        }
    }
}
