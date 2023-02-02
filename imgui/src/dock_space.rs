use std::ptr::null;

use crate::Ui;

impl Ui {
    pub fn dockspace_over_main_viewport(&self) {
        unsafe {
            sys::igDockSpaceOverViewport(
                sys::igGetMainViewport(),
                sys::ImGuiDockNodeFlags_PassthruCentralNode as i32,
                null(),
            );
        }
    }
}
