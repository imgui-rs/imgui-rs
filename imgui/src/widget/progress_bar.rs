use crate::sys;
// use crate::ImStr;
use crate::Ui;

/// Builder for a progress bar widget.
///
/// # Examples
///
/// ```no_run
/// # use imgui::*;
/// # let mut imgui = Context::create();
/// # let ui = imgui.frame();
/// ProgressBar::new(0.6)
///     .size([100.0, 12.0])
///     .overlay_text(im_str!("Progress!"))
///     .build(&ui);
/// ```
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct ProgressBar<T> {
    fraction: f32,
    size: [f32; 2],
    overlay_text: Option<T>,
}

impl ProgressBar<&'static str> {
    /// Creates a progress bar with a given fraction showing
    /// the progress (0.0 = 0%, 1.0 = 100%).
    ///
    /// The progress bar will be automatically sized to fill the entire width of the window if no
    /// custom size is specified.
    #[inline]
    #[doc(alias = "ProgressBar")]
    pub fn new(fraction: f32) -> Self {
        ProgressBar {
            fraction,
            size: [-1.0, 0.0],
            overlay_text: None,
        }
    }
}

impl<T: AsRef<str>> ProgressBar<T> {
    /// Creates a progress bar with a given fraction showing
    /// the progress (0.0 = 0%, 1.0 = 100%).
    ///
    /// The progress bar will be automatically sized to fill the entire width of the window if no
    /// custom size is specified.
    #[inline]
    #[doc(alias = "ProgressBar")]
    pub fn new_with_overlay(fraction: f32, overlay_text: T) -> Self {
        ProgressBar {
            fraction,
            size: [-1.0, 0.0],
            overlay_text: Some(overlay_text),
        }
    }

    /// Sets the size of the progress bar.
    ///
    /// Negative values will automatically align to the end of the axis, zero will let the progress
    /// bar choose a size, and positive values will use the given size.
    #[inline]
    pub fn size(mut self, size: [f32; 2]) -> Self {
        self.size = size;
        self
    }

    /// Builds the progress bar
    pub fn build(self, ui: &Ui) {
        unsafe {
            sys::igProgressBar(
                self.fraction,
                self.size.into(),
                ui.scratch_txt_opt(self.overlay_text),
            );
        }
    }
}
