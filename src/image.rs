use super::{ImTexture, ImTextureID, ImVec2, ImVec4};

use sys;

/// Represent an image about to be drawn
/// See [`Ui::image`].
///
/// Crate your image using the builder pattern then [`Image::build`] it.
pub struct Image {
    texture_id: ImTextureID,
    size: ImVec2,
    uv0: ImVec2,
    uv1: ImVec2,
    tint_col: ImVec4,
    border_col: ImVec4,
}

impl Image {
    pub fn new<T, S>(texture: &T, size: S) -> Image
    where
        T: ImTexture,
        S: Into<ImVec2>,
    {
        const DEFAULT_UV0: ImVec2 = ImVec2 { x: 0.0, y: 0.0 };
        const DEFAULT_UV1: ImVec2 = ImVec2 { x: 1.0, y: 1.0 };
        const DEFAULT_TINT_COL: ImVec4 = ImVec4 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        };
        const DEFAULT_BORDER_COL: ImVec4 = ImVec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        Image {
            texture_id: texture.get_id(),
            size: size.into(),
            uv0: DEFAULT_UV0,
            uv1: DEFAULT_UV1,
            tint_col: DEFAULT_TINT_COL,
            border_col: DEFAULT_BORDER_COL,
        }
    }

    /// Set uv0 (default `[0.0, 0.0]`)
    pub fn uv0<T: Into<ImVec2>>(mut self, uv0: T) -> Self {
        self.uv0 = uv0.into();
        self
    }

    /// Set uv1 (default `[1.0, 1.0]`)
    pub fn uv1<T: Into<ImVec2>>(mut self, uv1: T) -> Self {
        self.uv1 = uv1.into();
        self
    }

    /// Set tint color (default: no tint color)
    pub fn tint_col<T: Into<ImVec4>>(mut self, tint_col: T) -> Self {
        self.tint_col = tint_col.into();
        self
    }

    /// Set border color (default: no border)
    pub fn border_col<T: Into<ImVec4>>(mut self, border_col: T) -> Self {
        self.border_col = border_col.into();
        self
    }

    /// Draw image where the cursor currently is
    pub fn build(self) {
        unsafe {
            sys::igImage(
                self.texture_id,
                self.size,
                self.uv0,
                self.uv1,
                self.tint_col,
                self.border_col,
            );
        }
    }
}
