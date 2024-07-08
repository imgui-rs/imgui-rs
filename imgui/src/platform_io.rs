use std::{
    ffi::{c_void, CStr},
    os::raw::{c_char, c_int},
};

use crate::{
    internal::{ImVector, RawCast},
    Io, ViewportFlags,
};

/// Holds the information needed to enable multiple viewports.
#[repr(C)]
pub struct PlatformIo {
    pub(crate) platform_create_window: Option<unsafe extern "C" fn(*mut Viewport)>,
    pub(crate) platform_destroy_window: Option<unsafe extern "C" fn(*mut Viewport)>,
    pub(crate) platform_show_window: Option<unsafe extern "C" fn(*mut Viewport)>,
    pub(crate) platform_set_window_pos: Option<unsafe extern "C" fn(*mut Viewport, sys::ImVec2)>,
    pub(crate) platform_get_window_pos: Option<unsafe extern "C" fn(*mut Viewport) -> sys::ImVec2>,
    pub(crate) platform_set_window_size: Option<unsafe extern "C" fn(*mut Viewport, sys::ImVec2)>,
    pub(crate) platform_get_window_size: Option<unsafe extern "C" fn(*mut Viewport) -> sys::ImVec2>,
    pub(crate) platform_set_window_focus: Option<unsafe extern "C" fn(*mut Viewport)>,
    pub(crate) platform_get_window_focus: Option<unsafe extern "C" fn(*mut Viewport) -> bool>,
    pub(crate) platform_get_window_minimized: Option<unsafe extern "C" fn(*mut Viewport) -> bool>,
    pub(crate) platform_set_window_title:
        Option<unsafe extern "C" fn(*mut Viewport, *const c_char)>,
    pub(crate) platform_set_window_alpha: Option<unsafe extern "C" fn(*mut Viewport, f32)>,
    pub(crate) platform_update_window: Option<unsafe extern "C" fn(*mut Viewport)>,
    pub(crate) platform_render_window: Option<unsafe extern "C" fn(*mut Viewport, *mut c_void)>,
    pub(crate) platform_swap_buffers: Option<unsafe extern "C" fn(*mut Viewport, *mut c_void)>,
    pub(crate) platform_get_window_dpi_scale: Option<unsafe extern "C" fn(*mut Viewport) -> f32>,
    pub(crate) platform_on_changed_viewport: Option<unsafe extern "C" fn(*mut Viewport)>,
    pub(crate) platform_create_vk_surface:
        Option<unsafe extern "C" fn(*mut Viewport, u64, *const c_void, *mut u64) -> c_int>,

    pub(crate) renderer_create_window: Option<unsafe extern "C" fn(*mut Viewport)>,
    pub(crate) renderer_destroy_window: Option<unsafe extern "C" fn(*mut Viewport)>,
    pub(crate) renderer_set_window_size: Option<unsafe extern "C" fn(*mut Viewport, sys::ImVec2)>,
    pub(crate) renderer_render_window: Option<unsafe extern "C" fn(*mut Viewport, *mut c_void)>,
    pub(crate) renderer_swap_buffers: Option<unsafe extern "C" fn(*mut Viewport, *mut c_void)>,

    /// Holds information about the available monitors.
    /// Should be initialized and updated by the [`PlatformViewportBackend`].
    pub monitors: ImVector<PlatformMonitor>,

    pub(crate) viewports: ImVector<*mut Viewport>,
}

unsafe impl RawCast<sys::ImGuiPlatformIO> for PlatformIo {}

