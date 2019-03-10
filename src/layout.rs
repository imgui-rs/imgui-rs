use crate::sys;
use crate::Ui;

impl<'ui> Ui<'ui> {
    pub fn separator(&self) {
        unsafe { sys::igSeparator() }
    }
    pub fn new_line(&self) {
        unsafe { sys::igNewLine() }
    }
    pub fn spacing(&self) {
        unsafe { sys::igSpacing() }
    }
}
