use super::{ImVec2, Image, TextureCache, TextureCacheError, Ui};

use std::cell::RefCell;
use std::ops::Deref;

/// A wrapper around the Ui<'a> to allow it to render custom textures.
pub struct TexturedUi<'ui, 'tc, T: 'tc> {
    ui: &'ui Ui<'ui>,
    texture_cache: RefCell<&'tc mut TextureCache<T>>,
}

impl<'ui, 'tc, T> Deref for TexturedUi<'ui, 'tc, T> {
    type Target = Ui<'ui>;
    fn deref(&self) -> &Ui<'ui> {
        &self.ui
    }
}

impl<'ui, 'tc, T> TexturedUi<'ui, 'tc, T> {
    pub fn init(ui: &'ui Ui<'ui>, texture_cache: &'tc mut TextureCache<T>) -> Self {
        TexturedUi {
            ui,
            texture_cache: RefCell::new(texture_cache),
        }
    }
}

/// # Texture related functions
impl<'ui, 'tc, T> TexturedUi<'ui, 'tc, T> {
    /// Register a texture with the api
    pub fn make_texture(&self, name: String, texture: T) -> Result<(), TextureCacheError> {
        self.texture_cache.borrow_mut().add_texture(name, texture)?;
        Ok(())
    }

    /// Replaces the texture bound to a name
    pub fn replace_texture(&self, name: &str, texture: T) -> Result<(), TextureCacheError> {
        self.texture_cache
            .borrow_mut()
            .replace_texture(name, texture)?;
        Ok(())
    }

    /// Removes a binding between a texture and a name
    pub fn remove_texture(&self, name: &str) -> Result<T, TextureCacheError> {
        let (_, tex) = self.texture_cache.borrow_mut().remove_texture(name)?;
        Ok(tex)
    }
}

/// # Image related functions
impl<'ui, 'tc, T> TexturedUi<'ui, 'tc, T> {
    /// constructs a new image using a previously registered texture
    pub fn image<S>(&self, texture_name: &str, size: S) -> Image<TextureCacheError>
    where
        S: Into<ImVec2>,
    {
        let texture = self
            .texture_cache
            .borrow()
            .retrieve_texture_id(texture_name);
        Image::new(texture, size)
    }
}
