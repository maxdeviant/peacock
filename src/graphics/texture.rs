use sfml::graphics::Texture as SfTexture;

use crate::error::Error;
use crate::Result;

/// A texture.
#[derive(Debug)]
pub struct Texture {
    pub(crate) texture: SfTexture,
}

impl Texture {
    /// Creates a new [`Texture`] from a file.
    pub fn from_file(filename: &str) -> Result<Self> {
        let texture = SfTexture::from_file(filename).ok_or(Error)?;
        Ok(Self { texture })
    }
}
