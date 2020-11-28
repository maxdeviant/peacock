use sdl2::rect::Rect as SdlRect;

use crate::error::{AnyhowContext, Sdl2Error};
use crate::graphics::{Color, Drawable, Font};
use crate::vector2::Vector2f;
use crate::{Context, Result};

#[derive(Debug)]
pub struct Text<'a> {
    pub(crate) string: &'a str,
    pub(crate) font: &'a Font,
}

impl<'a> Text<'a> {
    pub fn new(string: &'a str, font: &'a Font) -> Self {
        Self { string, font }
    }
}

/// The parameters for drawing [`Text`] to the current render target.
#[derive(Debug, Default)]
pub struct DrawTextParams {
    /// The position at which to draw the [`Text`].
    pub position: Vector2f,
}

impl<'a> Drawable for Text<'a> {
    type Params = DrawTextParams;

    fn draw(&self, ctx: &mut Context, params: &DrawTextParams) -> Result<()> {
        let texture_creator = ctx.canvas.texture_creator();

        let surface = self
            .font
            .font
            .render(self.string)
            .blended(Color::WHITE)
            .map_err(Sdl2Error::FontError)?;
        let texture = texture_creator.create_texture_from_surface(&surface)?;
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

        unsafe {
            texture.destroy();
        }

        Ok(())
    }
}
