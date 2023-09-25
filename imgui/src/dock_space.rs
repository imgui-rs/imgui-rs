use std::ptr::null;

use crate::Ui;

impl Ui {
    pub fn dockspace_over_main_viewport(&self) {
        unsafe {
            sys::ImGui_DockSpaceOverViewport(
                sys::ImGui_GetMainViewport(),
                sys::ImGuiDockNodeFlags_PassthruCentralNode as i32,
                null(),
            );
        }
    }
}
