use std::fmt;
use std::panic::catch_unwind;
use std::process;

/// Trait for IME data backends
pub trait ImeDataBackend: 'static {
    /// Callback to start/stop text input and notify OS of the text input rect
    fn set_ime_data(&mut self, viewport: &mut crate::Viewport, data: PlatformImeData);
}

/// IME data passed to the [ImeDataContext] callback
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PlatformImeData {
    pub want_visible: bool,
    pub input_pos: [f32; 2],
    pub input_line_height: f32,
}

pub(crate) struct ImeDataContext {
    backend: Box<dyn ImeDataBackend>,
}

impl ImeDataContext {
    /// Creates a new [ImeDataContext]
    pub(crate) fn new<T: ImeDataBackend>(backend: T) -> ImeDataContext {
        ImeDataContext {
            backend: Box::new(backend) as Box<dyn ImeDataBackend>,
        }
    }

    pub(crate) fn dummy() -> ImeDataContext {
        ImeDataContext {
            backend: Box::new(DummyImeDataContext),
        }
    }
}

/// Non-functioning placeholder
pub(crate) struct DummyImeDataContext;
impl ImeDataBackend for DummyImeDataContext {
    fn set_ime_data(&mut self, _: &mut crate::Viewport, _: PlatformImeData) {
        // empty
    }
}

impl fmt::Debug for ImeDataContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImeDataContext")
            .field("backend", &(&(*self.backend) as *const _))
            .finish()
    }
}

pub(crate) unsafe extern "C" fn set_ime_data(
    _: *mut sys::ImGuiContext,
    viewport: *mut sys::ImGuiViewport,
    data: *mut sys::ImGuiPlatformImeData,
) {
    let result = catch_unwind(|| {
        let data = unsafe { *(data as *mut crate::PlatformImeData).as_ref().unwrap() };

        let user_data = unsafe { (*sys::igGetPlatformIO()).Platform_ImeUserData };
        let ctx = &mut *(user_data as *mut ImeDataContext);

        ctx.backend
            .set_ime_data(&mut *(viewport as *mut crate::Viewport), data);
    });
    result.unwrap_or_else(|_| {
        eprintln!("IME data setter panicked");
        process::abort();
    });
}
