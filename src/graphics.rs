mod animation;
mod color;
mod font;
mod sprite_batch;
mod text;
mod texture_atlas;

pub use sfml::graphics::{IntRect, Sprite, Texture, Transformable, VertexArray, View, ViewRef};

pub use self::animation::*;
pub use self::color::*;
pub use self::font::*;
pub use self::sprite_batch::*;
pub use self::text::*;
pub use self::texture_atlas::*;

use sfml::graphics::{
    Color as SfColor, Font as SfFont, RenderStates as SfRenderStates, RenderTarget, Text as SfText,
    Transform as SfTransform,
};

use crate::{Context, Vector2f};

pub trait Drawable {
    fn draw(&self, context: &mut Context);
}

/// Clears the screen using the given [`Color`].
pub fn clear(ctx: &mut Context, color: Color) {
    ctx.window.clear(&SfColor::from(color));
}

/// Draws a [`Drawable`] object to the current render target.
pub fn draw(ctx: &mut Context, drawable: &Drawable) {
    drawable.draw(ctx)
}

/// Draws a [`Sprite`] to the current render target.
pub fn draw_sprite(ctx: &mut Context, sprite: &Sprite) {
    ctx.window.draw_sprite(sprite, SfRenderStates::default())
}

/// The parameters for drawing [`Text`] to the current render target.
#[derive(Debug, Default)]
pub struct DrawTextParams {
    /// The position at which to draw the [`Text`].
    pub position: Vector2f,
}

/// Draws some [`Text`] to the current render target.
pub fn draw_text(ctx: &mut Context, text: &Text, params: DrawTextParams) {
    let font: SfFont = text.font.into();
    let text = SfText::new(text.string, &font, text.size);
    let mut transform = SfTransform::IDENTITY;
    transform.translate(params.position.x, params.position.y);
    ctx.window.draw_text(
        &text,
        SfRenderStates {
            transform,
            ..Default::default()
        },
    )
}

/// Draws a [`VertexArray`] to the current render target.
pub(crate) fn draw_vertex_array(ctx: &mut Context, vertex_array: &VertexArray, texture: &Texture) {
    ctx.window.draw_vertex_array(
        vertex_array,
        SfRenderStates {
            texture: Some(&texture),
            ..Default::default()
        },
    )
}
