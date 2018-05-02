use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::Hasher;
use std::ops::Deref;
use std::rc::Rc;

use super::{ImStr, ImTextureID};

/// Trait that an object representing a texture usable by the drawing back-end
/// should implement.
pub trait ImTexture {
    /// Get [`ImTextureID`]
    fn get_id(&self) -> ImTextureID;
    /// Query texture size, in pixels (for convenience)
    fn get_size(&self) -> (u32, u32);
}

/// A handle to a texture.
///
/// Wraps a fat pointer to an trait object [`ImTexture`].
//
//  `Rc` is necessary to make `Box<ImTexture>` clonable.
//  We need to clone it to extract its value from a `TextureCache`, an type
//  internal to imgui-rs, which internally contains a `RefCell`.
#[derive(Clone)]
pub struct AnyTexture(Rc<Box<ImTexture>>);

impl AnyTexture {
    /// Create a new [`AnyTexture`] from an object implementing the [`ImTexture`] trait.
    fn new<T: 'static + ImTexture>(texture: T) -> Self { AnyTexture(Rc::new(Box::new(texture))) }
}

impl ImTexture for AnyTexture {
    fn get_id(&self) -> ImTextureID { self.deref().get_id() }
    fn get_size(&self) -> (u32, u32) { self.deref().get_size() }
}

/// Allows to directly use the methods implemnted on [`AnyTexture`]
impl Deref for AnyTexture {
    type Target = Box<ImTexture>;

    fn deref(&self) -> &Self::Target { Deref::deref(&self.0) }
}

/// Trait defining how an external type can be converted into a type implementing
/// [`ImTexture`].
///
/// Typically implemented to convert a native type (e.g. Texture2d from the
/// glium crate) to a type defined in the back-end implemnting [`ImTexture`].
pub trait IntoImTexture<T>
where
    T: ImTexture,
{
    fn into_texture(self) -> T;
}

/// Trait defining how an object implementing [`ImTexture`] should be converted
/// back to the native texture type used by the back-end.
pub trait FromImTexture {
    fn from_im_texture<T: ImTexture>(texture: &T) -> &Self {
        let texture = texture.get_id();
        Self::from_id(texture)
    }
    fn from_id<'a>(texture_id: ImTextureID) -> &'a Self;
}

/// Owns all the custom textures used by ImGui.
///
/// Use interior mutability to register or delete textures.
pub struct TextureCache(RefCell<BTreeMap<u64, AnyTexture>>);

impl TextureCache {
    pub fn new() -> Self { TextureCache(RefCell::new(BTreeMap::new())) }

    /// Register the texture inside the cache.
    ///
    /// Swap and return some texture if another texture with the same name was
    /// already registered.
    pub fn register_texture<T>(&self, name: &ImStr, texture: T) -> Option<AnyTexture>
    where
        T: 'static + ImTexture,
    {
        let id = hash_imstring(name);
        self.0.borrow_mut().insert(id, AnyTexture::new(texture))
    }

    /// Get the texture in the cache with the given name, if it exists.
    pub fn get_texture(&self, name: &ImStr) -> Option<AnyTexture> {
        let id = hash_imstring(name);
        self.0.borrow().get(&id).map(Clone::clone)
    }
}

/// Used to compute the ID of a texture
fn hash_imstring(string: &ImStr) -> u64 {
    let mut h = DefaultHasher::new();
    h.write(string.to_str().as_bytes());
    h.finish()
}
