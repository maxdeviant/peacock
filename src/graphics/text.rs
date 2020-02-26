use sdl2::rect::Rect as SdlRect;

use crate::graphics::{Color, Drawable, Font};
use crate::vector2::Vector2f;
use crate::Context;

#[derive(Debug)]
pub struct Text<'a> {
    pub(crate) string: &'a str,
    pub(crate) font: &'a Font,
    pub(crate) size: u32,
}

impl<'a> Text<'a> {
    pub fn new(string: &'a str, font: &'a Font, size: u32) -> Self {
        Self { string, font, size }
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

    fn draw(&self, ctx: &mut Context, params: &DrawTextParams) {
        let texture_creator = ctx.canvas.texture_creator();

        let surface = self
            .font
            .font
            .render(self.string)
            .blended(Color::WHITE)
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
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
            .expect("Failed to copy texture to canvas");
    }
}
