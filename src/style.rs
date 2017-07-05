extern crate imgui_sys;

use ImVec2;
use Ui;
use imgui_sys::ImGuiStyleVar;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StyleVar {
    Alpha(f32),
    WindowPadding(ImVec2),
    WindowRounding(f32),
    WindowMinSize(ImVec2),
    ChildWindowRounding(f32),
    FramePadding(ImVec2),
    FrameRounding(f32),
    ItemSpacing(ImVec2),
    ItemInnerSpacing(ImVec2),
    IndentSpacing(f32),
    GrabMinSize(f32),
    ButtonTextAlign(ImVec2),
}

impl<'ui> Ui<'ui> {
    fn push_style_var(&self, style_var: StyleVar) {
        use StyleVar::*;
        use imgui_sys::{igPushStyleVar, igPushStyleVarVec};
        match style_var {
            Alpha(v) => unsafe { igPushStyleVar(ImGuiStyleVar::Alpha, v) },
            WindowPadding(v) => unsafe { igPushStyleVarVec(ImGuiStyleVar::WindowPadding, v) },
            WindowRounding(v) => unsafe { igPushStyleVar(ImGuiStyleVar::WindowRounding, v) },
            WindowMinSize(v) => unsafe { igPushStyleVarVec(ImGuiStyleVar::WindowMinSize, v) },
            ChildWindowRounding(v) => unsafe { igPushStyleVar(ImGuiStyleVar::ChildWindowRounding, v) },
            FramePadding(v) => unsafe { igPushStyleVarVec(ImGuiStyleVar::FramePadding, v) },
            FrameRounding(v) => unsafe { igPushStyleVar(ImGuiStyleVar::FrameRounding, v) },
            ItemSpacing(v) => unsafe { igPushStyleVarVec(ImGuiStyleVar::ItemSpacing, v) },
            ItemInnerSpacing(v) => unsafe { igPushStyleVarVec(ImGuiStyleVar::ItemInnerSpacing, v) },
            IndentSpacing(v) => unsafe { igPushStyleVar(ImGuiStyleVar::IndentSpacing, v) },
            GrabMinSize(v) => unsafe { igPushStyleVar(ImGuiStyleVar::GrabMinSize, v) },
            ButtonTextAlign(v) => unsafe { igPushStyleVarVec(ImGuiStyleVar::ButtonTextAlign, v) }
        }
    }

    /// Runs a function after temporarily pushing a value to the style stack.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = ImGui::init();
    /// # let ui = imgui.frame((0, 0), (0, 0), 0.1);
    /// ui.with_style_var(StyleVar::Alpha(0.2), || {
    ///     ui.text(im_str!("AB"));
    /// });
    /// ui.with_style_var(StyleVar::Alpha(0.4), || {
    ///     ui.text(im_str!("CD"));
    /// });
    /// ```
    pub fn with_style_var<F: FnOnce()>(&self, style_var: StyleVar, f: F) {
        self.push_style_var(style_var);
        f();
        unsafe { imgui_sys::igPopStyleVar(1) }
    }

    /// Runs a function after temporarily pushing an array of values into the stack. Supporting
    /// multiple is also easy since you can freely mix and match them in a safe manner.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use imgui::*;
    /// # let mut imgui = ImGui::init();
    /// # let ui = imgui.frame((0, 0), (0, 0), 0.1);
    /// # let styles = [StyleVar::Alpha(0.2), StyleVar::WindowPadding(ImVec2::new(1.0, 1.0))];
    /// ui.with_style_vars(&styles, || {
    ///     ui.text(im_str!("A"));
    ///     ui.text(im_str!("B"));
    ///     ui.text(im_str!("C"));
    ///     ui.text(im_str!("D"));
    /// });
    /// ```
    pub fn with_style_vars<F: FnOnce()>(&self, style_vars: &[StyleVar], f: F) {
        for &style_var in style_vars {
            self.push_style_var(style_var);
        }
        f();
        unsafe { imgui_sys::igPopStyleVar(style_vars.len() as i32) };
    }
}