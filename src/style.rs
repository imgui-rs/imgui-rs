use crate::sys;

/// User interface style/colors
#[repr(transparent)]
#[derive(Debug)]
pub struct Style(pub sys::ImGuiStyle);

impl Style {
    /// Scales all sizes in the style
    pub fn scale_all_sizes(&mut self, scale_factor: f32) {
        unsafe {
            sys::ImGuiStyle_ScaleAllSizes(&mut self.0, scale_factor);
        }
    }
    /// Replaces current colors with classic Dear ImGui style
    pub fn use_classic_colors(&mut self) -> &mut Self {
        unsafe {
            sys::igStyleColorsClassic(&mut self.0);
        }
        self
    }
    /// Replaces current colors with a new, recommended style
    pub fn use_dark_colors(&mut self) -> &mut Self {
        unsafe {
            sys::igStyleColorsDark(&mut self.0);
        }
        self
    }
    /// Replaces current colors with a light style. Best used with borders and a custom, thicker
    /// font
    pub fn use_light_colors(&mut self) -> &mut Self {
        unsafe {
            sys::igStyleColorsLight(&mut self.0);
        }
        self
    }
}

impl Style {
    pub fn alpha(&self) -> f32 {
        self.0.Alpha
    }
    pub fn set_alpha(&mut self, value: f32) -> &mut Self {
        self.0.Alpha = value;
        self
    }
    pub fn window_padding(&self) -> (f32, f32) {
        self.0.WindowPadding.into()
    }
    pub fn set_window_padding(&mut self, value: (f32, f32)) -> &mut Self {
        self.0.WindowPadding = value.into();
        self
    }
    pub fn window_rounding(&self) -> f32 {
        self.0.WindowRounding
    }
    pub fn set_window_rounding(&mut self, value: f32) -> &mut Self {
        self.0.WindowRounding = value;
        self
    }
    pub fn window_border_size(&self) -> f32 {
        self.0.WindowBorderSize
    }
    pub fn set_window_border_size(&mut self, value: f32) -> &mut Self {
        self.0.WindowBorderSize = value;
        self
    }
    pub fn window_min_size(&self) -> (f32, f32) {
        self.0.WindowMinSize.into()
    }
    pub fn set_window_min_size(&mut self, value: (f32, f32)) -> &mut Self {
        self.0.WindowMinSize = value.into();
        self
    }
    pub fn window_title_align(&self) -> (f32, f32) {
        self.0.WindowTitleAlign.into()
    }
    pub fn set_window_title_align(&mut self, value: (f32, f32)) -> &mut Self {
        self.0.WindowTitleAlign = value.into();
        self
    }
    pub fn child_rounding(&self) -> f32 {
        self.0.ChildRounding
    }
    pub fn set_child_rounding(&mut self, value: f32) -> &mut Self {
        self.0.ChildRounding = value;
        self
    }
    pub fn child_border_size(&self) -> f32 {
        self.0.ChildBorderSize
    }
    pub fn set_child_border_size(&mut self, value: f32) -> &mut Self {
        self.0.ChildBorderSize = value;
        self
    }
    pub fn popup_rounding(&self) -> f32 {
        self.0.PopupRounding
    }
    pub fn set_popup_rounding(&mut self, value: f32) -> &mut Self {
        self.0.PopupRounding = value;
        self
    }
    pub fn popup_border_size(&self) -> f32 {
        self.0.PopupBorderSize
    }
    pub fn set_popup_border_size(&mut self, value: f32) -> &mut Self {
        self.0.PopupBorderSize = value;
        self
    }
    pub fn frame_padding(&self) -> (f32, f32) {
        self.0.FramePadding.into()
    }
    pub fn set_frame_padding(&mut self, value: (f32, f32)) -> &mut Self {
        self.0.FramePadding = value.into();
        self
    }
    pub fn frame_rounding(&self) -> f32 {
        self.0.FrameRounding
    }
    pub fn set_frame_rounding(&mut self, value: f32) -> &mut Self {
        self.0.FrameRounding = value;
        self
    }
    pub fn frame_border_size(&self) -> f32 {
        self.0.FrameBorderSize
    }
    pub fn set_frame_border_size(&mut self, value: f32) -> &mut Self {
        self.0.FrameBorderSize = value;
        self
    }
    pub fn item_spacing(&self) -> (f32, f32) {
        self.0.ItemSpacing.into()
    }
    pub fn set_item_spacing(&mut self, value: (f32, f32)) -> &mut Self {
        self.0.ItemSpacing = value.into();
        self
    }
    pub fn item_inner_spacing(&self) -> (f32, f32) {
        self.0.ItemInnerSpacing.into()
    }
    pub fn set_item_inner_spacing(&mut self, value: (f32, f32)) -> &mut Self {
        self.0.ItemInnerSpacing = value.into();
        self
    }
    pub fn touch_extra_padding(&self) -> (f32, f32) {
        self.0.TouchExtraPadding.into()
    }
    pub fn set_touch_extra_padding(&mut self, value: (f32, f32)) -> &mut Self {
        self.0.TouchExtraPadding = value.into();
        self
    }
    pub fn indent_spacing(&self) -> f32 {
        self.0.IndentSpacing
    }
    pub fn set_indent_spacing(&mut self, value: f32) -> &mut Self {
        self.0.IndentSpacing = value;
        self
    }
    pub fn columns_min_spacing(&self) -> f32 {
        self.0.ColumnsMinSpacing
    }
    pub fn set_columns_min_spacing(&mut self, value: f32) -> &mut Self {
        self.0.ColumnsMinSpacing = value;
        self
    }
    pub fn scrollbar_size(&self) -> f32 {
        self.0.ScrollbarSize
    }
    pub fn set_scrollbar_size(&mut self, value: f32) -> &mut Self {
        self.0.ScrollbarSize = value;
        self
    }
    pub fn scrollbar_rounding(&self) -> f32 {
        self.0.ScrollbarRounding
    }
    pub fn set_scrollbar_rounding(&mut self, value: f32) -> &mut Self {
        self.0.ScrollbarRounding = value;
        self
    }
    pub fn grab_min_size(&self) -> f32 {
        self.0.GrabMinSize
    }
    pub fn set_grab_min_size(&mut self, value: f32) -> &mut Self {
        self.0.GrabMinSize = value;
        self
    }
    pub fn grab_rounding(&self) -> f32 {
        self.0.GrabRounding
    }
    pub fn set_grab_rounding(&mut self, value: f32) -> &mut Self {
        self.0.GrabRounding = value;
        self
    }
    pub fn tab_rounding(&self) -> f32 {
        self.0.TabRounding
    }
    pub fn set_tab_rounding(&mut self, value: f32) -> &mut Self {
        self.0.TabRounding = value;
        self
    }
    pub fn tab_border_size(&self) -> f32 {
        self.0.TabBorderSize
    }
    pub fn set_tab_border_size(&mut self, value: f32) -> &mut Self {
        self.0.TabBorderSize = value;
        self
    }
    pub fn button_text_align(&self) -> (f32, f32) {
        self.0.ButtonTextAlign.into()
    }
    pub fn set_button_text_align(&mut self, value: (f32, f32)) -> &mut Self {
        self.0.ButtonTextAlign = value.into();
        self
    }
    pub fn display_window_padding(&self) -> (f32, f32) {
        self.0.DisplayWindowPadding.into()
    }
    pub fn set_display_window_padding(&mut self, value: (f32, f32)) -> &mut Self {
        self.0.DisplayWindowPadding = value.into();
        self
    }
    pub fn display_safe_area_padding(&self) -> (f32, f32) {
        self.0.DisplaySafeAreaPadding.into()
    }
    pub fn set_display_safe_area_padding(&mut self, value: (f32, f32)) -> &mut Self {
        self.0.DisplaySafeAreaPadding = value.into();
        self
    }
    pub fn mouse_cursor_scale(&self) -> f32 {
        self.0.MouseCursorScale
    }
    pub fn set_mouse_cursor_scale(&mut self, value: f32) -> &mut Self {
        self.0.MouseCursorScale = value;
        self
    }
    pub fn anti_aliased_lines(&self) -> bool {
        self.0.AntiAliasedLines
    }
    pub fn set_anti_aliased_lines(&mut self, value: bool) -> &mut Self {
        self.0.AntiAliasedLines = value;
        self
    }
    pub fn anti_aliased_fill(&self) -> bool {
        self.0.AntiAliasedFill
    }
    pub fn set_anti_aliased_fill(&mut self, value: bool) -> &mut Self {
        self.0.AntiAliasedFill = value;
        self
    }
    pub fn curve_tessellation_tol(&self) -> f32 {
        self.0.CurveTessellationTol
    }
    pub fn set_curve_tessellation_tol(&mut self, value: f32) -> &mut Self {
        self.0.CurveTessellationTol = value;
        self
    }
    pub fn color(&self, col: StyleColor) -> (f32, f32, f32, f32) {
        self.0.Colors[col as usize].into()
    }
    pub fn set_color(&mut self, col: StyleColor, value: (f32, f32, f32, f32)) -> &mut Self {
        self.0.Colors[col as usize] = value.into();
        self
    }
}

