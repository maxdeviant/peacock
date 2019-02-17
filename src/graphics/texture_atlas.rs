use std::collections::HashMap;

use crate::graphics::Image;

#[derive(Debug)]
pub struct TextureAtlas {
    textures: HashMap<String, Image>,
}

impl TextureAtlas {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    /// Adds a [`Image`] to the [`TextureAtlas`] under the specified key.
    pub fn add_texture(&mut self, key: &str, texture: Image) {
        self.textures.insert(key.to_string(), texture);
    }

    /// Returns a reference to the [`Image`] stored under the specified key.
    pub fn get_texture(&self, key: &str) -> Option<&Image> {
        self.textures.get(key)
    }
}
