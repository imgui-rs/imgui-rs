use crate::{Context, DrawData};

pub trait Renderer<T> {
    type Error;
    type Texture;
    fn reload_font_texture(&mut self, ctx: &mut Context) -> Result<(), Self::Error>;
    fn register_texture(&mut self, texture: Self::Texture) -> TextureId;
    fn get_texture(&self, texture_id: TextureId) -> Option<&Self::Texture>;
    fn deregister_texture(&mut self, texture_id: TextureId) -> Option<Self::Texture>;
    fn render_draw_data(&mut self, draw_data: &DrawData, target: &mut T)
        -> Result<(), Self::Error>;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TextureId(usize);

impl TextureId {
    pub fn id(self) -> usize {
        self.0
    }
}

impl From<usize> for TextureId {
    fn from(id: usize) -> Self {
        TextureId(id)
    }
}

impl<T> From<*const T> for TextureId {
    fn from(ptr: *const T) -> Self {
        TextureId(ptr as usize)
    }
}

impl<T> From<*mut T> for TextureId {
    fn from(ptr: *mut T) -> Self {
        TextureId(ptr as usize)
    }
}