#[test]
#[cfg(test)]
fn test_platform_io_memory_layout() {
    use std::mem;
    assert_eq!(
        mem::size_of::<PlatformIo>(),
        mem::size_of::<sys::ImGuiPlatformIO>()
    );
    assert_eq!(
        mem::align_of::<PlatformIo>(),
        mem::align_of::<sys::ImGuiPlatformIO>()
    );
    use sys::ImGuiPlatformIO;
    macro_rules! assert_field_offset {
        ($l:ident, $r:ident) => {
            assert_eq!(
                memoffset::offset_of!(PlatformIo, $l),
                memoffset::offset_of!(ImGuiPlatformIO, $r)
            );
        };
    }

    assert_field_offset!(platform_create_window, Platform_CreateWindow);
    assert_field_offset!(platform_destroy_window, Platform_DestroyWindow);
    assert_field_offset!(platform_show_window, Platform_ShowWindow);
    assert_field_offset!(platform_set_window_pos, Platform_SetWindowPos);
    assert_field_offset!(platform_get_window_pos, Platform_GetWindowPos);
    assert_field_offset!(platform_set_window_size, Platform_SetWindowSize);
    assert_field_offset!(platform_get_window_size, Platform_GetWindowSize);
    assert_field_offset!(platform_set_window_focus, Platform_SetWindowFocus);
    assert_field_offset!(platform_get_window_focus, Platform_GetWindowFocus);
    assert_field_offset!(platform_get_window_minimized, Platform_GetWindowMinimized);
    assert_field_offset!(platform_set_window_title, Platform_SetWindowTitle);
    assert_field_offset!(platform_set_window_alpha, Platform_SetWindowAlpha);
    assert_field_offset!(platform_update_window, Platform_UpdateWindow);
    assert_field_offset!(platform_render_window, Platform_RenderWindow);
    assert_field_offset!(platform_swap_buffers, Platform_SwapBuffers);
    assert_field_offset!(platform_get_window_dpi_scale, Platform_GetWindowDpiScale);
    assert_field_offset!(platform_on_changed_viewport, Platform_OnChangedViewport);
    assert_field_offset!(platform_create_vk_surface, Platform_CreateVkSurface);

    assert_field_offset!(renderer_create_window, Renderer_CreateWindow);
    assert_field_offset!(renderer_destroy_window, Renderer_DestroyWindow);
    assert_field_offset!(renderer_set_window_size, Renderer_SetWindowSize);
    assert_field_offset!(renderer_render_window, Renderer_RenderWindow);
    assert_field_offset!(renderer_swap_buffers, Renderer_SwapBuffers);

    assert_field_offset!(monitors, Monitors);
    assert_field_offset!(viewports, Viewports);
}

/// Trait holding functions needed when the platform integration supports viewports.
///
/// Register it via [`Context::set_platform_backend()`](crate::context::Context::set_platform_backend())
pub trait PlatformViewportBackend: 'static {
    /// Called by imgui when a new [`Viewport`] is created.
    ///
    /// # Notes
    /// This function should initiate the creation of a platform window.
    /// The window should be invisible.
    fn create_window(&mut self, viewport: &mut Viewport);
    /// Called by imgui when a [`Viewport`] is about to be destroyed.
    ///
    /// # Notes
    /// This function should initiate the destruction of the platform window.
    fn destroy_window(&mut self, viewport: &mut Viewport);
    /// Called by imgui to make a [`Viewport`] visible.
    fn show_window(&mut self, viewport: &mut Viewport);
    /// Called by imgui to reposition a [`Viewport`].
    ///
    /// # Notes
    /// `pos` specifies the position of the windows content area (excluding title bar etc.)
    fn set_window_pos(&mut self, viewport: &mut Viewport, pos: [f32; 2]);
    /// Called by imgui to get the position of a [`Viewport`].
    ///
    /// # Notes
    /// You should return the position of the window's content area (excluding title bar etc.)
    fn get_window_pos(&mut self, viewport: &mut Viewport) -> [f32; 2];
    /// Called by imgui to set the size of a [`Viewport`].
    ///
    /// # Notes
    /// `size` specifies the size of the window's content area (excluding title bar etc.)
    fn set_window_size(&mut self, viewport: &mut Viewport, size: [f32; 2]);
    /// Called by imgui to get the size of a [`Viewport`].
    ///
    /// # Notes
    /// you should return the size of the window's content area (excluding title bar etc.)
    fn get_window_size(&mut self, viewport: &mut Viewport) -> [f32; 2];
    /// Called by imgui to make a [`Viewport`] steal the focus.
    fn set_window_focus(&mut self, viewport: &mut Viewport);
    /// Called by imgui to query whether a [`Viewport`] is in focus.
    fn get_window_focus(&mut self, viewport: &mut Viewport) -> bool;
    /// Called by imgui to query whether a [`Viewport`] is minimized.
    fn get_window_minimized(&mut self, viewport: &mut Viewport) -> bool;
    /// Called by imgui to set a [`Viewport`] title.
    fn set_window_title(&mut self, viewport: &mut Viewport, title: &str);
    /// Called by imgui to set the opacity of an entire [`Viewport`].
    ///
    /// If your backend does not support opactiy, it is safe to just do nothing in this function.
    fn set_window_alpha(&mut self, viewport: &mut Viewport, alpha: f32);
    fn update_window(&mut self, viewport: &mut Viewport);
    fn render_window(&mut self, viewport: &mut Viewport);
    fn swap_buffers(&mut self, viewport: &mut Viewport);
    fn create_vk_surface(
        &mut self,
        viewport: &mut Viewport,
        instance: u64,
        out_surface: &mut u64,
    ) -> i32;
}