impl Style {
    /// Returns an immutable reference to the underlying raw Dear ImGui style
    pub fn raw(&self) -> &sys::ImGuiStyle {
        &self.0
    }
    /// Returns a mutable reference to the underlying raw Dear ImGui style
    pub fn raw_mut(&mut self) -> &mut sys::ImGuiStyle {
        &mut self.0
    }
}

/// A color identifier for styling
#[repr(u32)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum StyleColor {
    Text = sys::ImGuiCol_Text,
    TextDisabled = sys::ImGuiCol_TextDisabled,
    /// Background of normal windows
    WindowBg = sys::ImGuiCol_WindowBg,
    /// Background of child windows
    ChildBg = sys::ImGuiCol_ChildBg,
    /// Background of popups, menus, tooltips windows
    PopupBg = sys::ImGuiCol_PopupBg,
    Border = sys::ImGuiCol_Border,
    BorderShadow = sys::ImGuiCol_BorderShadow,
    /// Background of checkbox, radio button, plot, slider, text input
    FrameBg = sys::ImGuiCol_FrameBg,
    FrameBgHovered = sys::ImGuiCol_FrameBgHovered,
    FrameBgActive = sys::ImGuiCol_FrameBgActive,
    TitleBg = sys::ImGuiCol_TitleBg,
    TitleBgActive = sys::ImGuiCol_TitleBgActive,
    TitleBgCollapsed = sys::ImGuiCol_TitleBgCollapsed,
    MenuBarBg = sys::ImGuiCol_MenuBarBg,
    ScrollbarBg = sys::ImGuiCol_ScrollbarBg,
    ScrollbarGrab = sys::ImGuiCol_ScrollbarGrab,
    ScrollbarGrabHovered = sys::ImGuiCol_ScrollbarGrabHovered,
    ScrollbarGrabActive = sys::ImGuiCol_ScrollbarGrabActive,
    CheckMark = sys::ImGuiCol_CheckMark,
    SliderGrab = sys::ImGuiCol_SliderGrab,
    SliderGrabActive = sys::ImGuiCol_SliderGrabActive,
    Button = sys::ImGuiCol_Button,
    ButtonHovered = sys::ImGuiCol_ButtonHovered,
    ButtonActive = sys::ImGuiCol_ButtonActive,
    Header = sys::ImGuiCol_Header,
    HeaderHovered = sys::ImGuiCol_HeaderHovered,
    HeaderActive = sys::ImGuiCol_HeaderActive,
    Separator = sys::ImGuiCol_Separator,
    SeparatorHovered = sys::ImGuiCol_SeparatorHovered,
    SeparatorActive = sys::ImGuiCol_SeparatorActive,
    ResizeGrip = sys::ImGuiCol_ResizeGrip,
    ResizeGripHovered = sys::ImGuiCol_ResizeGripHovered,
    ResizeGripActive = sys::ImGuiCol_ResizeGripActive,
    Tab = sys::ImGuiCol_Tab,
    TabHovered = sys::ImGuiCol_TabHovered,
    TabActive = sys::ImGuiCol_TabActive,
    TabUnfocused = sys::ImGuiCol_TabUnfocused,
    TabUnfocusedActive = sys::ImGuiCol_TabUnfocusedActive,
    PlotLines = sys::ImGuiCol_PlotLines,
    PlotLinesHovered = sys::ImGuiCol_PlotLinesHovered,
    PlotHistogram = sys::ImGuiCol_PlotHistogram,
    PlotHistogramHovered = sys::ImGuiCol_PlotHistogramHovered,
    TextSelectedBg = sys::ImGuiCol_TextSelectedBg,
    DragDropTarget = sys::ImGuiCol_DragDropTarget,
    /// Gamepad/keyboard: current highlighted item
    NavHighlight = sys::ImGuiCol_NavHighlight,
    /// Highlight window when using CTRL+TAB
    NavWindowingHighlight = sys::ImGuiCol_NavWindowingHighlight,
    /// Darken/colorize entire screen behind the CTRL+TAB window list, when active
    NavWindowingDimBg = sys::ImGuiCol_NavWindowingDimBg,
    /// Darken/colorize entire screen behind a modal window, when one is active
    ModalWindowDimBg = sys::ImGuiCol_ModalWindowDimBg,
}

