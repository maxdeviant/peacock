mod animation;
mod color;
mod sprite_batch;
mod texture_atlas;

use sfml::graphics::RenderStates as SfRenderStates;
pub use sfml::graphics::{
    Drawable as SfDrawable, IntRect, RenderTarget, Sprite, Texture, Transformable, VertexArray,
    View, ViewRef,
};

pub use self::animation::*;
pub use self::color::*;
pub use self::sprite_batch::*;
pub use self::texture_atlas::*;

use crate::Context;

pub trait Drawable {
    fn draw(&self, context: &mut Context);
}

/// Clears the screen using the given [`Color`].
pub fn clear(ctx: &mut Context, color: &Color) {
    ctx.window.clear(&color.into());
}

/// Draws a [`Drawable`] object to the current render target.
pub fn draw(ctx: &mut Context, drawable: &Drawable) {
    drawable.draw(ctx)
}

/// Draws a [`Sprite`] to the current render target.
pub fn draw_sprite(ctx: &mut Context, sprite: &Sprite) {
    ctx.window.draw_sprite(sprite, SfRenderStates::default())
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
