use std::ffi::{c_char, c_void};

use crate::{internal::RawCast, ViewportFlags};

/// Holds the information needed to enable multiple viewports.
#[repr(C)]
pub struct PlatformIo {
    pub(crate) get_clipboard_text_fn:
        Option<unsafe extern "C" fn(*mut sys::ImGuiContext) -> *const c_char>,

    pub(crate) set_clipboard_text_fn:
        Option<unsafe extern "C" fn(*mut sys::ImGuiContext, *const c_char)>,

    pub(crate) clipboard_user_data: *mut c_void,

    pub(crate) open_in_shell_fn:
        Option<unsafe extern "C" fn(ctx: *mut sys::ImGuiContext, path: *const c_char) -> bool>,
    pub(crate) open_in_shell_user_data: *mut c_void,
    pub(crate) set_ime_data_fn: Option<
        unsafe extern "C" fn(
            ctx: *mut sys::ImGuiContext,
            viewport: *mut sys::ImGuiViewport,
            data: *mut sys::ImGuiPlatformImeData,
        ),
    >,
    pub(crate) ime_user_data: *mut c_void,
    pub(crate) locale_decimal_point: sys::ImWchar,

    #[cfg(feature = "docking")]
    pub(crate) platform_create_window: Option<unsafe extern "C" fn(*mut Viewport)>,
    #[cfg(feature = "docking")]
    pub(crate) platform_destroy_window: Option<unsafe extern "C" fn(*mut Viewport)>,
    #[cfg(feature = "docking")]
    pub(crate) platform_show_window: Option<unsafe extern "C" fn(*mut Viewport)>,
    #[cfg(feature = "docking")]
    pub(crate) platform_set_window_pos: Option<unsafe extern "C" fn(*mut Viewport, sys::ImVec2)>,
    #[cfg(feature = "docking")]
    pub(crate) platform_get_window_pos: Option<unsafe extern "C" fn(*mut Viewport) -> sys::ImVec2>,
    #[cfg(feature = "docking")]
    pub(crate) platform_set_window_size: Option<unsafe extern "C" fn(*mut Viewport, sys::ImVec2)>,
    #[cfg(feature = "docking")]
    pub(crate) platform_get_window_size: Option<unsafe extern "C" fn(*mut Viewport) -> sys::ImVec2>,
    #[cfg(feature = "docking")]
    pub(crate) platform_set_window_focus: Option<unsafe extern "C" fn(*mut Viewport)>,
    #[cfg(feature = "docking")]
    pub(crate) platform_get_window_focus: Option<unsafe extern "C" fn(*mut Viewport) -> bool>,
    #[cfg(feature = "docking")]
    pub(crate) platform_get_window_minimized: Option<unsafe extern "C" fn(*mut Viewport) -> bool>,
    #[cfg(feature = "docking")]
    pub(crate) platform_set_window_title:
        Option<unsafe extern "C" fn(*mut Viewport, *const c_char)>,
    #[cfg(feature = "docking")]
    pub(crate) platform_set_window_alpha: Option<unsafe extern "C" fn(*mut Viewport, f32)>,
    #[cfg(feature = "docking")]
    pub(crate) platform_update_window: Option<unsafe extern "C" fn(*mut Viewport)>,
    #[cfg(feature = "docking")]
    pub(crate) platform_render_window: Option<unsafe extern "C" fn(*mut Viewport, *mut c_void)>,
    #[cfg(feature = "docking")]
    pub(crate) platform_swap_buffers: Option<unsafe extern "C" fn(*mut Viewport, *mut c_void)>,
    #[cfg(feature = "docking")]
    pub(crate) platform_get_window_dpi_scale: Option<unsafe extern "C" fn(*mut Viewport) -> f32>,
    #[cfg(feature = "docking")]
    pub(crate) platform_on_changed_viewport: Option<unsafe extern "C" fn(*mut Viewport)>,
    #[cfg(feature = "docking")]
    pub(crate) platform_create_vk_surface:
        Option<unsafe extern "C" fn(*mut Viewport, u64, *const c_void, *mut u64) -> c_int>,

    #[cfg(feature = "docking")]
    pub(crate) renderer_create_window: Option<unsafe extern "C" fn(*mut Viewport)>,
    #[cfg(feature = "docking")]
    pub(crate) renderer_destroy_window: Option<unsafe extern "C" fn(*mut Viewport)>,
    #[cfg(feature = "docking")]
    pub(crate) renderer_set_window_size: Option<unsafe extern "C" fn(*mut Viewport, sys::ImVec2)>,
    #[cfg(feature = "docking")]
    pub(crate) renderer_render_window: Option<unsafe extern "C" fn(*mut Viewport, *mut c_void)>,
    #[cfg(feature = "docking")]
    pub(crate) renderer_swap_buffers: Option<unsafe extern "C" fn(*mut Viewport, *mut c_void)>,

    /// Holds information about the available monitors.
    /// Should be initialized and updated by the [`PlatformViewportBackend`].
    #[cfg(feature = "docking")]
    pub monitors: ImVector<PlatformMonitor>,

    #[cfg(feature = "docking")]
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

    assert_field_offset!(get_clipboard_text_fn, Platform_GetClipboardTextFn);
    assert_field_offset!(set_clipboard_text_fn, Platform_SetClipboardTextFn);
    assert_field_offset!(clipboard_user_data, Platform_ClipboardUserData);
    assert_field_offset!(open_in_shell_fn, Platform_OpenInShellFn);
    assert_field_offset!(open_in_shell_user_data, Platform_OpenInShellUserData);
    assert_field_offset!(set_ime_data_fn, Platform_SetImeDataFn);
    assert_field_offset!(ime_user_data, Platform_ImeUserData);
    assert_field_offset!(locale_decimal_point, Platform_LocaleDecimalPoint);

    #[cfg(feature = "docking")]
    {
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
    pub platform_handle: *mut c_void,
    pub platform_handle_raw: *mut c_void,

    #[cfg(feature = "docking")]
    pub dpi_scale: f32,
    #[cfg(feature = "docking")]
    pub(crate) parent_viewport_id: crate::Id,
    #[cfg(feature = "docking")]
    pub(crate) draw_data: *mut crate::DrawData,

    #[cfg(feature = "docking")]
    pub renderer_user_data: *mut c_void,
    #[cfg(feature = "docking")]
    pub platform_user_data: *mut c_void,
    #[cfg(feature = "docking")]
    pub platform_window_created: bool,
    #[cfg(feature = "docking")]
    pub platform_request_move: bool,
    #[cfg(feature = "docking")]
    pub platform_request_resize: bool,
    #[cfg(feature = "docking")]
    pub platform_request_close: bool,
}

#[cfg(feature = "docking")]
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
    assert_field_offset!(platform_handle, PlatformHandle);
    assert_field_offset!(platform_handle_raw, PlatformHandleRaw);

    #[cfg(feature = "docking")]
    {
        assert_field_offset!(dpi_scale, DpiScale);
        assert_field_offset!(parent_viewport_id, ParentViewportId);
        assert_field_offset!(draw_data, DrawData);

        assert_field_offset!(renderer_user_data, RendererUserData);
        assert_field_offset!(platform_user_data, PlatformUserData);
        assert_field_offset!(platform_window_created, PlatformWindowCreated);
        assert_field_offset!(platform_request_move, PlatformRequestMove);
        assert_field_offset!(platform_request_resize, PlatformRequestResize);
        assert_field_offset!(platform_request_close, PlatformRequestClose);
    }
}
