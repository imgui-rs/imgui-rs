use std::collections::HashMap;

/// An opaque texture identifier
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct TextureId(usize);

impl TextureId {
    /// Creates a new texture id with the given identifier.
    #[inline]
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    /// Returns the id of the TextureId.
    #[inline]
    pub const fn id(self) -> usize {
        self.0
    }
}

impl From<usize> for TextureId {
    #[inline]
    fn from(id: usize) -> Self {
        TextureId(id)
    }
}

impl<T> From<*const T> for TextureId {
    #[inline]
    fn from(ptr: *const T) -> Self {
        TextureId(ptr as usize)
    }
}

impl<T> From<*mut T> for TextureId {
    #[inline]
    fn from(ptr: *mut T) -> Self {
        TextureId(ptr as usize)
    }
}

#[test]
fn test_texture_id_memory_layout() {
    use std::mem;
    assert_eq!(
        mem::size_of::<TextureId>(),
        mem::size_of::<sys::ImTextureID>()
    );
    assert_eq!(
        mem::align_of::<TextureId>(),
        mem::align_of::<sys::ImTextureID>()
    );
}

/// Generic texture mapping for use by renderers.
#[derive(Debug, Default)]
pub struct Textures<T> {
    textures: HashMap<usize, T, NoHashHasherBuilder>,
    next: usize,
}

impl<T> Textures<T> {
    // TODO: once std exposes a `const fn` constructor for HashMap this can be
    // `const fn`
    pub fn new() -> Self {
        Textures {
            textures: Default::default(),
            next: 0,
        }
    }

    pub fn insert(&mut self, texture: T) -> TextureId {
        let id = self.next;
        self.textures.insert(id, texture);
        self.next += 1;
        TextureId::from(id)
    }

    pub fn replace(&mut self, id: TextureId, texture: T) -> Option<T> {
        self.textures.insert(id.0, texture)
    }

    pub fn remove(&mut self, id: TextureId) -> Option<T> {
        self.textures.remove(&id.0)
    }

    pub fn get(&self, id: TextureId) -> Option<&T> {
        self.textures.get(&id.0)
    }

    pub fn get_mut(&mut self, id: TextureId) -> Option<&mut T> {
        self.textures.get_mut(&id.0)
    }
}

#[derive(Default)]
struct NoHashHasherBuilder;
impl std::hash::BuildHasher for NoHashHasherBuilder {
    type Hasher = NoHashHasher;
    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        NoHashHasher(0)
    }
}

struct NoHashHasher(u64);
impl std::hash::Hasher for NoHashHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, _: &[u8]) {
        panic!("This hasher only supports usize.");
    }

    #[inline(always)]
    fn write_usize(&mut self, i: usize) {
        self.0 = i as u64;
    }
}
