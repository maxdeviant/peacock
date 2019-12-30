mod color;
mod context;
mod image;
mod rectangle;

pub use self::color::*;
pub(crate) use self::context::*;
pub use self::image::*;
pub use self::rectangle::*;

use sdl2::rect::Rect as SdlRect;

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

/// Draws an [`Image`] to the current render target.
pub fn draw_image(ctx: &mut Context, image: &Image, params: DrawImageParams) {
    let texture = ctx.graphics.textures.get(&image.texture).unwrap();
    let texture_query = texture.query();

    let (width, height) = if let Some(clip_rect) = params.clip_rect {
        (clip_rect.width, clip_rect.height)
    } else {
        (texture_query.width as i32, texture_query.height as i32)
    };

    let clip_rect = if let Some(clip_rect) = params.clip_rect {
        Some(SdlRect::new(
            clip_rect.x,
            clip_rect.y,
            clip_rect.width as u32,
            clip_rect.height as u32,
        ))
    } else {
        None
    };

    ctx.canvas.copy(
        &texture,
        clip_rect,
        SdlRect::new(
            params.position.x as i32,
            params.position.y as i32,
            width as u32,
            height as u32,
        ),
    );
}

/// The parameters for drawing [`Text`] to the current render target.
#[derive(Debug, Default)]
pub struct DrawTextParams {
    /// The position at which to draw the [`Text`].
    pub position: Vector2f,
}
