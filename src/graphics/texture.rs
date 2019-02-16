use sfml::graphics::{Image as SfImage, Texture as SfTexture};

use crate::error::Error;
use crate::graphics::Color;
use crate::{Result, Vector2u};

/// A texture.
#[derive(Debug)]
pub struct Texture {
    // TODO: This shouldn't be public.
    pub texture: SfTexture,
}

impl Texture {
    /// Creates a new [`Texture`] from a file.
    pub fn from_file(filename: &str) -> Result<Self> {
        let texture = SfTexture::from_file(filename).ok_or(Error)?;
        Ok(Self { texture })
    }

    /// Creates a new [`Texture`] and fills it with the specified [`Color`].
    pub fn from_color(size: Vector2u, color: Color) -> Result<Self> {
        let image = SfImage::from_color(size.x, size.y, &color.into()).ok_or(Error)?;
        let texture = SfTexture::from_image(&image).ok_or(Error)?;
        Ok(Self { texture })
    }
}
