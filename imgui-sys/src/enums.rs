/// A color identifier for styling
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImGuiCol {
    Text,
    TextDisabled,
    WindowBg,
    ChildBg,
    PopupBg,
    Border,
    BorderShadow,
    FrameBg,
    FrameBgHovered,
    FrameBgActive,
    TitleBg,
    TitleBgActive,
    TitleBgCollapsed,
    MenuBarBg,
    ScrollbarBg,
    ScrollbarGrab,
    ScrollbarGrabHovered,
    ScrollbarGrabActive,
    CheckMark,
    SliderGrab,
    SliderGrabActive,
    Button,
    ButtonHovered,
    ButtonActive,
    Header,
    HeaderHovered,
    HeaderActive,
    Separator,
    SeparatorHovered,
    SeparatorActive,
    ResizeGrip,
    ResizeGripHovered,
    ResizeGripActive,
    CloseButton,
    CloseButtonHovered,
    CloseButtonActive,
    PlotLines,
    PlotLinesHovered,
    PlotHistogram,
    PlotHistogramHovered,
    TextSelectedBg,
    ModalWindowDarkening,
    DragDropTarget,
}
impl ImGuiCol {
    pub fn values() -> &'static [ImGuiCol] {
        use ImGuiCol::*;
        static values: &'static [ImGuiCol] = &[
            Text,
            TextDisabled,
            WindowBg,
            ChildBg,
            PopupBg,
            Border,
            BorderShadow,
            FrameBg,
            FrameBgHovered,
            FrameBgActive,
            TitleBg,
            TitleBgActive,
            TitleBgCollapsed,
            MenuBarBg,
            ScrollbarBg,
            ScrollbarGrab,
            ScrollbarGrabHovered,
            ScrollbarGrabActive,
            CheckMark,
            SliderGrab,
            SliderGrabActive,
            Button,
            ButtonHovered,
            ButtonActive,
            Header,
            HeaderHovered,
            HeaderActive,
            Separator,
            SeparatorHovered,
            SeparatorActive,
            ResizeGrip,
            ResizeGripHovered,
            ResizeGripActive,
            CloseButton,
            CloseButtonHovered,
            CloseButtonActive,
            PlotLines,
            PlotLinesHovered,
            PlotHistogram,
            PlotHistogramHovered,
            TextSelectedBg,
            ModalWindowDarkening,
            DragDropTarget,
        ];
        values
    }
}
pub const ImGuiCol_COUNT: usize = 43;

/// A variable identifier for styling
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImGuiStyleVar {
    Alpha,
    WindowPadding,
    WindowRounding,
    WindowBorderSize,
    WindowMinSize,
    ChildRounding,
    ChildBorderSize,
    PopupRounding,
    PopupBorderSize,
    FramePadding,
    FrameRounding,
    FrameBorderSize,
    ItemSpacing,
    ItemInnerSpacing,
    IndentSpacing,
    GrabMinSize,
    ButtonTextAlign,
}
pub const ImGuiStyleVar_COUNT: usize = 17;

/// A key identifier (ImGui-side enum)
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImGuiKey {
    Tab,
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
    PageUp,
    PageDown,
    Home,
    End,
    Delete,
    Backspace,
    Enter,
    Escape,
    A,
    C,
    V,
    X,
    Y,
    Z,
}
pub const ImGuiKey_COUNT: usize = 19;

/// A mouse cursor identifier
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImGuiMouseCursor {
    None = -1,
    Arrow,
    TextInput,
    Move,
    ResizeNS,
    ResizeEW,
    ResizeNESW,
    ResizeNWSE,
}
pub const ImGuiMouseCursor_COUNT: usize = 7;
