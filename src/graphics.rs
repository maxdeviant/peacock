mod animation;
mod color;
mod font;
mod image;
mod rectangle;
mod sprite_batch;
mod text;
mod texture_atlas;
mod view;

pub use self::animation::*;
pub use self::color::*;
pub use self::font::*;
pub use self::image::*;
pub use self::rectangle::*;
pub use self::sprite_batch::*;
pub use self::text::*;
pub use self::texture_atlas::*;
pub use self::view::*;

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
    let mut sprite = SfSprite::with_texture(&image.texture);
    sprite.set_position(SfVector2f::from(params.position));

    if let Some(clip_rect) = params.clip_rect {
        sprite.set_texture_rect(&clip_rect.into());
    }

    if let Some(color) = params.color {
        sprite.set_color(&color.into());
    }

    if let Some(scale) = params.scale {
        sprite.set_scale(scale);
    }

    ctx.window.draw_sprite(&sprite, SfRenderStates::default())
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
    let mut text = SfText::new(text.string, &font, text.size);
    text.set_position(SfVector2f::from(params.position));
    ctx.window.draw_text(&text, SfRenderStates::default())
}

/// Draws a [`VertexArray`] to the current render target.
pub(crate) fn draw_vertex_array(ctx: &mut Context, vertex_array: &SfVertexArray, texture: &Image) {
    ctx.window.draw_vertex_array(
        vertex_array,
        SfRenderStates {
            texture: Some(&texture.texture),
            ..Default::default()
        },
    )
}
