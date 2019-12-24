use crate::graphics::{self, Color, Drawable, Image, Rectangle};
use crate::{Context, Vector2f};

#[derive(Debug)]
pub struct DrawSpriteParams {
    pub position: Vector2f,

    pub clip_rect: Rectangle<i32>,

    pub color: Option<Color>,
}

pub struct SpriteBatch<'t> {
    texture: &'t Image,
    vertices: SfVertexArray,
    sprite_count: usize,
}

impl<'t> SpriteBatch<'t> {
    /// The initial size of the vertex array, in batch units.
    const INITIAL_VERTEX_ARRAY_SIZE: usize = 256;

    pub fn new(texture: &'t Image) -> Self {
        let vertices = SfVertexArray::new(
            SfPrimitiveType::Quads,
            SpriteBatch::INITIAL_VERTEX_ARRAY_SIZE * 4,
        );

        Self {
            texture,
            vertices,
            sprite_count: 0,
        }
    }

    pub fn draw_sprite(&mut self, params: DrawSpriteParams) {
        let index = self.sprite_count * 4;

        if index >= self.vertices.vertex_count() {
            self.vertices.resize(self.vertices.vertex_count() + 4);
        }

        let texture_rect = params.clip_rect;
        let size = Vector2f::new(texture_rect.width as f32, texture_rect.height as f32);

        self.vertices[index + 0].position = params.position.into();
        self.vertices[index + 1].position = (params.position + Vector2f::new(size.x, 0.0)).into();
        self.vertices[index + 2].position = (params.position + size).into();
        self.vertices[index + 3].position = (params.position + Vector2f::new(0.0, size.y)).into();

        let position = Vector2f::new(texture_rect.x as f32, texture_rect.y as f32);

        self.vertices[index + 0].tex_coords = position.into();
        self.vertices[index + 1].tex_coords = (position + Vector2f::new(size.x, 0.0)).into();
        self.vertices[index + 2].tex_coords = (position + size).into();
        self.vertices[index + 3].tex_coords = (position + Vector2f::new(0.0, size.y)).into();

        let color: SfColor = params.color.unwrap_or(Color::WHITE).into();

        self.vertices[index + 0].color = color;
        self.vertices[index + 1].color = color;
        self.vertices[index + 2].color = color;
        self.vertices[index + 3].color = color;

        self.sprite_count += 1;
    }
}

impl<'t> Drawable for SpriteBatch<'t> {
    fn draw(&self, ctx: &mut Context) {
        graphics::draw_vertex_array(ctx, &self.vertices, self.texture)
    }
}
