use std::marker::PhantomData;
use std::os::raw::c_void;

use crate::render::renderer::TextureId;
use crate::sys;
use crate::Ui;

/// Represent an image about to be drawn.
/// See [`Ui::image`].
///
/// Create your image using the builder pattern then [`Image::build`] it.
#[must_use]
pub struct Image<'ui> {
    texture_id: TextureId,
    size: [f32; 2],
    uv0: [f32; 2],
    uv1: [f32; 2],
    tint_col: [f32; 4],
    border_col: [f32; 4],
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui> Image<'ui> {
    pub fn new(_: &Ui<'ui>, texture_id: TextureId, size: [f32; 2]) -> Self {
        const DEFAULT_UV0: [f32; 2] = [0.0, 0.0];
        const DEFAULT_UV1: [f32; 2] = [1.0, 1.0];
        const DEFAULT_TINT_COL: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const DEFAULT_BORDER_COL: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        Image {
            texture_id,
            size,
            uv0: DEFAULT_UV0,
            uv1: DEFAULT_UV1,
            tint_col: DEFAULT_TINT_COL,
            border_col: DEFAULT_BORDER_COL,
            _phantom: PhantomData,
        }
    }
    /// Set size (default based on texture)
    pub fn size(mut self, size: [f32; 2]) -> Self {
        self.size = size;
        self
    }
    /// Set uv0 (default `[0.0, 0.0]`)
    pub fn uv0(mut self, uv0: [f32; 2]) -> Self {
        self.uv0 = uv0;
        self
    }
    /// Set uv1 (default `[1.0, 1.0]`)
    pub fn uv1(mut self, uv1: [f32; 2]) -> Self {
        self.uv1 = uv1;
        self
    }
    /// Set tint color (default: no tint color)
    pub fn tint_col(mut self, tint_col: [f32; 4]) -> Self {
        self.tint_col = tint_col;
        self
    }
    /// Set border color (default: no border)
    pub fn border_col(mut self, border_col: [f32; 4]) -> Self {
        self.border_col = border_col;
        self
    }
    /// Draw image where the cursor currently is
    pub fn build(self) {
        unsafe {
            sys::igImage(
                self.texture_id.id() as *mut c_void,
                self.size.into(),
                self.uv0.into(),
                self.uv1.into(),
                self.tint_col.into(),
                self.border_col.into(),
            );
        }
    }
}

/// Represent an image button about to be drawn.
/// See [`Ui::image_button`].
///
/// Create your image button using the builder pattern then [`ImageButton::build`] it.
#[must_use]
pub struct ImageButton<'ui> {
    texture_id: TextureId,
    size: [f32; 2],
    uv0: [f32; 2],
    uv1: [f32; 2],
    frame_padding: i32,
    bg_col: [f32; 4],
    tint_col: [f32; 4],
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui> ImageButton<'ui> {
    pub fn new(_: &Ui<'ui>, texture_id: TextureId, size: [f32; 2]) -> Self {
        const DEFAULT_UV0: [f32; 2] = [0.0, 0.0];
        const DEFAULT_UV1: [f32; 2] = [1.0, 1.0];
        const DEFAULT_FRAME_PADDING: i32 = -1;
        const DEFAULT_BG_COL: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        const DEFAULT_TINT_COL: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        ImageButton {
            texture_id,
            size: size,
            uv0: DEFAULT_UV0,
            uv1: DEFAULT_UV1,
            frame_padding: DEFAULT_FRAME_PADDING,
            bg_col: DEFAULT_BG_COL,
            tint_col: DEFAULT_TINT_COL,
            _phantom: PhantomData,
        }
    }
    /// Set size (default based on texture)
    pub fn size(mut self, size: [f32; 2]) -> Self {
        self.size = size;
        self
    }
    /// Set uv0 (default `[0.0, 0.0]`)
    pub fn uv0(mut self, uv0: [f32; 2]) -> Self {
        self.uv0 = uv0;
        self
    }
    /// Set uv1 (default `[1.0, 1.0]`)
    pub fn uv1(mut self, uv1: [f32; 2]) -> Self {
        self.uv1 = uv1;
        self
    }
    /// Set frame padding (default: uses frame padding from style).
    /// frame_padding < 0: uses frame padding from style (default)
    /// frame_padding = 0: no framing
    /// frame_padding > 0: set framing size
    pub fn frame_padding(mut self, frame_padding: i32) -> Self {
        self.frame_padding = frame_padding.into();
        self
    }
    /// Set tint color (default: no tint color)
    pub fn tint_col(mut self, tint_col: [f32; 4]) -> Self {
        self.tint_col = tint_col;
        self
    }
    /// Set background color (default: no background color)
    pub fn background_col(mut self, bg_col: [f32; 4]) -> Self {
        self.bg_col = bg_col;
        self
    }
    /// Draw image button where the cursor currently is
    pub fn build(self) -> bool {
        unsafe {
            sys::igImageButton(
                self.texture_id.id() as *mut c_void,
                self.size.into(),
                self.uv0.into(),
                self.uv1.into(),
                self.frame_padding,
                self.bg_col.into(),
                self.tint_col.into(),
            )
        }
    }
}
