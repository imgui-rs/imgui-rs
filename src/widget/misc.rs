use std::ops::{BitAnd, BitAndAssign, BitOrAssign, Not};

use crate::string::ImStr;
use crate::sys;
use crate::Ui;

/// A cardinal direction
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    Left = sys::ImGuiDir_Left,
    Right = sys::ImGuiDir_Right,
    Up = sys::ImGuiDir_Up,
    Down = sys::ImGuiDir_Down,
}

impl<'ui> Ui<'ui> {
    pub fn button(&self, label: &ImStr, size: [f32; 2]) -> bool {
        unsafe { sys::igButton(label.as_ptr(), size.into()) }
    }
    pub fn small_button(&self, label: &ImStr) -> bool {
        unsafe { sys::igSmallButton(label.as_ptr()) }
    }
    pub fn invisible_button(&self, id: &ImStr, size: [f32; 2]) -> bool {
        unsafe { sys::igInvisibleButton(id.as_ptr(), size.into()) }
    }
    pub fn arrow_button(&self, id: &ImStr, direction: Direction) -> bool {
        unsafe { sys::igArrowButton(id.as_ptr(), direction as i32) }
    }
    pub fn checkbox(&self, label: &ImStr, value: &mut bool) -> bool {
        unsafe { sys::igCheckbox(label.as_ptr(), value as *mut bool) }
    }
    pub fn checkbox_flags<T>(&self, label: &ImStr, flags: &mut T, flags_value: T) -> bool
    where
        T: Copy + PartialEq + BitOrAssign + BitAndAssign + BitAnd<Output = T> + Not<Output = T>,
    {
        let mut value = *flags & flags_value == flags_value;
        let pressed = self.checkbox(label, &mut value);
        if pressed {
            if value {
                *flags |= flags_value;
            } else {
                *flags &= !flags_value;
            }
        }
        pressed
    }
    /// Constructs a simple radio button. If `active` is true, the button is considered selected.
    ///
    /// Returns true if this button was clicked
    pub fn radio_button_bool(&self, label: &ImStr, active: bool) -> bool {
        unsafe { sys::igRadioButtonBool(label.as_ptr(), active) }
    }
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
    pub fn bullet(&self) {
        unsafe { sys::igBullet() };
    }
}
