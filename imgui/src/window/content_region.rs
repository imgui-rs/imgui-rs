use crate::sys;
use crate::Ui;

/// # Content region
impl Ui {
    /// Equal to `ui.content_region_max()` - `ui.cursor_pos()`
    #[doc(alias = "GetContentRegionAvail")]
    pub fn content_region_avail(&self) -> [f32; 2] {
        let mut out = sys::ImVec2::zero();
        unsafe { sys::igGetContentRegionAvail(&mut out) };
        out.into()
    }
}
