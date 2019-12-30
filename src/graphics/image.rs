use sdl2::rect::Point;
use sdl2::render::{TextureCreator, Texture as SdlTexture};

use crate::graphics::{AssetRef, Color};
use crate::{Context, Vector2u, Result};

#[derive(Debug)]
pub struct Image {
    pub(crate) texture: AssetRef,
}

impl Image {
    pub fn from_color(ctx: &mut Context, size: Vector2u, color: Color) -> Result<Self> {
        let texture_creator = ctx.canvas.texture_creator();
        let mut texture = texture_creator.create_texture_target(None, size.x, size.y).unwrap();

        ctx.canvas.with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.set_draw_color(color);
            texture_canvas.clear();
        }).unwrap();

        let texture_ref = AssetRef(ctx.graphics.counter);

        ctx.graphics.counter += 1;

        ctx.graphics.textures.insert(texture_ref, texture);

        Ok(Self { texture: texture_ref })
    }
}
