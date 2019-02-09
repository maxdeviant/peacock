use std::ops::Add;

use sfml::graphics::{
    Drawable, PrimitiveType, RenderStates, RenderTarget, Sprite, Texture, Transformable,
    VertexArray,
};
use sfml::system::Vector2f;

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

        self.vertices[index + 0].position = sprite.position();
        self.vertices[index + 1].position = sprite.position().add(Vector2f::new(8.0, 0.0));
        self.vertices[index + 2].position = sprite.position().add(Vector2f::new(8.0, 8.0));
        self.vertices[index + 3].position = sprite.position().add(Vector2f::new(0.0, 8.0));

        let texture_rect = sprite.texture_rect();
        let position = Vector2f::new(texture_rect.left as f32, texture_rect.top as f32);
        let size = Vector2f::new(texture_rect.width as f32, texture_rect.height as f32);

        self.vertices[index + 0].tex_coords = position;
        self.vertices[index + 1].tex_coords = position.add(Vector2f::new(size.x, 0.0));
        self.vertices[index + 2].tex_coords = position.add(size);
        self.vertices[index + 3].tex_coords = position.add(Vector2f::new(0.0, size.y));

        self.vertices[index + 0].color = sprite.color();
        self.vertices[index + 1].color = sprite.color();
        self.vertices[index + 2].color = sprite.color();
        self.vertices[index + 3].color = sprite.color();

        self.sprite_count += 1;
    }
}

impl<'t> Drawable for SpriteBatch<'t> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut RenderTarget,
        states: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let render_states = RenderStates {
            texture: Some(&self.texture),
            ..states
        };
        target.draw_with_renderstates(&self.vertices, render_states);
    }
}