/// Used to get the current Contexts [`PlatformViewportContext`].
fn get_platform_ctx() -> &'static mut PlatformViewportContext {
    unsafe {
        // should be safe as it is impossible to call any imgui function on a non-active context.
        &mut *((*(sys::igGetIO() as *const Io)).backend_platform_user_data
            as *mut PlatformViewportContext)
    }
}

/// Used to get the current Contexts [`RendererViewportContext`].
fn get_renderer_ctx() -> &'static mut RendererViewportContext {
    unsafe {
        // should be safe as it is impossible to call any imgui function on a non-active context.
        &mut *((*(sys::igGetIO() as *const Io)).backend_renderer_user_data
            as *mut RendererViewportContext)
    }
}

pub(crate) extern "C" fn platform_create_window(viewport: *mut Viewport) {
    let ctx = get_platform_ctx();
    ctx.backend.create_window(unsafe { &mut *viewport });
}
pub(crate) extern "C" fn platform_destroy_window(viewport: *mut Viewport) {
    let ctx = get_platform_ctx();
    ctx.backend.destroy_window(unsafe { &mut *viewport });
}
pub(crate) extern "C" fn platform_show_window(viewport: *mut Viewport) {
    let ctx = get_platform_ctx();
    ctx.backend.show_window(unsafe { &mut *viewport });
}
pub(crate) extern "C" fn platform_set_window_pos(viewport: *mut Viewport, pos: sys::ImVec2) {
    let ctx = get_platform_ctx();
    ctx.backend
        .set_window_pos(unsafe { &mut *viewport }, [pos.x, pos.y]);
}
pub(crate) extern "C" fn platform_get_window_pos(
    viewport: *mut Viewport,
    out_pos: *mut sys::ImVec2,
) {
    let ctx = get_platform_ctx();
    let pos = ctx.backend.get_window_pos(unsafe { &mut *viewport });
    unsafe {
        *out_pos = sys::ImVec2::new(pos[0], pos[1]);
    }
}
pub(crate) extern "C" fn platform_set_window_size(viewport: *mut Viewport, size: sys::ImVec2) {
    let ctx = get_platform_ctx();
    ctx.backend
        .set_window_size(unsafe { &mut *viewport }, [size.x, size.y]);
}
pub(crate) extern "C" fn platform_get_window_size(
    viewport: *mut Viewport,
    out_size: *mut sys::ImVec2,
) {
    let ctx = get_platform_ctx();
    let size = ctx.backend.get_window_size(unsafe { &mut *viewport });
    unsafe {
        *out_size = sys::ImVec2::new(size[0], size[1]);
    }
}
pub(crate) extern "C" fn platform_set_window_focus(viewport: *mut Viewport) {
    let ctx = get_platform_ctx();
    ctx.backend.set_window_focus(unsafe { &mut *viewport });
}
pub(crate) extern "C" fn platform_get_window_focus(viewport: *mut Viewport) -> bool {
    let ctx = get_platform_ctx();
    ctx.backend.get_window_focus(unsafe { &mut *viewport })
}
pub(crate) extern "C" fn platform_get_window_minimized(viewport: *mut Viewport) -> bool {
    let ctx = get_platform_ctx();
    ctx.backend.get_window_minimized(unsafe { &mut *viewport })
}
pub(crate) extern "C" fn platform_set_window_title(viewport: *mut Viewport, title: *const c_char) {
    let ctx = get_platform_ctx();
    let title = unsafe { CStr::from_ptr(title).to_str().unwrap() };
    ctx.backend
        .set_window_title(unsafe { &mut *viewport }, title);
}
pub(crate) extern "C" fn platform_set_window_alpha(viewport: *mut Viewport, alpha: f32) {
    let ctx = get_platform_ctx();
    ctx.backend
        .set_window_alpha(unsafe { &mut *viewport }, alpha);
}
pub(crate) extern "C" fn platform_update_window(viewport: *mut Viewport) {
    let ctx = get_platform_ctx();
    ctx.backend.update_window(unsafe { &mut *viewport });
}
pub(crate) extern "C" fn platform_render_window(viewport: *mut Viewport, _arg: *mut c_void) {
    let ctx = get_platform_ctx();
    ctx.backend.render_window(unsafe { &mut *viewport });
}
pub(crate) extern "C" fn platform_swap_buffers(viewport: *mut Viewport, _arg: *mut c_void) {
    let ctx = get_platform_ctx();
    ctx.backend.swap_buffers(unsafe { &mut *viewport });
}
pub(crate) extern "C" fn platform_create_vk_surface(
    viewport: *mut Viewport,
    instance: u64,
    _arg: *const c_void,
    out_surface: *mut u64,
) -> c_int {
    let ctx = get_platform_ctx();
    ctx.backend
        .create_vk_surface(unsafe { &mut *viewport }, instance, unsafe {
            &mut *out_surface
        })
}

