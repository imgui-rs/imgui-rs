use parking_lot::ReentrantMutex;
use std::cell::UnsafeCell;
use std::ffi::{CStr, CString};
use std::ops::Drop;
use std::path::PathBuf;
use std::ptr;

use crate::clipboard::{ClipboardBackend, ClipboardContext};
use crate::fonts::atlas::{FontAtlas, FontId, SharedFontAtlas};
use crate::io::Io;
use crate::style::Style;
use crate::{sys, DrawData};
use crate::{MouseCursor, Ui};

#[cfg(feature = "docking")]
use crate::docking_utils;

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
    shared_font_atlas: Option<SharedFontAtlas>,
    ini_filename: Option<CString>,
    log_filename: Option<CString>,
    platform_name: Option<CString>,
    renderer_name: Option<CString>,
    // we need to box this because we hand imgui a pointer to it,
    // and we don't want to deal with finding `clipboard_ctx`.
    // we also put it in an unsafecell since we're going to give
    // imgui a mutable pointer to it.
    clipboard_ctx: Box<UnsafeCell<ClipboardContext>>,

    ui: Ui,
}

// This mutex needs to be used to guard all public functions that can affect the underlying
// Dear ImGui active context
static CTX_MUTEX: ReentrantMutex<()> = parking_lot::const_reentrant_mutex(());

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
    #[doc(alias = "CreateContext")]
    pub fn create() -> Self {
        Self::create_internal(None)
    }
    /// Creates a new active imgui-rs context with a shared font atlas.
    ///
    /// # Panics
    ///
    /// Panics if an active context already exists
    #[doc(alias = "CreateContext")]
    pub fn create_with_shared_font_atlas(shared_font_atlas: SharedFontAtlas) -> Self {
        Self::create_internal(Some(shared_font_atlas))
    }
    /// Suspends this context so another context can be the active context.
    #[doc(alias = "CreateContext")]
    pub fn suspend(self) -> SuspendedContext {
        let _guard = CTX_MUTEX.lock();
        assert!(
            self.is_current_context(),
            "context to be suspended is not the active context"
        );
        clear_current_context();
        SuspendedContext(self)
    }
    /// Returns the path to the ini file, or None if not set
    pub fn ini_filename(&self) -> Option<PathBuf> {
        let io = self.io();
        if io.ini_filename.is_null() {
            None
        } else {
            let s = unsafe { CStr::from_ptr(io.ini_filename) };
            Some(PathBuf::from(s.to_str().ok()?))
        }
    }
    /// Sets the path to the ini file (default is "imgui.ini")
    ///
    /// Pass None to disable automatic .Ini saving.
    pub fn set_ini_filename<T: Into<Option<PathBuf>>>(&mut self, ini_filename: T) {
        let ini_filename: Option<PathBuf> = ini_filename.into();
        let ini_filename = ini_filename.and_then(|v| CString::new(v.to_str()?).ok());

        self.io_mut().ini_filename = ini_filename
            .as_ref()
            .map(|x| x.as_ptr())
            .unwrap_or(ptr::null());
        self.ini_filename = ini_filename;
    }
    /// Returns the path to the log file, or None if not set
    // TODO: why do we return an `Option<PathBuf>` instead of an `Option<&Path>`?
    pub fn log_filename(&self) -> Option<PathBuf> {
        let io = self.io();
        if io.log_filename.is_null() {
            None
        } else {
            let cstr = unsafe { CStr::from_ptr(io.log_filename) };
            Some(PathBuf::from(cstr.to_str().ok()?))
        }
    }
    /// Sets the log filename (default is "imgui_log.txt").
    pub fn set_log_filename<T: Into<Option<PathBuf>>>(&mut self, log_filename: T) {
        let log_filename = log_filename
            .into()
            .and_then(|v| CString::new(v.to_str()?).ok());

        self.io_mut().log_filename = log_filename
            .as_ref()
            .map(|x| x.as_ptr())
            .unwrap_or(ptr::null());
        self.log_filename = log_filename;
    }
    /// Returns the backend platform name, or None if not set
    pub fn platform_name(&self) -> Option<&str> {
        let io = self.io();
        if io.backend_platform_name.is_null() {
            None
        } else {
            let cstr = unsafe { CStr::from_ptr(io.backend_platform_name) };
            cstr.to_str().ok()
        }
    }
    /// Sets the backend platform name
    pub fn set_platform_name<T: Into<Option<String>>>(&mut self, platform_name: T) {
        let platform_name: Option<CString> =
            platform_name.into().and_then(|v| CString::new(v).ok());
        self.io_mut().backend_platform_name = platform_name
            .as_ref()
            .map(|x| x.as_ptr())
            .unwrap_or(ptr::null());
        self.platform_name = platform_name;
    }
    /// Returns the backend renderer name, or None if not set
    pub fn renderer_name(&self) -> Option<&str> {
        let io = self.io();
        if io.backend_renderer_name.is_null() {
            None
        } else {
            let cstr = unsafe { CStr::from_ptr(io.backend_renderer_name) };
            cstr.to_str().ok()
        }
    }
    /// Sets the backend renderer name
    pub fn set_renderer_name<T: Into<Option<String>>>(&mut self, renderer_name: T) {
        let renderer_name: Option<CString> =
            renderer_name.into().and_then(|v| CString::new(v).ok());

        self.io_mut().backend_renderer_name = renderer_name
            .as_ref()
            .map(|x| x.as_ptr())
            .unwrap_or(ptr::null());

        self.renderer_name = renderer_name;
    }
    /// Loads settings from a string slice containing settings in .Ini file format
    #[doc(alias = "LoadIniSettingsFromMemory")]
    pub fn load_ini_settings(&mut self, data: &str) {
        unsafe { sys::igLoadIniSettingsFromMemory(data.as_ptr() as *const _, data.len()) }
    }
    /// Saves settings to a mutable string buffer in .Ini file format
    #[doc(alias = "SaveInitSettingsToMemory")]
    pub fn save_ini_settings(&mut self, buf: &mut String) {
        let data = unsafe { CStr::from_ptr(sys::igSaveIniSettingsToMemory(ptr::null_mut())) };
        buf.push_str(&data.to_string_lossy());
    }
    /// Sets the clipboard backend used for clipboard operations
    pub fn set_clipboard_backend<T: ClipboardBackend>(&mut self, backend: T) {
        let clipboard_ctx: Box<UnsafeCell<_>> = Box::new(ClipboardContext::new(backend).into());
        let platform_io = unsafe {
            // safe because PlatformIo is a transparent wrapper around sys::ImGuiPlatformIO
            // and &mut self ensures exclusive ownership of PlatformIo.
            &mut *(sys::igGetPlatformIO() as *mut crate::PlatformIo)
        };
        platform_io.set_clipboard_text_fn = Some(crate::clipboard::set_clipboard_text);
        platform_io.get_clipboard_text_fn = Some(crate::clipboard::get_clipboard_text);

        platform_io.clipboard_user_data = clipboard_ctx.get() as *mut _;
        self.clipboard_ctx = clipboard_ctx;
    }
    fn create_internal(mut shared_font_atlas: Option<SharedFontAtlas>) -> Self {
        let _guard = CTX_MUTEX.lock();
        assert!(
            no_current_context(),
            "A new active context cannot be created, because another one already exists"
        );

        let shared_font_atlas_ptr = match &mut shared_font_atlas {
            Some(shared_font_atlas) => shared_font_atlas.as_ptr_mut(),
            None => ptr::null_mut(),
        };
        // Dear ImGui implicitly sets the current context during igCreateContext if the current
        // context doesn't exist
        let raw = unsafe { sys::igCreateContext(shared_font_atlas_ptr) };

        Context {
            raw,
            shared_font_atlas,
            ini_filename: None,
            log_filename: None,
            platform_name: None,
            renderer_name: None,
            clipboard_ctx: Box::new(ClipboardContext::dummy().into()),
            ui: Ui {
                buffer: UnsafeCell::new(crate::string::UiBuffer::new(1024)),
            },
        }
    }
    fn is_current_context(&self) -> bool {
        let ctx = unsafe { sys::igGetCurrentContext() };
        self.raw == ctx
    }
}

