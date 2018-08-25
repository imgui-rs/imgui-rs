use std::collections::hash_map::{Entry, HashMap};
use std::fmt;
use std::fmt::{Display, Formatter};
pub type TextureID = usize;

/// The texture cache provides two core functionalities:
///   - it provides a mapping between String to usizes
///       - these usizes are given to imgui to manage
///   - it provides a mapping between usizes to a generic type T
///       - this type is set by the calling library
///
/// Notes
/// ==========
/// There is a core conflict with the way in which glium and gfx deal with textures:
///   - in gfx, textures are handles to their underlying data, and thus can be cloned
///   - in glium, Texture2D, textures are the owners of their underlying data and can
///     not be cloned
/// Thus to meet the lowest common denominator, we need anything representing textures to
/// be clonable - which probably means using Rc<Texture2D> for glium.
///
/// We also can not just use a list as the `texture_map`, as it would make removing textures
/// impossible. Potentially an Arena structure could be used, but for now we're using a simple
/// approach
pub struct TextureCache<T> {
    /// used to provide unique ids for each texture
    last_id: usize,
    /// maps between strings and usizes - used when constructing textures
    name_map: HashMap<String, TextureID>,
    /// maps between usizes and the underlying texture objects
    texture_map: HashMap<TextureID, T>,
    /// holds the id of the font_texture - used to avoid colissions
    font_texture_id: Option<TextureID>,
}

impl<T> Default for TextureCache<T> {
    fn default() -> Self {
        TextureCache {
            last_id: 0,
            name_map: HashMap::default(),
            texture_map: HashMap::default(),
            font_texture_id: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextureCacheError {
    IDNotFound,
    NameExists,
    NameNotFound,
    FontTextureNotSet,
}
impl Display for TextureCacheError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            &TextureCacheError::IDNotFound => write!(f, "Provided TextureID not found"),
            &TextureCacheError::NameExists => write!(f, "Attempted rebinding of existing name"),
            &TextureCacheError::NameNotFound => write!(f, "Provided name not found"),
            &TextureCacheError::FontTextureNotSet => write!(f, "Font Texture has not set"),
        }
    }
}

impl<T> TextureCache<T> {
    pub fn set_font_texture_id(&mut self, font_texture_id: TextureID) {
        self.font_texture_id = Some(font_texture_id);

        // avoid the colission
        if self.last_id == font_texture_id {
            self.last_id += 1;
        }
    }
}

// Name interface
// =================
// Contains all methods for manipulating the binding between names and textures
impl<T> TextureCache<T> {
    /// Adds a mapping for a texture, returning a unique id that can be used to retrieve the texture
    ///
    /// called by `ui.make_texture("")`
    pub fn add_texture(
        &mut self,
        name: String,
        texture: T,
    ) -> Result<TextureID, TextureCacheError> {
        // we need to know what ids fonts will be using to avoid colissions.
        let font_id = match self.font_texture_id.as_ref() {
            Some(font_id) => *font_id,
            None => return Err(TextureCacheError::FontTextureNotSet),
        };

        let entry = self.name_map.entry(name);
        let new_id = self.last_id;
        match entry {
            Entry::Vacant(vacant_entry) => {
                // first store the mapping from the name to the id
                vacant_entry.insert(new_id);
                // then store the mapping from the id to the texture
                self.texture_map.insert(new_id, texture);

                self.last_id += 1;
                // avoid colission with the font texture
                if self.last_id == font_id {
                    self.last_id += 1;
                }

                Ok(new_id)
            }
            Entry::Occupied(_) => Err(TextureCacheError::NameExists),
        }
    }

    /// Replaces a texture's mapping, which means that subsequent retrievals with the unique id
    /// will return a different texture.
    /// returns the old texture although currently this is not used.
    pub fn replace_texture(
        &mut self,
        name: &str,
        texture: T,
    ) -> Result<(TextureID, T), TextureCacheError> {
        // we need to know what ids fonts will be using to avoid colissions.
        if self.font_texture_id.is_none() {
            return Err(TextureCacheError::FontTextureNotSet);
        }

        if let Some(id) = self.name_map.get(name) {
            if let Entry::Occupied(mut entry) = self.texture_map.entry(*id) {
                let old_texture = entry.insert(texture);
                let id = *id;

                Ok((id, old_texture))
            } else {
                // this id has been retrieved internally, and so should always be valid
                // or rather, it is a semantic error for the texture cache to contain a
                // a name that doesn't bind to a texture
                unreachable!();
            }
        } else {
            Err(TextureCacheError::NameNotFound)
        }
    }

