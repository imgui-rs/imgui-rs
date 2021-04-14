use bitflags::bitflags;
use std::ops::{BitAnd, BitAndAssign, BitOrAssign, Not};

use crate::string::ImStr;
use crate::sys;
use crate::{Direction, Ui};

bitflags!(
    /// Flags for invisible buttons
    #[repr(transparent)]
    pub struct ButtonFlags: u32 {
        /// React on left mouse button
        const MOUSE_BUTTON_LEFT = sys::ImGuiButtonFlags_MouseButtonLeft;
        /// React on right mouse button
        const MOUSE_BUTTON_RIGHT = sys::ImGuiButtonFlags_MouseButtonRight;
        /// React on middle mouse button
        const MOUSE_BUTTON_MIDDLE = sys::ImGuiButtonFlags_MouseButtonMiddle;
    }
);

/// # Widgets: Miscellaneous
impl<'ui> Ui<'ui> {
    /// Renders a clickable button.
    ///
    /// Returns true if this button was clicked.
    ///
    /// This is the equivalent of [button_with_size](Self::button_with_size)
    /// with `size` set to `[0.0, 0.0]`, which will size the button to the
    /// label's width in the current style.
    /// the current style.
    #[doc(alias = "Button")]
    pub fn button(&self, label: &ImStr) -> bool {
        self.button_with_size(label, [0.0, 0.0])
    }

    /// Renders a clickable button.
    ///
    /// Returns true if this button was clicked.
    ///
    /// Setting `size` as `[0.0, 0.0]` will size the button to the label's width in
    /// the current style.
    #[doc(alias = "Button")]
    pub fn button_with_size(&self, label: &ImStr, size: [f32; 2]) -> bool {
        unsafe { sys::igButton(label.as_ptr(), size.into()) }
    }
    /// Renders a small clickable button that is easy to embed in text.
    ///
    /// Returns true if this button was clicked.
    #[doc(alias = "SmallButton")]
    pub fn small_button(&self, label: &ImStr) -> bool {
        unsafe { sys::igSmallButton(label.as_ptr()) }
    }
    /// Renders a widget with button behaviour without the visual look.
    ///
    /// Returns true if this button was clicked.
    #[doc(alias = "InvisibleButton")]
    pub fn invisible_button(&self, id: &ImStr, size: [f32; 2]) -> bool {
        unsafe { sys::igInvisibleButton(id.as_ptr(), size.into(), 0) }
    }
    /// Renders a widget with button behaviour without the visual look.
    ///
    /// Returns true if this button was clicked.
    #[doc(alias = "InvisibleButton")]
    pub fn invisible_button_flags(&self, id: &ImStr, size: [f32; 2], flags: ButtonFlags) -> bool {
        unsafe { sys::igInvisibleButton(id.as_ptr(), size.into(), flags.bits() as i32) }
    }
    /// Renders a square button with an arrow shape.
    ///
    /// Returns true if this button was clicked.
    #[doc(alias = "ArrowButton")]
    pub fn arrow_button(&self, id: &ImStr, direction: Direction) -> bool {
        unsafe { sys::igArrowButton(id.as_ptr(), direction as i32) }
    }
    /// Renders a simple checkbox.
    ///
    /// Returns true if this checkbox was clicked.
    #[doc(alias = "Checkbox")]
    pub fn checkbox(&self, label: &ImStr, value: &mut bool) -> bool {
        unsafe { sys::igCheckbox(label.as_ptr(), value as *mut bool) }
    }
    /// Renders a checkbox suitable for toggling bit flags using a mask.
    ///
    /// Returns true if this checkbox was clicked.
    pub fn checkbox_flags<T>(&self, label: &ImStr, flags: &mut T, mask: T) -> bool
    where
        T: Copy + PartialEq + BitOrAssign + BitAndAssign + BitAnd<Output = T> + Not<Output = T>,
    {
        let mut value = *flags & mask == mask;
        let pressed = self.checkbox(label, &mut value);
        if pressed {
            if value {
                *flags |= mask;
            } else {
                *flags &= !mask;
            }
        }
        pressed
    }
    /// Renders a simple radio button.
    ///
    /// Returns true if this radio button was clicked.
    #[doc(alias = "RadioButtonBool")]
    pub fn radio_button_bool(&self, label: &ImStr, active: bool) -> bool {
        unsafe { sys::igRadioButton_Bool(label.as_ptr(), active) }
    }
    /// Renders a radio button suitable for choosing an arbitrary value.
    ///
    /// Returns true if this radio button was clicked.
    #[doc(alias = "RadioButtonBool")]
    pub fn radio_button<T>(&self, label: &ImStr, value: &mut T, button_value: T) -> bool
    where
        T: Copy + PartialEq,
    {
        let pressed = self.radio_button_bool(label, *value == button_value);
        if pressed {
            *value = button_value;
        }
        pressed
    }
    /// Renders a small circle and keeps the cursor on the same line
    #[doc(alias = "Bullet")]
    pub fn bullet(&self) {
        unsafe { sys::igBullet() };
    }
}
