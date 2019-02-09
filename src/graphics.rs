use sfml::graphics::{Color, RenderTarget};

mod sprite_batch;

use crate::Context;

pub use self::sprite_batch::*;

pub fn clear(ctx: &mut Context, color: &Color) {
    ctx.window.clear(color);
}
