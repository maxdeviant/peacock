use sdl2::image::LoadTexture;
use sdl2::rect::Rect as SdlRect;

use crate::graphics::{AssetRef, Color, Drawable, Rectangle};
use crate::{Context, Result, Vector2f, Vector2u};

#[derive(Debug)]
pub struct Image {
    pub(crate) texture: AssetRef,
}

impl Image {
    pub fn from_file(ctx: &mut Context, filename: &str) -> Result<Self> {
        let texture_creator = ctx.canvas.texture_creator();
        let texture = texture_creator.load_texture(filename).unwrap();

        let texture_ref = AssetRef(ctx.graphics.counter);

        ctx.graphics.counter += 1;

        ctx.graphics.textures.insert(texture_ref, texture);

        Ok(Self {
            texture: texture_ref,
        })
    }

    pub fn from_color(ctx: &mut Context, size: Vector2u, color: Color) -> Result<Self> {
        let texture_creator = ctx.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_target(None, size.x, size.y)
            .unwrap();

        ctx.canvas
            .with_texture_canvas(&mut texture, |texture_canvas| {
                texture_canvas.set_draw_color(color);
                texture_canvas.clear();
            })
            .unwrap();

        let texture_ref = AssetRef(ctx.graphics.counter);

        ctx.graphics.counter += 1;

        ctx.graphics.textures.insert(texture_ref, texture);

        Ok(Self {
            texture: texture_ref,
        })
    }
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

impl Drawable for Image {
    type Params = DrawImageParams;

    fn draw(&self, ctx: &mut Context, params: &DrawImageParams) {
        let texture = ctx.graphics.textures.get(&self.texture).unwrap();
        let texture_query = texture.query();

        let (width, height) = if let Some(clip_rect) = params.clip_rect {
            (clip_rect.width, clip_rect.height)
        } else {
            (texture_query.width as i32, texture_query.height as i32)
        };

        let clip_rect = params.clip_rect.map(|clip_rect| {
            SdlRect::new(
                clip_rect.x,
                clip_rect.y,
                clip_rect.width as u32,
                clip_rect.height as u32,
            )
        });

        ctx.canvas
            .copy(
                &texture,
                clip_rect,
                SdlRect::new(
                    params.position.x as i32,
                    params.position.y as i32,
                    width as u32,
                    height as u32,
                ),
            )
            .expect("Failed to copy texture to canvas");
    }
}
