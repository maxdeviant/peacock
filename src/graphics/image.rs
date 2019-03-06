use sfml::graphics::{Image as SfImage, Texture as SfTexture};

use crate::error::Error;
use crate::graphics::Color;
use crate::{Result, Vector2u};

/// An image.
#[derive(Debug)]
pub struct Image {
    pub(crate) texture: SfTexture,
}

impl Image {
    /// Creates a new [`Image`] from a file.
    pub fn from_file(filename: &str) -> Result<Self> {
        let texture = SfTexture::from_file(filename).ok_or(Error)?;
        Ok(Self { texture })
    }

    /// Creates a new [`Image`] and fills it with the specified [`Color`].
    pub fn from_color(size: Vector2u, color: Color) -> Result<Self> {
        let image = SfImage::from_color(size.x, size.y, &color.into()).ok_or(Error)?;
        let texture = SfTexture::from_image(&image).ok_or(Error)?;
        Ok(Self { texture })
    }

    /// Returns the size of the [`Image`], in pixels.
    pub fn size(&self) -> Vector2u {
        self.texture.size().into()
    }
}
