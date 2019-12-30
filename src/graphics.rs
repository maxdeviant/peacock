mod color;
mod rectangle;

pub use self::color::*;
pub use self::rectangle::*;

use crate::{Context, Vector2f};

pub trait Drawable {
    fn draw(&self, context: &mut Context);
}

/// Clears the screen using the given [`Color`].
pub fn clear(ctx: &mut Context, color: Color) {
    ctx.canvas.set_draw_color(color);
    ctx.canvas.clear();
}

/// Draws a [`Drawable`] object to the current render target.
pub fn draw(ctx: &mut Context, drawable: &Drawable) {
    drawable.draw(ctx)
}

/// The parameters for drawing an [`Image`] to the current render target.
#[derive(Debug)]
pub struct DrawImageParams {
    /// The position at which to draw the [`Image`].
    pub position: Vector2f,

    pub clip_rect: Option<Rectangle<i32>>,

    pub color: Option<Color>,

    pub scale: Option<Vector2f>,
}

impl Default for DrawImageParams {
    fn default() -> Self {
        Self {
            position: Vector2f::ZERO,
            clip_rect: None,
            color: None,
            scale: None,
        }
    }
}

/// The parameters for drawing [`Text`] to the current render target.
#[derive(Debug, Default)]
pub struct DrawTextParams {
    /// The position at which to draw the [`Text`].
    pub position: Vector2f,
}