impl Drop for Context {
    #[doc(alias = "DestroyContext")]
    fn drop(&mut self) {
        let _guard = CTX_MUTEX.lock();
        // If this context is the active context, Dear ImGui automatically deactivates it during
        // destruction
        unsafe {
            // end the frame if necessary...
            if !sys::igGetCurrentContext().is_null() && sys::igGetFrameCount() > 0 {
                sys::igEndFrame();
            }
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
    #[doc(alias = "CreateContext")]
    pub fn create() -> Self {
        Self::create_internal(None)
    }

    /// Creates a new suspended imgui-rs context with a shared font atlas.
    pub fn create_with_shared_font_atlas(shared_font_atlas: SharedFontAtlas) -> Self {
        Self::create_internal(Some(shared_font_atlas))
    }
    /// Attempts to activate this suspended context.
    ///
    /// If there is no active context, this suspended context is activated and `Ok` is returned,
    /// containing the activated context.
    /// If there is already an active context, nothing happens and `Err` is returned, containing
    /// the original suspended context.
    #[doc(alias = "SetCurrentContext")]
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
    fn create_internal(shared_font_atlas: Option<SharedFontAtlas>) -> Self {
        let _guard = CTX_MUTEX.lock();
        let raw = unsafe { sys::igCreateContext(ptr::null_mut()) };
        let ctx = Context {
            raw,
            shared_font_atlas,
            ini_filename: None,
            log_filename: None,
            platform_name: None,
            renderer_name: None,
            clipboard_ctx: Box::new(ClipboardContext::dummy().into()),
            ui: Ui {
                buffer: UnsafeCell::new(crate::string::UiBuffer::new(1024)),
            },
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
    let atlas = SharedFontAtlas::create();
    let suspended1 = SuspendedContext::create_with_shared_font_atlas(atlas.clone());
    let mut ctx2 = Context::create_with_shared_font_atlas(atlas);
    {
        let _borrow = ctx2.fonts();
    }
    let _suspended2 = ctx2.suspend();
    let mut ctx = suspended1.activate().unwrap();
    let _borrow = ctx.fonts();
}

#[test]
fn test_ini_load_save() {
    let (_guard, mut ctx) = crate::test::test_ctx();

    #[cfg(feature = "docking")]
    let data = "[Window][Debug##Default]
Pos=60,60
Size=400,400
Collapsed=0";

    #[cfg(not(feature = "docking"))]
    let data = "[Window][Debug##Default]
Pos=60,60
Size=400,400";

    ctx.load_ini_settings(data);
    let mut buf = String::new();
    ctx.save_ini_settings(&mut buf);
    assert_eq!(data.trim(), buf.trim());
}

#[test]
fn test_default_ini_filename() {
    let _guard = crate::test::TEST_MUTEX.lock();
    let ctx = Context::create();
    assert_eq!(ctx.ini_filename(), Some(PathBuf::from("imgui.ini")));
}

#[test]
fn test_set_ini_filename() {
    let (_guard, mut ctx) = crate::test::test_ctx();
    ctx.set_ini_filename(Some(PathBuf::from("test.ini")));
    assert_eq!(ctx.ini_filename(), Some(PathBuf::from("test.ini")));
}

#[test]
fn test_default_log_filename() {
    let _guard = crate::test::TEST_MUTEX.lock();
    let ctx = Context::create();
    assert_eq!(ctx.log_filename(), Some(PathBuf::from("imgui_log.txt")));
}

#[test]
fn test_set_log_filename() {
    let (_guard, mut ctx) = crate::test::test_ctx();
    ctx.set_log_filename(Some(PathBuf::from("test.log")));
    assert_eq!(ctx.log_filename(), Some(PathBuf::from("test.log")));
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
    #[doc(alias = "GetStyle")]
    pub fn style(&self) -> &Style {
        unsafe {
            // safe because Style is a transparent wrapper around sys::ImGuiStyle
            &*(sys::igGetStyle() as *const Style)
        }
    }
    /// Returns a mutable reference to the user interface style
    #[doc(alias = "GetStyle")]
    pub fn style_mut(&mut self) -> &mut Style {
        unsafe {
            // safe because Style is a transparent wrapper around sys::ImGuiStyle
            &mut *(sys::igGetStyle() as *mut Style)
        }
    }
    /// Returns a mutable reference to the font atlas.
    pub fn fonts(&mut self) -> &mut FontAtlas {
        // we take this with an `&mut Self` here, which means
        // that we can't get the sharedfontatlas through safe code
        // otherwise
        unsafe { &mut *self.io_mut().fonts }
    }

    /// Attempts to clone the interior shared font atlas **if it exists**.
    pub fn clone_shared_font_atlas(&mut self) -> Option<SharedFontAtlas> {
        self.shared_font_atlas.clone()
    }

    /// Starts a new frame. Use [`new_frame`] instead.
    ///
    /// [`new_frame`]: Self::new_frame
    pub fn frame(&mut self) -> &mut Ui {
        self.new_frame()
    }

    /// Starts a new frame and returns an `Ui` instance for constructing a user interface.
    #[doc(alias = "NewFrame")]
    pub fn new_frame(&mut self) -> &mut Ui {
        // Clear default font if it no longer exists. This could be an error in the future
        let default_font = self.io().font_default;
        if !default_font.is_null() && self.fonts().get_font(FontId(default_font)).is_none() {
            self.io_mut().font_default = ptr::null_mut();
        }
        // TODO: precondition checks
        unsafe {
            sys::igNewFrame();
        }

        &mut self.ui
    }

    /// Renders the frame and returns a reference to the resulting draw data.
    ///
    /// This should only be called after calling [`new_frame`].
    ///
    /// [`new_frame`]: Self::new_frame
    #[doc(alias = "Render", alias = "GetDrawData")]
    pub fn render(&mut self) -> &DrawData {
        unsafe {
            sys::igRender();
            &*(sys::igGetDrawData() as *mut DrawData)
        }
    }

    /// Returns the currently desired mouse cursor type.
    ///
    /// This was set *last frame* by the [Ui] object, and will be reset when
    /// [new_frame] is called.
    ///
    /// Returns `None` if no cursor should be displayed
    ///
    /// [new_frame]: Self::new_frame
    #[doc(alias = "GetMouseCursor")]
    pub fn mouse_cursor(&self) -> Option<MouseCursor> {
        match unsafe { sys::igGetMouseCursor() } {
            sys::ImGuiMouseCursor_Arrow => Some(MouseCursor::Arrow),
            sys::ImGuiMouseCursor_TextInput => Some(MouseCursor::TextInput),
            sys::ImGuiMouseCursor_ResizeAll => Some(MouseCursor::ResizeAll),
            sys::ImGuiMouseCursor_ResizeNS => Some(MouseCursor::ResizeNS),
            sys::ImGuiMouseCursor_ResizeEW => Some(MouseCursor::ResizeEW),
            sys::ImGuiMouseCursor_ResizeNESW => Some(MouseCursor::ResizeNESW),
            sys::ImGuiMouseCursor_ResizeNWSE => Some(MouseCursor::ResizeNWSE),
            sys::ImGuiMouseCursor_Hand => Some(MouseCursor::Hand),
            sys::ImGuiMouseCursor_NotAllowed => Some(MouseCursor::NotAllowed),
            _ => None,
        }
    }
}

#[cfg(feature = "docking")]
impl Context {
    /// Returns an immutable reference to the Context's [`PlatformIo`](crate::PlatformIo) object.
    pub fn platform_io(&self) -> &crate::PlatformIo {
        unsafe {
            // safe because PlatformIo is a transparent wrapper around sys::ImGuiPlatformIO
            // and &self ensures we have shared ownership of PlatformIo.
            &*(sys::igGetPlatformIO() as *const crate::PlatformIo)
        }
    }
    /// Returns a mutable reference to the Context's [`PlatformIo`](crate::PlatformIo) object.
    pub fn platform_io_mut(&mut self) -> &mut crate::PlatformIo {
        unsafe {
            // safe because PlatformIo is a transparent wrapper around sys::ImGuiPlatformIO
            // and &mut self ensures exclusive ownership of PlatformIo.
            &mut *(sys::igGetPlatformIO() as *mut crate::PlatformIo)
        }
    }

    /// Returns an immutable reference to the main [`Viewport`](crate::Viewport)
    pub fn main_viewport(&self) -> &crate::Viewport {
        unsafe {
            // safe because Viewport is a transparent wrapper around sys::ImGuiViewport
            // and &self ensures we have shared ownership.
            &*(sys::igGetMainViewport() as *mut crate::Viewport)
        }
    }
    /// Returns a mutable reference to the main [`Viewport`](crate::Viewport)
    pub fn main_viewport_mut(&mut self) -> &mut crate::Viewport {
        unsafe {
            // safe because Viewport is a transparent wrapper around sys::ImGuiViewport
            // and &mut self ensures we have exclusive ownership.
            &mut *(sys::igGetMainViewport() as *mut crate::Viewport)
        }
    }
    /// Tries to find and return a Viewport identified by `id`.
    ///
    /// # Returns
    /// A [`Viewport`](crate::Viewport) with the given `id`
    /// or `None` if no [`Viewport`](crate::Viewport) exists.
    pub fn viewport_by_id(&self, id: crate::Id) -> Option<&crate::Viewport> {
        unsafe {
            // safe because Viewport is a transparent wrapper around sys::ImGuiViewport
            // and &self ensures shared ownership.
            (sys::igFindViewportByID(id.0) as *const crate::Viewport).as_ref()
        }
    }
    /// Tries to find and return a Viewport identified by `id`.
    ///
    /// # Returns
    /// A [`Viewport`](crate::Viewport) with the given `id`
    /// or `None` if no [`Viewport`](crate::Viewport) exists.
    pub fn viewport_by_id_mut(&mut self, id: crate::Id) -> Option<&mut crate::Viewport> {
        unsafe {
            // safe because Viewport is a transparent wrapper around sys::ImGuiViewport
            // and &mut self ensures exclusive ownership.
            (sys::igFindViewportByID(id.0) as *mut crate::Viewport).as_mut()
        }
    }
    /// Returns an iterator containing every [`Viewport`](crate::Viewport) that currently exists.
    pub fn viewports(&self) -> impl Iterator<Item = &crate::Viewport> {
        let slice = self.platform_io().viewports.as_slice();
        // safe because &self ensures shared ownership
        unsafe { slice.iter().map(|ptr| &**ptr) }
    }
    /// Returns an iterator containing every [`Viewport`](crate::Viewport) that currently exists.
    pub fn viewports_mut(&mut self) -> impl Iterator<Item = &mut crate::Viewport> {
        let slice = self.platform_io_mut().viewports.as_slice();
        // safe because &mut self ensures exclusive ownership
        unsafe { slice.iter().map(|ptr| &mut **ptr) }
    }

    /// Installs a [`PlatformViewportBackend`](crate::PlatformViewportBackend) that is used to
    /// create platform windows on demand if a window is dragged outside of the main viewport.
    pub fn set_platform_backend<T: crate::PlatformViewportBackend>(&mut self, backend: T) {
        let ctx = crate::PlatformViewportContext {
            backend: Box::new(backend),
        };

        crate::PLATFORM_VIEWPORT_CONTEXT.with_borrow_mut(|c| *c = Some(ctx));

        let pio = self.platform_io_mut();
        pio.platform_create_window = Some(docking_utils::platform_create_window);
        pio.platform_destroy_window = Some(docking_utils::platform_destroy_window);
        pio.platform_show_window = Some(docking_utils::platform_show_window);
        pio.platform_set_window_pos = Some(docking_utils::platform_set_window_pos);
        // since pio.platform_get_window_pos is not a C compatible function, cimgui provides an extra function to set it.
        unsafe {
            docking_utils::ImGuiPlatformIO_Set_Platform_GetWindowPos(
                pio,
                docking_utils::platform_get_window_pos,
            );
        }
        pio.platform_set_window_size = Some(docking_utils::platform_set_window_size);
        // since pio.platform_get_window_size is not a C compatible function, cimgui provides an extra function to set it.
        unsafe {
            docking_utils::ImGuiPlatformIO_Set_Platform_GetWindowSize(
                pio,
                docking_utils::platform_get_window_size,
            );
        }
        pio.platform_set_window_focus = Some(docking_utils::platform_set_window_focus);
        pio.platform_get_window_focus = Some(docking_utils::platform_get_window_focus);
        pio.platform_get_window_minimized = Some(docking_utils::platform_get_window_minimized);
        pio.platform_set_window_title = Some(docking_utils::platform_set_window_title);
        pio.platform_set_window_alpha = Some(docking_utils::platform_set_window_alpha);
        pio.platform_update_window = Some(docking_utils::platform_update_window);
        pio.platform_render_window = Some(docking_utils::platform_render_window);
        pio.platform_swap_buffers = Some(docking_utils::platform_swap_buffers);
        pio.platform_create_vk_surface = Some(docking_utils::platform_create_vk_surface);
    }
    /// Installs a [`RendererViewportBackend`](crate::RendererViewportBackend) that is used to
    /// render extra viewports created by ImGui.
    pub fn set_renderer_backend<T: crate::RendererViewportBackend>(&mut self, backend: T) {
        let ctx = crate::RendererViewportContext {
            backend: Box::new(backend),
        };

        crate::RENDERER_VIEWPORT_CONTEXT.with_borrow_mut(|c| *c = Some(ctx));

        let pio = self.platform_io_mut();
        pio.renderer_create_window = Some(docking_utils::renderer_create_window);
        pio.renderer_destroy_window = Some(docking_utils::renderer_destroy_window);
        pio.renderer_set_window_size = Some(docking_utils::renderer_set_window_size);
        pio.renderer_render_window = Some(docking_utils::renderer_render_window);
        pio.renderer_swap_buffers = Some(docking_utils::renderer_swap_buffers);
    }
    /// Updates the extra Viewports created by ImGui.
    /// Has to be called every frame if Viewports are enabled.
    pub fn update_platform_windows(&mut self) {
        unsafe {
            sys::igUpdatePlatformWindows();
        }
    }
    /// Basically just calls the [`PlatformViewportBackend`](crate::PlatformViewportBackend) and [`RendererViewportBackend`](crate::RendererViewportBackend)
    /// functions. If you render your extra viewports manually this function is not needed at all.
    pub fn render_platform_windows_default(&mut self) {
        unsafe {
            sys::igRenderPlatformWindowsDefault(std::ptr::null_mut(), std::ptr::null_mut());
        }
    }
}
