mod animation;
mod color;
mod context;
mod font;
mod image;
mod rectangle;
mod text;
mod view;

pub use self::animation::*;
pub use self::color::*;
pub(crate) use self::context::*;
pub use self::font::*;
pub use self::image::*;
pub use self::rectangle::*;
pub use self::text::*;
pub use self::view::*;

use crate::{Context, Result};

pub trait Drawable {
    type Params;

    fn draw(&self, ctx: &mut Context, params: &Self::Params) -> Result<()>;
}

/// Clears the screen using the given [`Color`].
pub fn clear(ctx: &mut Context, color: Color) {
    ctx.canvas.set_draw_color(color);
    ctx.canvas.clear();
}

/// Draws a [`Drawable`] object to the current render target.
pub fn draw<D: Drawable>(ctx: &mut Context, drawable: &D, params: &D::Params) -> Result<()> {
    drawable.draw(ctx, params)
}
