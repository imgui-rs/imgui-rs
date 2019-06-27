/// A primary data type
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImGuiDataType {
    /// `i32` (C data type `int`)
    S32,
    /// `u32` (C data type `unsigned int`)
    U32,
    /// `i64` (C data type `long long`, `__int64`)
    S64,
    /// `u64` (C data type `unsigned long long`, `unsigned __int64`)
    U64,
    /// `f32` (C data type `float`)
    Float,
    /// `f64` (C data type `double`)
    Double,
}
impl ImGuiDataType {
    /// All possible `ImGuiDataType` variants
    pub const VARIANTS: [ImGuiDataType; 6] = [
        ImGuiDataType::S32,
        ImGuiDataType::U32,
        ImGuiDataType::S64,
        ImGuiDataType::U64,
        ImGuiDataType::Float,
        ImGuiDataType::Double,
    ];
}

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
    Insert,
    Delete,
    Backspace,
    Space,
    Enter,
    Escape,
    /// for text edit CTRL+A: select all
    A,
    /// for text edit CTRL+C: copy
    C,
    /// for text edit CTRL+V: paste
    V,
    /// for text edit CTRL+X: cut
    X,
    /// for text edit CTRL+Y: redo
    Y,
    /// for text edit CTRL+Z: undo
    Z,
}
impl ImGuiKey {
    /// All possible `ImGuiKey` variants
    pub const VARIANTS: [ImGuiKey; 21] = [
        ImGuiKey::Tab,
        ImGuiKey::LeftArrow,
        ImGuiKey::RightArrow,
        ImGuiKey::UpArrow,
        ImGuiKey::DownArrow,
        ImGuiKey::PageUp,
        ImGuiKey::PageDown,
        ImGuiKey::Home,
        ImGuiKey::End,
        ImGuiKey::Insert,
        ImGuiKey::Delete,
        ImGuiKey::Backspace,
        ImGuiKey::Space,
        ImGuiKey::Enter,
        ImGuiKey::Escape,
        ImGuiKey::A,
        ImGuiKey::C,
        ImGuiKey::V,
        ImGuiKey::X,
        ImGuiKey::Y,
        ImGuiKey::Z,
    ];
    pub const COUNT: usize = 21;
}

/// A mouse cursor identifier
///
/// User code may request binding to display given cursor, which is why we have some cursors that
/// are marked unused here
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImGuiMouseCursor {
    None = -1,
    Arrow = 0,
    /// When hovering over InputText, etc.
    TextInput,
    /// (Unused by imgui functions)
    ResizeAll,
    /// When hovering over an horizontal border
    ResizeNS,
    /// When hovering over a vertical border or a column
    ResizeEW,
    /// When hovering over the bottom-left corner of a window
    ResizeNESW,
    /// When hovering over the bottom-right corner of a window
    ResizeNWSE,
    /// (Unused by imgui functions. Use for e.g. hyperlinks)
    Hand,
}
impl ImGuiMouseCursor {
    /// All possible `ImGuiMouseCursor` variants, except None
    pub const VARIANTS: [ImGuiMouseCursor; 8] = [
        // None variant intentionally skipped
        ImGuiMouseCursor::Arrow,
        ImGuiMouseCursor::TextInput,
        ImGuiMouseCursor::ResizeAll,
        ImGuiMouseCursor::ResizeNS,
        ImGuiMouseCursor::ResizeEW,
        ImGuiMouseCursor::ResizeNESW,
        ImGuiMouseCursor::ResizeNWSE,
        ImGuiMouseCursor::Hand,
    ];
}
