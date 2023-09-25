use crate::sys;
use crate::Ui;

/// # Content region
impl Ui {
    /// Returns the current content boundaries (in *window coordinates*)
    #[doc(alias = "GetContentRegionMax")]
    pub fn content_region_max(&self) -> [f32; 2] {
        unsafe { sys::ImGui_GetContentRegionMax().into() }
    }
    /// Equal to `ui.content_region_max()` - `ui.cursor_pos()`
    #[doc(alias = "GetContentRegionAvail")]
    pub fn content_region_avail(&self) -> [f32; 2] {
        unsafe { sys::ImGui_GetContentRegionAvail().into() }
    }
    /// Content boundaries min (in *window coordinates*).
    ///
    /// Roughly equal to [0.0, 0.0] - scroll.
    #[doc(alias = "GetContentRegionMin")]
    pub fn window_content_region_min(&self) -> [f32; 2] {
        unsafe { sys::ImGui_GetWindowContentRegionMin().into() }
    }
    /// Content boundaries max (in *window coordinates*).
    ///
    /// Roughly equal to [0.0, 0.0] + size - scroll.
    #[doc(alias = "GetContentRegionMax")]
    pub fn window_content_region_max(&self) -> [f32; 2] {
        unsafe { sys::ImGui_GetWindowContentRegionMax().into() }
    }
    #[doc(alias = "GetContentRegionWidth")]
    #[deprecated(
        since = "0.9.0",
        note = "Removed in Dear ImGui 1.85, 'not very useful in practice' and can be done with window_content_region_min/_max"
    )]
    pub fn window_content_region_width(&self) -> f32 {
        self.window_content_region_max()[0] - self.window_content_region_min()[0]
    }
}
