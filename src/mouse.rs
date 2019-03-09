use crate::sys;
use crate::Ui;

/// Represents one of the supported mouse buttons
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MouseButton {
    Left = 0,
    Right = 1,
    Middle = 2,
    Extra1 = 3,
    Extra2 = 4,
}

/// Mouse cursor type identifier
#[repr(i32)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum MouseCursor {
    Arrow = sys::ImGuiMouseCursor_Arrow,
    /// Automatically used when hovering over text inputs, etc.
    TextInput = sys::ImGuiMouseCursor_TextInput,
    /// Not used automatically
    ResizeAll = sys::ImGuiMouseCursor_ResizeAll,
    /// Automatically used when hovering over a horizontal border
    ResizeNS = sys::ImGuiMouseCursor_ResizeNS,
    /// Automatically used when hovering over a vertical border or a column
    ResizeEW = sys::ImGuiMouseCursor_ResizeEW,
    /// Automatically used when hovering over the bottom-left corner of a window
    ResizeNESW = sys::ImGuiMouseCursor_ResizeNESW,
    /// Automatically used when hovering over the bottom-right corner of a window
    ResizeNWSE = sys::ImGuiMouseCursor_ResizeNWSE,
    /// Not used automatically, use for e.g. hyperlinks
    Hand = sys::ImGuiMouseCursor_Hand,
}
impl MouseCursor {
    /// All possible `MouseCursor` varirants
    pub const VARIANTS: [MouseCursor; 8] = [
        MouseCursor::Arrow,
        MouseCursor::TextInput,
        MouseCursor::ResizeAll,
        MouseCursor::ResizeNS,
        MouseCursor::ResizeEW,
        MouseCursor::ResizeNESW,
        MouseCursor::ResizeNWSE,
        MouseCursor::Hand,
    ];
    const SKIPPED_COUNT: usize = 1;
    /// Total count of `MouseCursor` variants
    pub const COUNT: usize = sys::ImGuiMouseCursor_COUNT as usize - MouseCursor::SKIPPED_COUNT;
}

impl<'ui> Ui<'ui> {
    /// Get the currently desired mouse cursor type.
    ///
    /// Returns `None` if no cursor should be displayed
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
            _ => None,
        }
    }
    /// Set the desired mouse cursor type.
    ///
    /// Passing `None` hides the mouse cursor.
    pub fn set_mouse_cursor(&self, cursor_type: Option<MouseCursor>) {
        unsafe {
            sys::igSetMouseCursor(
                cursor_type
                    .map(|x| x as i32)
                    .unwrap_or(sys::ImGuiMouseCursor_None),
            );
        }
    }
}
