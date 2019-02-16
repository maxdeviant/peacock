use sfml::graphics::{PrimitiveType, Sprite, Transformable, VertexArray};
use sfml::system::Vector2f;

use crate::graphics::{self, Drawable, Texture};
use crate::Context;

pub struct SpriteBatch<'t> {
    texture: &'t Texture,
    vertices: VertexArray,
    sprite_count: usize,
}

impl<'t> SpriteBatch<'t> {
    /// The initial size of the vertex array, in batch units.
    const INITIAL_VERTEX_ARRAY_SIZE: usize = 256;

    pub fn new(texture: &'t Texture) -> Self {
        let vertices = VertexArray::new(
            PrimitiveType::Quads,
            SpriteBatch::INITIAL_VERTEX_ARRAY_SIZE * 4,
        );

        Self {
            texture,
            vertices,
            sprite_count: 0,
        }
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite) {
        let index = self.sprite_count * 4;

        if index >= self.vertices.vertex_count() {
            self.vertices.resize(self.vertices.vertex_count() + 4);
        }

        let texture_rect = sprite.texture_rect();
        let size = Vector2f::new(texture_rect.width as f32, texture_rect.height as f32);

        self.vertices[index + 0].position = sprite.position();
        self.vertices[index + 1].position = sprite.position() + Vector2f::new(size.x, 0.0);
        self.vertices[index + 2].position = sprite.position() + size;
        self.vertices[index + 3].position = sprite.position() + Vector2f::new(0.0, size.y);

        let position = Vector2f::new(texture_rect.left as f32, texture_rect.top as f32);

        self.vertices[index + 0].tex_coords = position;
        self.vertices[index + 1].tex_coords = position + Vector2f::new(size.x, 0.0);
        self.vertices[index + 2].tex_coords = position + size;
        self.vertices[index + 3].tex_coords = position + Vector2f::new(0.0, size.y);

        self.vertices[index + 0].color = sprite.color();
        self.vertices[index + 1].color = sprite.color();
        self.vertices[index + 2].color = sprite.color();
        self.vertices[index + 3].color = sprite.color();

        self.sprite_count += 1;
    }
}

impl<'t> Drawable for SpriteBatch<'t> {
    fn draw(&self, ctx: &mut Context) {
        graphics::draw_vertex_array(ctx, &self.vertices, self.texture)
    }
}
