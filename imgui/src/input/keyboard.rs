use crate::sys;
use crate::Ui;

/// A key identifier
#[repr(u32)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[allow(missing_docs)] // Self-describing
pub enum Key {
    Tab = sys::ImGuiKey_Tab,
    LeftArrow = sys::ImGuiKey_LeftArrow,
    RightArrow = sys::ImGuiKey_RightArrow,
    UpArrow = sys::ImGuiKey_UpArrow,
    DownArrow = sys::ImGuiKey_DownArrow,
    PageUp = sys::ImGuiKey_PageUp,
    PageDown = sys::ImGuiKey_PageDown,
    Home = sys::ImGuiKey_Home,
    End = sys::ImGuiKey_End,
    Insert = sys::ImGuiKey_Insert,
    Delete = sys::ImGuiKey_Delete,
    Backspace = sys::ImGuiKey_Backspace,
    Space = sys::ImGuiKey_Space,
    Enter = sys::ImGuiKey_Enter,
    Escape = sys::ImGuiKey_Escape,
    LeftCtrl = sys::ImGuiKey_LeftCtrl,
    LeftShift = sys::ImGuiKey_LeftShift,
    LeftAlt = sys::ImGuiKey_LeftAlt,
    LeftSuper = sys::ImGuiKey_LeftSuper,
    RightCtrl = sys::ImGuiKey_RightCtrl,
    RightShift = sys::ImGuiKey_RightShift,
    RightAlt = sys::ImGuiKey_RightAlt,
    RightSuper = sys::ImGuiKey_RightSuper,
    Menu = sys::ImGuiKey_Menu,
    Alpha0 = sys::ImGuiKey_0,
    Alpha1 = sys::ImGuiKey_1,
    Alpha2 = sys::ImGuiKey_2,
    Alpha3 = sys::ImGuiKey_3,
    Alpha4 = sys::ImGuiKey_4,
    Alpha5 = sys::ImGuiKey_5,
    Alpha6 = sys::ImGuiKey_6,
    Alpha7 = sys::ImGuiKey_7,
    Alpha8 = sys::ImGuiKey_8,
    Alpha9 = sys::ImGuiKey_9,
    A = sys::ImGuiKey_A,
    B = sys::ImGuiKey_B,
    C = sys::ImGuiKey_C,
    D = sys::ImGuiKey_D,
    E = sys::ImGuiKey_E,
    F = sys::ImGuiKey_F,
    G = sys::ImGuiKey_G,
    H = sys::ImGuiKey_H,
    I = sys::ImGuiKey_I,
    J = sys::ImGuiKey_J,
    K = sys::ImGuiKey_K,
    L = sys::ImGuiKey_L,
    M = sys::ImGuiKey_M,
    N = sys::ImGuiKey_N,
    O = sys::ImGuiKey_O,
    P = sys::ImGuiKey_P,
    Q = sys::ImGuiKey_Q,
    R = sys::ImGuiKey_R,
    S = sys::ImGuiKey_S,
    T = sys::ImGuiKey_T,
    U = sys::ImGuiKey_U,
    V = sys::ImGuiKey_V,
    W = sys::ImGuiKey_W,
    X = sys::ImGuiKey_X,
    Y = sys::ImGuiKey_Y,
    Z = sys::ImGuiKey_Z,
    F1 = sys::ImGuiKey_F1,
    F2 = sys::ImGuiKey_F2,
    F3 = sys::ImGuiKey_F3,
    F4 = sys::ImGuiKey_F4,
    F5 = sys::ImGuiKey_F5,
    F6 = sys::ImGuiKey_F6,
    F7 = sys::ImGuiKey_F7,
    F8 = sys::ImGuiKey_F8,
    F9 = sys::ImGuiKey_F9,
    F10 = sys::ImGuiKey_F10,
    F11 = sys::ImGuiKey_F11,
    F12 = sys::ImGuiKey_F12,
    Apostrophe = sys::ImGuiKey_Apostrophe,
    Comma = sys::ImGuiKey_Comma,
    Minus = sys::ImGuiKey_Minus,
    Period = sys::ImGuiKey_Period,
    Slash = sys::ImGuiKey_Slash,
    Semicolon = sys::ImGuiKey_Semicolon,
    Equal = sys::ImGuiKey_Equal,
    LeftBracket = sys::ImGuiKey_LeftBracket,
    Backslash = sys::ImGuiKey_Backslash,
    RightBracket = sys::ImGuiKey_RightBracket,
    GraveAccent = sys::ImGuiKey_GraveAccent,
    CapsLock = sys::ImGuiKey_CapsLock,
    ScrollLock = sys::ImGuiKey_ScrollLock,
    NumLock = sys::ImGuiKey_NumLock,
    PrintScreen = sys::ImGuiKey_PrintScreen,
    Pause = sys::ImGuiKey_Pause,
    Keypad0 = sys::ImGuiKey_Keypad0,
    Keypad1 = sys::ImGuiKey_Keypad1,
    Keypad2 = sys::ImGuiKey_Keypad2,
    Keypad3 = sys::ImGuiKey_Keypad3,
    Keypad4 = sys::ImGuiKey_Keypad4,
    Keypad5 = sys::ImGuiKey_Keypad5,
    Keypad6 = sys::ImGuiKey_Keypad6,
    Keypad7 = sys::ImGuiKey_Keypad7,
    Keypad8 = sys::ImGuiKey_Keypad8,
    Keypad9 = sys::ImGuiKey_Keypad9,
    KeypadDecimal = sys::ImGuiKey_KeypadDecimal,
    KeypadDivide = sys::ImGuiKey_KeypadDivide,
    KeypadMultiply = sys::ImGuiKey_KeypadMultiply,
    KeypadSubtract = sys::ImGuiKey_KeypadSubtract,
    KeypadAdd = sys::ImGuiKey_KeypadAdd,
    KeypadEnter = sys::ImGuiKey_KeypadEnter,
    KeypadEqual = sys::ImGuiKey_KeypadEqual,
    GamepadStart = sys::ImGuiKey_GamepadStart,
    GamepadBack = sys::ImGuiKey_GamepadBack,
    GamepadFaceLeft = sys::ImGuiKey_GamepadFaceLeft,
    GamepadFaceRight = sys::ImGuiKey_GamepadFaceRight,
    GamepadFaceUp = sys::ImGuiKey_GamepadFaceUp,
    GamepadFaceDown = sys::ImGuiKey_GamepadFaceDown,
    GamepadDpadLeft = sys::ImGuiKey_GamepadDpadLeft,
    GamepadDpadRight = sys::ImGuiKey_GamepadDpadRight,
    GamepadDpadUp = sys::ImGuiKey_GamepadDpadUp,
    GamepadDpadDown = sys::ImGuiKey_GamepadDpadDown,
    GamepadL1 = sys::ImGuiKey_GamepadL1,
    GamepadR1 = sys::ImGuiKey_GamepadR1,
    GamepadL2 = sys::ImGuiKey_GamepadL2,
    GamepadR2 = sys::ImGuiKey_GamepadR2,
    GamepadL3 = sys::ImGuiKey_GamepadL3,
    GamepadR3 = sys::ImGuiKey_GamepadR3,
    GamepadLStickLeft = sys::ImGuiKey_GamepadLStickLeft,
    GamepadLStickRight = sys::ImGuiKey_GamepadLStickRight,
    GamepadLStickUp = sys::ImGuiKey_GamepadLStickUp,
    GamepadLStickDown = sys::ImGuiKey_GamepadLStickDown,
    GamepadRStickLeft = sys::ImGuiKey_GamepadRStickLeft,
    GamepadRStickRight = sys::ImGuiKey_GamepadRStickRight,
    GamepadRStickUp = sys::ImGuiKey_GamepadRStickUp,
    GamepadRStickDown = sys::ImGuiKey_GamepadRStickDown,
    MouseLeft = sys::ImGuiKey_MouseLeft,
    MouseRight = sys::ImGuiKey_MouseRight,
    MouseMiddle = sys::ImGuiKey_MouseMiddle,
    MouseX1 = sys::ImGuiKey_MouseX1,
    MouseX2 = sys::ImGuiKey_MouseX2,
    MouseWheelX = sys::ImGuiKey_MouseWheelX,
    MouseWheelY = sys::ImGuiKey_MouseWheelY,
    ReservedForModCtrl = sys::ImGuiKey_ReservedForModCtrl,
    ReservedForModShift = sys::ImGuiKey_ReservedForModShift,
    ReservedForModAlt = sys::ImGuiKey_ReservedForModAlt,
    ReservedForModSuper = sys::ImGuiKey_ReservedForModSuper,
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
        Key::LeftCtrl,
        Key::LeftShift,
        Key::LeftAlt,
        Key::LeftSuper,
        Key::RightCtrl,
        Key::RightShift,
        Key::RightAlt,
        Key::RightSuper,
        Key::Menu,
        Key::Alpha0,
        Key::Alpha1,
        Key::Alpha2,
        Key::Alpha3,
        Key::Alpha4,
        Key::Alpha5,
        Key::Alpha6,
        Key::Alpha7,
        Key::Alpha8,
        Key::Alpha9,
        Key::A,
        Key::B,
        Key::C,
        Key::D,
        Key::E,
        Key::F,
        Key::G,
        Key::H,
        Key::I,
        Key::J,
        Key::K,
        Key::L,
        Key::M,
        Key::N,
        Key::O,
        Key::P,
        Key::Q,
        Key::R,
        Key::S,
        Key::T,
        Key::U,
        Key::V,
        Key::W,
        Key::X,
        Key::Y,
        Key::Z,
        Key::F1,
        Key::F2,
        Key::F3,
        Key::F4,
        Key::F5,
        Key::F6,
        Key::F7,
        Key::F8,
        Key::F9,
        Key::F10,
        Key::F11,
        Key::F12,
        Key::Apostrophe,
        Key::Comma,
        Key::Minus,
        Key::Period,
        Key::Slash,
        Key::Semicolon,
        Key::Equal,
        Key::LeftBracket,
        Key::Backslash,
        Key::RightBracket,
        Key::GraveAccent,
        Key::CapsLock,
        Key::ScrollLock,
        Key::NumLock,
        Key::PrintScreen,
        Key::Pause,
        Key::Keypad0,
        Key::Keypad1,
        Key::Keypad2,
        Key::Keypad3,
        Key::Keypad4,
        Key::Keypad5,
        Key::Keypad6,
        Key::Keypad7,
        Key::Keypad8,
        Key::Keypad9,
        Key::KeypadDecimal,
        Key::KeypadDivide,
        Key::KeypadMultiply,
        Key::KeypadSubtract,
        Key::KeypadAdd,
        Key::KeypadEnter,
        Key::KeypadEqual,
        Key::GamepadStart,
        Key::GamepadBack,
        Key::GamepadFaceLeft,
        Key::GamepadFaceRight,
        Key::GamepadFaceUp,
        Key::GamepadFaceDown,
        Key::GamepadDpadLeft,
        Key::GamepadDpadRight,
        Key::GamepadDpadUp,
        Key::GamepadDpadDown,
        Key::GamepadL1,
        Key::GamepadR1,
        Key::GamepadL2,
        Key::GamepadR2,
        Key::GamepadL3,
        Key::GamepadR3,
        Key::GamepadLStickLeft,
        Key::GamepadLStickRight,
        Key::GamepadLStickUp,
        Key::GamepadLStickDown,
        Key::GamepadRStickLeft,
        Key::GamepadRStickRight,
        Key::GamepadRStickUp,
        Key::GamepadRStickDown,
        Key::MouseLeft,
        Key::MouseRight,
        Key::MouseMiddle,
        Key::MouseX1,
        Key::MouseX2,
        Key::MouseWheelX,
        Key::MouseWheelY,
        Key::ReservedForModCtrl,
        Key::ReservedForModShift,
        Key::ReservedForModAlt,
        Key::ReservedForModSuper,
    ];
    /// Total count of `Key` variants
    pub const COUNT: usize = sys::ImGuiKey_NamedKey_COUNT as usize;
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
    #[inline]
    fn as_offset(self) -> i32 {
        match self {
            FocusedWidget::Previous => -1,
            FocusedWidget::Next => 0,
            FocusedWidget::Offset(offset) => offset as i32,
        }
    }
}

