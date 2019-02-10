use std::collections::HashMap;

use crate::graphics::Texture;

#[derive(Debug)]
pub struct TextureAtlas {
    textures: HashMap<String, Texture>,
}

impl TextureAtlas {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    /// Adds a [`Texture`] to the [`TextureAtlas`] under the specified key.
    pub fn add_texture(&mut self, key: &str, texture: Texture) {
        self.textures.insert(key.to_string(), texture);
    }

    /// Returns a reference to the [`Texture`] stored under the specified key.
    pub fn get_texture(&self, key: &str) -> Option<&Texture> {
        self.textures.get(key)
    }
}
