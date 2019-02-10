use peacock::error::Result;
use peacock::graphics::{self, Color, IntRect, Sprite, SpriteBatch, Texture, Transformable};
use peacock::time;
use peacock::window;
use peacock::{Context, ContextBuilder, State};

struct GameState {
    sprite_sheet: Texture,
}

impl GameState {
    fn new() -> Self {
        let sprite_sheet = Texture::from_file("examples/res/0x72_dungeon_ii.png")
            .expect("Could not load sprite sheet!");

        Self { sprite_sheet }
    }
}

impl State for GameState {
    fn update(&mut self, _ctx: &mut Context) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> Result<()> {
        let mut sprite_batch = SpriteBatch::new(&self.sprite_sheet);

        for x in 0..100 {
            for y in 0..100 {
                let mut sprite = Sprite::new();
                sprite.set_position((x as f32 * 16.0, y as f32 * 16.0));
                sprite.set_texture_rect(&IntRect::new(131, 173, 14, 19));
                sprite_batch.draw_sprite(&sprite);
            }
        }

        graphics::draw(ctx, &sprite_batch);

        window::set_title(
            ctx,
            &format!("Sprite Batching - {:.0} FPS", time::get_fps(ctx)),
        );

        Ok(())
    }
}

fn main() -> Result<()> {
    ContextBuilder::new("Sprite Batching", 1920, 1080)
        .build()?
        .run(&mut GameState::new())
}