/// The default [`PlatformViewportBackend`], does nothing.
pub(crate) struct DummyPlatformViewportBackend {}
impl PlatformViewportBackend for DummyPlatformViewportBackend {
    fn create_window(&mut self, _viewport: &mut Viewport) {
        unimplemented!()
    }

    fn destroy_window(&mut self, _viewport: &mut Viewport) {
        unimplemented!()
    }

    fn show_window(&mut self, _viewport: &mut Viewport) {
        unimplemented!()
    }

    fn set_window_pos(&mut self, _viewport: &mut Viewport, _pos: [f32; 2]) {
        unimplemented!()
    }

    fn get_window_pos(&mut self, _viewport: &mut Viewport) -> [f32; 2] {
        unimplemented!()
    }

    fn set_window_size(&mut self, _viewport: &mut Viewport, _size: [f32; 2]) {
        unimplemented!()
    }

    fn get_window_size(&mut self, _viewport: &mut Viewport) -> [f32; 2] {
        unimplemented!()
    }

    fn set_window_focus(&mut self, _viewport: &mut Viewport) {
        unimplemented!()
    }

    fn get_window_focus(&mut self, _viewport: &mut Viewport) -> bool {
        unimplemented!()
    }

    fn get_window_minimized(&mut self, _viewport: &mut Viewport) -> bool {
        unimplemented!()
    }

    fn set_window_title(&mut self, _viewport: &mut Viewport, _title: &str) {
        unimplemented!()
    }

    fn set_window_alpha(&mut self, _viewport: &mut Viewport, _alpha: f32) {
        unimplemented!()
    }

    fn update_window(&mut self, _viewport: &mut Viewport) {
        unimplemented!()
    }

    fn render_window(&mut self, _viewport: &mut Viewport) {
        unimplemented!()
    }

    fn swap_buffers(&mut self, _viewport: &mut Viewport) {
        unimplemented!()
    }

    fn create_vk_surface(
        &mut self,
        _viewport: &mut Viewport,
        _instance: u64,
        _out_surface: &mut u64,
    ) -> i32 {
        unimplemented!()
    }
}

/// Just holds a [`PlatformViewportBackend`].
pub(crate) struct PlatformViewportContext {
    pub(crate) backend: Box<dyn PlatformViewportBackend>,
}

