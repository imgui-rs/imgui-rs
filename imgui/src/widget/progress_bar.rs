use std::ptr;

use crate::string::ImStr;
use crate::sys;
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
pub struct ProgressBar<'a> {
    fraction: f32,
    size: [f32; 2],
    overlay_text: Option<&'a ImStr>,
}

impl<'a> ProgressBar<'a> {
    /// Creates a progress bar with a given fraction showing
    /// the progress (0.0 = 0%, 1.0 = 100%).
    ///
    /// The progress bar will be automatically sized to fill the entire width of the window if no
    /// custom size is specified.
    #[inline]
    #[doc(alias = "ProgressBar")]
    pub const fn new(fraction: f32) -> ProgressBar<'a> {
        ProgressBar {
            fraction,
            size: [-1.0, 0.0],
            overlay_text: None,
        }
    }

    /// Sets an optional text that will be drawn over the progress bar.
    #[inline]
    pub const fn overlay_text(mut self, overlay_text: &'a ImStr) -> ProgressBar {
        self.overlay_text = Some(overlay_text);
        self
    }

    /// Sets the size of the progress bar.
    ///
    /// Negative values will automatically align to the end of the axis, zero will let the progress
    /// bar choose a size, and positive values will use the given size.
    #[inline]
    pub const fn size(mut self, size: [f32; 2]) -> Self {
        self.size = size;
        self
    }

    /// Builds the progress bar
    pub fn build(self, _: &Ui) {
        unsafe {
            sys::igProgressBar(
                self.fraction,
                self.size.into(),
                self.overlay_text.map(|x| x.as_ptr()).unwrap_or(ptr::null()),
            );
        }
    }
}
