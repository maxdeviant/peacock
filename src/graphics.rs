mod sprite_batch;

pub use sfml::graphics::Color;
use sfml::graphics::RenderTarget;

pub use self::sprite_batch::*;
use crate::Context;

/// Clears the screen using the given [`Color`].
pub fn clear(ctx: &mut Context, color: &Color) {
    ctx.window.clear(color);
}