impl PlatformViewportContext {
    pub(crate) fn dummy() -> Self {
        Self {
            backend: Box::new(DummyPlatformViewportBackend {}),
        }
    }
}

/// Trait that holds optional functions for a rendering backend to support multiple viewports.
///
/// It is completely fine to not use this Backend at all, as all functions are optional.
pub trait RendererViewportBackend: 'static {
    /// Called after [`PlatformViewportBackend::create_window()`].
    fn create_window(&mut self, viewport: &mut Viewport);
    /// Called before [`PlatformViewportBackend::destroy_window()`].
    fn destroy_window(&mut self, viewport: &mut Viewport);
    /// Called after [`PlatformViewportBackend::set_window_size()`].
    fn set_window_size(&mut self, viewport: &mut Viewport, size: [f32; 2]);
    fn render_window(&mut self, viewport: &mut Viewport);
    fn swap_buffers(&mut self, viewport: &mut Viewport);
}

pub(crate) extern "C" fn renderer_create_window(viewport: *mut Viewport) {
    let ctx = get_renderer_ctx();
    ctx.backend.create_window(unsafe { &mut *viewport });
}
pub(crate) extern "C" fn renderer_destroy_window(viewport: *mut Viewport) {
    let ctx = get_renderer_ctx();
    ctx.backend.destroy_window(unsafe { &mut *viewport });
}
pub(crate) extern "C" fn renderer_set_window_size(viewport: *mut Viewport, size: sys::ImVec2) {
    let ctx = get_renderer_ctx();
    ctx.backend
        .set_window_size(unsafe { &mut *viewport }, [size.x, size.y]);
}
pub(crate) extern "C" fn renderer_render_window(viewport: *mut Viewport, _arg: *mut c_void) {
    let ctx = get_renderer_ctx();
    ctx.backend.render_window(unsafe { &mut *viewport });
}
pub(crate) extern "C" fn renderer_swap_buffers(viewport: *mut Viewport, _arg: *mut c_void) {
    let ctx = get_renderer_ctx();
    ctx.backend.swap_buffers(unsafe { &mut *viewport });
}

/// The default [`RendererViewportBackend`], does nothing.
pub(crate) struct DummyRendererViewportBackend {}
impl RendererViewportBackend for DummyRendererViewportBackend {
    fn create_window(&mut self, _viewport: &mut Viewport) {
        unimplemented!()
    }

    fn destroy_window(&mut self, _viewport: &mut Viewport) {
        unimplemented!()
    }

    fn set_window_size(&mut self, _viewport: &mut Viewport, _size: [f32; 2]) {
        unimplemented!()
    }

    fn render_window(&mut self, _viewport: &mut Viewport) {
        unimplemented!()
    }

    fn swap_buffers(&mut self, _viewport: &mut Viewport) {
        unimplemented!()
    }
}

/// Just holds a [`RendererViewportBackend`].
pub(crate) struct RendererViewportContext {
    pub(crate) backend: Box<dyn RendererViewportBackend>,
}

impl RendererViewportContext {
    pub(crate) fn dummy() -> Self {
        Self {
            backend: Box::new(DummyRendererViewportBackend {}),
        }
    }
}

/// Describes an ImGui Viewport.
#[repr(C)]
pub struct Viewport {
    /// The unique ID of this Viewport.
    pub id: crate::Id,
    /// Flags that describe how the Viewport should behave.
    pub flags: ViewportFlags,
    pub pos: [f32; 2],
    pub size: [f32; 2],
    pub work_pos: [f32; 2],
    pub work_size: [f32; 2],
    pub dpi_scale: f32,
    pub(crate) parent_viewport_id: crate::Id,
    pub(crate) draw_data: *mut crate::DrawData,

    pub renderer_user_data: *mut c_void,
    pub platform_user_data: *mut c_void,
    pub platform_handle: *mut c_void,
    pub platform_handle_raw: *mut c_void,
    pub platform_window_created: bool,
    pub platform_request_move: bool,
    pub platform_request_resize: bool,
    pub platform_request_close: bool,
}