/// # Input: Keyboard
impl Ui {
    /// Returns the key index of the given key identifier.
    ///
    /// Equivalent to indexing the Io struct `key_map` field: `ui.io().key_map[key]`
    #[inline]
    #[doc(alias = "GetKeyIndex")]
    fn key_index(&self, key: Key) -> u32 {
        unsafe { sys::igGetKeyIndex(key as u32) }
    }
    /// Returns true if the key is being held.
    ///
    /// Equivalent to indexing the Io struct `keys_down` field: `ui.io().keys_down[key_index]`
    #[inline]
    #[doc(alias = "IsKeyDown")]
    pub fn is_key_down(&self, key: Key) -> bool {
        let key_index = self.key_index(key);
        self.is_key_index_down(key_index)
    }

    /// Same as [`is_key_down`](Self::is_key_down) but takes a key index. The meaning of
    /// index is defined by your backend implementation.
    #[inline]
    #[doc(alias = "IsKeyDown")]
    pub fn is_key_index_down(&self, key_index: u32) -> bool {
        cfg_if::cfg_if!{
            if #[cfg(feature = "docking")] {
                unsafe { sys::igIsKeyDown_Nil(key_index) }
            } else {
                unsafe { sys::igIsKeyDown(key_index) }
            }
        }
    }

    /// Returns true if the key was pressed (went from !down to down).
    ///
    /// Affected by key repeat settings (`io.key_repeat_delay`, `io.key_repeat_rate`)
    #[inline]
    #[doc(alias = "IsKeyPressed")]
    pub fn is_key_pressed(&self, key: Key) -> bool {
        let key_index = self.key_index(key);
        self.is_key_index_pressed(key_index)
    }

    /// Same as [`is_key_pressed`](Self::is_key_pressed) but takes a key index.
    ///
    /// The meaning of index is defined by your backend
    /// implementation.
    #[inline]
    #[doc(alias = "IsKeyPressed")]
    pub fn is_key_index_pressed(&self, key_index: u32) -> bool {
        cfg_if::cfg_if!{
            if #[cfg(feature = "docking")] {
                unsafe { sys::igIsKeyPressed_Bool(key_index, true) }
            } else {
                unsafe { sys::igIsKeyPressed(key_index, true) }
            }
        }
    }

    /// Returns true if the key was pressed (went from !down to down).
    ///
    /// Is **not** affected by key repeat settings (`io.key_repeat_delay`, `io.key_repeat_rate`)
    #[inline]
    #[doc(alias = "IsKeyPressed")]
    pub fn is_key_pressed_no_repeat(&self, key: Key) -> bool {
        let key_index = self.key_index(key);
        self.is_key_index_pressed_no_repeat(key_index)
    }

    /// Same as [`is_key_pressed_no_repeat`](Self::is_key_pressed_no_repeat)
    /// but takes a key index.
    ///
    /// The meaning of index is defined by your backend
    /// implementation.
    #[inline]
    #[doc(alias = "IsKeyPressed")]
    pub fn is_key_index_pressed_no_repeat(&self, key_index: u32) -> bool {
        cfg_if::cfg_if!{
            if #[cfg(feature = "docking")] {
                unsafe { sys::igIsKeyPressed_Bool(key_index, false) }
            } else {
                unsafe { sys::igIsKeyPressed(key_index, false) }
            }
        }
    }

    /// Returns true if the key was released (went from down to !down)
    #[inline]
    #[doc(alias = "IsKeyReleased")]
    pub fn is_key_released(&self, key: Key) -> bool {
        let key_index = self.key_index(key);
        self.is_key_index_released(key_index)
    }

    /// Same as [`is_key_released`](Self::is_key_released) but takes a key index.
    ///
    /// The meaning of index is defined by your backend
    /// implementation.
    #[inline]
    #[doc(alias = "IsKeyReleased")]
    pub fn is_key_index_released(&self, key_index: u32) -> bool {
        cfg_if::cfg_if!{
            if #[cfg(feature = "docking")] {
                unsafe { sys::igIsKeyReleased_Nil(key_index) }
            } else {
                unsafe { sys::igIsKeyReleased(key_index) }
            }
        }
    }

    /// Returns a count of key presses using the given repeat rate/delay settings.
    ///
    /// Usually returns 0 or 1, but might be >1 if `rate` is small enough that `io.delta_time` >
    /// `rate`.
    #[inline]
    #[doc(alias = "GetKeyPressedAmount")]
    pub fn key_pressed_amount(&self, key: Key, repeat_delay: f32, rate: f32) -> u32 {
        let key_index = self.key_index(key);
        self.key_index_pressed_amount(key_index, repeat_delay, rate)
    }

    /// Same as [`crate::Ui::key_pressed_amount`] but takes a key index.
    #[inline]
    #[doc(alias = "GetKeyPressedAmount")]
    pub fn key_index_pressed_amount(&self, key_index: u32, repeat_delay: f32, rate: f32) -> u32 {
        unsafe { sys::igGetKeyPressedAmount(key_index, repeat_delay, rate) as u32 }
    }

    /// Focuses keyboard on the next widget.
    ///
    /// This is the equivalent to [set_keyboard_focus_here_with_offset](Self::set_keyboard_focus_here_with_offset)
    /// with `target_widget` set to `FocusedWidget::Next`.
    #[inline]
    #[doc(alias = "SetKeyboardFocusHere")]
    pub fn set_keyboard_focus_here(&self) {
        self.set_keyboard_focus_here_with_offset(FocusedWidget::Next);
    }

    /// Focuses keyboard on a widget relative to current position.
    #[inline]
    #[doc(alias = "SetKeyboardFocusHere")]
    pub fn set_keyboard_focus_here_with_offset(&self, target_widget: FocusedWidget) {
        unsafe {
            sys::igSetKeyboardFocusHere(target_widget.as_offset());
        }
    }
}
