use std::mem;
use std::ops::Deref;

use glium::Texture2d;
use imgui::{FromImTexture, ImTexture, ImTextureID, IntoImTexture};

/// Handle to a glium texture
///
/// Implements [`Deref`] to get direct access to the underlying [`Texture2d`]
/// object.
pub struct Texture(Texture2d);

impl ImTexture for Texture {
    fn get_id(&self) -> ImTextureID { unsafe { mem::transmute(self) } }
    fn get_size(&self) -> (u32, u32) { self.0.dimensions() }
}

impl IntoImTexture<Texture> for Texture2d {
    fn into_texture(self) -> Texture { Texture(self) }
}

impl Deref for Texture {
    type Target = Texture2d;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl FromImTexture for Texture {
    fn from_id<'a>(texture_id: ImTextureID) -> &'a Self {
        unsafe { mem::transmute::<_, &Texture>(texture_id) }
    }
}