impl Viewport {
    /// Returns the draw data of the respective Viewport.
    pub fn draw_data(&self) -> &crate::DrawData {
        unsafe { &*self.draw_data }
    }
}

#[test]
#[cfg(test)]
fn test_viewport_memory_layout() {
    use std::mem;
    assert_eq!(
        mem::size_of::<Viewport>(),
        mem::size_of::<sys::ImGuiViewport>()
    );
    assert_eq!(
        mem::align_of::<Viewport>(),
        mem::align_of::<sys::ImGuiViewport>()
    );
    use sys::ImGuiViewport;
    macro_rules! assert_field_offset {
        ($l:ident, $r:ident) => {
            assert_eq!(
                memoffset::offset_of!(Viewport, $l),
                memoffset::offset_of!(ImGuiViewport, $r)
            );
        };
    }

    assert_field_offset!(id, ID);
    assert_field_offset!(flags, Flags);
    assert_field_offset!(pos, Pos);
    assert_field_offset!(size, Size);
    assert_field_offset!(work_pos, WorkPos);
    assert_field_offset!(work_size, WorkSize);
    assert_field_offset!(dpi_scale, DpiScale);
    assert_field_offset!(parent_viewport_id, ParentViewportId);
    assert_field_offset!(draw_data, DrawData);

    assert_field_offset!(renderer_user_data, RendererUserData);
    assert_field_offset!(platform_user_data, PlatformUserData);
    assert_field_offset!(platform_handle, PlatformHandle);
    assert_field_offset!(platform_handle_raw, PlatformHandleRaw);
    assert_field_offset!(platform_window_created, PlatformWindowCreated);
    assert_field_offset!(platform_request_move, PlatformRequestMove);
    assert_field_offset!(platform_request_resize, PlatformRequestResize);
    assert_field_offset!(platform_request_close, PlatformRequestClose);
}

/// Describes a monitor that can be used by ImGui.
#[repr(C)]
pub struct PlatformMonitor {
    /// Position of the monitor on the virtual desktop.
    pub main_pos: [f32; 2],
    /// Size of the monitor on the virtual desktop.
    pub main_size: [f32; 2],
    /// Working position of the monitor, should exclude task bar etc.
    ///
    /// Set to `main_pos` if not known.
    pub work_pos: [f32; 2],
    /// Working size of the monitor, should exclude task bar etc.
    ///
    /// Set to `work_size` if not known.
    pub work_size: [f32; 2],
    pub dpi_scale: f32,
    pub platform_handle: *mut c_void,
}

#[test]
#[cfg(test)]
fn test_platform_monitor_memory_layout() {
    use std::mem;
    assert_eq!(
        mem::size_of::<PlatformMonitor>(),
        mem::size_of::<sys::ImGuiPlatformMonitor>()
    );
    assert_eq!(
        mem::align_of::<PlatformMonitor>(),
        mem::align_of::<sys::ImGuiPlatformMonitor>()
    );
    use sys::ImGuiPlatformMonitor;
    macro_rules! assert_field_offset {
        ($l:ident, $r:ident) => {
            assert_eq!(
                memoffset::offset_of!(PlatformMonitor, $l),
                memoffset::offset_of!(ImGuiPlatformMonitor, $r)
            );
        };
    }

    assert_field_offset!(main_pos, MainPos);
    assert_field_offset!(main_size, MainSize);
    assert_field_offset!(work_pos, WorkPos);
    assert_field_offset!(work_size, WorkSize);
    assert_field_offset!(dpi_scale, DpiScale);
    assert_field_offset!(platform_handle, PlatformHandle);
}

extern "C" {
    pub(crate) fn ImGuiPlatformIO_Set_Platform_GetWindowPos(
        pio: *mut PlatformIo,
        func: extern "C" fn(*mut Viewport, *mut sys::ImVec2),
    );
    pub(crate) fn ImGuiPlatformIO_Set_Platform_GetWindowSize(
        pio: *mut PlatformIo,
        func: extern "C" fn(*mut Viewport, *mut sys::ImVec2),
    );
}
