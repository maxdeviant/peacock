mod sprite_batch;
mod texture_atlas;

pub use sfml::graphics::{Color, IntRect, Texture};

pub use self::sprite_batch::*;
pub use self::texture_atlas::*;

use sfml::graphics::RenderTarget;

use crate::Context;

/// Clears the screen using the given [`Color`].
pub fn clear(ctx: &mut Context, color: &Color) {
    ctx.window.clear(color);
}
