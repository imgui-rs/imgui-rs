use crate::sys;
use crate::Ui;

/// A key identifier
#[repr(u32)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Key {
    Tab = sys::ImGuiKey_Tab as u32,
    LeftArrow = sys::ImGuiKey_LeftArrow as u32,
    RightArrow = sys::ImGuiKey_RightArrow as u32,
    UpArrow = sys::ImGuiKey_UpArrow as u32,
    DownArrow = sys::ImGuiKey_DownArrow as u32,
    PageUp = sys::ImGuiKey_PageUp as u32,
    PageDown = sys::ImGuiKey_PageDown as u32,
    Home = sys::ImGuiKey_Home as u32,
    End = sys::ImGuiKey_End as u32,
    Insert = sys::ImGuiKey_Insert as u32,
    Delete = sys::ImGuiKey_Delete as u32,
    Backspace = sys::ImGuiKey_Backspace as u32,
    Space = sys::ImGuiKey_Space as u32,
    Enter = sys::ImGuiKey_Enter as u32,
    Escape = sys::ImGuiKey_Escape as u32,
    KeyPadEnter = sys::ImGuiKey_KeyPadEnter as u32,
    A = sys::ImGuiKey_A as u32,
    C = sys::ImGuiKey_C as u32,
    V = sys::ImGuiKey_V as u32,
    X = sys::ImGuiKey_X as u32,
    Y = sys::ImGuiKey_Y as u32,
    Z = sys::ImGuiKey_Z as u32,
}

impl Key {
    /// All possible `Key` variants
    pub const VARIANTS: [Key; Key::COUNT] = [
        Key::Tab,
        Key::LeftArrow,
        Key::RightArrow,
        Key::UpArrow,
        Key::DownArrow,
        Key::PageUp,
        Key::PageDown,
        Key::Home,
        Key::End,
        Key::Insert,
        Key::Delete,
        Key::Backspace,
        Key::Space,
        Key::Enter,
        Key::Escape,
        Key::KeyPadEnter,
        Key::A,
        Key::C,
        Key::V,
        Key::X,
        Key::Y,
        Key::Z,
    ];
    /// Total count of `Key` variants
    pub const COUNT: usize = sys::ImGuiKey_COUNT as usize;
}

#[test]
fn test_key_variants() {
    for (idx, &value) in Key::VARIANTS.iter().enumerate() {
        assert_eq!(idx, value as usize);
    }
}

/// Target widget selection for keyboard focus
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum FocusedWidget {
    /// Previous widget
    Previous,
    /// Next widget
    Next,
    /// Widget using a relative positive offset (0 is the next widget).
    ///
    /// Use this to access sub components of a multiple component widget.
    Offset(u32),
}

impl FocusedWidget {
    fn as_offset(self) -> i32 {
        match self {
            FocusedWidget::Previous => -1,
            FocusedWidget::Next => 0,
            FocusedWidget::Offset(offset) => offset as i32,
        }
    }
}

/// # Input: Keyboard
impl<'ui> Ui<'ui> {
    /// Returns the key index of the given key identifier.
    ///
    /// Equivalent to indexing the Io struct `key_map` field: `ui.io().key_map[key]`
    pub fn key_index(&self, key: Key) -> u32 {
        unsafe { sys::igGetKeyIndex(key as i32) as u32 }
    }
    /// Returns true if the key is being held.
    ///
    /// Equivalent to indexing the Io struct `keys_down` field: `ui.io().keys_down[key_index]`
    pub fn is_key_down(&self, key_index: u32) -> bool {
        unsafe { sys::igIsKeyDown(key_index as i32) }
    }
    /// Returns true if the key was pressed (went from !down to down).
    ///
    /// Affected by key repeat settings (`io.key_repeat_delay`, `io.key_repeat_rate`)
    pub fn is_key_pressed(&self, key_index: u32) -> bool {
        unsafe { sys::igIsKeyPressed(key_index as i32, true) }
    }
    /// Returns true if the key was released (went from down to !down)
    pub fn is_key_released(&self, key_index: u32) -> bool {
        unsafe { sys::igIsKeyReleased(key_index as i32) }
    }
    /// Returns a count of key presses using the given repeat rate/delay settings.
    ///
    /// Usually returns 0 or 1, but might be >1 if `rate` is small enough that `io.delta_time` >
    /// `rate`.
    pub fn key_pressed_amount(&self, key_index: u32, repeat_delay: f32, rate: f32) -> u32 {
        unsafe { sys::igGetKeyPressedAmount(key_index as i32, repeat_delay, rate) as u32 }
    }
    /// Focuses keyboard on a widget relative to current position
    pub fn set_keyboard_focus_here(&self, target_widget: FocusedWidget) {
        unsafe {
            sys::igSetKeyboardFocusHere(target_widget.as_offset());
        }
    }
}
