use sdl2::rect::Rect as SdlRect;

use crate::error::{AnyhowContext, Sdl2Error};
use crate::graphics::{AssetRef, Color, Drawable, Font};
use crate::vector2::Vector2f;
use crate::{Context, Result};

#[derive(Debug)]
pub struct Text {
    pub(crate) string: String,
    pub(crate) texture: AssetRef,
}

impl Text {
    pub fn new<S: Into<String>>(ctx: &mut Context, string: S, font: &Font) -> Result<Self> {
        let texture_creator = ctx.canvas.texture_creator();

        let string = string.into();
        let surface = font
            .font
            .render(&string)
            .blended(Color::WHITE)
            .map_err(Sdl2Error::FontError)?;
        let texture = texture_creator.create_texture_from_surface(&surface)?;

        let texture_ref = AssetRef(ctx.graphics.counter);

        ctx.graphics.counter += 1;

        ctx.graphics.textures.insert(texture_ref, texture);

        Ok(Self {
            string,
            texture: texture_ref,
        })
    }
}

/// The parameters for drawing [`Text`] to the current render target.
#[derive(Debug, Default)]
pub struct DrawTextParams {
    /// The position at which to draw the [`Text`].
    pub position: Vector2f,
}

impl Drawable for Text {
    type Params = DrawTextParams;

    fn draw(&self, ctx: &mut Context, params: &DrawTextParams) -> Result<()> {
        let texture = ctx.graphics.textures.get(&self.texture).unwrap();
        let texture_query = texture.query();

        ctx.canvas
            .copy(
                &texture,
                None,
                SdlRect::new(
                    params.position.x as i32,
                    params.position.y as i32,
                    texture_query.width as u32,
                    texture_query.height as u32,
                ),
            )
            .map_err(Sdl2Error::ErrorMessage)
            .context("Failed to copy texture to canvas")?;

        Ok(())
    }
}
