use std::collections::HashMap;

use sdl2::render::Texture as SdlTexture;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct AssetRef(pub(crate) u32);

pub(crate) struct GraphicsContext {
    pub(crate) textures: HashMap<AssetRef, SdlTexture>,
    pub(crate) counter: u32,
}

impl GraphicsContext {
    pub(crate) fn new() -> Self {
        Self {
            textures: HashMap::new(),
            counter: 0
        }
    }
}
