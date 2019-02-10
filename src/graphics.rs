mod animation;
mod color;
mod sprite_batch;
mod texture_atlas;

pub use sfml::graphics::{
    Drawable, IntRect, RenderTarget, Sprite, Texture, Transformable, View, ViewRef,
};

pub use self::animation::*;
pub use self::color::*;
pub use self::sprite_batch::*;
pub use self::texture_atlas::*;

use crate::Context;

/// Clears the screen using the given [`Color`].
pub fn clear(ctx: &mut Context, color: &Color) {
    ctx.window.clear(&color.into());
}

/// Draws a [`Drawable`] object to the current render target.
pub fn draw(ctx: &mut Context, drawable: &Drawable) {
    ctx.window.draw(drawable)
}
