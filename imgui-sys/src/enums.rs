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