impl StyleColor {
    /// All possible `StyleColor` variants
    pub const VARIANTS: [StyleColor; StyleColor::COUNT] = [
        StyleColor::Text,
        StyleColor::TextDisabled,
        StyleColor::WindowBg,
        StyleColor::ChildBg,
        StyleColor::PopupBg,
        StyleColor::Border,
        StyleColor::BorderShadow,
        StyleColor::FrameBg,
        StyleColor::FrameBgHovered,
        StyleColor::FrameBgActive,
        StyleColor::TitleBg,
        StyleColor::TitleBgActive,
        StyleColor::TitleBgCollapsed,
        StyleColor::MenuBarBg,
        StyleColor::ScrollbarBg,
        StyleColor::ScrollbarGrab,
        StyleColor::ScrollbarGrabHovered,
        StyleColor::ScrollbarGrabActive,
        StyleColor::CheckMark,
        StyleColor::SliderGrab,
        StyleColor::SliderGrabActive,
        StyleColor::Button,
        StyleColor::ButtonHovered,
        StyleColor::ButtonActive,
        StyleColor::Header,
        StyleColor::HeaderHovered,
        StyleColor::HeaderActive,
        StyleColor::Separator,
        StyleColor::SeparatorHovered,
        StyleColor::SeparatorActive,
        StyleColor::ResizeGrip,
        StyleColor::ResizeGripHovered,
        StyleColor::ResizeGripActive,
        StyleColor::Tab,
        StyleColor::TabHovered,
        StyleColor::TabActive,
        StyleColor::TabUnfocused,
        StyleColor::TabUnfocusedActive,
        StyleColor::PlotLines,
        StyleColor::PlotLinesHovered,
        StyleColor::PlotHistogram,
        StyleColor::PlotHistogramHovered,
        StyleColor::TextSelectedBg,
        StyleColor::DragDropTarget,
        StyleColor::NavHighlight,
        StyleColor::NavWindowingHighlight,
        StyleColor::NavWindowingDimBg,
        StyleColor::ModalWindowDimBg,
    ];
    pub const COUNT: usize = sys::ImGuiCol_COUNT as usize;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StyleVar {
    Alpha(f32),
    WindowPadding((f32, f32)),
    WindowRounding(f32),
    WindowBorderSize(f32),
    WindowMinSize((f32, f32)),
    WindowTitleAlign((f32, f32)),
    ChildRounding(f32),
    ChildBorderSize(f32),
    PopupRounding(f32),
    PopupBorderSize(f32),
    FramePadding((f32, f32)),
    FrameRounding(f32),
    FrameBorderSize(f32),
    ItemSpacing((f32, f32)),
    ItemInnerSpacing((f32, f32)),
    IndentSpacing(f32),
    ScrollbarSize(f32),
    ScrollbarRounding(f32),
    GrabMinSize(f32),
    GrabRounding(f32),
    TabRounding(f32),
    ButtonTextAlign((f32, f32)),
}

#[test]
fn test_style_scaling() {
    let (_guard, mut ctx) = crate::test::test_ctx();
    let style = ctx.style_mut();
    style
        .set_window_padding((1.0, 2.0))
        .set_window_rounding(3.0)
        .set_window_min_size((4.0, 5.0))
        .set_child_rounding(6.0)
        .set_popup_rounding(7.0)
        .set_frame_padding((8.0, 9.0))
        .set_frame_rounding(10.0)
        .set_item_spacing((11.0, 12.0))
        .set_item_inner_spacing((13.0, 14.0))
        .set_touch_extra_padding((15.0, 16.0))
        .set_indent_spacing(17.0)
        .set_columns_min_spacing(18.0)
        .set_scrollbar_size(19.0)
        .set_scrollbar_rounding(20.0)
        .set_grab_min_size(21.0)
        .set_grab_rounding(22.0)
        .set_tab_rounding(23.0)
        .set_display_window_padding((24.0, 25.0))
        .set_display_safe_area_padding((26.0, 27.0))
        .set_mouse_cursor_scale(28.0)
        .scale_all_sizes(2.0);
    assert_eq!(style.window_padding(), (2.0, 4.0));
    assert_eq!(style.window_rounding(), 6.0);
    assert_eq!(style.window_min_size(), (8.0, 10.0));
    assert_eq!(style.child_rounding(), 12.0);
    assert_eq!(style.popup_rounding(), 14.0);
    assert_eq!(style.frame_padding(), (16.0, 18.0));
    assert_eq!(style.frame_rounding(), 20.0);
    assert_eq!(style.item_spacing(), (22.0, 24.0));
    assert_eq!(style.item_inner_spacing(), (26.0, 28.0));
    assert_eq!(style.touch_extra_padding(), (30.0, 32.0));
    assert_eq!(style.indent_spacing(), 34.0);
    assert_eq!(style.columns_min_spacing(), 36.0);
    assert_eq!(style.scrollbar_size(), 38.0);
    assert_eq!(style.scrollbar_rounding(), 40.0);
    assert_eq!(style.grab_min_size(), 42.0);
    assert_eq!(style.grab_rounding(), 44.0);
    assert_eq!(style.tab_rounding(), 46.0);
    assert_eq!(style.display_window_padding(), (48.0, 50.0));
    assert_eq!(style.display_safe_area_padding(), (52.0, 54.0));
    assert_eq!(style.mouse_cursor_scale(), 56.0);
}

#[test]
fn test_style_memory_layout() {
    use std::mem;
    assert_eq!(mem::size_of::<Style>(), mem::size_of::<sys::ImGuiStyle>());
    assert_eq!(mem::align_of::<Style>(), mem::align_of::<sys::ImGuiStyle>());
}

#[test]
fn test_style_color_variants() {
    for (idx, &value) in StyleColor::VARIANTS.iter().enumerate() {
        assert_eq!(idx, value as usize);
    }
}