    /// Removes a named texture from the texture cache, also invalidating the texture's id
    pub fn remove_texture(&mut self, name: &str) -> Result<(TextureID, T), TextureCacheError> {
        // we need to know what ids fonts will be using to avoid colissions.
        if self.font_texture_id.is_none() {
            return Err(TextureCacheError::FontTextureNotSet);
        }

        if let Some(id) = self.name_map.remove(name) {
            if let Some(old_texture) = self.texture_map.remove(&id) {
                Ok((id, old_texture))
            } else {
                // this id has been retrieved internally, and so should always be valid
                // or rather, it is a semantic error for the texture cache to contain a
                // a name that doesn't bind to a texture
                unreachable!();
            }
        } else {
            Err(TextureCacheError::NameNotFound)
        }
    }

    /// Returns the id for a texture - called internally by `ui.image("name",...)`
    pub fn retrieve_texture_id(&self, name: &str) -> Result<TextureID, TextureCacheError> {
        self.name_map
            .get(name)
            .map(|id| *id)
            .ok_or(TextureCacheError::NameNotFound)
    }
}

// ID interface
// =================
// Contains all methods for retrieving textures from ids
impl<T> TextureCache<T> {
    /// Retrieves the texture object given an id
    /// Used within the render_draw_list functions - `imgui.texture_cache.retrieve_texture(cmd.texture_id)?`
    pub fn retrieve_texture(&self, id: &TextureID) -> Result<&T, TextureCacheError> {
        // we need to know what ids fonts will be using to avoid colissions.
        if self.font_texture_id.is_none() {
            return Err(TextureCacheError::FontTextureNotSet);
        }

        // Note: Is it worth cloning here - or should the Clone constraint just be removed?
        self.texture_map
            .get(id)
            .ok_or(TextureCacheError::IDNotFound)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn construct_texture_cache() -> TextureCache<i32> {
        let mut cache = TextureCache::<i32>::default();
        cache.set_font_texture_id(0);
        cache
    }

    #[test]
    fn inserting_textures_returns_id() {
        let mut cache = construct_texture_cache();
        let insert_result = cache.add_texture("example_name".into(), 10);
        assert!(insert_result.is_ok());
    }

    #[test]
    fn inserting_textures_returns_unique_id() {
        let mut cache = construct_texture_cache();
        let insert_result = cache.add_texture("example_name".into(), 10);
        let id = insert_result.unwrap();
        let insert_result = cache.add_texture("example_name_2".into(), 10);
        let id2 = insert_result.unwrap();
        assert_ne!(id, id2);
    }

    #[test]
    fn name_colissions_return_err() {
        let mut cache = construct_texture_cache();
        let insert_result = cache.add_texture("example_name".into(), 10);
        let insert_result = cache.add_texture("example_name".into(), 10);
        assert!(insert_result.is_err());
    }

    #[test]
    fn inserted_items_can_be_retrieved_by_id() {
        let mut cache = construct_texture_cache();
        let id = cache.add_texture("example_name".into(), 10).unwrap();
        let internal = cache.retrieve_texture(&id).unwrap();
        assert_eq!(*internal, 10);
    }

    #[test]
    fn inserted_items_id_can_be_retrieved_by_name() {
        let mut cache = construct_texture_cache();
        let id = cache.add_texture("example_name".into(), 10).unwrap();
        let other_id = cache.retrieve_texture_id("example_name").unwrap();
        assert_eq!(other_id, id);
    }

    #[test]
    fn removing_items_returns_old_value() {
        let mut cache = construct_texture_cache();
        cache.add_texture("example_name".into(), 10);
        let (_, value) = cache.remove_texture("example_name").unwrap();
        assert_eq!(value, 10);
    }

    #[test]
    fn retrieving_removed_items_by_id_fails() {
        let mut cache = construct_texture_cache();
        let id = cache.add_texture("example_name".into(), 10).unwrap();
        cache.remove_texture("example_name").unwrap();
        let result = cache.retrieve_texture(&id);
        assert!(result.is_err());
    }

    #[test]
    fn retrieving_removed_item_ids_by_name_fails() {
        let mut cache = construct_texture_cache();
        cache.add_texture("example_name".into(), 10);
        cache.remove_texture("example_name").unwrap();
        let result = cache.retrieve_texture_id("example_name");
        assert!(result.is_err());
    }

    #[test]
    fn replacing_item_updates_value() {
        let mut cache = construct_texture_cache();
        let id = cache.add_texture("example_name".into(), 10).unwrap();
        cache.replace_texture("example_name", 20).unwrap();
        let result = cache.retrieve_texture(&id).unwrap();
        assert_eq!(*result, 20);
    }

}
